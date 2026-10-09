"""Orthographic rules of grafia concisă.

A digraph ie/ia/ea/oa is compressed when both vowels are in the same
syllable (no hiatus). Within a syllable the scan is left to right and
matches never overlap.

Optional rule "ee" (switch): -ee lemmas follow the femeie pattern.
  - final -ee, singular:             idee    -> ideẹ
  - final -ee, plural of -eu lemma:  bombee  -> bombeẹ
  - final -eele/-eelor, -eu lemma:   bombeele -> bombeẹle, bombeelor -> bombeẹlor
  - final -euri/-eurile/-eurilor, -eu lemma of more than 4 letters:
                                     bombeuri -> bombeẹ, bombeurile -> bombeẹle,
                                     bombeurilor -> bombeẹlor (not pneuri)
  - final -ee, plural of -ee lemma:  logoree -> logorei
  - final -eea, -ee or -eu lemma:    ideea   -> ideė     (femeia), spontaneea -> spontaneė
  - final -eea, -eea headword:       Niceea  -> Niceė    (not aceea, oareceea)
  - final -eei, same lemmas as -eea: ideei   -> ideii    (femeii), Galileei -> Galileii
  - final -eele, -ee lemma:          orhideele  -> orhideile  (femeile)
  - final -eelor, -ee lemma:         orhideelor -> orhideilor (femeilor)
When one spelling is both a singular and a plural, the singular wins.

Optional rule "ai" (switch, on by default): â is written î, except in words
with the root român/rumân (câine -> cîine; român, aromân, rumânesc unchanged).

Long infinitives in -iare are not standard: a form of one is written as the
-iere form, with its cuts (apreciare -> apreciere, apreciărilor -> aprecierilor).

Apart from the -ee rule, a form with two syllabifications has two spellings
(a homograph: ia = the verb lua or the articulated ie). One is the default,
the other an alternative the user can switch to.
"""
import re
import unicodedata
from collections.abc import Callable, Container, Iterable, Mapping

DIGRAPHS = {"ie": "ẹ", "ia": "ė", "ea": "ē", "oa": "ō"}
REVERSE = {v: k for k, v in DIGRAPHS.items()}
# The suffix -ează/-ească after a hiatus i (studiază, sfiască, fiasca) is written
# -ėză/-ėscă/-ėsca; its ė reverts to a, not ia.
HIATUS_SUFFIX = re.compile(r"(?<=i)ė(?=(ză|scă|sca)$)", re.IGNORECASE)
# The â -> î switch keeps â only in words with the root român/rumân.
A_KEPT = re.compile(r"r[ou]mân", re.IGNORECASE)
_A_TO_I = str.maketrans("âÂ", "îÎ")

DIGRAPH = re.compile("|".join(DIGRAPHS))
CANDIDATE = re.compile(DIGRAPH.pattern + "|iăr")  # iăr: apreciări -> aprecieri
EE_FINAL = re.compile(r"(ee(a|i|le|lor)?|euri(le|lor)?)$")  # forms the -ee rule may change
EURI = re.compile(r"euri(le|lor)?$")  # -euri plurals, which the -ee rule writes -eẹ, like -ee
# Every spelling the -ee rule gives ends so (ideẹ, bombeẹle, ideė, ideii, orhideile, logorei)
EE_SPELLING = re.compile(r"e(ẹ|ẹle|ẹlor|ė|i|ii|ile|ilor)$")
WORD = re.compile(r"[^\W\d_]+")
_CEDILLA = str.maketrans("şţŞŢ", "șțȘȚ")


def normalize(word: str) -> str:
    """Lookup key: NFC, comma-below ș/ț, lowercase."""
    return unicodedata.normalize("NFC", word).translate(_CEDILLA).lower()


def has_candidate(word: str, ee: bool = False) -> bool:
    """True if the word can differ in grafia concisă (so needs a lookup)."""
    w = normalize(word)
    return CANDIDATE.search(w) is not None or (ee and EE_FINAL.search(w) is not None)


