//! The syllabification of a word: dexonline's cuts, completed by the fallback rules.
use std::collections::HashSet;

use super::*;

pub(super) const VOWELS: &str = "aeiouăâî";

pub(super) fn is_vowel(c: char) -> bool {
    VOWELS.contains(c)
}

/// What decided a pair of vowels by the fallback rules: a step of STEPS (`pasi.yaml`), by its
/// index there, or an entry of an exception table, by its index there.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rule {
    Step(usize),
    Entry(&'static Table, usize),
}

/// What decided whether a pair of vowels is cut: an explicit syllabification
/// (`silabisiri.yaml`, at this line), dexonline's complete or partial one, or the fallback rules.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Origin {
    Explicit(usize),
    Dexonline,
    Partial,
    Rule(Rule),
}

/// A pair of vowels of a word: the index of its second vowel, whether it is cut (a hiatus), and
/// what decided it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pair {
    pub at: usize,
    pub cut: bool,
    pub origin: Origin,
}

/// Where a syllabification comes from, from the weakest to the strongest.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Source {
    Rules,
    Partial,
    Dexonline,
    Manual,
}

impl Source {
    pub fn name(self) -> &'static str {
        match self {
            Source::Rules => "rules",
            Source::Partial => "partial",
            Source::Dexonline => "dexonline",
            Source::Manual => "manual",
        }
    }
}

/// Cuts from a dexonline hyphenation and the boundary span [lo, hi] they settle.
///
/// Uses the first comma-separated variant. It may be complete ('fa-mi-li-e'),
/// a word end ('-li-e'), a word start ('a-ce-tal-') or a word interior
/// ('-pioa-'). None if it does not fit the word (or an interior fits twice).
pub(super) fn dexonline_cuts(word: &[char], hyphenation: &str) -> Option<(HashSet<usize>, usize, usize)> {
    let h = hyphenation.split(',').next().unwrap_or("").trim();
    let (lead, trail) = (h.starts_with('-'), h.ends_with('-'));
    let frag = h.trim_matches('-');
    let letters: Vec<char> = frag.chars().filter(|&c| c != '-').collect();
    if letters.is_empty() {
        return None;
    }
    let start = if lead && trail {
        let found: Vec<usize> = (0..word.len()).filter(|&i| starts_with_at(word, &letters, i)).collect();
        if found.len() != 1 {
            return None;
        }
        found[0]
    } else if lead {
        word.len().checked_sub(letters.len())?
    } else {
        0
    };
    if !starts_with_at(word, &letters, start) || (!lead && !trail && letters != word) {
        return None;
    }
    let end = start + letters.len();
    let parts: Vec<&str> = frag.split('-').collect();
    let (mut cuts, mut k) = (HashSet::new(), start);
    for part in &parts[..parts.len() - 1] {
        k += part.chars().count();
        cuts.insert(k);
    }
    if lead {
        cuts.insert(start);
    }
    if trail {
        cuts.insert(end);
    }
    Some((
        cuts,
        if lead { start } else { start + 1 },
        if trail { end } else { end - 1 },
    ))
}

/// Fallback rules: is there a cut between the vowels word[b-1] and word[b]? And what decided
/// it: a step of STEPS (`pasi.yaml`), by its index, or the entry of a table that the step
/// consults. The first step that applies decides; a step with a table applies only where an
/// entry of the table applies too, otherwise the next step is tried. The last step applies to
/// every pair (a loading check of `pasi.yaml`).
///
/// `known`: normalized words that count as found (for the prefix step).
/// `verb`: the word is a verb infinitive (for the step of the -ii verbs).
pub(super) fn rule_cut(word: &[char], b: usize, known: &HashSet<String>, verb: bool) -> (bool, Rule) {
    for (i, step) in STEPS.iter().enumerate() {
        if !step.applies(word, b, known, verb) {
            continue;
        }
        match step.tabel {
            None => return (step.cut, Rule::Step(i)),
            Some(table) => {
                if let Some(e) = table.entry(word, b) {
                    return (step.cut, Rule::Entry(table, e));
                }
            }
        }
    }
    unreachable!("the last step of pasi.yaml applies to every pair")
}

