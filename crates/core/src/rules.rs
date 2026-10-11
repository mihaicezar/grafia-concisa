//! Orthographic rules of grafia concisă.
//!
//! A digraph ie/ia/ea/oa is compressed when both vowels are in the same
//! syllable (no hiatus). Within a syllable the scan is left to right and
//! matches never overlap.
//!
//! Optional rule "ee" (switch): -ee lemmas follow the femeie pattern.
//! ```text
//!   - final -ee, singular:             idee    -> ideẹ
//!   - final -ee, plural of -eu lemma:  bombee  -> bombeẹ
//!   - final -eele/-eelor, -eu lemma:   bombeele -> bombeẹle, bombeelor -> bombeẹlor
//!   - final -euri/-eurile/-eurilor, -eu lemma of more than 4 letters:
//!                                      bombeuri -> bombeẹ, bombeurile -> bombeẹle,
//!                                      bombeurilor -> bombeẹlor (not pneuri)
//!   - final -ee, plural of -ee lemma:  logoree -> logorei
//!   - final -eea, -ee or -eu lemma:    ideea   -> ideė     (femeia), spontaneea -> spontaneė
//!   - final -eea, -eea headword:       Niceea  -> Niceė    (not aceea, oareceea)
//!   - final -eei, same lemmas as -eea: ideei   -> ideii    (femeii), Galileei -> Galileii
//!   - final -eele, -ee lemma:          orhideele  -> orhideile  (femeile)
//!   - final -eelor, -ee lemma:         orhideelor -> orhideilor (femeilor)
//! ```
//! When one spelling is both a singular and a plural, the singular wins.
//!
//! Optional rule "ai" (switch, on by default): â is written î, except in words
//! with the root român/rumân (câine -> cîine; român, aromân, rumânesc unchanged).
//!
//! Long infinitives in -iare are not standard: a form of one is written as the
//! -iere form, with its cuts (apreciare -> apreciere, apreciărilor -> aprecierilor).
//!
//! Apart from the -ee rule, a form with two syllabifications has two spellings
//! (a homograph: ia = the verb lua or the articulated ie). One is the default,
//! the other an alternative the user can switch to.
//!
//! Words are indexed by character (a `Vec<char>`), not by byte; the code was
//! ported from Python, whose semantics some comments still cite.
use std::collections::{BTreeSet, HashMap, HashSet};
use std::sync::LazyLock;

use fancy_regex::{Captures, Regex};
use serde::Serialize;
use unicode_normalization::UnicodeNormalization;

use crate::Dictionary;

/// ie/ia/ea/oa and their concise letters.
pub const DIGRAPHS: [([char; 2], char); 4] = [
    (['i', 'e'], 'ẹ'),
    (['i', 'a'], 'ė'),
    (['e', 'a'], 'ē'),
    (['o', 'a'], 'ō'),
];

fn re(pattern: &str) -> Regex {
    Regex::new(pattern).expect("valid pattern")
}

// The suffix -ează/-ească after a hiatus i (studiază, sfiască, fiasca) is written
// -ėză/-ėscă/-ėsca; its ė reverts to a, not ia.
pub static HIATUS_SUFFIX: LazyLock<Regex> = LazyLock::new(|| re(r"(?i)(?<=i)ė(?=(ză|scă|sca)$)"));
// The â -> î switch keeps â only in words with the root român/rumân.
pub static A_KEPT: LazyLock<Regex> = LazyLock::new(|| re(r"(?i)r[ou]mân"));

pub static DIGRAPH: LazyLock<Regex> = LazyLock::new(|| re("ie|ia|ea|oa"));
pub static CANDIDATE: LazyLock<Regex> = LazyLock::new(|| re("ie|ia|ea|oa|iăr")); // iăr: apreciări -> aprecieri
pub static EE_FINAL: LazyLock<Regex> = LazyLock::new(|| re(r"(ee(a|i|le|lor)?|euri(le|lor)?)$")); // forms the -ee rule may change
pub static EURI: LazyLock<Regex> = LazyLock::new(|| re(r"euri(le|lor)?$")); // -euri plurals, which the -ee rule writes -eẹ, like -ee
// Every spelling the -ee rule gives ends so (ideẹ, bombeẹle, ideė, ideii, orhideile, logorei)
pub static EE_SPELLING: LazyLock<Regex> = LazyLock::new(|| re(r"e(ẹ|ẹle|ẹlor|ė|i|ii|ile|ilor)$"));
// Python's [^\W\d_]: letters and the non-decimal numbers
pub static WORD: LazyLock<Regex> = LazyLock::new(|| re(r"[\p{L}\p{Nl}\p{No}]+"));

/// True if `pattern` matches somewhere in `text`.
pub fn found(pattern: &Regex, text: &str) -> bool {
    pattern.is_match(text).expect("regex match")
}

/// Lookup key: NFC, comma-below ș/ț, lowercase.
pub fn normalize(word: &str) -> String {
    let nfc: String = word
        .nfc()
        .map(|c| match c {
            'ş' => 'ș',
            'ţ' => 'ț',
            'Ş' => 'Ș',
            'Ţ' => 'Ț',
            c => c,
        })
        .collect();
    nfc.to_lowercase()
}

/// True if the word can differ in grafia concisă (so needs a lookup): a digraph, iăr, an
/// -ee ending with the -ee switch, or an obsolete stem (alineez).
pub fn has_candidate(word: &str, ee: bool) -> bool {
    let w = normalize(word);
    found(&CANDIDATE, &w) || (ee && found(&EE_FINAL, &w)) || has_obsolete_stem(&w)
}

const VOWELS: &str = "aeiouăâî";
const LABIALS: &str = "pbfvm";

fn is_vowel(c: char) -> bool {
    VOWELS.contains(c)
}

