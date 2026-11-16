//! The rules checks of the exception tables of `reguli/` (`diftong-ia.yaml`, `hiat-ea.yaml`,
//! ...), and the trace of the syllabification. Each example of an entry must be decided by it,
//! and each counterexample excluded by it, by the fallback rules alone: with no dexonline, and
//! with the tests' known words. Each pattern and each item of a context needs a word that shows
//! it. The messages are in Romanian, by file, line and column, like the loading checks; in CI
//! (`GITHUB_ACTIONS`) also as annotations on the lines of the pull request.
//!
//! Run: cargo test -p grafia-concisa --test reguli, or cargo xtask reguli (which runs the loading
//! checks first).
#![allow(clippy::disallowed_methods)] // reads reguli/ and GITHUB_ACTIONS: a test, not the core
mod common;

use std::collections::HashSet;
use std::env;

use grafia_concisa::rules::{Entry as CoreEntry, Origin, Pair, Rule, TABLES, Table, rule_pairs, syllabify_traced};
use grafia_reguli::{CONTEXTS, Entry, Level, Message, Rules, Value, default_dir, load};

/// The tests' known words, for the prefix rule.
fn known() -> HashSet<String> {
    common::KNOWN.iter().map(|w| w.to_string()).collect()
}

/// The rules of reguli/: no loading error, or the core would not have been built.
fn rules() -> Rules {
    let (rules, messages) = load(&default_dir());
    rules.unwrap_or_else(|| panic!("reguli/ doesn't load: {messages:?}"))
}

/// What decided a pair, as a message names it.
fn decider(rule: Rule) -> String {
    match rule {
        Rule::Step(name) => format!("pasul „{name}”"),
        Rule::Entry(t, i) => format!("intrarea de la rândul {} din {}", t.line(i), t.file),
    }
}