VOWELS = frozenset("aeiouăâî")
LABIALS = frozenset("pbfvm")
IENT = re.compile(r"ien[tț](?:[aăei]|$)")
# Medial ia after a consonant is a hiatus (the -ia* endings: apropiat, social,
# specializat), except in these native stems (chiar, biată, piatră, amiază).
IA_DIPHTHONG = re.compile(
    r"chiar(?!e)|chiab|(?<!s)chiag|chiaun|unchiaș"   # not -chiare (trunchiare), schiagrafie
    r"|(?<!n)(?<!litur)(?<!^gor)ghia(?![tnl]|re|ce)"  # not unghiar, -ghiat, -ghian, -ghial,
                                                      # -ghiare, liturghiar, Gorghias, eleghiacesc
    r"|^biat(?!l)|(?<!hi)piatr|viaț(?!i)"       # not biatlon, hipiatric, deviație
    r"|(?:^|[aă])miaz(?!m)|(?<!e)miau(?!l)"     # not distomiază, miazmă, semiautomat, Miaulis
    r"|megiaș|caragial|barcian|vianu$|chiajn|^obial"
)
# ea is a hiatus in these neologism endings and stems (real, nuclear, creat,
# realizat); native deal, beat, asear, greșeală stay diphthongs.
EA_HIATUS = re.compile(
    r"(?<![cgdb])eal$"                                    # real, leal, boreal; not deal, podbeal
    r"|(?:lin|hl|baln)ear$|nuclear"                       # linear, cohlear, balnear, nuclearizat
    r"|(?:gr|aur|or|lin|cl|un|ct|id|al|pl)eat$|creat"     # laureat, alineat, nucleat, creator
    r"|(?:real|ideal)i|boreal|cereal"                     # realizat, idealist, cereale
    r"|reac[tț]|steat|meandr|aleat|beatif"                # reactor, reacție, steatoză, aleator
)
# Medial ie after p/b/f/v/m or c/g/ch/gh is a hiatus in these neologism stems and
# endings (igienă, pieton, sovietic, mielită, așchietor, bărbierit; not zbiera, miel, mielușor).
IE_HIATUS = re.compile(r"gien|pieton|cviet|societ|soviet|vien|miel(?:o|in|it|ie)|ietoa?r|(?<=[^zs])bier")
# oa is a hiatus in these neologism stems and endings (coagulat, coaliție, protozoar).
OA_HIATUS = re.compile(r"coagul|coabit|coali[tțz]|zoar$")
# A long infinitive in -iere after p/b/f/v/m or c/g/ch/gh is a hiatus in all its forms
# (copiere, învierea, aprecierilor), except in these native words and their forms
# (mierea, pieri). After a vowel it is a diphthong anyway (tăiere, înmuiere).
IERE_ENDINGS = ("e", "ea", "i", "ii", "ile", "ilor")  # after -ier: -iere, -ierea, -ieri, ...
IERE_DIPHTHONG = frozenset({"miere", "înmiere", "fiere", "piere"})
# Lemmas in -iare that are no long infinitives (fiare: s.f. pl.; mobiliare: s.n.)
IARE_KEPT = frozenset({"fiare", "mobiliare"})
_IAR = re.compile(r"(.*i-?)[aă](-?r)")  # the last -iar-/-iăr- of a word
# Headwords in -eea take -eė with the -ee switch (Niceė, Andreė), except the
# pronoun aceea and its kin, which keep -eē.
EEA_UNCHANGED = frozenset({"aceea", "oareceea"})
# A digraph right after one of these is a hiatus when the rest is a known word
# (re-abilitat, auto-analiză). The rest must have at least 4 letters (not neam,
# autoare), and prea- + a known word is one syllable (prea-curat, not pre-acurat).
PREFIXES = (
    "re", "pre", "ne", "tele", "co",
    "semi", "anti", "multi",
    "auto", "micro", "macro", "foto", "pseudo", "bio", "neo", "radio", "aero",
    "agro", "electro", "hidro", "termo", "geo", "proto", "socio", "psiho", "stereo",
)
# Syllabifications decided by Cezar. They beat both dexonline and the rules,
# and for a form they give all its spellings: other lemmas' records add none.
# Several readings = a homograph: the first is the default, the others are
# offered as alternatives.
SYLLABIFICATIONS = {
    "neant": ("ne-ant",),
    "înfieri": ("în-fi-e-ri",),
    "mielea": ("mi-e-lea",),  # al mielea, like al miilea
    "piei": ("piei",),        # not pi-ei (<- pia)
    "răspieri": ("răs-pie-ri",),  # răspĭér, a -í (a verb, no long infinitive)
    "vier": ("vier", "vi-er"),     # the boar; păzitor de vie (rare)
    "vieri": ("vieri", "vi-e-ri"),
    "viere": ("vi-e-re",),           # the long infinitive (not the boar's vocative)
    "ia": ("ia", "i-a"),             # the verb lua, the interjection; the articulated ie
    "miază": ("mia-ză", "mi-a-ză"),  # noon (amiază), myiasis
    "ie": ("i-e", "ie"),   # the blouse (articulated ia: i-a); the adverb ie
    # co- + a known word, but not a prefix
    "coardă": ("coar-dă",),
    "coastă": ("coas-tă",),
    "coacere": ("coa-ce-re",),
    "coacăț": ("coa-căț",),
    "coacăță": ("coa-că-ță",),
}


