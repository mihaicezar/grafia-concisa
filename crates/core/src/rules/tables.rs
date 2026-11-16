//! The exception tables of the fallback rules (`reguli/diftong-ia.yaml`, `hiat-ea.yaml`,
//! `hiat-ie.yaml`, `hiat-oa.yaml`): their entries, and how an entry applies to a pair of vowels.
//! The tables themselves are compiled from the rule files (see `lists.rs`), and the step of
//! `rule_cut` that consults each one is in `syllables.rs`.
use super::*;

// The lines of the rules in their files, for the trace (see `codegen.rs`): SYLLABIFICATION_LINES,
// the line of each word of SYLLABIFICATIONS; TABLES, every table; ENTRY_LINES, the line of each
// entry of each of them. They change nothing in a spelling, so RULES_HASH doesn't hash them.
include!(concat!(env!("OUT_DIR"), "/reguli_meta.rs"));

/// An exception table: its file, the pair of vowels it decides, whether its entries cut that
/// pair (a hiatus) or not (a diphthong), and its entries, in order.
#[derive(Debug, PartialEq, Eq)]
pub struct Table {
    pub file: &'static str,
    pub pair: [char; 2],
    pub cut: bool,
    pub entries: &'static [Entry],
}

/// An entry of a table. It applies to a pair of vowels when one of its patterns holds both of
/// them, and its contexts hold just before and just after the pattern.
#[derive(Debug, PartialEq, Eq)]
pub struct Entry {
    /// Letters, with `^` for the word's start and `$` for its end.
    pub tipar: &'static [&'static str],
    /// Just before the pattern: one of these, if any (`^` alone: the word's start; `^gor`: the
    /// word starts with gor).
    pub ante: &'static [&'static str],
    /// Just before the pattern: none of these.
    pub non_ante: &'static [&'static str],
    /// Just after the pattern: one of these, if any (`$` alone: the word's end; `e$`: e, then the
    /// word's end).
    pub post: &'static [&'static str],
    /// Just after the pattern: none of these.
    pub non_post: &'static [&'static str],
}

/// The letters of a pattern of an entry over a pair of vowels, and what its anchors and its
/// contexts say of them: the index of the pattern, whether its anchors hold, and the first item
/// of each context that holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Occurrence {
    pub pattern: usize,
    pub anchors: bool,
    pub ante: Option<usize>,
    pub non_ante: Option<usize>,
    pub post: Option<usize>,
    pub non_post: Option<usize>,
}

impl Table {
    /// The first entry that applies to the pair of vowels before `b` (word[b-1], word[b]), if it
    /// is the table's pair.
    pub fn entry(&self, word: &[char], b: usize) -> Option<usize> {
        if [word[b - 1], word[b]] != self.pair {
            return None;
        }
        self.entries.iter().position(|e| e.applies(word, b))
    }

    /// The line of its entry `i` in its file.
    pub fn line(&self, i: usize) -> usize {
        let t = TABLES
            .iter()
            .position(|t| t.file == self.file)
            .expect("a table of TABLES");
        ENTRY_LINES[t][i]
    }
}

impl Entry {
    /// True if the entry applies to the pair of vowels before `b`.
    pub fn applies(&self, word: &[char], b: usize) -> bool {
        self.occurrences(word, b).iter().any(|o| o.applies(self))
    }

    /// Every place where the letters of one of its patterns hold the pair of vowels before `b`.
    pub fn occurrences(&self, word: &[char], b: usize) -> Vec<Occurrence> {
        let mut found = Vec::new();
        for (pattern, p) in self.tipar.iter().enumerate() {
            let (start, rest) = p.strip_prefix('^').map_or((false, *p), |r| (true, r));
            let (end, letters) = rest.strip_suffix('$').map_or((false, rest), |r| (true, r));
            let n = letters.chars().count();
            // the pattern holds word[b-1] and word[b]: it starts at b-1 at the latest, and ends at
            // b+1 at the earliest
            for s in (b + 1).saturating_sub(n)..b {
                if !holds(letters, word, s) {
                    continue;
                }
                let e = s + n;
                let first_before = |items: &[&str]| items.iter().position(|i| before(i, word, s));
                let first_after = |items: &[&str]| items.iter().position(|i| after(i, word, e));
                found.push(Occurrence {
                    pattern,
                    anchors: (!start || s == 0) && (!end || e == word.len()),
                    ante: first_before(self.ante),
                    non_ante: first_before(self.non_ante),
                    post: first_after(self.post),
                    non_post: first_after(self.non_post),
                });
            }
        }
        found
    }
}

impl Occurrence {
    /// True if `entry`, whose occurrence this is, applies there: its anchors hold, one item of
    /// each of its contexts ante and post (if it has them) holds, and none of non_ante and
    /// non_post.
    pub fn applies(&self, entry: &Entry) -> bool {
        self.anchors
            && (entry.ante.is_empty() || self.ante.is_some())
            && self.non_ante.is_none()
            && (entry.post.is_empty() || self.post.is_some())
            && self.non_post.is_none()
    }
}

/// True if `word` holds the letters of `letters` from `at`.
fn holds(letters: &str, word: &[char], at: usize) -> bool {
    (at..).zip(letters.chars()).all(|(i, c)| word.get(i) == Some(&c))
}

/// True if the item of ante `item` ends right before `s` (from the word's start, with a `^`).
fn before(item: &str, word: &[char], s: usize) -> bool {
    let (start, letters) = item.strip_prefix('^').map_or((false, item), |r| (true, r));
    let n = letters.chars().count();
    s >= n && holds(letters, word, s - n) && (!start || s == n)
}

/// True if the item of post `item` starts right at `e` (up to the word's end, with a `$`).
fn after(item: &str, word: &[char], e: usize) -> bool {
    let (end, letters) = item.strip_suffix('$').map_or((false, item), |r| (true, r));
    holds(letters, word, e) && (!end || e + letters.chars().count() == word.len())
}
