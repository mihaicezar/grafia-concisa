//! Concise -> standard.
use std::collections::HashMap;

use unicode_normalization::UnicodeNormalization;

use super::*;
use crate::Dictionary;

pub(super) fn revert_word(word: &str) -> String {
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

pub(super) fn reverse(c: char) -> Option<[char; 2]> {
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
/// unknown words keep the character map (zzeẹ -> zzeie), except the spellings of the names
/// in NAMES and the words of AE_OE (Andreẹi -> Andreei, Cloẹ -> Cloe: see listed_form).
/// î is left as is, so not exact either once â -> î was applied (cîine stays cîine).
pub fn revert_segments(text: &str, dict: Option<&dyn Dictionary>) -> Vec<Segment> {
    let text: String = text.nfc().collect();
    let words: Vec<_> = WORD.find_iter(&text).map(|m| m.expect("regex match")).collect();
    let mut found_ee: HashMap<String, (String, Vec<String>)> = HashMap::new();
    if let Some(dict) = dict {
        for m in &words {
            let key = normalize(m.as_str());
            if !found(&EE_SPELLING, &key) {
                continue;
            }
            if let Some(entry) = dict.ee_revert(&key) {
                found_ee.insert(key, entry);
            } else if let Some(form) = listed_form(&key, dict) {
                found_ee.insert(key, (form, Vec::new()));
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

/// The form that `key` (normalized), an -ee spelling missing from `ee_revert`, reverts to when it
/// is the spelling of a name in NAMES or of a word in AE_OE, though missing from the database
/// (Andreẹi -> andreei, Leẹi -> leei, Andreẹlor -> andreelor, Zoẹi -> zoei, Cloẹ -> cloe; with
/// its other digraphs written or not: nēgoẹ, neagoẹ -> neagoe); None when its character-map
/// reading is a known word (laẹ: laie, mamaẹi: mamaiei).
pub(super) fn listed_form(key: &str, dict: &dyn Dictionary) -> Option<String> {
    // the last ẹ is the rule's e (andreẹi, zoẹi, cloẹ), the last ė its ea (aloė)
    let (at, c) = key.char_indices().rev().find(|&(_, c)| c == 'ẹ' || c == 'ė')?;
    let e = if c == 'ẹ' { "e" } else { "ea" };
    let form = revert_word(&format!("{}{e}{}", &key[..at], &key[at + c.len_utf8()..]));
    if (name_lemma(&form).is_none() && ae_oe_lemma(&form).is_none()) || dict.form(&revert_word(key)).is_some() {
        return None;
    }
    // the rule's spelling, read like the key: other digraphs may be left as they are (Neagoẹ: Nēgoẹ)
    let spelled = concise_by_rules(&form, true, &HashMap::new());
    spelled
        .first()
        .is_some_and(|s| revert_word(s) == revert_word(key))
        .then_some(form)
}

/// Concise -> standard by character map (see revert_segments), without the -ee switch.
pub fn revert_text(text: &str) -> String {
    revert_segments(text, None).into_iter().map(|s| s.t).collect()
}