// Matched at the start of the word's tail from the i (Python: IENT.match(word, b - 1)).
static IENT: LazyLock<Regex> = LazyLock::new(|| re(r"^ien[tț](?:[aăei]|$)"));
// Medial ia after a consonant is a hiatus (the -ia* endings: apropiat, social,
// specializat), except in these native stems (chiar, biată, piatră, amiază).
pub static IA_DIPHTHONG: LazyLock<Regex> = LazyLock::new(|| {
    re(concat!(
        r"chiar|chiab|(?<!s)chiag|chiaun|unchiaș",     // not schiagrafie
        r"|(?<!n)(?<!litur)(?<!^gor)ghia(?![tnl]|ce)", // not unghiar, -ghiat, -ghian, -ghial,
        // liturghiar, Gorghias, eleghiacesc
        r"|^biat(?!l)|(?<!hi)piatr|viaț(?!i)",   // not biatlon, hipiatric, deviație
        r"|(?:^|[aă])miaz(?!m)|(?<!e)miau(?!l)", // not distomiază, miazmă, semiautomat, Miaulis
        r"|megiaș|caragial|barcian|vianu$|chiajn|^obial",
    ))
});
// ea is a hiatus in these neologism endings and stems (real, nuclear, creat,
// realizat); native deal, beat, asear, greșeală stay diphthongs.
pub static EA_HIATUS: LazyLock<Regex> = LazyLock::new(|| {
    re(concat!(
        r"(?<![gdb])eal$",               // real, leal, boreal, interliceal; not mangeal, deal, podbeal
        r"|(?:lin|hl|baln)ear$|nuclear", // linear, cohlear, balnear, nuclearizat
        r"|(?:gr|aur|lor|lin|ucl|un|ct|id|upl)eat|creat", // laureat(ă), nucleat(ă), delineator; not cleat, pleată
        r"|(?:real|ideal)i|(?<!ră)(?<!sto)boreal|cereal", // realizat, idealist, cereale; not răboreală
        r"|reac[tț]|steat|meandr|aleat|beatif", // reactor, reacție, steatoză, aleator
    ))
});
// Medial ie after p/b/f/v/m or c/g/ch/gh is a hiatus in these neologism stems and
// endings (igienă, pieton, sovietic, mielită, așchietor, bărbierit; not zbiera, miel, mielușor).
pub static IE_HIATUS: LazyLock<Regex> =
    LazyLock::new(|| re(r"gien|pieton|cviet|societ|soviet|vien|miel(?:o|in|it|ie)|ietoa?r|(?<=[^zs])bier"));
// oa is a hiatus in these neologism stems and endings (coagulat, coaliție, protozoar).
pub static OA_HIATUS: LazyLock<Regex> = LazyLock::new(|| re(r"coagul|coabit|coali[țz]|zoar$"));
// A long infinitive in -iere after p/b/f/v/m or c/g/ch/gh is a hiatus in all its forms
// (copiere, învierea, aprecierilor), except in these native words and their forms
// (mierea, pieri). After a vowel it is a diphthong anyway (tăiere, înmuiere).
pub const IERE_ENDINGS: [&str; 6] = ["e", "ea", "i", "ii", "ile", "ilor"]; // after -ier: -iere, -ierea, -ieri, ...
// After -ier, ending an agent noun: -ier, -ieră, -iereasă and its forms (plumier, legumieră,
// rachiereasă, rachieresele)
pub const IER_ENDINGS: [&str; 8] = ["", "ă", "easă", "easa", "ese", "esei", "esele", "eselor"];
pub const IERE_DIPHTHONG: [&str; 4] = ["miere", "înmiere", "fiere", "piere"];
// Lemmas in -iare that are no long infinitives (fiare: s.f. pl.; mobiliare: s.n.)
pub const IARE_KEPT: [&str; 2] = ["fiare", "mobiliare"];
static IAR: LazyLock<Regex> = LazyLock::new(|| re(r"(.*i-?)[aă](-?r)")); // the last -iar-/-iăr- of a word
// Obsolete lemmas: (the lemma, syllabified, its current lemma, the stem its forms start
// with, whether dexonline lists it without its forms). A form of one is written as the
// current lemma's form with the same inflection (alineează -> aliniėză), and offers its own
// spelling, from this syllabification, on the switch (alineēză). Words with the stem are
// looked up even with no candidate digraph (alineez). For a lemma listed without its forms,
// a word missing from the database that starts with the stem takes the current lemma's
// stem instead (rachiareasa -> rachiereasa, or rachiarēsa); for the others it would only
// catch other words (Alinei). A form of several obsolete lemmas with different current
// lemmas is not rewritten: its own spellings, in the order of this list, are offered
// (alineam, of alinea and alini: alineam or alinēm).
pub const OBSOLETE: &[(&str, &str, &str, bool)] = &[
    ("a-li-ne-a", "alinia", "aline", false),
    ("a-li-ne-a-re", "aliniere", "aline", false),
    ("a-li-ni", "alina", "alin", false), // its imperfect is alinea's: alineam (alinea first)
    ("ra-chi-a-rea-să", "rachiereasă", "rachiare", true),
];

