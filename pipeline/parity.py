"""Compare the Rust port with Python on the full data (temporary: see rust-port-plan.md).

Reads the records and known words written by build_db.py --export, and writes the
same files as crates/core/examples/parity.rs does:
  references.tsv  syllabify_all of every distinct (lemma, hyphenation, verb)
  forms.tsv, eu_lemmas.txt, ea_lemmas.txt, iesc_lemmas.tsv, ee_revert.tsv, stats.tsv: the build
  convert.tsv     every stored form as the text "form Form FORM", for each ee/ai pair
  unknown.tsv     every stored form by rules alone, ee off and on
  revert.tsv      every stored spelling, ee off and on

Usage:
  uv run python pipeline/parity.py [DATA [OUT]]      (default: data, data/parity/python)
  cargo run --release -p grafia-concisa --example parity -- [DATA [OUT]]
  git diff --no-index --stat data/parity/python data/parity/rust
"""
import sqlite3
import sys
from collections.abc import Iterable
from contextlib import closing
from pathlib import Path

from build_db import from_text

from grafia_concisa.db import _clean, build
from grafia_concisa.rules import concise_by_rules, convert_text, revert_segments, syllabify_all

ROOT = Path(__file__).resolve().parent.parent
SWITCHES = ((False, True), (True, True), (False, False), (True, False))  # (ee, ai)


def render(segments: list[dict]) -> str:
    """One line per text: each segment's text, <its alternatives>, ?its unknown word."""
    out = []
    for s in segments:
        x = s["t"]
        if "alts" in s:
            x += "<" + "/".join(s["alts"]) + ">"
        if "unknown" in s:
            x += "?" + s["unknown"]
        out.append(x)
    return "¦".join(out)


def write(path: Path, lines: Iterable[str]) -> None:
    path.write_text("".join(f"{x}\n" for x in lines), encoding="utf-8", newline="\n")


def main() -> int:
    data = Path(sys.argv[1]) if len(sys.argv) > 1 else ROOT / "data"
    out = Path(sys.argv[2]) if len(sys.argv) > 2 else data / "parity" / "python"
    out.mkdir(parents=True, exist_ok=True)
    records = list(from_text(data / "records.tsv"))
    known = set((data / "known.txt").read_text(encoding="utf-8").splitlines())

    refs = set()
    for rec in records:
        if not isinstance(rec, str):
            refs.add((_clean(rec[1]), _clean(rec[2]), "V" in {_clean(x).upper() for x in rec[4:]}))
    write(out / "references.tsv", sorted(
        f"{lemma}\t{hyph}\t{'V' if verb else ''}\t"
        + " ".join(f"{s}/{source}" for s, source in syllabify_all(lemma, hyph, known, verb))
        for lemma, hyph, verb in refs
    ))

    db = out / "word_db.sqlite"
    stats = build(db, records, known)
    write(out / "stats.tsv", (f"{k}\t{v}" for k, v in sorted(stats.items())))
    with closing(sqlite3.connect(db)) as c:
        forms = {f: (cf, e, tuple((a or "").split()))
                 for f, cf, e, a in c.execute("SELECT form, concise_form, concise_ee, alternatives FROM forms")}
        eu = {x for (x,) in c.execute("SELECT lemma FROM eu_lemmas")}
        ea = {x for (x,) in c.execute("SELECT lemma FROM ea_lemmas")}
        iesc = dict(c.execute("SELECT lemma, reference FROM iesc_lemmas"))
        ee_revert = {k: (f, tuple((a or "").split()))
                     for k, f, a in c.execute("SELECT concise, form, alternatives FROM ee_revert")}
    db.unlink()  # the same files on both sides
    write(out / "forms.tsv", (f"{f}\t{c}\t{e or ''}\t{' '.join(a)}" for f, (c, e, a) in sorted(forms.items())))
    write(out / "eu_lemmas.txt", sorted(eu))
    write(out / "ea_lemmas.txt", sorted(ea))
    write(out / "iesc_lemmas.tsv", (f"{k}\t{v}" for k, v in sorted(iesc.items())))
    write(out / "ee_revert.tsv", (f"{k}\t{f}\t{' '.join(a)}" for k, (f, a) in sorted(ee_revert.items())))

    def lookup(keys):
        return {k: forms[k] for k in keys if k in forms}

    def lookup_ee(keys):
        return {k: ee_revert[k] for k in keys if k in ee_revert}

    def lookup_known(words):  # as in server.py
        found = {}
        for w in words:
            if w.endswith("eu"):
                if w in eu:
                    found[w] = ""
            elif w.endswith("ea"):
                if w in ea:
                    found[w] = ""
            elif w.endswith("iesc"):
                if w in iesc:
                    found[w] = iesc[w]
            elif w in forms:
                found[w] = ""
        return found

    def converted(form: str):
        text = f"{form} {form[:1].upper() + form[1:]} {form.upper()}"
        for ee, ai in SWITCHES:
            yield f"{form}\t{int(ee)}{int(ai)}\t{render(convert_text(text, lookup, ee, lookup_known, ai))}"

    write(out / "convert.tsv", (line for form in sorted(forms) for line in converted(form)))
    write(out / "unknown.tsv", (f"{form}\t{int(ee)}\t{' '.join(concise_by_rules(form, ee))}"
                                for form in sorted(forms) for ee in (False, True)))
    spellings = sorted({x for c, e, alts in forms.values() for x in (c, e, *alts) if x})
    write(out / "revert.tsv", (f"{x}\t{int(ee)}\t{render(revert_segments(x, lookup_ee if ee else None))}"
                               for x in spellings for ee in (False, True)))
    print(f"{out}: {stats}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