def _dexonline_cuts(word: str, hyphenation: str) -> tuple[set[int], int, int] | None:
    """Cuts from a dexonline hyphenation and the boundary span [lo, hi] they settle.

    Uses the first comma-separated variant. It may be complete ('fa-mi-li-e'),
    a word end ('-li-e'), a word start ('a-ce-tal-') or a word interior
    ('-pioa-'). None if it does not fit the word (or an interior fits twice).
    """
    h = hyphenation.split(",")[0].strip()
    lead, trail = h.startswith("-"), h.endswith("-")
    frag = h.strip("-")
    letters = frag.replace("-", "")
    if not letters:
        return None
    if lead and trail:
        found = [i for i in range(len(word)) if word.startswith(letters, i)]
        if len(found) != 1:
            return None
        start = found[0]
    elif lead:
        start = len(word) - len(letters)
    else:
        start = 0
    if start < 0 or not word.startswith(letters, start) or (not lead and not trail and letters != word):
        return None
    end = start + len(letters)
    cuts, k = set(), start
    for part in frag.split("-")[:-1]:
        k += len(part)
        cuts.add(k)
    if lead:
        cuts.add(start)
    if trail:
        cuts.add(end)
    return cuts, start if lead else start + 1, end if trail else end - 1


def _rule_cut(word: str, b: int, known: Container[str], verb: bool = False) -> bool:
    """Fallback rules: is there a cut between the vowels word[b-1] and word[b]?

    `known`: normalized words that count as found (for the prefix rule).
    `verb`: the word is a verb infinitive (for the -ii rule).
    Incomplete by design, to be refined:
      - e-e: hiatus (idee -> ide-e; the -ee switch builds on this);
      - a prefix (PREFIXES) followed by a known word: hiatus (re-abilitat);
      - -ient, -ienț/-ient + a/ă/e/i: hiatus (client, ambient, experiență);
      - -iabil, -eabil: hiatus (fiabil, agreabil);
      - prefix bi- before en: hiatus (bienal);
      - ea in the EA_HIATUS neologism endings and stems: hiatus (real, nuclear,
        creat, realizat; not deal, beat, greșeală);
      - oa in the OA_HIATUS neologism stems and endings: hiatus (coagulat,
        coaliție, protozoar);
      - ii word-final after a consonant, in a verb infinitive: hiatus (sfi-i,
        so sfiește, sfiască keep i-e, i-a; not the plural copii);
      - ie/ia at word start or after a vowel (ă, î included): diphthong
        (iarbă, femeie, tăia);
      - ie/ia word-final after a consonant: hiatus (familie, vie);
      - ia medial after a consonant: hiatus (apropiat, pian, specializat),
        except in the IA_DIPHTHONG stems (chiar, biată, piatră) and, like
        -iere, final -iare after p/b/f/v/m or c/g/ch/gh (fiare);
      - ie medial after p/b/f/v/m or c/g/ch/gh: diphthong (fier, piept),
        except the IE_HIATUS stems (igienă, bărbierit, așchietor), the
        adjectives in -iesc (zapciesc, surugiesc), final -ier/-ieră after a
        vowel and a consonant (plumier, legumieră; not fier, zbier) and the
        long infinitives in -iere with all their endings (copiere, învierea,
        aprecierilor, schiere), though not in IERE_DIPHTHONG (miere, mierea,
        pieri);
        after other consonants: hiatus (prieten, cartier);
      - anything else, including ea and oa: no cut (seară, soare).
    """
    pair = word[b - 1 : b + 1]
    if pair == "ee":
        return True
    rest = word[b:]
    if len(rest) >= 4 and word[:b] in PREFIXES and rest in known:
        return not (word[:b] == "pre" and word[4:] in known)
    if pair == "ie" and IENT.match(word, b - 1):
        return True
    if pair in ("ia", "ea") and word.startswith("abil", b):
        return True
    if b == 2 and word.startswith("bien"):
        return True
    if pair == "ea" and _inside(EA_HIATUS, word, b) or pair == "oa" and _inside(OA_HIATUS, word, b):
        return True
    if verb and pair == "ii" and b + 1 == len(word) and b >= 2 and word[b - 2] not in VOWELS:
        return True
    if pair not in ("ie", "ia") or b == 1 or word[b - 2] in VOWELS:
        return False
    if b + 1 == len(word):
        return True
    prev = word[b - 2]
    labial_or_palatal = prev in LABIALS or prev in "cg" or word[b - 3 : b - 1] in ("ch", "gh")
    if pair == "ia":
        if labial_or_palatal and word.endswith("iare") and b == len(word) - 3:
            return False  # like -iere: fiare, fiere
        return not _inside(IA_DIPHTHONG, word, b)
    if labial_or_palatal:
        if word.startswith("ier", b - 1) and word[b + 2 :] in IERE_ENDINGS:  # copiere, învierea
            return word[: b + 2] + "e" not in IERE_DIPHTHONG  # not mierea, pieri
        if word[b - 1 :] in ("ier", "ieră"):  # plumier, legumieră; not fier, zbier
            return any(c in VOWELS for c in word[: b - 2])
        return word[b - 1 :] == "iesc" or _inside(IE_HIATUS, word, b)
    return True