// Headwords in -eea take -eė with the -ee switch (Niceė, Andreė), except the
// pronoun aceea and its kin, which keep -eē.
pub const EEA_UNCHANGED: [&str; 2] = ["aceea", "oareceea"];
// A digraph right after one of these is a hiatus when the rest is a known word
// (re-abilitat, auto-analiză). The rest must have at least 4 letters (not neam,
// autoare), and prea- + a known word is one syllable (prea-curat, not pre-acurat).
pub const PREFIXES: &[&str] = &[
    "re", "pre", "ne", "tele", "co", "semi", "anti", "multi", "auto", "micro", "macro", "foto", "pseudo", "bio", "neo",
    "radio", "aero", "agro", "electro", "hidro", "termo", "geo", "proto", "socio", "psiho", "stereo",
];
// Syllabifications decided by Cezar. They beat both dexonline and the rules,
// and for a form they give all its spellings: other lemmas' records add none.
// Several readings = a homograph: the first is the default, the others are
// offered as alternatives.
pub const SYLLABIFICATIONS: &[(&str, &[&str])] = &[
    ("neant", &["ne-ant"]),
    ("înfieri", &["în-fi-e-ri"]),
    ("mielea", &["mi-e-lea"]),     // al mielea, like al miilea
    ("piei", &["piei"]),           // not pi-ei (<- pia)
    ("răspieri", &["răs-pie-ri"]), // răspĭér, a -í (a verb, no long infinitive)
    ("vier", &["vier", "vi-er"]),  // the boar; păzitor de vie (rare)
    ("vieri", &["vieri", "vi-e-ri"]),
    ("viere", &["vi-e-re"]),           // the long infinitive (not the boar's vocative)
    ("ia", &["ia", "i-a"]),            // the verb lua, the interjection; the articulated ie
    ("miază", &["mia-ză", "mi-a-ză"]), // noon (amiază), myiasis
    ("ie", &["i-e", "ie"]),            // the blouse (articulated ia: i-a); the adverb ie
    // co- + a known word, but not a prefix
    ("coardă", &["coar-dă"]),
    ("coastă", &["coas-tă"]),
    ("coacere", &["coa-ce-re"]),
    ("coacăț", &["coa-căț"]),
    ("coacăță", &["coa-că-ță"]),
];

/// The readings of `word` in SYLLABIFICATIONS, if listed.
pub fn syllabifications(word: &str) -> Option<&'static [&'static str]> {
    SYLLABIFICATIONS.iter().find(|(w, _)| *w == word).map(|(_, r)| *r)
}

/// Where a syllabification comes from, from the weakest to the strongest.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Source {
    Rules,
    Partial,
    Dexonline,
    Manual,
}

impl Source {
    pub fn name(self) -> &'static str {
        match self {
            Source::Rules => "rules",
            Source::Partial => "partial",
            Source::Dexonline => "dexonline",
            Source::Manual => "manual",
        }
    }
}

fn chars(s: &str) -> Vec<char> {
    s.chars().collect()
}

fn text(chars: &[char]) -> String {
    chars.iter().collect()
}

/// `word[at:]` starts with `part` (Python: word.startswith(part, at)).
fn starts_with_at(word: &[char], part: &[char], at: usize) -> bool {
    word.get(at..).is_some_and(|rest| rest.starts_with(part))
}

/// Python's s[:-n], by character.
fn drop_last(chars: &[char], n: usize) -> String {
    text(&chars[..chars.len().saturating_sub(n)])
}

/// Cuts from a dexonline hyphenation and the boundary span [lo, hi] they settle.
///
/// Uses the first comma-separated variant. It may be complete ('fa-mi-li-e'),
/// a word end ('-li-e'), a word start ('a-ce-tal-') or a word interior
/// ('-pioa-'). None if it does not fit the word (or an interior fits twice).
fn dexonline_cuts(word: &[char], hyphenation: &str) -> Option<(HashSet<usize>, usize, usize)> {
    let h = hyphenation.split(',').next().unwrap_or("").trim();
    let (lead, trail) = (h.starts_with('-'), h.ends_with('-'));
    let frag = h.trim_matches('-');
    let letters: Vec<char> = frag.chars().filter(|&c| c != '-').collect();
    if letters.is_empty() {
        return None;
    }
    let start = if lead && trail {
        let found: Vec<usize> = (0..word.len()).filter(|&i| starts_with_at(word, &letters, i)).collect();
        if found.len() != 1 {
            return None;
        }
        found[0]
    } else if lead {
        word.len().checked_sub(letters.len())?
    } else {
        0
    };
    if !starts_with_at(word, &letters, start) || (!lead && !trail && letters != word) {
        return None;
    }
    let end = start + letters.len();
    let parts: Vec<&str> = frag.split('-').collect();
    let (mut cuts, mut k) = (HashSet::new(), start);
    for part in &parts[..parts.len() - 1] {
        k += part.chars().count();
        cuts.insert(k);
    }
    if lead {
        cuts.insert(start);
    }
    if trail {
        cuts.insert(end);
    }
    Some((
        cuts,
        if lead { start } else { start + 1 },
        if trail { end } else { end - 1 },
    ))
}

