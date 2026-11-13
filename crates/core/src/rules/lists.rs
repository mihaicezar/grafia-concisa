//! The hand lists: decisions of Cezar, by word.

// The lists of the rule files of reguli/, compiled by the build script (codegen.rs); the files'
// comments say what each holds: SYLLABIFICATIONS (silabisiri.yaml), each word with its
// readings, the default first; OBSOLETE (invechite.yaml: lemme), each (lema, actuala, tulpina,
// fara_forme); and one constant per list of words, which grafia_reguli::LISTS names with its
// file and key (IARE_KEPT, NAMES, AE_OE, ...).
include!(concat!(env!("OUT_DIR"), "/reguli.rs"));

// A long infinitive in -iere after p/b/f/v/m or c/g/ch/gh is a hiatus in all its forms
// (copiere, învierea, aprecierilor), except in these native words and their forms
// (mierea, pieri). After a vowel it is a diphthong anyway (tăiere, înmuiere).
pub const IERE_ENDINGS: [&str; 6] = ["e", "ea", "i", "ii", "ile", "ilor"]; // after -ier: -iere, -ierea, -ieri, ...
// After -ier, ending an agent noun: -ier, -ieră, -iereasă and its forms (plumier, legumieră,
// rachiereasă, rachieresele)
pub const IER_ENDINGS: [&str; 8] = ["", "ă", "easă", "easa", "ese", "esei", "esele", "eselor"];
pub const IERE_DIPHTHONG: [&str; 4] = ["miere", "înmiere", "fiere", "piere"];

// A digraph right after one of these is a hiatus when the rest is a known word
// (re-abilitat, auto-analiză). The rest must have at least 4 letters (not neam,
// autoare), and prea- + a known word is one syllable (prea-curat, not pre-acurat).
pub const PREFIXES: &[&str] = &[
    "re", "pre", "ne", "tele", "co", "semi", "anti", "multi", "auto", "micro", "macro", "foto", "pseudo", "bio", "neo",
    "radio", "aero", "agro", "electro", "hidro", "termo", "geo", "proto", "socio", "psiho", "stereo",
];

/// The readings of `word` in SYLLABIFICATIONS, if listed.
pub fn syllabifications(word: &str) -> Option<&'static [&'static str]> {
    SYLLABIFICATIONS.iter().find(|(w, _)| *w == word).map(|(_, r)| *r)
}
