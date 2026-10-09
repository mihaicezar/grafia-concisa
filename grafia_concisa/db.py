"""Build word_db.sqlite from inflected-form records.

A record is (form, lemma, lemma_hyphenation, number, *flags):
  - lemma_hyphenation: dexonline's hyphenation of the lemma, complete, partial
    or empty; completed by the fallback rules (see rules.syllabify);
  - number: 'P' for plural, anything else singular;
  - flags, in any order (others, '' included, are ignored): 'V' if the lemma is a
    verb; 'D' if it is DEX-MDA-DOOM (a definition of its own in MDA*, DEX*, NODEX*
    or DOOM*), which picks the form an -ee spelling reverts to; 'F' if it is a
    lowercase feminine noun or unclassified (dexonline model F, T or IL), which
    marks a lemma in -iare as a long infinitive (rules.long_infinitive).
A bare string is a syllabified headword (dev/test input).
"""
import os
import sqlite3
from collections import Counter, defaultdict
from collections.abc import Container, Iterable
from pathlib import Path

from .rules import (
    EE_FINAL,
    EE_SPELLING,
    EURI,
    HIATUS_SUFFIX,
    SYLLABIFICATIONS,
    WORD,
    concise_ee,
    concise_from_reference,
    has_candidate,
    iere_spelling,
    long_infinitive,
    normalize,
    revert_text,
    syllabify_all,
)

SCHEMA = """
CREATE TABLE forms (
    form         TEXT PRIMARY KEY,   -- normalized standard spelling
    concise_form TEXT NOT NULL,      -- base rules
    concise_ee   TEXT,               -- with the -ee rule; NULL if identical
    alternatives TEXT                -- other spellings of a homograph, space-separated; NULL if none
) WITHOUT ROWID;
CREATE TABLE eu_lemmas (
    lemma TEXT PRIMARY KEY           -- normalized lemma in -eu, for plurals missing from dexonline
) WITHOUT ROWID;                     -- (bombeele <- bombeu)
CREATE TABLE ea_lemmas (
    lemma TEXT PRIMARY KEY           -- normalized lemma in -ea, for genitives missing from dexonline
) WITHOUT ROWID;                     -- (Andreei <- Andreea)
CREATE TABLE ee_revert (
    concise      TEXT PRIMARY KEY,   -- a spelling with the -ee rule (aleẹ)
    form         TEXT NOT NULL,      -- the standard form it reverts to (alee, not aleie)
    alternatives TEXT                -- the other forms of a tie, space-separated; NULL if none
) WITHOUT ROWID;                     -- (bordeẹ: bordeie or bordee)
CREATE TABLE iesc_lemmas (
    lemma     TEXT PRIMARY KEY,      -- normalized lemma in -iesc, for adjective forms missing from
    reference TEXT NOT NULL          -- dexonline (harabagiasca <- harabagiesc), and its syllabification
) WITHOUT ROWID;                     -- (harabagi-esc)
"""

Record = str | tuple[str, ...]
SOURCE_RANK = {"rules": 0, "partial": 1, "dexonline": 2, "manual": 3}
# Romanian alphabetical order: ă, â after a, î after i, ș after s, ț after t
_RO_ORDER = str.maketrans({c: chr(0x10000 + i) for i, c in enumerate("aăâbcdefghiîjklmnopqrsștțuvwxyz")})


def _clean(value: str | None) -> str:
    return normalize((value or "").strip())