/// Fallback rules: is there a cut between the vowels word[b-1] and word[b]?
///
/// `known`: normalized words that count as found (for the prefix rule).
/// `verb`: the word is a verb infinitive (for the -ii rule).
/// Incomplete by design, to be refined:
///   - e-e: hiatus (idee -> ide-e; the -ee switch builds on this);
///   - a prefix (PREFIXES) followed by a known word: hiatus (re-abilitat);
///   - -ient, -ienț/-ient + a/ă/e/i: hiatus (client, ambient, experiență);
///   - -iabil, -eabil after a consonant: hiatus (fiabil, agreabil; not debreiabil);
///   - prefix bi- before en: hiatus (bienal);
///   - ea in the EA_HIATUS neologism endings and stems: hiatus (real, nuclear,
///     creat, realizat; not deal, beat, greșeală);
///   - oa in the OA_HIATUS neologism stems and endings: hiatus (coagulat,
///     coaliție, protozoar);
///   - ii word-final after a consonant, in a verb infinitive: hiatus (sfi-i,
///     so sfiește, sfiască keep i-e, i-a; not the plural copii);
///   - ie/ia at word start or after a vowel (ă, î included): diphthong
///     (iarbă, femeie, tăia);
///   - ie/ia word-final after a consonant: hiatus (familie, vie);
///   - ia medial after a consonant: hiatus (apropiat, pian, specializat),
///     except in the IA_DIPHTHONG stems (chiar, biată, piatră) and, like
///     -iere, final -iare after p/b/f/v/m or c/g/ch/gh (fiare);
///   - ie medial after p/b/f/v/m or c/g/ch/gh: diphthong (fier, piept),
///     except the IE_HIATUS stems (igienă, bărbierit, așchietor), the
///     adjectives in -iesc (zapciesc, surugiesc), final -ier/-ieră/-iereasă (IER_ENDINGS) after a
///     vowel and a consonant (plumier, legumieră; not fier, zbier) and the
///     long infinitives in -iere with all their endings (copiere, învierea,
///     aprecierilor, schiere), though not in IERE_DIPHTHONG (miere, mierea,
///     pieri);
///     after other consonants: hiatus (prieten, cartier);
///   - anything else, including ea and oa: no cut (seară, soare).
fn rule_cut(word: &[char], b: usize, known: &HashSet<String>, verb: bool) -> bool {
    let pair = [word[b - 1], word[b]];
    if pair == ['e', 'e'] {
        return true;
    }
    let head = text(&word[..b]);
    if word.len() - b >= 4 && PREFIXES.contains(&head.as_str()) && known.contains(&text(&word[b..])) {
        return !(head == "pre" && known.contains(&text(&word[4..])));
    }
    if pair == ['i', 'e'] && found(&IENT, &text(&word[b - 1..])) {
        return true;
    }
    if (pair == ['i', 'a'] || pair == ['e', 'a'])
        && starts_with_at(word, &['a', 'b', 'i', 'l'], b)
        && (b == 1 || !is_vowel(word[b - 2]))
    {
        return true; // fiabil, agreabil; not debreiabil (de-bre-ia)
    }
    if b == 2 && word.starts_with(&['b', 'i', 'e', 'n']) {
        return true;
    }
    if pair == ['e', 'a'] && inside(&EA_HIATUS, word, b) || pair == ['o', 'a'] && inside(&OA_HIATUS, word, b) {
        return true;
    }
    if verb && pair == ['i', 'i'] && b + 1 == word.len() && b >= 2 && !is_vowel(word[b - 2]) {
        return true;
    }
    if !(pair == ['i', 'e'] || pair == ['i', 'a']) || b == 1 || is_vowel(word[b - 2]) {
        return false;
    }
    if b + 1 == word.len() {
        return true;
    }
    let prev = word[b - 2];
    let labial_or_palatal = LABIALS.contains(prev)
        || "cg".contains(prev)
        || b >= 3 && matches!(word[b - 3..b - 1], ['c', 'h'] | ['g', 'h']);
    if pair == ['i', 'a'] {
        if labial_or_palatal && word.ends_with(&['i', 'a', 'r', 'e']) && b == word.len() - 3 {
            return false; // like -iere: fiare, fiere
        }
        return !inside(&IA_DIPHTHONG, word, b);
    }
    if labial_or_palatal {
        if starts_with_at(word, &['i', 'e', 'r'], b - 1) {
            let end = text(&word[b + 2..]);
            if IERE_ENDINGS.contains(&end.as_str()) {
                // copiere, învierea; not mierea, pieri
                return !IERE_DIPHTHONG.contains(&(text(&word[..b + 2]) + "e").as_str());
            }
            if IER_ENDINGS.contains(&end.as_str()) {
                // plumier, legumieră, rachiereasă, rachieresele; not fier, zbier
                return word[..b - 2].iter().any(|&c| is_vowel(c));
            }
        }
        return text(&word[b - 1..]) == "iesc" || inside(&IE_HIATUS, word, b);
    }
    true
}

/// True if a match of `pattern` in `word` spans the boundary b.
fn inside(pattern: &Regex, word: &[char], b: usize) -> bool {
    let at: usize = word[..b].iter().map(|c| c.len_utf8()).sum();
    pattern
        .find_iter(&text(word))
        .any(|m| m.map(|m| m.start() < at && at < m.end()).expect("regex match"))
}

/// The place in OBSOLETE, the syllabification and the current lemma of an obsolete
/// `lemma` (alinea: 0, a-li-ne-a, alinia).
pub fn obsolete(lemma: &str) -> Option<(usize, &'static str, &'static str)> {
    OBSOLETE
        .iter()
        .position(|(old, ..)| old.chars().filter(|&c| c != '-').eq(lemma.chars()))
        .map(|i| (i, OBSOLETE[i].0, OBSOLETE[i].1))
}

/// True if `word` starts with the stem of an obsolete lemma in OBSOLETE (alineez).
pub fn has_obsolete_stem(word: &str) -> bool {
    OBSOLETE.iter().any(|(_, _, stem, _)| word.starts_with(stem))
}

/// A `word` missing from the database with the stem of an obsolete lemma that dexonline
/// lists without its forms: the word with the current lemma's stem instead, and the
/// obsolete lemma, syllabified (rachiareasa -> rachiereasa, ra-chi-a-rea-să; not Alinei).
pub fn obsolete_stem(word: &str) -> Option<(String, &'static str)> {
    OBSOLETE
        .iter()
        .find(|&&(_, _, stem, missing)| missing && word.starts_with(stem))
        .map(|&(old, new, stem, _)| {
            let n = stem.chars().count();
            let current: String = new.chars().take(n).chain(word.chars().skip(n)).collect();
            (current, old)
        })
}

/// True if `lemma`, a feminine noun or unclassified in dexonline, is a long
/// infinitive in -iare (apreciare, tăiare; not fiare). Input normalized.
pub fn long_infinitive(lemma: &str) -> bool {
    lemma.ends_with("iare") && !IARE_KEPT.contains(&lemma)
}

/// The -iere spelling of a long infinitive in -iare, its form or its
/// hyphenation: the last -iar-/-iăr- becomes -ier- (apreciare -> apreciere,
/// apreciărilor -> aprecierilor, a-pre-ci-a-re -> a-pre-ci-e-re).
pub fn iere_spelling(word: &str) -> String {
    IAR.replacen(word, 1, "${1}e${2}").into_owned()
}

