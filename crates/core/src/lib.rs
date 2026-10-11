//! grafia concisă: ie/ia/ea/oa in one syllable (no hiatus) are written ẹ/ė/ē/ō.
//!
//! `rules` holds the rules, `build` the database builder, `datafile` the file the
//! database is shipped in, and this module the lookups and the answers the apps
//! show: `convert`, `revert` and `lookup`. `Conversion` and `Reversion` serialize
//! to the JSON the web page renders.
pub mod build;
pub mod datafile;
pub mod rules;

use std::collections::{BTreeSet, HashMap};

use indexmap::IndexMap;
use serde::Serialize;

pub use datafile::DataFile;
pub use rules::{Row, Segment};

/// The rules a data file is built with: a hash of the code of `rules.rs` and
/// `build.rs`, without comments or whitespace. The data file records it, and the
/// web app's build refuses a data file built with other rules.
pub const RULES_HASH: u64 = code_hash(
    code_hash(FNV_OFFSET, include_str!("rules.rs")),
    include_str!("build.rs"),
);

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0100_0000_01b3;

/// FNV-1a of `source` from `hash`, skipping whitespace and the lines that are
/// comments (`//`, `///`, `//!`), so that only a change of the code changes it.
const fn code_hash(mut hash: u64, source: &str) -> u64 {
    let bytes = source.as_bytes();
    let (mut i, mut line_start) = (0, true);
    while i < bytes.len() {
        let b = bytes[i];
        if b == b'\n' {
            line_start = true;
        } else if line_start && b == b'/' && i + 1 < bytes.len() && bytes[i + 1] == b'/' {
            while i + 1 < bytes.len() && bytes[i + 1] != b'\n' {
                i += 1;
            }
        } else if !b.is_ascii_whitespace() {
            line_start = false;
            hash = (hash ^ b as u64).wrapping_mul(FNV_PRIME);
        }
        i += 1;
    }
    hash
}

/// The word database: the tables of the spec's *Data* section, by normalized key.
/// Implemented by `build::Tables`, in memory, and by `DataFile`.
pub trait Dictionary {
    /// The `forms` row of a form.
    fn form(&self, form: &str) -> Option<Row>;
    /// True if `lemma` is in `eu_lemmas`.
    fn eu_lemma(&self, lemma: &str) -> bool;
    /// True if `lemma` is in `ea_lemmas`.
    fn ea_lemma(&self, lemma: &str) -> bool;
    /// The syllabification of a lemma in `iesc_lemmas`.
    fn iesc_lemma(&self, lemma: &str) -> Option<String>;
    /// The `ee_revert` entry of an -ee spelling: the form it reverts to, and the other forms of a tie.
    fn ee_revert(&self, concise: &str) -> Option<(String, Vec<String>)>;
}

/// Those of `words` that exist, with their syllabification: lemmas in -eu, -ea or
/// -iesc from their tables (an -iesc lemma's syllabification, '' for the others),
/// any other word (-ee) among the forms.
pub fn known_lemmas(dict: &dyn Dictionary, words: &BTreeSet<String>) -> HashMap<String, String> {
    words
        .iter()
        .filter_map(|w| {
            let found = if w.ends_with("eu") {
                dict.eu_lemma(w).then(String::new)
            } else if w.ends_with("ea") {
                dict.ea_lemma(w).then(String::new)
            } else if w.ends_with("iesc") {
                dict.iesc_lemma(w)
            } else {
                dict.form(w).map(|_| String::new())
            };
            found.map(|s| (w.clone(), s))
        })
        .collect()
}

/// The switches: the optional -ee rule (default off) and â -> î (default on).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Options {
    pub ee: bool,
    pub ai: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options { ee: false, ai: true }
    }
}

/// A converted text: the segments joined, the segments, and the unknown words, sorted.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Conversion {
    pub text: String,
    pub segments: Vec<Segment>,
    pub unknown: Vec<String>,
}

