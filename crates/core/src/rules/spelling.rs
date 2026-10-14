//! The concise spelling of a form, from the cuts of its syllabified lemma.
use std::collections::HashSet;

use super::*;

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

pub(super) fn digraph(a: char, b: char) -> Option<char> {
    DIGRAPHS.iter().find(|(pair, _)| *pair == [a, b]).map(|&(_, c)| c)
}

/// 'soa-re' -> 'sōre'; 're-al' -> 'real'. Input must be normalized.
pub fn concise_from_syllables(syllabified: &str) -> String {
    concise_from_reference(&syllabified.replace('-', ""), syllabified, false)
}
