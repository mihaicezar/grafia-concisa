//! The steps of the fallback rules (`reguli/pasi.yaml`) and their exception tables
//! (`diftong-ia.yaml`, `hiat-ea.yaml`, `hiat-ie.yaml`, `hiat-oa.yaml`): how a step, or an entry
//! of a table, applies to a pair of vowels. The steps, the tables and the classes of letters
//! (`clase.yaml`) are compiled from the rule files (see `lists.rs`); `rule_cut`, in
//! `syllables.rs`, runs the steps.
use std::collections::HashSet;

use super::*;

// The names and the lines of the rules in their files, for the trace (see `codegen.rs`):
// SYLLABIFICATION_LINES, the line of each word of SYLLABIFICATIONS; TABLES, every table;
// ENTRY_LINES, the line of each entry of each of them; STEP_NAMES and STEP_LINES, the name and
// the line of each step of STEPS. They change nothing in a spelling, so RULES_HASH doesn't hash
// them.
include!(concat!(env!("OUT_DIR"), "/reguli_meta.rs"));

/// A step of the fallback rules: where it applies (its patterns and contexts, or its prefixes),
/// and whether it cuts the pair (a hiatus) or not (a diphthong). With a table, it applies only
/// where an entry of the table applies too.
#[derive(Debug, PartialEq, Eq)]
pub struct Step {
    /// Its patterns and contexts; none: it applies to every pair, unless it has prefixes.
    pub when: Entry,
    /// The word starts with one of these, and the pair is the prefix's last vowel and the next
    /// letter.
    pub prefixe: &'static [&'static str],
    /// With prefixes: the rest of the word is a known word of at least this many letters.
    pub rest_cunoscut: usize,
    /// Not when the word starts with one of these, followed by a known word.
    pub non_prefixe: &'static [&'static str],
    /// Only in a verb infinitive.
    pub verb: bool,
    pub tabel: Option<&'static Table>,
    pub cut: bool,
}

impl Step {
    /// True if the step applies to the pair of vowels before `b` (word[b-1], word[b]), its table
    /// aside. `known` and `verb` are as for `rule_cut`.
    pub fn applies(&self, word: &[char], b: usize, known: &HashSet<String>, verb: bool) -> bool {
        if self.verb && !verb {
            return false;
        }
        if !self.prefixe.is_empty() {
            return self.prefix(word, b).is_some()
                && word.len() - b >= self.rest_cunoscut
                && known.contains(&text(&word[b..]))
                && self.non_prefix(word, known).is_none();
        }
        self.when.tipar.is_empty() || self.when.applies(word, b)
    }

    /// The prefix that ends right before `b`, if the word starts with one: its index.
    pub fn prefix(&self, word: &[char], b: usize) -> Option<usize> {
        self.prefixe
            .iter()
            .position(|p| p.chars().count() == b && holds(p, word, 0))
    }

    /// The item of non_prefixe that the word starts with, followed by a known word: its index.
    pub fn non_prefix(&self, word: &[char], known: &HashSet<String>) -> Option<usize> {
        self.non_prefixe.iter().position(|p| {
            let n = p.chars().count();
            holds(p, word, 0) && known.contains(&text(&word[n.min(word.len())..]))
        })
    }
}

/// The name of the step `i` of STEPS, and its line in `pasi.yaml`.
pub fn step_name(i: usize) -> (&'static str, usize) {
    (STEP_NAMES[i], STEP_LINES[i])
}

/// An exception table: its file, the pair of vowels it decides, whether its entries cut that
/// pair (a hiatus) or not (a diphthong), and its entries, in order.
#[derive(Debug, PartialEq, Eq)]
pub struct Table {
    pub file: &'static str,
    pub pair: [char; 2],
    pub cut: bool,
    pub entries: &'static [Entry],
}

/// An entry of a table, or the patterns and contexts of a step. It applies to a pair of vowels
/// when one of its patterns holds both of them, and its contexts hold just before and just after
/// the pattern. A capital in a context is a class: C a consonant (any letter that is not a
/// vowel), V a vowel, or one of CLASSES (`clase.yaml`).
#[derive(Debug, PartialEq, Eq)]
pub struct Entry {
    /// Letters, with `^` for the word's start and `$` for its end.
    pub tipar: &'static [&'static str],
    /// Just before the pattern: one of these, if any (`^` alone: the word's start; `^gor`: the
    /// word starts with gor; `^CC`: two consonants, from the word's start).
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
    // where the item may start: at the word's start, with a `^`; without, up to s, and no
    // further back than its longest reading (a class's digraph has two letters, a loading check)
    let first = if start {
        0
    } else {
        s.saturating_sub(2 * letters.chars().count())
    };
    let last = if start { 0 } else { s };
    (first..=last).any(|t| ends(letters, word, t).contains(&s))
}

/// True if the item of post `item` starts right at `e` (up to the word's end, with a `$`).
fn after(item: &str, word: &[char], e: usize) -> bool {
    let (end, letters) = item.strip_suffix('$').map_or((false, item), |r| (true, r));
    let ends = ends(letters, word, e);
    if end {
        ends.contains(&word.len())
    } else {
        !ends.is_empty()
    }
}

/// Where the letters and classes of `item` can end when they start at `at`: several places,
/// since a class may hold letters and digraphs (P: c, ch).
fn ends(item: &str, word: &[char], at: usize) -> Vec<usize> {
    let mut ends = vec![at];
    for c in item.chars() {
        ends = ends
            .into_iter()
            .flat_map(|e| lengths(c, word, e).into_iter().map(move |n| e + n))
            .collect();
    }
    ends
}

/// The lengths of the letters of `word` from `at` that `c` matches: 1 for its letter, or for C,
/// a consonant, and V, a vowel; and for a class of CLASSES, the length of each of its letters and
/// digraphs that `word` holds there.
fn lengths(c: char, word: &[char], at: usize) -> Vec<usize> {
    let one = |ok: bool| if ok { vec![1] } else { Vec::new() };
    match (c, word.get(at)) {
        (_, None) => Vec::new(),
        ('C', Some(&l)) => one(!is_vowel(l)),
        ('V', Some(&l)) => one(is_vowel(l)),
        (c, Some(_)) if c.is_ascii_uppercase() => CLASSES
            .iter()
            .filter(|(name, _)| *name == c)
            .flat_map(|(_, letters)| letters.iter())
            .filter(|l| holds(l, word, at))
            .map(|l| l.chars().count())
            .collect(),
        (c, Some(&l)) => one(c == l),
    }
}