def build(
    path: Path, records: Iterable[Record], known: Container[str] = frozenset()
) -> dict[str, int]:
    """Write the database atomically.

    `known`: all normalized dexonline word forms, for the rules' prefix check.

    Only forms that can differ are stored (ie/ia/ea/oa/iăr, or final -ee/-eea/-eei/-eele/
    -eelor/-euri/-eurile/-eurilor).
    A form of a long infinitive in -iare (flag 'F', rules.long_infinitive) is
    converted as its -iere spelling (apreciare, apreciări -> apreciere, aprecieri),
    unless another record gives the same form (conciliare: the adjective conciliar).
    A form in rules.SYLLABIFICATIONS takes all its spellings from there, whatever
    its lemma (piei: pẹi only, not also piei <- pia).
    All lemmas in -eu and -ea are stored too (tables eu_lemmas, ea_lemmas), for
    the -ee rule on unknown words: plurals in -eele/-eelor (bombeele <- bombeu)
    and genitives in -eei (Andreei <- Andreea). So are the lemmas in -iesc, with
    their syllabification (iesc_lemmas), for the forms of adjectives missing from
    dexonline (harabagiasca <- harabagi-esc).
    Base form: any record beats one of a long infinitive (above); then a record
    backed by an explicit, dexonline or partial syllabification beats one from
    rules alone (agreare <- agrea a-gre-a, not the headword agreare by rules);
    then a record where the form is the lemma itself (its own syllabification)
    beats one derived from another lemma; then an explicit syllabification beats
    dexonline, which beats a partial one; otherwise the first record wins. The
    winner is the default spelling; the others become alternatives (see below).
    -ee form: computed from the final base form; a singular reading beats a plural.

    Alternatives: the other spellings a form gets from its records (a
    homograph: ia = the verb lua or the articulated ie). Only spellings backed
    by an explicit, dexonline or partial syllabification count; one from rules
    alone is likely a rules error, not a second reading. Except: when one
    record gives the -ėză/-ėscă/-ėsca suffix, every spelling counts (liază:
    the noun liază or the verb lia, liėză).

    Reverse -ee (table ee_revert): a spelling with the -ee rule reverts to one of
    the forms that give it, with the rule or unchanged (zeii: the form zeii, or
    zeei by the rule). A DEX-MDA-DOOM form beats one without (alee, not aleie);
    among DEX-MDA-DOOM forms, a plural in -ee beats one in -euri (apogee, not
    apogeuri). The forms still tied are all offered (bordeẹ: bordeie or bordee),
    the default being the character map's result, else the first in Romanian
    alphabetical order. Stored only where the character map alone is not the answer.
    """
    stats = Counter(read=0, invalid=0, no_candidate=0, derived=0, ee_no_rule=0, homographs=0)
    # (lemma, hyphenation, verb) -> [(syllabified, source rank)], one per reading
    references: dict[tuple[str, str, bool], list[tuple[str, int]]] = {}
    # form -> [concise, (own, source rank), -ee readings [(plural, lemma)],
    #          {spelling: backed by more than rules}, DEX-MDA-DOOM]
    rows: dict[str, list] = {}
    # forms with no candidate, spelled like an -ee spelling (zeii) -> DEX-MDA-DOOM
    unchanged: dict[str, bool] = {}
    lemmas: dict[str, set[str]] = {"eu": set(), "ea": set()}  # by ending
    iesc: dict[str, str] = {}  # lemma in -iesc -> syllabified

    for rec in records:
        stats["read"] += 1
        if isinstance(rec, str):
            hyph = _clean(rec)
            form = lemma = hyph.replace("-", "")
            number, flags = "", set()
        else:
            form, lemma, hyph, number = (_clean(x) for x in rec[:4])
            flags = {_clean(x).upper() for x in rec[4:]}

        if not WORD.fullmatch(form) or not WORD.fullmatch(lemma):
            stats["invalid"] += 1
            continue
        verb, dex = "V" in flags, "D" in flags
        for end, found in lemmas.items():
            if lemma.endswith(end):
                found.add(lemma)
        if lemma.endswith("iesc") and lemma not in iesc:
            iesc[lemma] = syllabify_all(lemma, hyph, known, verb)[0][0]
        if not has_candidate(form, ee=True):
            stats["no_candidate"] += 1
            if EE_SPELLING.search(form):
                unchanged[form] = unchanged.get(form, False) or dex
            continue

        spelled, ref = form, (lemma, hyph, verb)  # the spelling to convert, and its lemma
        if form in SYLLABIFICATIONS:  # Cezar's syllabification of the form decides
            ref = (form, "", False)  # the verb flag only matters to the rules
        elif "F" in flags and long_infinitive(lemma):  # apreciare -> apreciere
            # only the first hyphenation variant is used (rules._dexonline_cuts)
            first = hyph.split(",")[0].strip()
            spelled, ref = iere_spelling(form), (iere_spelling(lemma), iere_spelling(first), verb)
            stats["long_infinitive"] += 1
        if ref not in references:
            readings = syllabify_all(*ref[:2], known, verb)
            references[ref] = [(r, SOURCE_RANK[source]) for r, source in readings]
            stats[f"lemma_{readings[0][1]}"] += 1

        for reference, rank in references[ref]:
            # a long infinitive loses to any other record (conciliare: the adjective),
            # and rules alone to a backed syllabification (agreare <- agrea a-gre-a)
            priority = (spelled == form, rank > SOURCE_RANK["rules"], spelled == ref[0], rank)
            concise = concise_from_reference(spelled, reference)
            row = rows.get(form)
            if row is None:
                row = rows[form] = [concise, priority, [], {}, False]
                stats["derived"] += not priority[2]
            elif priority > row[1]:
                stats["derived"] += row[1][2] - priority[2]
                row[0], row[1] = concise, priority
            # a long infinitive's -iere spelling changes letters, not cuts: no alternative
            backed = rank > SOURCE_RANK["rules"] and spelled == form
            row[3][concise] = row[3].get(concise, False) or backed
        row[4] = row[4] or dex

        reading = (number.upper() == "P", lemma)
        if EE_FINAL.search(form) and reading not in row[2]:
            row[2].append(reading)

    out = []
    for form, (concise, _, readings, spellings, _) in rows.items():
        ee = next(
            (e for plural, lemma in sorted(readings, key=lambda r: r[0])
             if (e := concise_ee(form, concise, lemma, plural)) is not None),
            None,
        )
        stats["ee_no_rule"] += bool(readings) and ee is None
        suffix = any(HIATUS_SUFFIX.search(x) for x in spellings)
        alts = [x for x, backed in spellings.items() if (backed or suffix) and x != concise]
        stats["homographs"] += bool(alts)
        out.append((form, concise, ee if ee != concise else None, " ".join(alts) or None))

    by_spelling = defaultdict(list)  # spelling with the -ee rule -> [(form, -ee rule applied)]
    dex = {form: row[4] for form, row in rows.items()} | unchanged
    for form, concise, ee, _ in out:
        by_spelling[ee or concise].append((form, ee is not None))
    for form in unchanged:
        by_spelling[form].append((form, False))
    reverse = []
    for spelling, forms in by_spelling.items():
        if any(changed for _, changed in forms):
            plain = revert_text(spelling)
            rank = {f: (dex[f], dex[f] and not EURI.search(f)) for f, _ in forms}
            top = max(rank.values())
            tied = sorted((f for f in rank if rank[f] == top), key=lambda f: f.translate(_RO_ORDER))
            best = plain if plain in tied else tied[0]
            alts = [f for f in tied if f != best]
            if best != plain or alts:
                reverse.append((spelling, best, " ".join(alts) or None))

    path = Path(path)
    tmp = path.with_name(path.name + ".tmp")
    tmp.unlink(missing_ok=True)
    conn = sqlite3.connect(tmp)
    try:
        conn.executescript(SCHEMA)
        conn.executemany(
            "INSERT INTO forms VALUES (?, ?, ?, ?)",
            out,
        )
        conn.executemany("INSERT INTO ee_revert VALUES (?, ?, ?)", reverse)
        for end, found in lemmas.items():
            conn.executemany(f"INSERT INTO {end}_lemmas VALUES (?)", ((x,) for x in sorted(found)))
        conn.executemany("INSERT INTO iesc_lemmas VALUES (?, ?)", sorted(iesc.items()))
        conn.commit()
        conn.execute("VACUUM")
    finally:
        conn.close()
    os.replace(tmp, path)
    stats["stored"] = len(rows)
    stats["ee"] = sum(1 for _, _, e, _ in out if e)
    stats["ee_revert"] = len(reverse)
    stats["ee_revert_ties"] = sum(1 for *_, a in reverse if a)
    for end, found in lemmas.items():
        stats[f"{end}_lemmas"] = len(found)
    stats["iesc_lemmas"] = len(iesc)
    return dict(stats)
