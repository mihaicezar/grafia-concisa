//! Regexes, normalization, the text of a word: case, characters, â -> î.
use std::sync::LazyLock;

use fancy_regex::{Captures, Regex};
use unicode_normalization::UnicodeNormalization;

use super::*;

/// ie/ia/ea/oa and their concise letters.
pub const DIGRAPHS: [([char; 2], char); 4] = [
    (['i', 'e'], 'ẹ'),
    (['i', 'a'], 'ė'),
    (['e', 'a'], 'ē'),
    (['o', 'a'], 'ō'),
];

pub(super) fn re(pattern: &str) -> Regex {
    Regex::new(pattern).expect("valid pattern")
}

// The suffix -ează/-ească after a hiatus i (studiază, sfiască, fiasca) is written
// -ėză/-ėscă/-ėsca; its ė reverts to a, not ia.
pub static HIATUS_SUFFIX: LazyLock<Regex> = LazyLock::new(|| re(r"(?i)(?<=i)ė(?=(ză|scă|sca)$)"));
// The â -> î switch keeps â only in words with the root român/rumân.
pub static A_KEPT: LazyLock<Regex> = LazyLock::new(|| re(r"(?i)r[ou]mân"));

pub static DIGRAPH: LazyLock<Regex> = LazyLock::new(|| re("ie|ia|ea|oa"));
pub static CANDIDATE: LazyLock<Regex> = LazyLock::new(|| re("ie|ia|ea|oa|iăr")); // iăr: apreciări -> aprecieri
// forms the -ee rule may change, besides those of the lemmas in AE_OE (ae_oe_lemma)
pub static EE_FINAL: LazyLock<Regex> = LazyLock::new(|| re(r"(ee(a|i|le|lor)?|euri(le|lor)?|oae)$"));
pub static EURI: LazyLock<Regex> = LazyLock::new(|| re(r"euri(le|lor)?$")); // -euri plurals, which the -ee rule writes -eẹ, like -ee
// Every spelling the -ee rule gives ends so (ideẹ, bombeẹle, ideė, ideii, zeẹi, orhideile, logorei;
// Naẹ, Băbōẹ, merēẹ; aloė, aloii, Zoẹi, aloile, Zoẹle)
pub static EE_SPELLING: LazyLock<Regex> =
    LazyLock::new(|| re(r"e(ẹ|ẹle|ẹlor|ė|i|ii|ẹi|ile|ilor)$|[aoōē]ẹ$|o(ė|ii|ẹi|ile|ilor|ẹle|ẹlor)$"));
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
    found(&CANDIDATE, &w) || (ee && ee_final(&w)) || has_obsolete_stem(&w)
}

/// True if the -ee rule may change `word` (normalized): an -ee ending (EE_FINAL), or a
/// form of a lemma in AE_OE (Nae, aloei).
pub fn ee_final(word: &str) -> bool {
    found(&EE_FINAL, word) || ae_oe_lemma(word).is_some()
}

pub(super) fn chars(s: &str) -> Vec<char> {
    s.chars().collect()
}

pub(super) fn text(chars: &[char]) -> String {
    chars.iter().collect()
}

/// `word[at:]` starts with `part` (Python: word.startswith(part, at)).
pub(super) fn starts_with_at(word: &[char], part: &[char], at: usize) -> bool {
    word.get(at..).is_some_and(|rest| rest.starts_with(part))
}

/// Python's s[:-n], by character.
pub(super) fn drop_last(chars: &[char], n: usize) -> String {
    text(&chars[..chars.len().saturating_sub(n)])
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
pub(super) fn is_upper(s: &str) -> bool {
    let mut cased = false;
    for c in s.chars() {
        if c.is_lowercase() || is_titlecase(c) {
            return false;
        }
        cased |= c.is_uppercase();
    }
    cased
}

pub(super) fn is_titlecase(c: char) -> bool {
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