/// Syllabified `word` and its source.
///
/// A syllabification in SYLLABIFICATIONS is used as is (its first reading;
/// see syllabify_all). Otherwise, cuts inside the span covered by the
/// dexonline hyphenation are taken from it and prevail. Elsewhere only cuts
/// between two vowels are placed, by the fallback rules; those are the only
/// cuts grafia concisă uses. `known` holds the words the prefix rule may
/// find; `verb` marks a verb infinitive. Input normalized.
pub fn syllabify(word: &str, hyphenation: &str, known: &HashSet<String>, verb: bool) -> (String, Source) {
    if let Some(readings) = syllabifications(word) {
        return (readings[0].to_string(), Source::Manual);
    }
    let w = chars(word);
    let dex = if hyphenation.is_empty() {
        None
    } else {
        dexonline_cuts(&w, hyphenation)
    };
    let source = match &dex {
        None => Source::Rules,
        Some((_, lo, hi)) if *lo <= 1 && hi + 1 >= w.len() => Source::Dexonline,
        Some(_) => Source::Partial,
    };
    let (cuts, lo, hi) = dex.unwrap_or((HashSet::new(), 1, 0));
    let mut out: String = w.iter().take(1).collect();
    for b in 1..w.len() {
        let cut = if lo <= b && b <= hi {
            cuts.contains(&b)
        } else {
            is_vowel(w[b - 1]) && is_vowel(w[b]) && rule_cut(&w, b, known, verb)
        };
        if cut {
            out.push('-');
        }
        out.push(w[b]);
    }
    (out, source)
}

/// Every reading of `word`: all SYLLABIFICATIONS entries, else syllabify's one.
pub fn syllabify_all(word: &str, hyphenation: &str, known: &HashSet<String>, verb: bool) -> Vec<(String, Source)> {
    match syllabifications(word) {
        Some(readings) => readings.iter().map(|s| (s.to_string(), Source::Manual)).collect(),
        None => vec![syllabify(word, hyphenation, known, verb)],
    }
}

/// Concise spelling of `form`, with syllable cuts taken from `reference`.
///
/// `reference` is the syllabified lemma, which is the form itself when the
/// form is a headword. Both normalized. For a vowel pair at i, i+1:
///   - both letters in the prefix shared with the lemma: lemma's cut decides
///     (reală <- re-al: hiatus; soarele <- soa-re: diphthong);
///   - an i whose following lemma vowel is replaced at the same place:
///     lemma's cut decides (familia <- fa-mi-li-e: hiatus; femeia <-
///     fe-me-ie: diphthong), except that replacing the final u of an -iu
///     lemma gives a hiatus (argintie <- ar-gin-tiu, cheflie <- che-fliu);
///   - an ending appended after the final i of a verb (`verb`) whose i
///     follows a consonant: hiatus (știa, știam <- ști, neștie <- ne-ști;
///     not locuia <- lo-cu-i);
///   - otherwise (appended ending, stem alternation, or o/e before a
///     replaced vowel): diphthong (ideea <- i-de-e, creează <- cre-a,
///     doarme <- dor-mi, ploaie <- plo-ua, scheaună <- sche-u-na).
///
/// The suffix -ează/-ească after a hiatus i keeps an e form, as in the other
/// forms: -ază of an i-a verb -> -ėză (studiază <- stu-di-a, like studiez),
/// -ască of an i-i verb -> -ėscă (sfiască <- sfi-i, like sfiesc), -ască/-asca
/// of an i-esc adjective -> -ėscă/-ėsca (fiască, fiasca <- fi-esc, like fiesc).
pub fn concise_from_reference(form: &str, reference: &str, verb: bool) -> String {
    let lemma: Vec<char> = reference.chars().filter(|&c| c != '-').collect();
    let f = chars(form);
    let parts: Vec<&str> = reference.split('-').collect();
    let (mut cuts, mut k) = (HashSet::new(), 0);
    for part in &parts[..parts.len() - 1] {
        k += part.chars().count();
        cuts.insert(k);
    }
    let mut p = 0;
    while p < f.len().min(lemma.len()) && f[p] == lemma[p] {
        p += 1;
    }

    let hiatus = |i: usize| {
        if i + 1 < p {
            return cuts.contains(&(i + 1));
        }
        if i + 1 == p && p < lemma.len() && is_vowel(lemma[p]) && f[i] == 'i' {
            return cuts.contains(&p) || lemma[p - 1..] == ['i', 'u'];
        }
        // știa, știam <- ști; not locuia <- lo-cu-i
        verb && i + 1 == p && p == lemma.len() && f[i] == 'i' && p >= 2 && !is_vowel(lemma[p - 2])
    };

    let mut out = Vec::with_capacity(f.len());
    let mut i = 0;
    while i < f.len() {
        match f.get(i + 1).and_then(|&next| digraph(f[i], next)) {
            Some(c) if !hiatus(i) => {
                out.push(c);
                i += 2;
            }
            _ => {
                out.push(f[i]);
                i += 1;
            }
        }
    }
    let lemma = text(&lemma);
    if reference.ends_with("i-a") && form == lemma.clone() + "ză" {
        return drop_last(&out, 3) + "ėză";
    }
    if reference.ends_with("i-i") && form == drop_last(&chars(&lemma), 1) + "ască" {
        return drop_last(&out, 4) + "ėscă";
    }
    let stem = drop_last(&chars(&lemma), 3);
    if reference.ends_with("i-esc") && (form == stem.clone() + "ască" || form == stem + "asca") {
        return drop_last(&out, 4) + "ė" + &text(&f[f.len() - 3..]);
    }
    text(&out)
}

fn digraph(a: char, b: char) -> Option<char> {
    DIGRAPHS.iter().find(|(pair, _)| *pair == [a, b]).map(|&(_, c)| c)
}

/// 'soa-re' -> 'sōre'; 're-al' -> 'real'. Input must be normalized.
pub fn concise_from_syllables(syllabified: &str) -> String {
    concise_from_reference(&syllabified.replace('-', ""), syllabified, false)
}