def _inside(pattern: re.Pattern, word: str, b: int) -> bool:
    """True if a match of `pattern` in `word` spans the boundary b."""
    return any(m.start() < b < m.end() for m in pattern.finditer(word))


def long_infinitive(lemma: str) -> bool:
    """True if `lemma`, a feminine noun or unclassified in dexonline, is a long
    infinitive in -iare (apreciare, tăiare; not fiare). Input normalized."""
    return lemma.endswith("iare") and lemma not in IARE_KEPT


def iere_spelling(word: str) -> str:
    """The -iere spelling of a long infinitive in -iare, its form or its
    hyphenation: the last -iar-/-iăr- becomes -ier- (apreciare -> apreciere,
    apreciărilor -> aprecierilor, a-pre-ci-a-re -> a-pre-ci-e-re)."""
    return _IAR.sub(r"\1e\2", word, count=1)


def syllabify(
    word: str, hyphenation: str = "", known: Container[str] = frozenset(), verb: bool = False
) -> tuple[str, str]:
    """Syllabified `word` and its source: 'manual', 'dexonline', 'partial' or 'rules'.

    A syllabification in SYLLABIFICATIONS is used as is (its first reading;
    see syllabify_all). Otherwise, cuts inside the span covered by the
    dexonline hyphenation are taken from it and prevail. Elsewhere only cuts
    between two vowels are placed, by the fallback rules; those are the only
    cuts grafia concisă uses. `known` holds the words the prefix rule may
    find; `verb` marks a verb infinitive. Input normalized.
    """
    if word in SYLLABIFICATIONS:
        return SYLLABIFICATIONS[word][0], "manual"
    dex = _dexonline_cuts(word, hyphenation) if hyphenation else None
    cuts, lo, hi = dex or (set(), 1, 0)
    out = [word[:1]]
    for b in range(1, len(word)):
        if lo <= b <= hi:
            cut = b in cuts
        else:
            cut = word[b - 1] in VOWELS and word[b] in VOWELS and _rule_cut(word, b, known, verb)
        out.append("-" + word[b] if cut else word[b])
    if dex is None:
        source = "rules"
    else:
        source = "dexonline" if lo <= 1 and hi >= len(word) - 1 else "partial"
    return "".join(out), source


