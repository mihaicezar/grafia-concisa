//! Standard -> concise: the database rows, the resolution of words, the text split into segments.
use std::collections::{BTreeSet, HashMap};

use serde::Serialize;
use unicode_normalization::UnicodeNormalization;

use super::*;
use crate::Dictionary;

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
/// [Alineare, Aliniere], [Alineam, Alinam, Aliniam]), or `literal` (the text of a `[ ]` span, brackets dropped, left
/// as it is). In JSON, `alts`, `unknown`, `obsolete` and `literal` are left out when empty or false.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct Segment {
    pub t: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alts: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub obsolete: Vec<String>,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub literal: bool,
}

impl Segment {
    /// True for text with nothing to show or choose: no alternatives, no unknown or obsolete word, not a span.
    pub fn is_plain(&self) -> bool {
        self.alts.is_empty() && self.unknown.is_none() && self.obsolete.is_empty() && !self.literal
    }
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
                let changeable = found(&DIGRAPH, k) || ee && ee_final(k);
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
pub(super) fn plain(segments: &mut Vec<Segment>, chunk: &str) {
    if chunk.is_empty() {
        return;
    }
    match segments.last_mut() {
        Some(last) if last.is_plain() => last.t.push_str(chunk),
        _ => segments.push(Segment {
            t: chunk.to_string(),
            ..Segment::default()
        }),
    }
}

/// Converts a text with no span: segments, words without a candidate digraph never looked up,
/// candidate words missing from the database converted by rules alone (`unknown` holds the
/// original word). A homograph's segment, or an unknown word's with several spellings, has `alts`:
/// all its spellings, the default (also in `t`) first.
fn convert_chunk(text: &str, dict: &dyn Dictionary, ee: bool) -> Vec<Segment> {
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
                ..Segment::default()
            });
        } else {
            plain(&mut segments, &options[0]);
        }
        pos = m.end();
    }
    plain(&mut segments, &text[pos..]);
    segments
}

/// Split text into segments. The inside of a `[ ]` span (see split_literals) is left as it is, in
/// a segment of its own (`literal`), without its brackets; the rest is converted (convert_chunk).
///
/// `ai` switches on â -> î (see a_to_i), applied to every word after the lookup, except those
/// of a span.
pub fn convert_text(text: &str, dict: &dyn Dictionary, ee: bool, ai: bool) -> Vec<Segment> {
    let mut segments = Vec::new();
    for piece in split_literals(text) {
        match piece {
            Piece::Text(chunk) => {
                for s in convert_chunk(chunk, dict, ee) {
                    if s.is_plain() {
                        plain(&mut segments, &s.t);
                    } else {
                        segments.push(s);
                    }
                }
            }
            Piece::Literal(inside) => segments.push(Segment {
                t: inside.to_string(),
                literal: true,
                ..Segment::default()
            }),
        }
    }
    if ai {
        for s in segments.iter_mut().filter(|s| !s.literal) {
            s.t = a_to_i(&s.t);
            s.alts = s.alts.iter().map(|x| a_to_i(x)).collect();
        }
    }
    segments
}
