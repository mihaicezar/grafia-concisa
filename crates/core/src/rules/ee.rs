//! The optional -eẹ/aẹ/oẹ rule, the lemmas of unknown words, and their spelling by rules alone.
use std::collections::{HashMap, HashSet};

use super::*;

/// The lemma in AE_OE of `word` (normalized): the word itself, or the -oe lemma of a form in
/// -oea, -oei, -oele, -oelor (Nae -> nae, aloei -> aloe; not vitae, naei).
pub fn ae_oe_lemma(word: &str) -> Option<&'static str> {
    ["", "a", "i", "le", "lor"].iter().find_map(|end| {
        let stem = word.strip_suffix(end)?;
        AE_OE
            .iter()
            .copied()
            .find(|&w| w == stem && (end.is_empty() || w.ends_with("oe")))
    })
}

/// The name in NAMES of a genitive or an articulated plural in -eei, -eele, -eelor:
/// Andreei, Andreelor -> andreea; Leei -> leea.
pub fn name_lemma(word: &str) -> Option<&'static str> {
    let end = ["eei", "eele", "eelor"].into_iter().find(|end| word.ends_with(end))?;
    let stem = &word[..word.len() - end.len() + 2]; // andree
    NAMES
        .iter()
        .copied()
        .find(|&name| name == stem || name.strip_suffix('a') == Some(stem))
}

/// The -eu lemma of a plural in -eele/-eelor: bombeele, bombeelor -> bombeu.
pub fn eu_lemma(word: &str) -> Option<String> {
    ["eele", "eelor"]
        .iter()
        .find(|end| word.ends_with(*end))
        .map(|end| word[..word.len() - (end.len() - 1)].to_string() + "u")
}

/// The -iesc lemma of an adjective form: harabagiasca, harabagiești -> harabagiesc.
pub fn iesc_lemma(word: &str) -> Option<String> {
    IESC_ENDINGS
        .iter()
        .find(|end| word.ends_with(&format!("i{end}")))
        .map(|end| word[..word.len() - end.len()].to_string() + "esc")
}

/// Lemmas an unknown word may have, in order of preference.
///
/// For the -ee rule: andreei -> andreea, andree; galileele -> galileea, galileu; bombeuri, bombeurile
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
    // -eele/-eelor: an -eea lemma (Galileelor <- Galileea), else an -eu one (bombeele <- bombeu)
    eu_lemma(word)
        .into_iter()
        .flat_map(|eu| [format!("{}eea", &eu[..eu.len() - 2]), eu])
        .collect()
}

/// Concise spellings of a word missing from the database, by rules alone.
///
/// Input normalized; the default spelling comes first. `known` maps known
/// lemma candidates to their syllabification ('' when not needed). A form of
/// an -iesc adjective in `known` is derived from it (harabagiasca ->
/// harabagiėsca, harabagiești <- harabagi-esc). With the -ee switch the word is read
/// as a singular, except:
///   - the genitive or articulated plural of a name in NAMES (name_lemma): Andreẹi,
///     Andreẹlor <- Andreea, Leẹi <- Leea;
///   - a form of a lemma in AE_OE (ae_oe_lemma): Zoẹi <- Zoe, aloii <- aloe;
///   - -eei: the genitive of the first of its `lemma_candidates` in `known`
///     (Niceii <- Niceea, ideii <- idee), otherwise -eẹi (the default) or -eii
///     (Zzeẹi, Zzeii); the masculine of an -ee lemma is not looked for;
///   - -euri/-eurile/-eurilor: -eẹ/-eẹle/-eẹlor if its -eu lemma is in `known`
///     (bombeuri <- bombeu), otherwise unchanged;
///   - -eele/-eelor: the plural of its -eea lemma if that is in `known` and not
///     EEA_UNCHANGED (Galileilor <- Galileea, not by the false masculine galileu); else
///     of coree (EE_NOT_MASCULINE: coreile, not by coreu); else of its -eu lemma if
///     that is in `known` (bombeẹle <- bombeu); otherwise of an -ee lemma (bombeile,
///     the default) or of an -eu lemma (bombeẹle).
pub fn concise_by_rules(word: &str, ee: bool, known: &HashMap<String, String>) -> Vec<String> {
    if let Some(reference) = iesc_lemma(word).and_then(|iesc| known.get(&iesc)) {
        return vec![concise_from_reference(word, reference, false)];
    }
    let concise = concise_from_syllables(&syllabify(word, "", &HashSet::new(), false).0);
    if ee && let Some(name) = name_lemma(word) {
        // the number matters only for a word in -ee
        return vec![concise_ee(word, &concise, name, false, false).unwrap_or(concise)];
    }
    if ee && let Some(lemma) = ae_oe_lemma(word) {
        return vec![concise_ee(word, &concise, lemma, false, false).unwrap_or(concise)];
    }
    if ee && word.ends_with("eei") {
        if let Some(lemma) = lemma_candidates(word).iter().find(|x| known.contains_key(*x)) {
            return vec![concise_ee(word, &concise, lemma, false, false).unwrap_or(concise)];
        }
        let stem = drop_last(&chars(&concise), 2);
        return vec![stem.clone() + "ẹi", stem + "ii"];
    }
    if ee && ["euri", "eurile", "eurilor"].iter().any(|end| word.ends_with(end)) {
        let lemmas: Vec<String> = lemma_candidates(word)
            .into_iter()
            .filter(|x| known.contains_key(x))
            .collect();
        let spelling = lemmas
            .first()
            .and_then(|lemma| concise_ee(word, &concise, lemma, true, false));
        return vec![spelling.unwrap_or(concise)];
    }
    if ee && let Some(eu) = eu_lemma(word) {
        let stem = &eu[..eu.len() - 2];
        let (eea, ee_lemma) = (format!("{stem}eea"), format!("{stem}ee"));
        let lemmas = if known.contains_key(&eea) && !EEA_UNCHANGED.contains(&eea.as_str()) {
            // the plural of a known -eea lemma, not of a false masculine (Iudeelor <- Iudeea, not iudeu)
            vec![eea]
        } else if EE_NOT_MASCULINE.contains(&ee_lemma.as_str()) {
            vec![ee_lemma] // coreele <- coree, not coreu
        } else if known.contains_key(&eu) {
            vec![eu]
        } else {
            vec![ee_lemma, eu]
        };
        return lemmas
            .iter()
            .map(|lemma| concise_ee(word, &concise, lemma, true, false).unwrap_or(concise.clone()))
            .collect();
    }
    if ee && found(&EE_FINAL, word) {
        return vec![concise_ee(word, &concise, word, false, false).unwrap_or(concise)];
    }
    vec![concise]
}