def syllabify_all(
    word: str, hyphenation: str = "", known: Container[str] = frozenset(), verb: bool = False
) -> list[tuple[str, str]]:
    """Every reading of `word`: all SYLLABIFICATIONS entries, else syllabify's one."""
    if word in SYLLABIFICATIONS:
        return [(s, "manual") for s in SYLLABIFICATIONS[word]]
    return [syllabify(word, hyphenation, known, verb)]


def concise_from_reference(form: str, reference: str) -> str:
    """Concise spelling of `form`, with syllable cuts taken from `reference`.

    `reference` is the syllabified lemma, which is the form itself when the
    form is a headword. Both normalized. For a vowel pair at i, i+1:
      - both letters in the prefix shared with the lemma: lemma's cut decides
        (reală <- re-al: hiatus; soarele <- soa-re: diphthong);
      - an i whose following lemma vowel is replaced at the same place:
        lemma's cut decides (familia <- fa-mi-li-e: hiatus; femeia <-
        fe-me-ie: diphthong), except that replacing the final u of an -iu
        lemma gives a hiatus (argintie <- ar-gin-tiu, cheflie <- che-fliu);
      - otherwise (appended ending, stem alternation, or o/e before a
        replaced vowel): diphthong (ideea <- i-de-e, creează <- cre-a,
        doarme <- dor-mi, ploaie <- plo-ua, scheaună <- sche-u-na).
    The suffix -ează/-ească after a hiatus i keeps an e form, as in the other
    forms: -ază of an i-a verb -> -ėză (studiază <- stu-di-a, like studiez),
    -ască of an i-i verb -> -ėscă (sfiască <- sfi-i, like sfiesc), -ască/-asca
    of an i-esc adjective -> -ėscă/-ėsca (fiască, fiasca <- fi-esc, like fiesc).
    """
    lemma = reference.replace("-", "")
    cuts, k = set(), 0
    for part in reference.split("-")[:-1]:
        k += len(part)
        cuts.add(k)
    p = 0
    while p < min(len(form), len(lemma)) and form[p] == lemma[p]:
        p += 1

    def hiatus(i: int) -> bool:
        if i + 1 < p:
            return i + 1 in cuts
        if i + 1 == p and p < len(lemma) and lemma[p] in VOWELS and form[i] == "i":
            return p in cuts or lemma[p - 1 :] == "iu"
        return False

    out, i = [], 0
    while i < len(form):
        pair = form[i : i + 2]
        if pair in DIGRAPHS and not hiatus(i):
            out.append(DIGRAPHS[pair])
            i += 2
        else:
            out.append(form[i])
            i += 1
    concise = "".join(out)
    if reference.endswith("i-a") and form == lemma + "ză":
        return concise[:-3] + "ėză"
    if reference.endswith("i-i") and form == lemma[:-1] + "ască":
        return concise[:-4] + "ėscă"
    if reference.endswith("i-esc") and form in (lemma[:-3] + "ască", lemma[:-3] + "asca"):
        return concise[:-4] + "ė" + form[-3:]
    return concise