/// The -eu lemma of a plural in -eele/-eelor: bombeele, bombeelor -> bombeu.
pub fn eu_lemma(word: &str) -> Option<String> {
    ["eele", "eelor"]
        .iter()
        .find(|end| word.ends_with(*end))
        .map(|end| word[..word.len() - (end.len() - 1)].to_string() + "u")
}

/// The endings of an -esc adjective (fiesc: fiești, fiescu, ..., fiască, fiasca)
pub const IESC_ENDINGS: [&str; 9] = [
    "ești", "escu", "escul", "escului", "eștii", "eștile", "eștilor", "ască", "asca",
];

/// The -iesc lemma of an adjective form: harabagiasca, harabagiești -> harabagiesc.
pub fn iesc_lemma(word: &str) -> Option<String> {
    IESC_ENDINGS
        .iter()
        .find(|end| word.ends_with(&format!("i{end}")))
        .map(|end| word[..word.len() - end.len()].to_string() + "esc")
}

/// Lemmas an unknown word may have, in order of preference.
///
/// For the -ee rule: andreei -> andreea, andree; bombeele, bombeuri, bombeurile
/// -> bombeu. For an -iesc adjective: harabagiasca -> harabagiesc. A candidate
/// in -eea, -eu or -iesc counts as known when it is a lemma, one in -ee when it
/// is any form.
pub fn lemma_candidates(word: &str) -> Vec<String> {
    if let Some(iesc) = iesc_lemma(word) {
        return vec![iesc];
    }
    if word.ends_with("eei") {
        let stem = &word[..word.len() - 1];
        return vec![stem.to_string() + "a", stem.to_string()];
    }
    if ["euri", "eurile", "eurilor"].iter().any(|end| word.ends_with(end)) {
        let eu = word.rfind("eu").expect("an -eu ending");
        return vec![word[..eu + 2].to_string()];
    }
    eu_lemma(word).into_iter().collect()
}

/// Concise spellings of a word missing from the database, by rules alone.
///
/// Input normalized; the default spelling comes first. `known` maps known
/// lemma candidates to their syllabification ('' when not needed). A form of
/// an -iesc adjective in `known` is derived from it (harabagiasca ->
/// harabagiėsca, harabagiești <- harabagi-esc). With the -ee switch the word is read
/// as a singular, except:
///   - -eei: the genitive of the first of its `lemma_candidates` in `known`
///     (Andreii <- Andreea, ideii <- idee), otherwise unchanged (the default)
///     or -eii (Zzeei, Zzeii);
///   - -euri/-eurile/-eurilor: -eẹ/-eẹle/-eẹlor if its -eu lemma is in `known`
///     (bombeuri <- bombeu), otherwise unchanged;
///   - -eele/-eelor: the plural of its -eu lemma if that is in `known`
///     (bombeẹle <- bombeu), otherwise of an -ee lemma (bombeile, the default)
///     or of an -eu lemma (bombeẹle).
pub fn concise_by_rules(word: &str, ee: bool, known: &HashMap<String, String>) -> Vec<String> {
    if let Some(reference) = iesc_lemma(word).and_then(|iesc| known.get(&iesc)) {
        return vec![concise_from_reference(word, reference, false)];
    }
    let concise = concise_from_syllables(&syllabify(word, "", &HashSet::new(), false).0);
    if ee && word.ends_with("eei") {
        if let Some(lemma) = lemma_candidates(word).iter().find(|x| known.contains_key(*x)) {
            return vec![concise_ee(word, &concise, lemma, false).unwrap_or(concise)];
        }
        let genitive = drop_last(&chars(&concise), 3) + "eii";
        return vec![concise, genitive];
    }
    if ee && ["euri", "eurile", "eurilor"].iter().any(|end| word.ends_with(end)) {
        let lemmas: Vec<String> = lemma_candidates(word)
            .into_iter()
            .filter(|x| known.contains_key(x))
            .collect();
        let spelling = lemmas.first().and_then(|lemma| concise_ee(word, &concise, lemma, true));
        return vec![spelling.unwrap_or(concise)];
    }
    if ee && let Some(eu) = eu_lemma(word) {
        let lemmas = if known.contains_key(&eu) {
            vec![eu]
        } else {
            vec![eu[..eu.len() - 2].to_string() + "ee", eu]
        };
        return lemmas
            .iter()
            .map(|lemma| concise_ee(word, &concise, lemma, true).unwrap_or(concise.clone()))
            .collect();
    }
    if ee && found(&EE_FINAL, word) {
        return vec![concise_ee(word, &concise, word, false).unwrap_or(concise)];
    }
    vec![concise]
}

/// Apply the -ee rule on top of the base concise form; None if it doesn't apply.
///
/// All arguments normalized; `form` is the standard spelling of `concise`.
pub fn concise_ee(form: &str, concise: &str, lemma: &str, plural: bool) -> Option<String> {
    let c = chars(concise);
    for (end, new) in [("euri", "eẹ"), ("eurile", "eẹle"), ("eurilor", "eẹlor")] {
        if form.ends_with(end) {
            // the -eu lemma itself, longer than 4 letters: bombeu, not pneu
            if lemma == &form[..form.len() - (end.len() - 2)] && lemma.chars().count() > 4 {
                return Some(drop_last(&c, end.len()) + new);
            }
            return None;
        }
    }
    if form.ends_with("eea") || form.ends_with("eei") {
        let lemma_ok =
            lemma.ends_with("ee") || lemma.ends_with("eu") || lemma.ends_with("eea") && !EEA_UNCHANGED.contains(&lemma);
        if !lemma_ok {
            return None;
        }
        if form.ends_with("eei") {
            return Some(drop_last(&c, 3) + "eii");
        }
        if concise.ends_with("eē") {
            // i-de-ea
            return Some(drop_last(&c, 1) + "ė");
        }
        return None;
    }
    if form.ends_with("ee") {
        if !plural || lemma.ends_with("eu") {
            return Some(drop_last(&c, 2) + "eẹ");
        }
        if lemma.ends_with("ee") {
            return Some(drop_last(&c, 2) + "ei");
        }
        return None;
    }
    if lemma.ends_with("eu") {
        if form.ends_with("eele") {
            return Some(drop_last(&c, 4) + "eẹle");
        }
        if form.ends_with("eelor") {
            return Some(drop_last(&c, 5) + "eẹlor");
        }
        return None;
    }
    if !lemma.ends_with("ee") {
        return None;
    }
    if form.ends_with("eele") {
        return Some(drop_last(&c, 4) + "eile");
    }
    if form.ends_with("eelor") {
        return Some(drop_last(&c, 5) + "eilor");
    }
    None
}

