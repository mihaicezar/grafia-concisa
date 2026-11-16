//! The hand lists: decisions of Cezar, by word.
use super::*;

// The lists of the rule files of reguli/, compiled by the build script (codegen.rs); the files'
// comments say what each holds: SYLLABIFICATIONS (silabisiri.yaml), each word with its
// readings, the default first; OBSOLETE (invechite.yaml: leme), each (lema, actuala, tulpina,
// fara_forme); one constant per list of words, which grafia_reguli::LISTS names with its file
// and key (IARE_KEPT, NAMES, AE_OE, ...); one Table per exception table of grafia_reguli::TABLES
// (IA_DIPHTHONG, EA_HIATUS, IE_HIATUS, OA_HIATUS); CLASSES (clase.yaml), each class of letters
// with its letters and digraphs; and STEPS (pasi.yaml), the steps of the fallback rules, in
// order (see tables.rs).
include!(concat!(env!("OUT_DIR"), "/reguli.rs"));

/// The readings of `word` in SYLLABIFICATIONS, if listed.
pub fn syllabifications(word: &str) -> Option<&'static [&'static str]> {
    SYLLABIFICATIONS.iter().find(|(w, _)| *w == word).map(|(_, r)| *r)
}