/// Apply the -ee rule on top of the base concise form; None if it doesn't apply.
///
/// All arguments normalized; `form` is the standard spelling of `concise`. `masculine`: the
/// lemma, in -ee, has a masculine in -eu (zee <- zeu). A kept lemma (in -eu, in -ee with a
/// masculine, or a name in NAMES) keeps the e of its genitive and plural: zeẹi, zeẹle, zeẹ
/// (not zeii, zeile, zei, like femeii, femeile, femei).
pub fn concise_ee(form: &str, concise: &str, lemma: &str, plural: bool, masculine: bool) -> Option<String> {
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
    if form.ends_with("oae") {
        return Some(drop_last(&c, 1) + "ẹ"); // Băbōe
    }
    if AE_OE.contains(&lemma) {
        return concise_ae_oe(form, &c, NAMES.contains(&lemma));
    }
    let kept = lemma.ends_with("eu") || lemma.ends_with("ee") && masculine || NAMES.contains(&lemma);
    if form.ends_with("eea") || form.ends_with("eei") {
        let lemma_ok =
            lemma.ends_with("ee") || lemma.ends_with("eu") || lemma.ends_with("eea") && !EEA_UNCHANGED.contains(&lemma);
        if !lemma_ok {
            return None;
        }
        if form.ends_with("eei") {
            return Some(drop_last(&c, 2) + if kept { "ẹi" } else { "ii" });
        }
        if concise.ends_with("eē") {
            // i-de-ea
            return Some(drop_last(&c, 1) + "ė");
        }
        return None;
    }
    if form.ends_with("ee") {
        if !plural || kept {
            return Some(drop_last(&c, 2) + "eẹ");
        }
        if lemma.ends_with("ee") {
            return Some(drop_last(&c, 2) + "ei");
        }
        return None;
    }
    if kept {
        if form.ends_with("eele") {
            return Some(drop_last(&c, 4) + "eẹle");
        }
        if form.ends_with("eelor") {
            return Some(drop_last(&c, 5) + "eẹlor");
        }
        return None;
    }
    // an -ee lemma, or an -eea one that is not a name (Galileelor -> Galileilor)
    if !lemma.ends_with("ee") && !(lemma.ends_with("eea") && !EEA_UNCHANGED.contains(&lemma)) {
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

/// The -ee rule on a form of a lemma in AE_OE, of `concise` (its characters): -ae/-oe ->
/// -aẹ/-oẹ (Naẹ, aloẹ), -oea -> -oė (aloė), and -oei, -oele, -oelor like femeii, femeile
/// (aloii, aloile, aloilor) or, for a `name`, with ẹ (Zoẹi, Zoẹle, Zoẹlor).
pub(super) fn concise_ae_oe(form: &str, concise: &[char], name: bool) -> Option<String> {
    if form.ends_with("ae") || form.ends_with("oe") {
        return Some(drop_last(concise, 1) + "ẹ");
    }
    if form.ends_with("oea") {
        // a-lo-ea
        return concise.ends_with(&['o', 'ē']).then(|| drop_last(concise, 1) + "ė");
    }
    let e = if name { "ẹ" } else { "i" };
    ["i", "le", "lor"]
        .into_iter()
        .find(|end| form.ends_with(&format!("oe{end}")))
        .map(|end| drop_last(concise, end.len() + 1) + e + end)
}