/// â -> î in every word of `text` (NFC), except those with the root român/rumân.
pub fn a_to_i(text: &str) -> String {
    WORD.replace_all(text, |caps: &Captures<str>| {
        let word = &caps[0];
        if found(&A_KEPT, word) {
            word.to_string()
        } else {
            word.replace('â', "î").replace('Â', "Î")
        }
    })
    .into_owned()
}

/// Python's str.isupper(): at least one cased character, and no lowercase or titlecase one.
fn is_upper(s: &str) -> bool {
    let mut cased = false;
    for c in s.chars() {
        if c.is_lowercase() || is_titlecase(c) {
            return false;
        }
        cased |= c.is_uppercase();
    }
    cased
}

fn is_titlecase(c: char) -> bool {
    !c.is_uppercase() && !c.is_lowercase() && !c.to_uppercase().eq(std::iter::once(c))
}

/// Carry the case pattern of `template` (ALL CAPS / Initial / lower) onto `word`.
pub fn apply_case(template: &str, word: &str) -> String {
    if template.chars().count() > 1 && is_upper(template) {
        return word.to_uppercase();
    }
    let first = template.chars().next().map(String::from).unwrap_or_default();
    if is_upper(&first) {
        let mut w = word.chars();
        return w
            .next()
            .map(|c| c.to_uppercase().collect::<String>())
            .unwrap_or_default()
            + w.as_str();
    }
    word.to_string()
}

/// A `forms` row: the concise spelling, the one with the -ee rule (None if
/// identical), a homograph's other spellings, and an obsolete form's current forms
/// (alineare: aliniere, its concise spelling the current form's and its own the
/// alternative; alineam: alinam and aliniam, its spellings its own two).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Row {
    pub concise: String,
    pub concise_ee: Option<String>,
    pub alternatives: Vec<String>,
    pub current: Vec<String>,
}

/// A piece of converted text: `t`, plus `alts` (all its spellings, default
/// first; empty if one), `unknown` (the original word, if missing from the
/// database) and `obsolete` (an obsolete word, then its current forms, in the word's case:
/// [Alineare, Aliniere], [Alineam, Alinam, Aliniam]). In JSON, `alts`, `unknown` and `obsolete` are left out when empty.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct Segment {
    pub t: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alts: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub obsolete: Vec<String>,
}

/// A word's spellings, default first; whether it is unknown; and an obsolete word's current forms.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Resolution {
    pub spellings: Vec<String>,
    pub unknown: bool,
    pub current: Vec<String>,
}

/// Normalized candidate words -> their resolution.
///
/// Words missing from the database are converted by rules alone (concise_by_rules),
/// and are unknown unless they have no digraph or -ee ending (zziăr: nothing to change).
/// Their lemma candidates are looked up with known_lemmas. One with an obsolete stem
/// (OBSOLETE) is converted with the current stem, and its own spelling, from the
/// obsolete lemma's syllabification, is the alternative (rachiareasa: rachierēsa or
/// rachiarēsa).
pub fn resolve(keys: &BTreeSet<String>, dict: &dyn Dictionary, ee: bool) -> HashMap<String, Resolution> {
    let rows: HashMap<&String, Row> = keys.iter().filter_map(|k| Some((k, dict.form(k)?))).collect();
    let candidates: BTreeSet<String> = keys
        .iter()
        .filter(|k| !rows.contains_key(k))
        .flat_map(|k| lemma_candidates(k))
        .collect();
    let known = crate::known_lemmas(dict, &candidates);
    let mut out = HashMap::new();
    for k in keys {
        let entry = match rows.get(k) {
            Some(row) => {
                let first = match &row.concise_ee {
                    Some(e) if ee => e.clone(),
                    _ => row.concise.clone(),
                };
                Resolution {
                    spellings: std::iter::once(first).chain(row.alternatives.iter().cloned()).collect(),
                    unknown: false,
                    current: row.current.clone(),
                }
            }
            None => {
                let changeable = found(&DIGRAPH, k) || ee && found(&EE_FINAL, k);
                match obsolete_stem(k) {
                    Some((current, reference)) => {
                        let mut spellings = vec![concise_by_rules(&current, ee, &known).swap_remove(0)];
                        let old = concise_from_reference(k, reference, false);
                        if old != spellings[0] {
                            spellings.push(old);
                        }
                        Resolution {
                            spellings,
                            unknown: changeable,
                            current: vec![current],
                        }
                    }
                    None => Resolution {
                        spellings: concise_by_rules(k, ee, &known),
                        unknown: changeable,
                        current: Vec::new(),
                    },
                }
            }
        };
        out.insert(k.clone(), entry);
    }
    out
}

/// Appends plain text, merged into the last segment when that is plain too.
fn plain(segments: &mut Vec<Segment>, chunk: &str) {
    if chunk.is_empty() {
        return;
    }
    match segments.last_mut() {
        Some(last) if last.alts.is_empty() && last.unknown.is_none() && last.obsolete.is_empty() => {
            last.t.push_str(chunk)
        }
        _ => segments.push(Segment {
            t: chunk.to_string(),
            ..Segment::default()
        }),
    }
}