def concise_from_syllables(syllabified: str) -> str:
    """'soa-re' -> 'sōre'; 're-al' -> 'real'. Input must be normalized."""
    return concise_from_reference(syllabified.replace("-", ""), syllabified)


def eu_lemma(word: str) -> str | None:
    """The -eu lemma of a plural in -eele/-eelor: bombeele, bombeelor -> bombeu."""
    for end in ("eele", "eelor"):
        if word.endswith(end):
            return word[: 1 - len(end)] + "u"
    return None


# The endings of an -esc adjective (fiesc: fiești, fiescu, ..., fiască, fiasca)
IESC_ENDINGS = ("ești", "escu", "escul", "escului", "eștii", "eștile", "eștilor", "ască", "asca")


def iesc_lemma(word: str) -> str | None:
    """The -iesc lemma of an adjective form: harabagiasca, harabagiești -> harabagiesc."""
    for end in IESC_ENDINGS:
        if word.endswith("i" + end):
            return word[: -len(end)] + "esc"
    return None


def lemma_candidates(word: str) -> tuple[str, ...]:
    """Lemmas an unknown word may have, in order of preference.

    For the -ee rule: andreei -> andreea, andree; bombeele, bombeuri, bombeurile
    -> bombeu. For an -iesc adjective: harabagiasca -> harabagiesc. A candidate
    in -eea, -eu or -iesc counts as known when it is a lemma, one in -ee when it
    is any form.
    """
    if iesc := iesc_lemma(word):
        return (iesc,)
    if word.endswith("eei"):
        return (word[:-1] + "a", word[:-1])
    if word.endswith(("euri", "eurile", "eurilor")):
        return (word[: word.rindex("eu") + 2],)
    eu = eu_lemma(word)
    return (eu,) if eu else ()


def concise_by_rules(
    word: str, ee: bool = False, known: Mapping[str, str] = {}
) -> tuple[str, ...]:
    """Concise spellings of a word missing from the database, by rules alone.

    Input normalized; the default spelling comes first. `known` maps known
    lemma candidates to their syllabification ('' when not needed). A form of
    an -iesc adjective in `known` is derived from it (harabagiasca ->
    harabagiėsca, harabagiești <- harabagi-esc). With the -ee switch the word is read as a
    singular, except:
      - -eei: the genitive of the first of its `lemma_candidates` in `known`
        (Andreii <- Andreea, ideii <- idee), otherwise unchanged (the default)
        or -eii (Zzeei, Zzeii);
      - -euri/-eurile/-eurilor: -eẹ/-eẹle/-eẹlor if its -eu lemma is in `known`
        (bombeuri <- bombeu), otherwise unchanged;
      - -eele/-eelor: the plural of its -eu lemma if that is in `known`
        (bombeẹle <- bombeu), otherwise of an -ee lemma (bombeile, the default)
        or of an -eu lemma (bombeẹle).
    """
    if (iesc := iesc_lemma(word)) in known:
        return (concise_from_reference(word, known[iesc]),)
    concise = concise_from_syllables(syllabify(word)[0])
    if ee and word.endswith("eei"):
        for lemma in lemma_candidates(word):
            if lemma in known:
                return (concise_ee(word, concise, lemma, False) or concise,)
        return (concise, concise[:-3] + "eii")
    if ee and word.endswith(("euri", "eurile", "eurilor")):
        lemmas = [x for x in lemma_candidates(word) if x in known]
        return ((lemmas and concise_ee(word, concise, lemmas[0], True)) or concise,)
    if ee and (eu := eu_lemma(word)):
        lemmas = [eu] if eu in known else [eu[:-2] + "ee", eu]
        return tuple(concise_ee(word, concise, lemma, True) for lemma in lemmas)
    if ee and EE_FINAL.search(word):
        return (concise_ee(word, concise, word, False) or concise,)
    return (concise,)