/// Standard -> concise. Words missing from the database are converted by rules
/// alone and listed in `unknown`. A homograph's segment lists all its spellings,
/// default first; so does an unknown word with the -ee switch in -eei (unchanged
/// or -eii) or -eele/-eelor (plural of an -ee or an -eu lemma), unless its lemma
/// is in the database (Andreei <- Andreea, bombeele <- bombeu: one spelling).
pub fn convert(text: &str, dict: &dyn Dictionary, options: Options) -> Conversion {
    let segments = rules::convert_text(text, dict, options.ee, options.ai);
    let unknown: BTreeSet<String> = segments.iter().filter_map(|s| s.unknown.clone()).collect();
    Conversion {
        text: segments.iter().map(|s| s.t.as_str()).collect(),
        segments,
        unknown: unknown.into_iter().collect(),
    }
}

/// A reverted text: the segments joined, and the segments.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Reversion {
    pub text: String,
    pub segments: Vec<Segment>,
}

/// Concise -> standard. With `ee`, the input was written with the -ee rule
/// (aleẹ -> alee, not aleie: see the ee_revert table), and a tie's segment lists
/// all its forms (bordeẹ: bordeie or bordee).
pub fn revert(text: &str, dict: &dyn Dictionary, ee: bool) -> Reversion {
    let segments = rules::revert_segments(text, if ee { Some(dict) } else { None });
    Reversion {
        text: segments.iter().map(|s| s.t.as_str()).collect(),
        segments,
    }
}

/// Words converted one by one: each word's default spelling (lowercase), the
/// unknown words, and all the spellings of those with several, default first.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Lookup {
    pub results: IndexMap<String, String>,
    pub unknown: Vec<String>,
    pub alternatives: IndexMap<String, Vec<String>>,
}

/// Converts each word on its own, as `convert` does.
pub fn lookup(words: &[&str], dict: &dyn Dictionary, options: Options) -> Lookup {
    let keys = words
        .iter()
        .filter(|w| rules::has_candidate(w, options.ee))
        .map(|w| rules::normalize(w))
        .collect();
    let resolved = rules::resolve(&keys, dict, options.ee);
    let mut out = Lookup {
        results: IndexMap::new(),
        unknown: Vec::new(),
        alternatives: IndexMap::new(),
    };
    for &w in words {
        let key = rules::normalize(w);
        let Some((spellings, unknown)) = resolved.get(&key) else {
            out.results.insert(w.to_string(), key);
            continue;
        };
        out.results.insert(w.to_string(), spellings[0].clone());
        if spellings.len() > 1 {
            out.alternatives.insert(w.to_string(), spellings.clone());
        }
        if *unknown {
            out.unknown.push(w.to_string());
        }
    }
    if options.ai {
        out.results.values_mut().for_each(|x| *x = rules::a_to_i(x));
        out.alternatives
            .values_mut()
            .flatten()
            .for_each(|x| *x = rules::a_to_i(x));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_hash_sees_only_code() {
        let hash = |source| code_hash(FNV_OFFSET, source);
        let code = "fn a() -> u8 {\n    1 // one\n}\n";
        // comment lines, indentation and blank lines don't count...
        assert_eq!(
            hash(code),
            hash("//! A.\n/// a\nfn a()\n    -> u8 {\n\n  // why\n1 // one\n}")
        );
        // ...the code does, and so does a comment after it
        assert_ne!(hash(code), hash("fn a() -> u8 {\n    2 // one\n}\n"));
        assert_ne!(hash(code), hash("fn a() -> u8 {\n    1 // two\n}\n"));
        // a single / starts no comment, even as the last byte; a comment may end the file
        assert_ne!(hash("a\n/ b"), hash("a\n/ c"));
        assert_ne!(hash("a\n/"), hash("a\n"));
        assert_eq!(hash("a\n// x"), hash("a"));
        // FNV-1a
        assert_eq!(hash("a"), 0xaf63_dc4c_8601_ec8c);
    }
}