/// The pairs of vowels of `word` (normalized), each decided by the fallback rules alone: with no
/// explicit or dexonline syllabification. `known` and `verb` are as for `syllabify`.
pub fn rule_pairs(word: &str, known: &HashSet<String>, verb: bool) -> Vec<Pair> {
    let w = chars(word);
    (1..w.len())
        .filter(|&b| is_vowel(w[b - 1]) && is_vowel(w[b]))
        .map(|b| {
            let (cut, rule) = rule_cut(&w, b, known, verb);
            Pair {
                at: b,
                cut,
                origin: Origin::Rule(rule),
            }
        })
        .collect()
}

/// Syllabified `word` and its source.
///
/// A syllabification in SYLLABIFICATIONS is used as is (its first reading;
/// see syllabify_all). Otherwise, cuts inside the span covered by the
/// dexonline hyphenation are taken from it and prevail. Elsewhere only cuts
/// between two vowels are placed, by the fallback rules; those are the only
/// cuts grafia concisă uses. `known` holds the words the prefix rule may
/// find; `verb` marks a verb infinitive. Input normalized.
pub fn syllabify(word: &str, hyphenation: &str, known: &HashSet<String>, verb: bool) -> (String, Source) {
    let (syllabified, source, _) = syllabify_traced(word, hyphenation, known, verb);
    (syllabified, source)
}

/// `syllabify`, and what decided each pair of vowels of the word, in order.
pub fn syllabify_traced(
    word: &str,
    hyphenation: &str,
    known: &HashSet<String>,
    verb: bool,
) -> (String, Source, Vec<Pair>) {
    let w = chars(word);
    let vowels = |b: usize| is_vowel(w[b - 1]) && is_vowel(w[b]);
    if let Some(i) = SYLLABIFICATIONS.iter().position(|(x, _)| *x == word) {
        let reading = SYLLABIFICATIONS[i].1[0];
        // a complete syllabification, with the word's letters (a loading check of silabisiri.yaml)
        let (cuts, _, _) = dexonline_cuts(&w, reading).unwrap_or_default();
        let pairs = (1..w.len())
            .filter(|&b| vowels(b))
            .map(|b| Pair {
                at: b,
                cut: cuts.contains(&b),
                origin: Origin::Explicit(SYLLABIFICATION_LINES[i]),
            })
            .collect();
        return (reading.to_string(), Source::Manual, pairs);
    }
    let dex = if hyphenation.is_empty() {
        None
    } else {
        dexonline_cuts(&w, hyphenation)
    };
    let source = match &dex {
        None => Source::Rules,
        Some((_, lo, hi)) if *lo <= 1 && hi + 1 >= w.len() => Source::Dexonline,
        Some(_) => Source::Partial,
    };
    let (cuts, lo, hi) = dex.unwrap_or((HashSet::new(), 1, 0));
    let mut out: String = w.iter().take(1).collect();
    let mut pairs = Vec::new();
    for b in 1..w.len() {
        let cut = if lo <= b && b <= hi {
            let cut = cuts.contains(&b);
            if vowels(b) {
                let origin = match source {
                    Source::Dexonline => Origin::Dexonline,
                    _ => Origin::Partial,
                };
                pairs.push(Pair { at: b, cut, origin });
            }
            cut
        } else if vowels(b) {
            let (cut, rule) = rule_cut(&w, b, known, verb);
            pairs.push(Pair {
                at: b,
                cut,
                origin: Origin::Rule(rule),
            });
            cut
        } else {
            false
        };
        if cut {
            out.push('-');
        }
        out.push(w[b]);
    }
    (out, source, pairs)
}

/// Every reading of `word`: all SYLLABIFICATIONS entries, else syllabify's one.
pub fn syllabify_all(word: &str, hyphenation: &str, known: &HashSet<String>, verb: bool) -> Vec<(String, Source)> {
    match syllabifications(word) {
        Some(readings) => readings.iter().map(|s| (s.to_string(), Source::Manual)).collect(),
        None => vec![syllabify(word, hyphenation, known, verb)],
    }
}