def concise_ee(form: str, concise: str, lemma: str, plural: bool) -> str | None:
    """Apply the -ee rule on top of the base concise form; None if it doesn't apply.

    All arguments normalized; `form` is the standard spelling of `concise`.
    """
    for end, new in (("euri", "eẹ"), ("eurile", "eẹle"), ("eurilor", "eẹlor")):
        if form.endswith(end):
            # the -eu lemma itself, longer than 4 letters: bombeu, not pneu
            if lemma == form[: 2 - len(end)] and len(lemma) > 4:
                return concise[: -len(end)] + new
            return None
    if form.endswith(("eea", "eei")):
        if not (lemma.endswith(("ee", "eu")) or lemma.endswith("eea") and lemma not in EEA_UNCHANGED):
            return None
        if form.endswith("eei"):
            return concise[:-3] + "eii"
        if concise.endswith("eē"):  # i-de-ea
            return concise[:-1] + "ė"
        return None
    if form.endswith("ee"):
        if not plural or lemma.endswith("eu"):
            return concise[:-2] + "eẹ"
        if lemma.endswith("ee"):
            return concise[:-2] + "ei"
        return None
    if lemma.endswith("eu"):
        if form.endswith("eele"):
            return concise[:-4] + "eẹle"
        if form.endswith("eelor"):
            return concise[:-5] + "eẹlor"
        return None
    if not lemma.endswith("ee"):
        return None
    if form.endswith("eele"):
        return concise[:-4] + "eile"
    if form.endswith("eelor"):
        return concise[:-5] + "eilor"
    return None


def a_to_i(text: str) -> str:
    """â -> î in every word of `text` (NFC), except those with the root român/rumân."""
    def word(m: re.Match) -> str:
        return m.group() if A_KEPT.search(m.group()) else m.group().translate(_A_TO_I)

    return WORD.sub(word, text)


def apply_case(template: str, word: str) -> str:
    """Carry the case pattern of `template` (ALL CAPS / Initial / lower) onto `word`."""
    if len(template) > 1 and template.isupper():
        return word.upper()
    if template[:1].isupper():
        return word[:1].upper() + word[1:]
    return word


Row = tuple[str, str | None, tuple[str, ...]]  # concise_form, concise_ee, alternatives
Lookup = Callable[[Iterable[str]], dict[str, Row]]
LookupKnown = Callable[[Iterable[str]], Mapping[str, str]]


def resolve(
    keys: set[str], lookup: Lookup, ee: bool = False, lookup_known: LookupKnown = lambda words: {}
) -> dict[str, tuple[tuple[str, ...], bool]]:
    """Normalized candidate words -> (their concise spellings, default first; unknown?).

    `lookup` maps normalized forms to (concise_form, concise_ee, alternatives).
    `lookup_known` maps those of the given words that exist to their
    syllabification, '' when not needed (see lemma_candidates, concise_by_rules).
    Words missing from the database are converted by rules alone (concise_by_rules),
    and are unknown unless they have no digraph or -ee ending (zziăr: nothing to change).
    """
    rows = lookup(keys) if keys else {}
    candidates = {x for k in keys - rows.keys() for x in lemma_candidates(k)}
    known = lookup_known(candidates) if candidates else {}
    out = {}
    for k in keys:
        if k in rows:
            c, e, alts = rows[k]
            out[k] = ((e if ee and e else c, *alts), False)
        else:
            changeable = DIGRAPH.search(k) is not None or ee and EE_FINAL.search(k) is not None
            out[k] = (concise_by_rules(k, ee, known), changeable)
    return out


