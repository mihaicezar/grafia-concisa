//! The syllabification of a word: dexonline's cuts, completed by the fallback rules.
use std::collections::HashSet;
use std::sync::LazyLock;

use fancy_regex::Regex;

use super::*;

pub(super) const VOWELS: &str = "aeiouăâî";
pub(super) const LABIALS: &str = "pbfvm";

pub(super) fn is_vowel(c: char) -> bool {
    VOWELS.contains(c)
}

// Matched at the start of the word's tail from the i (Python: IENT.match(word, b - 1)).
pub(super) static IENT: LazyLock<Regex> = LazyLock::new(|| re(r"^ien[tț](?:[aăei]|$)"));

// The exception tables, compiled from reguli/ (see tables.rs): IA_DIPHTHONG (diftong-ia.yaml),
// the native stems where medial ia after a consonant is a diphthong (chiar, biată, piatră);
// EA_HIATUS (hiat-ea.yaml), OA_HIATUS (hiat-oa.yaml) and IE_HIATUS (hiat-ie.yaml), the
// neologisms where ea, oa, and ie after p/b/f/v/m or c/g/ch/gh are a hiatus (real, coagulat,
// igienă).

/// What decided a pair of vowels by the fallback rules: a step of `rule_cut`, by its name, or
/// an entry of an exception table, by its index there.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rule {
    Step(&'static str),
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
/// it: a step, by its name, or the entry of a table that the step consults.
///
/// `known`: normalized words that count as found (for the prefix rule).
/// `verb`: the word is a verb infinitive (for the -ii rule).
/// Incomplete by design, to be refined:
///   - e-e: hiatus (idee -> ide-e; the -ee switch builds on this);
///   - a prefix (PREFIXES) followed by a known word: hiatus (re-abilitat);
///   - -ient, -ienț/-ient + a/ă/e/i: hiatus (client, ambient, experiență);
///   - -iabil, -eabil after a consonant: hiatus (fiabil, agreabil; not debreiabil);
///   - prefix bi- before en: hiatus (bienal);
///   - ea in the EA_HIATUS neologism endings and stems (hiat-ea.yaml): hiatus
///     (real, nuclear, creat, realizat; not deal, beat, greșeală);
///   - oa in the OA_HIATUS neologism stems and endings (hiat-oa.yaml): hiatus
///     (coagulat, coaliție, protozoar);
///   - ii word-final after a consonant, in a verb infinitive: hiatus (sfi-i,
///     so sfiește, sfiască keep i-e, i-a; not the plural copii);
///   - ie/ia at word start or after a vowel (ă, î included): diphthong
///     (iarbă, femeie, tăia);
///   - ie/ia word-final after a consonant: hiatus (familie, vie);
///   - ia medial after a consonant: hiatus (apropiat, pian, specializat),
///     except in the IA_DIPHTHONG stems (diftong-ia.yaml: chiar, biată,
///     piatră) and, like -iere, final -iare after p/b/f/v/m or c/g/ch/gh (fiare);
///   - ie medial after p/b/f/v/m or c/g/ch/gh: diphthong (fier, piept),
///     except the IE_HIATUS stems (hiat-ie.yaml: igienă, bărbierit, așchietor), the
///     adjectives in -iesc (zapciesc, surugiesc), final -ier/-ieră/-iereasă (IER_ENDINGS) after a
///     vowel and a consonant (plumier, legumieră; not fier, zbier) and the
///     long infinitives in -iere with all their endings (copiere, învierea,
///     aprecierilor, schiere), though not in IERE_DIPHTHONG (miere, mierea,
///     pieri);
///     after other consonants: hiatus (prieten, cartier);
///   - anything else, including ea and oa: no cut (seară, soare).
pub(super) fn rule_cut(word: &[char], b: usize, known: &HashSet<String>, verb: bool) -> (bool, Rule) {
    let step = |cut: bool, name: &'static str| (cut, Rule::Step(name));
    let pair = [word[b - 1], word[b]];
    if pair == ['e', 'e'] {
        return step(true, "e-e");
    }
    let head = text(&word[..b]);
    if word.len() - b >= 4 && PREFIXES.contains(&head.as_str()) && known.contains(&text(&word[b..])) {
        let prea = head == "pre" && known.contains(&text(&word[4..]));
        return step(!prea, "prefix și un cuvânt cunoscut");
    }
    if pair == ['i', 'e'] && found(&IENT, &text(&word[b - 1..])) {
        return step(true, "-ient");
    }
    if (pair == ['i', 'a'] || pair == ['e', 'a'])
        && starts_with_at(word, &['a', 'b', 'i', 'l'], b)
        && (b == 1 || !is_vowel(word[b - 2]))
    {
        return step(true, "-abil"); // fiabil, agreabil; not debreiabil (de-bre-ia)
    }
    if b == 2 && word.starts_with(&['b', 'i', 'e', 'n']) {
        return step(true, "bi-en");
    }
    for table in [&EA_HIATUS, &OA_HIATUS] {
        if let Some(i) = table.entry(word, b) {
            return (table.cut, Rule::Entry(table, i));
        }
    }
    if verb && pair == ['i', 'i'] && b + 1 == word.len() && b >= 2 && !is_vowel(word[b - 2]) {
        return step(true, "-ii final la verbe");
    }
    if !(pair == ['i', 'e'] || pair == ['i', 'a']) {
        return step(false, "restul");
    }
    if b == 1 || is_vowel(word[b - 2]) {
        return step(false, "ie, ia la început sau după vocală");
    }
    if b + 1 == word.len() {
        return step(true, "ie, ia final după consoană");
    }
    let prev = word[b - 2];
    let labial_or_palatal = LABIALS.contains(prev)
        || "cg".contains(prev)
        || b >= 3 && matches!(word[b - 3..b - 1], ['c', 'h'] | ['g', 'h']);
    if pair == ['i', 'a'] {
        if labial_or_palatal && word.ends_with(&['i', 'a', 'r', 'e']) && b == word.len() - 3 {
            return step(false, "-iare care nu e infinitiv lung"); // like -iere: fiare, fiere
        }
        if let Some(i) = IA_DIPHTHONG.entry(word, b) {
            return (IA_DIPHTHONG.cut, Rule::Entry(&IA_DIPHTHONG, i));
        }
        return step(true, "ia după consoană");
    }
    if labial_or_palatal {
        if starts_with_at(word, &['i', 'e', 'r'], b - 1) {
            let end = text(&word[b + 2..]);
            if IERE_ENDINGS.contains(&end.as_str()) {
                // copiere, învierea; not mierea, pieri
                if IERE_DIPHTHONG.contains(&(text(&word[..b + 2]) + "e").as_str()) {
                    return step(false, "-iere nativ");
                }
                return step(true, "-iere, infinitive lungi");
            }
            if IER_ENDINGS.contains(&end.as_str()) {
                // plumier, legumieră, rachiereasă, rachieresele; not fier, zbier
                return step(word[..b - 2].iter().any(|&c| is_vowel(c)), "-ier, -ieră, -iereasă");
            }
        }
        if text(&word[b - 1..]) == "iesc" {
            return step(true, "-iesc");
        }
        if let Some(i) = IE_HIATUS.entry(word, b) {
            return (IE_HIATUS.cut, Rule::Entry(&IE_HIATUS, i));
        }
        return step(false, "ie după labială sau palatală");
    }
    step(true, "ie după altă consoană")
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