/// The rules checks of the tables of `rules`, which must be those the core was built with.
fn check(rules: &Rules, known: &HashSet<String>) -> Vec<Message> {
    let mut messages = Vec::new();
    for t in TABLES {
        let entries = &rules.tables[t.file];
        let patterns = |e: &Entry| e.tipar.iter().map(|p| p.text.clone()).collect::<Vec<_>>();
        assert_eq!(
            entries.iter().map(patterns).collect::<Vec<_>>(),
            t.entries
                .iter()
                .map(|e| e.tipar.iter().map(|p| p.to_string()).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            "{}: the core was built with other rules",
            t.file
        );
        for (i, entry) in entries.iter().enumerate() {
            check_entry(t, i, entry, known, &mut messages);
        }
    }
    messages
}

/// The rules checks of the entry `i` of `t`, written as `entry`.
fn check_entry(t: &'static Table, i: usize, entry: &Entry, known: &HashSet<String>, messages: &mut Vec<Message>) {
    let mut error = |v: (usize, usize), text: String| {
        messages.push(Message {
            dir: "reguli".into(),
            file: t.file.into(),
            line: v.0,
            col: v.1,
            level: Level::Error,
            text,
        })
    };
    let at = |v: &Value| (v.line, v.col);
    let core = &t.entries[i];
    let pair: String = t.pair.iter().collect();
    let way = if t.cut { "hiat" } else { "diftong" };
    // the patterns and the items of the contexts that a word shows
    let mut patterns = vec![false; core.tipar.len()];
    let mut items = CONTEXTS.map(|_| Vec::new());
    for (shown, list) in items.iter_mut().zip(&entry.contexts) {
        *shown = vec![false; list.len()];
    }
    // the pairs of `word` that are the table's, with their trace
    let pairs = |word: &[char]| -> Vec<Pair> {
        let text: String = word.iter().collect();
        rule_pairs(&text, known, false)
            .into_iter()
            .filter(|p| [word[p.at - 1], word[p.at]] == t.pair)
            .collect()
    };
    for w in &entry.exemple {
        let word: Vec<char> = w.text.chars().collect();
        let applying: Vec<Pair> = pairs(&word).into_iter().filter(|p| core.applies(&word, p.at)).collect();
        let Some(first) = applying.first() else {
            error(
                at(w),
                format!(
                    "„{}” nu e decis de intrarea aceasta: niciun tipar al ei nu se aplică unei perechi {pair} din cuvânt.",
                    w.text
                ),
            );
            continue;
        };
        let decided: Vec<&Pair> = applying
            .iter()
            .filter(|p| p.origin == Origin::Rule(Rule::Entry(t, i)))
            .collect();
        if decided.is_empty() {
            let Origin::Rule(rule) = first.origin else {
                unreachable!("rule_pairs gives rules only")
            };
            error(
                at(w),
                format!(
                    "„{}” nu e decis de intrarea aceasta, ci de {}; alege un cuvânt pe care îl decide ea.",
                    w.text,
                    decider(rule)
                ),
            );
        }
        for p in decided {
            for o in core.occurrences(&word, p.at).iter().filter(|o| o.applies(core)) {
                patterns[o.pattern] = true;
                for (c, item) in [(0, o.ante), (2, o.post)] {
                    if let Some(k) = item {
                        items[c][k] = true;
                    }
                }
            }
        }
    }
    for c in &entry.contraexemple {
        let word: Vec<char> = c.text.chars().collect();
        let held: Vec<Pair> = pairs(&word)
            .into_iter()
            .filter(|p| !core.occurrences(&word, p.at).is_empty())
            .collect();
        if held.is_empty() {
            error(
                at(c),
                format!(
                    "„{}” nu are literele niciunui tipar al intrării, deci nu e un contraexemplu al ei.",
                    c.text
                ),
            );
        } else if held.iter().any(|p| core.applies(&word, p.at)) {
            error(
                at(c),
                format!(
                    "intrarea aceasta decide „{}”; un contraexemplu are literele unui tipar, dar contextele sau ^, $ îl exclud.",
                    c.text
                ),
            );
        } else if let Some(p) = held.iter().find(|p| p.cut == t.cut) {
            let Origin::Rule(rule) = p.origin else {
                unreachable!("rule_pairs gives rules only")
            };
            error(
                at(c),
                format!(
                    "„{}” e exclus de intrarea aceasta, dar {} îl face tot {way}; un contraexemplu e despărțit invers.",
                    c.text,
                    decider(rule)
                ),
            );
        }
        for p in &held {
            for o in core.occurrences(&word, p.at) {
                for (c, item) in [(1, o.non_ante), (3, o.non_post)] {
                    if let Some(k) = item {
                        items[c][k] = true;
                    }
                }
            }
        }
    }
    for (p, shown) in entry.tipar.iter().zip(&patterns) {
        if !shown {
            error(
                at(p),
                format!(
                    "tiparul „{}” nu are niciun exemplu; adaugă la exemple un cuvânt pe care intrarea îl decide prin el.",
                    p.text
                ),
            );
        }
    }
    for ((key, list), shown) in CONTEXTS.iter().zip(&entry.contexts).zip(&items) {
        for (item, shown) in list.iter().zip(shown) {
            if *shown {
                continue;
            }
            let text = if key.starts_with("non_") {
                format!(
                    "„{}” din {key} nu exclude niciun contraexemplu; adaugă la contraexemple un cuvânt pe care îl exclude.",
                    item.text
                )
            } else {
                format!(
                    "„{}” din {key} nu are niciun exemplu; adaugă la exemple un cuvânt pe care intrarea îl decide prin el.",
                    item.text
                )
            };
            error(at(item), text);
        }
    }
}

/// The rules checks of reguli/: each message printed, in CI also as an annotation, and none
/// expected.
#[test]
fn reguli() {
    let messages = check(&rules(), &known());
    let github = env::var_os("GITHUB_ACTIONS").is_some();
    for m in &messages {
        println!("{m}");
        if github {
            println!("{}", m.annotation());
        }
    }
    assert!(
        messages.is_empty(),
        "reguli/: exemplele au erori, în mesajele de mai sus"
    );
}

/// A change to the words of an entry, for a case of `CHECKS`.
#[derive(Clone, Copy, Debug)]
enum Change {
    Exemplu(&'static str),
    Contraexemplu(&'static str),
    FaraExemplu(&'static str),
    FaraContraexemplu(&'static str),
}

// (file, the first pattern of the entry, a change to its words, the messages it gives, as
// "file: eroare: text"; {P} is the line of the entry of the file whose first pattern is P).
// Add a case = add a line.
#[rustfmt::skip]
const CHECKS: &[(&str, &str, Change, &[&str])] = &[
    // an example decided by another step, or by an earlier entry, or by none of its patterns
    ("diftong-ia.yaml", "ghia", Change::Exemplu("ghiare"), &["diftong-ia.yaml: eroare: „ghiare” nu e decis de intrarea aceasta, ci de pasul „-iare care nu e infinitiv lung”; alege un cuvânt pe care îl decide ea."]),
    ("hiat-ea.yaml", "boreal", Change::Exemplu("boreal"), &["hiat-ea.yaml: eroare: „boreal” nu e decis de intrarea aceasta, ci de intrarea de la rândul {eal$} din hiat-ea.yaml; alege un cuvânt pe care îl decide ea."]),
    ("hiat-ie.yaml", "gien", Change::Exemplu("fier"), &["hiat-ie.yaml: eroare: „fier” nu e decis de intrarea aceasta: niciun tipar al ei nu se aplică unei perechi ie din cuvânt."]),
    // a counterexample with no pattern, decided by the entry, or decided the same way
    ("hiat-oa.yaml", "coagul", Change::Contraexemplu("soare"), &["hiat-oa.yaml: eroare: „soare” nu are literele niciunui tipar al intrării, deci nu e un contraexemplu al ei."]),
    ("hiat-oa.yaml", "coagul", Change::Contraexemplu("coagulant"), &["hiat-oa.yaml: eroare: intrarea aceasta decide „coagulant”; un contraexemplu are literele unui tipar, dar contextele sau ^, $ îl exclud."]),
    ("diftong-ia.yaml", "ghia", Change::Contraexemplu("unghiare"), &["diftong-ia.yaml: eroare: „unghiare” e exclus de intrarea aceasta, dar pasul „-iare care nu e infinitiv lung” îl face tot diftong; un contraexemplu e despărțit invers."]),
    // a pattern or an item of a context that no word shows
    ("hiat-ea.yaml", "nuclear", Change::FaraExemplu("steatoză"), &["hiat-ea.yaml: eroare: tiparul „steat” nu are niciun exemplu; adaugă la exemple un cuvânt pe care intrarea îl decide prin el."]),
    ("hiat-ea.yaml", "eat", Change::FaraExemplu("supleat"), &["hiat-ea.yaml: eroare: „upl” din ante nu are niciun exemplu; adaugă la exemple un cuvânt pe care intrarea îl decide prin el."]),
    ("hiat-ie.yaml", "miel", Change::FaraExemplu("mielom"), &["hiat-ie.yaml: eroare: „o” din post nu are niciun exemplu; adaugă la exemple un cuvânt pe care intrarea îl decide prin el."]),
    ("diftong-ia.yaml", "ghia", Change::FaraContraexemplu("liturghiar"), &["diftong-ia.yaml: eroare: „litur” din non_ante nu exclude niciun contraexemplu; adaugă la contraexemple un cuvânt pe care îl exclude."]),
    ("diftong-ia.yaml", "^biat", Change::FaraContraexemplu("biatlon"), &["diftong-ia.yaml: eroare: „l” din non_post nu exclude niciun contraexemplu; adaugă la contraexemple un cuvânt pe care îl exclude."]),
    ("diftong-ia.yaml", "miaz", Change::FaraExemplu("miazănoapte"), &["diftong-ia.yaml: eroare: „^” din ante nu are niciun exemplu; adaugă la exemple un cuvânt pe care intrarea îl decide prin el."]),
];

#[test]
fn checks() {
    let mut failures = Vec::new();
    let original = rules();
    for (file, first, change, expected) in CHECKS {
        let mut rules = original.clone();
        let entries = rules.tables.get_mut(file).expect("a table");
        let lines: Vec<(String, usize)> = entries.iter().map(|e| (e.tipar[0].text.clone(), e.line)).collect();
        let entry = entries
            .iter_mut()
            .find(|e| e.tipar[0].text == *first)
            .unwrap_or_else(|| panic!("{file}: no entry starts with the pattern {first}"));
        let word = |text: &str| Value {
            text: text.into(),
            line: entry.line,
            col: entry.col,
        };
        match *change {
            Change::Exemplu(w) => entry.exemple.push(word(w)),
            Change::Contraexemplu(w) => entry.contraexemple.push(word(w)),
            Change::FaraExemplu(w) => entry.exemple.retain(|v| v.text != w),
            Change::FaraContraexemplu(w) => entry.contraexemple.retain(|v| v.text != w),
        }
        let got: Vec<String> = check(&rules, &known())
            .iter()
            .map(|m| format!("{}: eroare: {}", m.file, m.text))
            .collect();
        let expected: Vec<String> = expected
            .iter()
            .map(|e| {
                lines.iter().fold(e.to_string(), |e, (p, line)| {
                    e.replace(&format!("{{{p}}}"), &line.to_string())
                })
            })
            .collect();
        if got != expected {
            failures.push(format!(
                "{file} {first} {change:?}:\n  got      {got:?}\n  expected {expected:?}"
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "{} failing:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

// (word, dexonline's hyphenation, its pairs of vowels: the index of the second vowel, hiat or
// diftong, and what decided it; {F:P} is the line in F of the entry whose first pattern is P, or
// of the word P of silabisiri.yaml). Add a case = add a line.
#[rustfmt::skip]
const TRACES: &[(&str, &str, &str)] = &[
    ("neant", "", "2 hiat silabisiri.yaml:{silabisiri.yaml:neant}"),
    ("miază", "", "2 diftong silabisiri.yaml:{silabisiri.yaml:miază}"), // the first reading
    ("familie", "fa-mi-li-e", "6 hiat dexonline"),
    ("familie", "-li-e", "6 hiat parțial"),
    ("biologie", "-lo-gi-e", "2 diftong pasul „restul”, 7 hiat parțial"),
    ("aripioară", "-pioa-", "5 diftong parțial, 6 diftong parțial"),
    ("real", "", "2 hiat hiat-ea.yaml:{hiat-ea.yaml:eal$}"),
    ("chiar", "", "3 diftong diftong-ia.yaml:{diftong-ia.yaml:chiar}"),
    ("bărbierit", "", "5 hiat hiat-ie.yaml:{hiat-ie.yaml:bier}"),
    ("protozoar", "", "7 hiat hiat-oa.yaml:{hiat-oa.yaml:coagul}"),
    ("piept", "", "2 diftong pasul „ie după labială sau palatală”"),
    ("fier", "", "2 diftong pasul „-ier, -ieră, -iereasă”"), // one syllable: no vowel before -ier
    ("caragialian", "", "6 diftong diftong-ia.yaml:{diftong-ia.yaml:chiar}, 9 hiat pasul „ia după consoană”"),
];

#[test]
fn traces() {
    let rules = rules();
    let mut lines: Vec<(String, usize)> = rules
        .silabisiri
        .iter()
        .map(|(w, _)| (format!("silabisiri.yaml:{}", w.text), w.line))
        .collect();
    for (file, entries) in &rules.tables {
        lines.extend(entries.iter().map(|e| (format!("{file}:{}", e.tipar[0].text), e.line)));
    }
    let origin = |o: Origin| match o {
        Origin::Explicit(line) => format!("silabisiri.yaml:{line}"),
        Origin::Dexonline => "dexonline".into(),
        Origin::Partial => "parțial".into(),
        Origin::Rule(Rule::Step(name)) => format!("pasul „{name}”"),
        Origin::Rule(Rule::Entry(t, i)) => format!("{}:{}", t.file, t.line(i)),
    };
    let mut failures = Vec::new();
    for (word, hyphenation, expected) in TRACES {
        let (_, source, pairs) = syllabify_traced(word, hyphenation, &known(), false);
        // by the rules alone, rule_pairs gives the same trace
        if source == grafia_concisa::rules::Source::Rules {
            assert_eq!(rule_pairs(word, &known(), false), pairs, "{word}");
        }
        let got: Vec<String> = pairs
            .iter()
            .map(|p| {
                format!(
                    "{} {} {}",
                    p.at,
                    if p.cut { "hiat" } else { "diftong" },
                    origin(p.origin)
                )
            })
            .collect();
        let expected = lines.iter().fold(expected.to_string(), |e, (k, line)| {
            e.replace(&format!("{{{k}}}"), &line.to_string())
        });
        if got.join(", ") != expected {
            failures.push(format!(
                "{word} {hyphenation:?}:\n  got      {got:?}\n  expected {expected:?}"
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "{} failing:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

// How an entry applies: (tipar, ante, non_ante, post, non_post, word, the index of the second
// vowel of the pair, whether it applies). Add a case = add a line.
type Match = (
    &'static [&'static str],
    &'static [&'static str],
    &'static [&'static str],
    &'static [&'static str],
    &'static [&'static str],
    &'static str,
    usize,
    bool,
);
#[rustfmt::skip]
const MATCHES: &[Match] = &[
    (&["oa"], &[], &[], &[], &[], "soare", 2, true),
    (&["so"], &[], &[], &[], &[], "soa", 2, false), // it holds the o, not the a
    (&["oar"], &[], &[], &[], &[], "soa", 2, false), // past the word's end
    (&["^soa"], &[], &[], &[], &[], "soare", 2, true),
    (&["^oa"], &[], &[], &[], &[], "soare", 2, false),
    (&["oa$"], &[], &[], &[], &[], "boa", 2, true),
    (&["oa$"], &[], &[], &[], &[], "boal", 2, false),
    // ante: letters, ^ alone, ^ and letters
    (&["oa"], &["s"], &[], &[], &[], "soare", 2, true),
    (&["oa"], &["z"], &[], &[], &[], "soare", 2, false),
    (&["oa"], &["^"], &[], &[], &[], "oare", 1, true),
    (&["oa"], &["^"], &[], &[], &[], "soare", 2, false),
    (&["oa"], &["^s"], &[], &[], &[], "soare", 2, true),
    (&["oa"], &["^s"], &[], &[], &[], "isoare", 3, false),
    (&["oa"], &[], &["s"], &[], &[], "soare", 2, false),
    (&["oa"], &[], &["^"], &[], &[], "soare", 2, true),
    // post: letters, $ alone, letters and $
    (&["oa"], &[], &[], &["re"], &[], "soare", 2, true),
    (&["oa"], &[], &[], &["$"], &[], "boa", 2, true),
    (&["oa"], &[], &[], &["$"], &[], "boal", 2, false),
    (&["oa"], &[], &[], &["re$"], &[], "soare", 2, true),
    (&["oa"], &[], &[], &["re$"], &[], "soarele", 2, false),
    (&["oa"], &[], &[], &[], &["re$"], "soarele", 2, true),
    (&["oa"], &[], &[], &[], &["re"], "soarele", 2, false),
];

#[test]
fn matches() {
    let mut failures = Vec::new();
    for &(tipar, ante, non_ante, post, non_post, word, b, expected) in MATCHES {
        let entry = CoreEntry {
            tipar,
            ante,
            non_ante,
            post,
            non_post,
        };
        let w: Vec<char> = word.chars().collect();
        if entry.applies(&w, b) != expected {
            failures.push(format!("{entry:?} {word} {b}: expected {expected}"));
        }
    }
    assert!(
        failures.is_empty(),
        "{} failing:\n{}",
        failures.len(),
        failures.join("\n")
    );
}