def convert_text(
    text: str,
    lookup: Lookup,
    ee: bool = False,
    lookup_known: LookupKnown = lambda words: {},
    ai: bool = True,
) -> list[dict]:
    """Split text into segments.

    `lookup`, `lookup_known`: see resolve.
    `ai` switches on â -> î (see a_to_i), applied to every word after the lookup.

    Words without a candidate digraph are never looked up. Candidate words
    missing from the database are converted by rules alone; their segment's
    `unknown` holds the original word. A homograph's segment, or an unknown
    word's with several spellings, has `alts`: all its spellings, the default
    (also in `t`) first.
    """
    text = unicodedata.normalize("NFC", text)
    matches = [m for m in WORD.finditer(text) if has_candidate(m.group(), ee)]
    resolved = resolve({normalize(m.group()) for m in matches}, lookup, ee, lookup_known)

    segments: list[dict] = []

    def plain(chunk: str) -> None:
        if not chunk:
            return
        if segments and len(segments[-1]) == 1:
            segments[-1]["t"] += chunk
        else:
            segments.append({"t": chunk})

    def spell(word: str, key: str, concise: str) -> str:
        # all hiatus: keep the original spelling untouched
        return word if concise == key else apply_case(word, concise)

    pos = 0
    for m in matches:
        plain(text[pos : m.start()])
        word, key = m.group(), normalize(m.group())
        spellings, unknown = resolved[key]
        options = [spell(word, key, x) for x in spellings]
        if unknown:
            segments.append({"t": options[0], "unknown": word})
            if len(options) > 1:
                segments[-1]["alts"] = options
        elif len(options) > 1:
            segments.append({"t": options[0], "alts": options})
        else:
            plain(options[0])
        pos = m.end()
    plain(text[pos:])
    if ai:
        for s in segments:
            s["t"] = a_to_i(s["t"])
            if "alts" in s:
                s["alts"] = [a_to_i(x) for x in s["alts"]]
    return segments


def _revert_word(word: str) -> str:
    all_upper = len(word) > 1 and word.isupper()
    suffix = HIATUS_SUFFIX.search(word)
    out = []
    for k, ch in enumerate(word):
        low = ch.lower()
        if suffix and k == suffix.start():
            out.append("A" if ch.isupper() else "a")
        elif low not in REVERSE:
            out.append(ch)
        elif ch == low:
            out.append(REVERSE[low])
        else:
            pair = REVERSE[low]
            out.append(pair.upper() if all_upper else pair[0].upper() + pair[1])
    return "".join(out)


def revert_segments(
    text: str, lookup_ee: Callable[[Iterable[str]], dict[str, tuple[str, tuple[str, ...]]]] | None = None
) -> list[dict]:
    """Concise -> standard by character map, except the suffix
    -iėză/-iėscă/-iėsca -> -iază/-iască/-iasca (studiėză, sfiėscă, fiėsca).

    `lookup_ee` (the -ee switch) maps normalized -ee spellings to (the standard form
    they revert to, the other forms of a tie) where the character map alone is not
    the answer (aleẹ -> alee, orhideile -> orhideele; bordeẹ -> bordeie or bordee).
    Segments as in convert_text: a tie's segment has `alts`, all its forms, default first.

    Exact for the base rules (but fiiască -> fiėscă reverts to fiască, fiiasca likewise).
    With the -ee rule, exact for the database forms it picks (aleẹ: alee, not aleie);
    unknown words keep the character map (zzeẹ -> zzeie).
    î is left as is, so not exact either once â -> î was applied (cîine stays cîine).
    """
    text = unicodedata.normalize("NFC", text)
    keys = {k for m in WORD.finditer(text) if EE_SPELLING.search(k := normalize(m.group()))}
    found = lookup_ee(keys) if lookup_ee and keys else {}
    segments: list[dict] = []

    def plain(chunk: str) -> None:
        if not chunk:
            return
        if segments and len(segments[-1]) == 1:
            segments[-1]["t"] += chunk
        else:
            segments.append({"t": chunk})

    pos = 0
    for m in WORD.finditer(text):
        plain(text[pos : m.start()])
        word = m.group()
        if (key := normalize(word)) in found:
            form, alts = found[key]
            options = [apply_case(word, x) for x in (form, *alts)]
            if alts:
                segments.append({"t": options[0], "alts": options})
            else:
                plain(options[0])
        else:
            plain(_revert_word(word))
        pos = m.end()
    plain(text[pos:])
    return segments


def revert_text(text: str) -> str:
    """Concise -> standard by character map (see revert_segments), without the -ee switch."""
    return "".join(s["t"] for s in revert_segments(text))