/// Split text into segments.
///
/// `ai` switches on â -> î (see a_to_i), applied to every word after the lookup.
///
/// Words without a candidate digraph are never looked up. Candidate words
/// missing from the database are converted by rules alone; their segment's
/// `unknown` holds the original word. A homograph's segment, or an unknown
/// word's with several spellings, has `alts`: all its spellings, the default
/// (also in `t`) first.
pub fn convert_text(text: &str, dict: &dyn Dictionary, ee: bool, ai: bool) -> Vec<Segment> {
    let text: String = text.nfc().collect();
    let matches: Vec<_> = WORD
        .find_iter(&text)
        .map(|m| m.expect("regex match"))
        .filter(|m| has_candidate(m.as_str(), ee))
        .collect();
    let keys = matches.iter().map(|m| normalize(m.as_str())).collect();
    let resolved = resolve(&keys, dict, ee);

    let mut segments = Vec::new();
    let mut pos = 0;
    for m in &matches {
        plain(&mut segments, &text[pos..m.start()]);
        let (word, key) = (m.as_str(), normalize(m.as_str()));
        let Resolution {
            spellings,
            unknown,
            current,
        } = &resolved[&key];
        let obsolete: Vec<String> = if current.is_empty() {
            Vec::new()
        } else {
            std::iter::once(word.to_string())
                .chain(current.iter().map(|c| apply_case(word, c)))
                .collect()
        };
        // all hiatus: keep the original spelling untouched
        let options: Vec<String> = spellings
            .iter()
            .map(|x| {
                if *x == key {
                    word.to_string()
                } else {
                    apply_case(word, x)
                }
            })
            .collect();
        if *unknown || options.len() > 1 || !obsolete.is_empty() {
            let alts = if options.len() > 1 { options.clone() } else { Vec::new() };
            segments.push(Segment {
                t: options[0].clone(),
                alts,
                unknown: unknown.then(|| word.to_string()),
                obsolete,
            });
        } else {
            plain(&mut segments, &options[0]);
        }
        pos = m.end();
    }
    plain(&mut segments, &text[pos..]);
    if ai {
        for s in &mut segments {
            s.t = a_to_i(&s.t);
            s.alts = s.alts.iter().map(|x| a_to_i(x)).collect();
        }
    }
    segments
}

fn revert_word(word: &str) -> String {
    let all_upper = word.chars().count() > 1 && is_upper(word);
    let suffix = HIATUS_SUFFIX.find(word).expect("regex match").map(|m| m.start());
    let mut out = String::with_capacity(word.len() + 4);
    for (k, ch) in word.char_indices() {
        let low: String = ch.to_lowercase().collect();
        let pair = low
            .chars()
            .next()
            .filter(|_| low.chars().count() == 1)
            .and_then(reverse);
        if suffix == Some(k) {
            out.push(if ch.is_uppercase() { 'A' } else { 'a' });
        } else if let Some([a, b]) = pair {
            if low == ch.to_string() {
                out.extend([a, b]);
            } else if all_upper {
                out.extend(a.to_uppercase().chain(b.to_uppercase()));
            } else {
                out.extend(a.to_uppercase());
                out.push(b);
            }
        } else {
            out.push(ch);
        }
    }
    out
}

fn reverse(c: char) -> Option<[char; 2]> {
    DIGRAPHS.iter().find(|&&(_, x)| x == c).map(|&(pair, _)| pair)
}

/// Concise -> standard by character map, except the suffix
/// -iėză/-iėscă/-iėsca -> -iază/-iască/-iasca (studiėză, sfiėscă, fiėsca).
///
/// With `dict` (the -ee switch), the `ee_revert` table maps normalized -ee
/// spellings to (the standard form they revert to, the other forms of a tie)
/// where the character map alone is not the answer (aleẹ -> alee, orhideile ->
/// orhideele; bordeẹ -> bordeie or bordee). Segments as in convert_text: a tie's
/// segment has `alts`, all its forms, default first.
///
/// Exact for the base rules (but fiiască -> fiėscă reverts to fiască, fiiasca likewise).
/// With the -ee rule, exact for the database forms it picks (aleẹ: alee, not aleie);
/// unknown words keep the character map (zzeẹ -> zzeie).
/// î is left as is, so not exact either once â -> î was applied (cîine stays cîine).
pub fn revert_segments(text: &str, dict: Option<&dyn Dictionary>) -> Vec<Segment> {
    let text: String = text.nfc().collect();
    let words: Vec<_> = WORD.find_iter(&text).map(|m| m.expect("regex match")).collect();
    let mut found_ee: HashMap<String, (String, Vec<String>)> = HashMap::new();
    if let Some(dict) = dict {
        for m in &words {
            let key = normalize(m.as_str());
            if found(&EE_SPELLING, &key)
                && let Some(entry) = dict.ee_revert(&key)
            {
                found_ee.insert(key, entry);
            }
        }
    }
    let mut segments = Vec::new();
    let mut pos = 0;
    for m in &words {
        plain(&mut segments, &text[pos..m.start()]);
        let word = m.as_str();
        match found_ee.get(&normalize(word)) {
            Some((form, alts)) => {
                let options: Vec<String> = std::iter::once(form).chain(alts).map(|x| apply_case(word, x)).collect();
                if alts.is_empty() {
                    plain(&mut segments, &options[0]);
                } else {
                    segments.push(Segment {
                        t: options[0].clone(),
                        alts: options,
                        ..Segment::default()
                    });
                }
            }
            None => plain(&mut segments, &revert_word(word)),
        }
        pos = m.end();
    }
    plain(&mut segments, &text[pos..]);
    segments
}

/// Concise -> standard by character map (see revert_segments), without the -ee switch.
pub fn revert_text(text: &str) -> String {
    revert_segments(text, None).into_iter().map(|s| s.t).collect()
}
