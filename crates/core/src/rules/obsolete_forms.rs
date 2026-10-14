//! Obsolete forms and long infinitives in -iare, written as their current forms.
use std::sync::LazyLock;

use fancy_regex::Regex;

use super::*;

pub(super) static IAR: LazyLock<Regex> = LazyLock::new(|| re(r"(.*i-?)[aă](-?r)")); // the last -iar-/-iăr- of a word

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
