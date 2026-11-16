//! The rules checks of the steps of the fallback rules (`reguli/pasi.yaml`) and of their
//! exception tables (`diftong-ia.yaml`, `hiat-ea.yaml`, ...), and the trace of the
//! syllabification. Each example of a step or of an entry must be decided by it, and each
//! counterexample excluded by it, by the fallback rules alone: with no dexonline, and with the
//! tests' known words. Each pattern, prefix and item of a context needs a word that shows it. The
//! messages are in Romanian, by file, line and column, like the loading checks; in CI
//! (`GITHUB_ACTIONS`) also as annotations on the lines of the pull request.
//!
//! Run: cargo test -p grafia-concisa --test reguli, or cargo xtask reguli (which runs the loading
//! checks first).
#![allow(clippy::disallowed_methods)] // reads reguli/ and GITHUB_ACTIONS: a test, not the core
mod common;

use std::collections::HashSet;
use std::env;

use grafia_concisa::rules::{
    Entry as CoreEntry, Origin, Pair, Rule, STEPS, TABLES, Table, rule_pairs, step_name, syllabify_traced,
};
use grafia_reguli::{CONTEXTS, Entry, Level, Message, PASI, Rules, Value, default_dir, load};

/// The tests' known words, for the prefix step.
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
        Rule::Step(i) => {
            let (name, line) = step_name(i);
            format!("pasul „{name}” ({PASI}:{line})")
        }
        Rule::Entry(t, i) => format!("intrarea de la rândul {} din {}", t.line(i), t.file),
    }
}

/// The rules checks of the steps and of the tables of `rules`, which must be those the core was
/// built with. A step with a table has its examples in the table.
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
            check_rule(Subject::Entry(t, i), entry, &[], &[], known, &mut messages);
        }
    }
    let written: Vec<(&str, Vec<&str>)> = rules
        .pasi
        .iter()
        .map(|s| {
            (
                s.pas.text.as_str(),
                s.entry.tipar.iter().map(|p| p.text.as_str()).collect(),
            )
        })
        .collect();
    let built: Vec<(&str, Vec<&str>)> = STEPS
        .iter()
        .enumerate()
        .map(|(i, s)| (step_name(i).0, s.when.tipar.to_vec()))
        .collect();
    assert_eq!(written, built, "{PASI}: the core was built with other rules");
    for (i, step) in rules.pasi.iter().enumerate() {
        if step.tabel.is_none() {
            check_rule(
                Subject::Step(i),
                &step.entry,
                &step.prefixe,
                &step.non_prefixe,
                known,
                &mut messages,
            );
        }
    }
    messages
}

/// What the rules checks check: the entry `i` of a table, or the step `i` of STEPS.
#[derive(Clone, Copy)]
enum Subject {
    Entry(&'static Table, usize),
    Step(usize),
}

impl Subject {
    fn rule(self) -> Rule {
        match self {
            Subject::Entry(t, i) => Rule::Entry(t, i),
            Subject::Step(i) => Rule::Step(i),
        }
    }

    fn file(self) -> &'static str {
        match self {
            Subject::Entry(t, _) => t.file,
            Subject::Step(_) => PASI,
        }
    }

    /// Its patterns and contexts.
    fn when(self) -> &'static CoreEntry {
        match self {
            Subject::Entry(t, i) => &t.entries[i],
            Subject::Step(i) => &STEPS[i].when,
        }
    }

    fn cut(self) -> bool {
        match self {
            Subject::Entry(t, _) => t.cut,
            Subject::Step(i) => STEPS[i].cut,
        }
    }

    /// Whether its examples are verb infinitives (`doar: verb`); its counterexamples never are.
    fn verb(self) -> bool {
        matches!(self, Subject::Step(i) if STEPS[i].verb)
    }

    /// True if it applies to the pair of vowels before `b`.
    fn applies(self, word: &[char], b: usize, known: &HashSet<String>, verb: bool) -> bool {
        match self {
            Subject::Entry(t, i) => [word[b - 1], word[b]] == t.pair && t.entries[i].applies(word, b),
            Subject::Step(i) => STEPS[i].applies(word, b, known, verb),
        }
    }

    /// True if a pattern or a prefix of it holds the pair of vowels before `b` (the table's pair,
    /// for an entry); a step with neither holds every pair.
    fn holds(self, word: &[char], b: usize) -> bool {
        let when = self.when();
        match self {
            Subject::Entry(t, _) => [word[b - 1], word[b]] == t.pair && !when.occurrences(word, b).is_empty(),
            Subject::Step(i) if !STEPS[i].prefixe.is_empty() => STEPS[i].prefix(word, b).is_some(),
            Subject::Step(_) => when.tipar.is_empty() || !when.occurrences(word, b).is_empty(),
        }
    }

    /// How the messages name it: the noun, "this" and "it".
    fn names(self) -> (&'static str, &'static str, &'static str) {
        match self {
            Subject::Entry(..) => ("intrarea", "aceasta", "ea"),
            Subject::Step(_) => ("pasul", "acesta", "el"),
        }
    }
}

/// The rules checks of `subject`, written as `entry`, with its prefixes and the items of its
/// non_prefixe, for a step.
fn check_rule(
    subject: Subject,
    entry: &Entry,
    prefixe: &[Value],
    non_prefixe: &[Value],
    known: &HashSet<String>,
    messages: &mut Vec<Message>,
) {
    let mut error = |v: (usize, usize), text: String| {
        messages.push(Message {
            dir: "reguli".into(),
            file: subject.file().into(),
            line: v.0,
            col: v.1,
            level: Level::Error,
            text,
        })
    };
    let at = |v: &Value| (v.line, v.col);
    let (noun, this, it) = subject.names();
    let when = subject.when();
    let way = if subject.cut() { "hiat" } else { "diftong" };
    let step = match subject {
        Subject::Step(i) => Some(&STEPS[i]),
        Subject::Entry(..) => None,
    };
    // the patterns, prefixes, items of the contexts and items of non_prefixe that a word shows
    let mut patterns = vec![false; when.tipar.len()];
    let mut prefixes = vec![false; prefixe.len()];
    let mut non_prefixes = vec![false; non_prefixe.len()];
    let mut items = CONTEXTS.map(|_| Vec::new());
    for (shown, list) in items.iter_mut().zip(&entry.contexts) {
        *shown = vec![false; list.len()];
    }
    for w in &entry.exemple {
        let word: Vec<char> = w.text.chars().collect();
        let verb = subject.verb();
        let pairs = rule_pairs(&w.text, known, verb);
        let applying: Vec<&Pair> = pairs
            .iter()
            .filter(|p| subject.applies(&word, p.at, known, verb))
            .collect();
        let Some(first) = applying.first() else {
            let text = match subject {
                Subject::Entry(t, _) => format!(
                    "„{}” nu e decis de intrarea aceasta: niciun tipar al ei nu se aplică unei perechi {} din cuvânt.",
                    w.text,
                    t.pair.iter().collect::<String>()
                ),
                Subject::Step(_) => format!(
                    "„{}” nu e decis de pasul acesta: nu se aplică niciunei perechi de vocale din cuvânt.",
                    w.text
                ),
            };
            error(at(w), text);
            continue;
        };
        let decided: Vec<&Pair> = applying
            .iter()
            .copied()
            .filter(|p| p.origin == Origin::Rule(subject.rule()))
            .collect();
        if decided.is_empty() {
            let Origin::Rule(rule) = first.origin else {
                unreachable!("rule_pairs gives rules only")
            };
            error(
                at(w),
                format!(
                    "„{}” nu e decis de {noun} {this}, ci de {}; alege un cuvânt pe care îl decide {it}.",
                    w.text,
                    decider(rule)
                ),
            );
        }
        for p in decided {
            if let Some(k) = step.and_then(|s| s.prefix(&word, p.at)) {
                prefixes[k] = true;
            }
            for o in when.occurrences(&word, p.at).iter().filter(|o| o.applies(when)) {
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
        let held: Vec<Pair> = rule_pairs(&c.text, known, false)
            .into_iter()
            .filter(|p| subject.holds(&word, p.at))
            .collect();
        if held.is_empty() {
            let text = match subject {
                Subject::Entry(..) => format!(
                    "„{}” nu are literele niciunui tipar al intrării, deci nu e un contraexemplu al ei.",
                    c.text
                ),
                Subject::Step(_) => format!(
                    "„{}” nu are literele niciunui tipar al pasului și nu începe cu niciun prefix al lui, deci nu e un contraexemplu al lui.",
                    c.text
                ),
            };
            error(at(c), text);
        } else if held.iter().any(|p| subject.applies(&word, p.at, known, false)) {
            let text = match subject {
                Subject::Entry(..) => format!(
                    "intrarea aceasta decide „{}”; un contraexemplu are literele unui tipar, dar contextele sau ^, $ îl exclud.",
                    c.text
                ),
                Subject::Step(_) => format!(
                    "pasul acesta decide „{}”; un contraexemplu are literele unui tipar, sau un prefix, dar o condiție a pasului îl exclude.",
                    c.text
                ),
            };
            error(at(c), text);
        } else if let Some(p) = held.iter().find(|p| p.cut == subject.cut()) {
            let Origin::Rule(rule) = p.origin else {
                unreachable!("rule_pairs gives rules only")
            };
            error(
                at(c),
                format!(
                    "„{}” e exclus de {noun} {this}, dar {} îl face tot {way}; un contraexemplu e despărțit invers.",
                    c.text,
                    decider(rule)
                ),
            );
        }
        for p in &held {
            for o in when.occurrences(&word, p.at) {
                for (c, item) in [(1, o.non_ante), (3, o.non_post)] {
                    if let Some(k) = item {
                        items[c][k] = true;
                    }
                }
            }
        }
        if let Some(k) = step
            .filter(|_| !held.is_empty())
            .and_then(|s| s.non_prefix(&word, known))
        {
            non_prefixes[k] = true;
        }
    }
    for (p, shown) in entry.tipar.iter().zip(&patterns) {
        if !shown {
            error(
                at(p),
                format!(
                    "tiparul „{}” nu are niciun exemplu; adaugă la exemple un cuvânt pe care {noun} îl decide prin el.",
                    p.text
                ),
            );
        }
    }
    for (p, shown) in prefixe.iter().zip(&prefixes) {
        if !shown {
            error(
                at(p),
                format!(
                    "prefixul „{}” nu are niciun exemplu; adaugă la exemple un cuvânt pe care pasul îl decide prin el.",
                    p.text
                ),
            );
        }
    }
    let lists = CONTEXTS
        .iter()
        .zip(&entry.contexts)
        .zip(&items)
        .map(|((key, list), shown)| (*key, list.as_slice(), shown))
        .chain([("non_prefixe", non_prefixe, &non_prefixes)]);
    for (key, list, shown) in lists {
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
                    "„{}” din {key} nu are niciun exemplu; adaugă la exemple un cuvânt pe care {noun} îl decide prin el.",
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

// (file, the first pattern of the entry, or the name of the step in pasi.yaml, a change to its
// words, the messages it gives, as "file: eroare: text"; {P} is the line of the entry of the file
// whose first pattern is P, or of the step named P). Add a case = add a line.
#[rustfmt::skip]
const CHECKS: &[(&str, &str, Change, &[&str])] = &[
    // an example decided by another step, or by an earlier entry, or by none of its patterns
    ("diftong-ia.yaml", "ghia", Change::Exemplu("ghiare"), &["diftong-ia.yaml: eroare: „ghiare” nu e decis de intrarea aceasta, ci de pasul „-iare care nu e infinitiv lung” (pasi.yaml:{-iare care nu e infinitiv lung}); alege un cuvânt pe care îl decide ea."]),
    ("hiat-ea.yaml", "boreal", Change::Exemplu("boreal"), &["hiat-ea.yaml: eroare: „boreal” nu e decis de intrarea aceasta, ci de intrarea de la rândul {eal$} din hiat-ea.yaml; alege un cuvânt pe care îl decide ea."]),
    ("hiat-ie.yaml", "gien", Change::Exemplu("fier"), &["hiat-ie.yaml: eroare: „fier” nu e decis de intrarea aceasta: niciun tipar al ei nu se aplică unei perechi ie din cuvânt."]),
    // a counterexample with no pattern, decided by the entry, or decided the same way
    ("hiat-oa.yaml", "coagul", Change::Contraexemplu("soare"), &["hiat-oa.yaml: eroare: „soare” nu are literele niciunui tipar al intrării, deci nu e un contraexemplu al ei."]),
    ("hiat-oa.yaml", "coagul", Change::Contraexemplu("coagulant"), &["hiat-oa.yaml: eroare: intrarea aceasta decide „coagulant”; un contraexemplu are literele unui tipar, dar contextele sau ^, $ îl exclud."]),
    ("diftong-ia.yaml", "ghia", Change::Contraexemplu("unghiare"), &["diftong-ia.yaml: eroare: „unghiare” e exclus de intrarea aceasta, dar pasul „-iare care nu e infinitiv lung” (pasi.yaml:{-iare care nu e infinitiv lung}) îl face tot diftong; un contraexemplu e despărțit invers."]),
    // a pattern or an item of a context that no word shows
    ("hiat-ea.yaml", "nuclear", Change::FaraExemplu("steatoză"), &["hiat-ea.yaml: eroare: tiparul „steat” nu are niciun exemplu; adaugă la exemple un cuvânt pe care intrarea îl decide prin el."]),
    ("hiat-ea.yaml", "eat", Change::FaraExemplu("supleat"), &["hiat-ea.yaml: eroare: „upl” din ante nu are niciun exemplu; adaugă la exemple un cuvânt pe care intrarea îl decide prin el."]),
    ("hiat-ie.yaml", "miel", Change::FaraExemplu("mielom"), &["hiat-ie.yaml: eroare: „o” din post nu are niciun exemplu; adaugă la exemple un cuvânt pe care intrarea îl decide prin el."]),
    ("diftong-ia.yaml", "ghia", Change::FaraContraexemplu("liturghiar"), &["diftong-ia.yaml: eroare: „litur” din non_ante nu exclude niciun contraexemplu; adaugă la contraexemple un cuvânt pe care îl exclude."]),
    ("diftong-ia.yaml", "^biat", Change::FaraContraexemplu("biatlon"), &["diftong-ia.yaml: eroare: „l” din non_post nu exclude niciun contraexemplu; adaugă la contraexemple un cuvânt pe care îl exclude."]),
    ("diftong-ia.yaml", "miaz", Change::FaraExemplu("miazănoapte"), &["diftong-ia.yaml: eroare: „^” din ante nu are niciun exemplu; adaugă la exemple un cuvânt pe care intrarea îl decide prin el."]),
    // the same, for a step
    ("pasi.yaml", "-abil", Change::Exemplu("debreiabil"), &["pasi.yaml: eroare: „debreiabil” nu e decis de pasul acesta: nu se aplică niciunei perechi de vocale din cuvânt."]),
    ("pasi.yaml", "ie după labială sau palatală", Change::Exemplu("plumier"), &["pasi.yaml: eroare: „plumier” nu e decis de pasul acesta, ci de pasul „-ier, -ieră, -iereasă” (pasi.yaml:{-ier, -ieră, -iereasă}); alege un cuvânt pe care îl decide el."]),
    ("pasi.yaml", "ie după altă consoană", Change::Exemplu("cartea"), &["pasi.yaml: eroare: „cartea” nu e decis de pasul acesta: nu se aplică niciunei perechi de vocale din cuvânt."]),
    ("pasi.yaml", "-ier, -ieră, -iereasă", Change::Contraexemplu("soare"), &["pasi.yaml: eroare: „soare” nu are literele niciunui tipar al pasului și nu începe cu niciun prefix al lui, deci nu e un contraexemplu al lui."]),
    ("pasi.yaml", "prefix și un cuvânt cunoscut", Change::Contraexemplu("pieton"), &["pasi.yaml: eroare: „pieton” nu are literele niciunui tipar al pasului și nu începe cu niciun prefix al lui, deci nu e un contraexemplu al lui."]),
    ("pasi.yaml", "-ier, -ieră, -iereasă", Change::Contraexemplu("plumier"), &["pasi.yaml: eroare: pasul acesta decide „plumier”; un contraexemplu are literele unui tipar, sau un prefix, dar o condiție a pasului îl exclude."]),
    ("pasi.yaml", "prefix și un cuvânt cunoscut", Change::Contraexemplu("reabilitat"), &["pasi.yaml: eroare: pasul acesta decide „reabilitat”; un contraexemplu are literele unui tipar, sau un prefix, dar o condiție a pasului îl exclude."]),
    ("pasi.yaml", "prefix și un cuvânt cunoscut", Change::Contraexemplu("neabil"), &["pasi.yaml: eroare: „neabil” e exclus de pasul acesta, dar pasul „-abil” (pasi.yaml:{-abil}) îl face tot hiat; un contraexemplu e despărțit invers."]),
    // a doar: verb step decides its examples as verbs, and excludes its counterexamples
    ("pasi.yaml", "-ii final la verbe", Change::Contraexemplu("pustii"), &[]),
    // a pattern, a prefix, or an item of a context or of non_prefixe that no word shows
    ("pasi.yaml", "nu e prefix", Change::FaraExemplu("coastă"), &["pasi.yaml: eroare: tiparul „^coast” nu are niciun exemplu; adaugă la exemple un cuvânt pe care pasul îl decide prin el."]),
    ("pasi.yaml", "prefix și un cuvânt cunoscut", Change::FaraExemplu("stereoautograf"), &["pasi.yaml: eroare: prefixul „stereo” nu are niciun exemplu; adaugă la exemple un cuvânt pe care pasul îl decide prin el."]),
    ("pasi.yaml", "-iesc", Change::FaraExemplu("fiesc"), &["pasi.yaml: eroare: „L” din ante nu are niciun exemplu; adaugă la exemple un cuvânt pe care pasul îl decide prin el."]),
    ("pasi.yaml", "-ient", Change::FaraExemplu("ambienta"), &["pasi.yaml: eroare: „a” din post nu are niciun exemplu; adaugă la exemple un cuvânt pe care pasul îl decide prin el."]),
    ("pasi.yaml", "-ier, -ieră, -iereasă", Change::FaraContraexemplu("zbier"), &["pasi.yaml: eroare: „^CC” din non_ante nu exclude niciun contraexemplu; adaugă la contraexemple un cuvânt pe care îl exclude."]),
    ("pasi.yaml", "prefix și un cuvânt cunoscut", Change::FaraContraexemplu("preacurat"), &["pasi.yaml: eroare: „prea” din non_prefixe nu exclude niciun contraexemplu; adaugă la contraexemple un cuvânt pe care îl exclude."]),
];

#[test]
fn checks() {
    let mut failures = Vec::new();
    let original = rules();
    for (file, first, change, expected) in CHECKS {
        let mut rules = original.clone();
        let mut lines: Vec<(String, usize)> = rules.pasi.iter().map(|s| (s.pas.text.clone(), s.entry.line)).collect();
        let entry = if *file == PASI {
            let step = rules.pasi.iter_mut().find(|s| s.pas.text == *first);
            &mut step.unwrap_or_else(|| panic!("{file}: no step {first}")).entry
        } else {
            let entries = rules.tables.get_mut(file).expect("a table");
            lines.extend(entries.iter().map(|e| (e.tipar[0].text.clone(), e.line)));
            entries
                .iter_mut()
                .find(|e| e.tipar[0].text == *first)
                .unwrap_or_else(|| panic!("{file}: no entry starts with the pattern {first}"))
        };
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
// diftong, and what decided it; {F:P} is the line in F of the entry whose first pattern is P, of
// the step P of pasi.yaml, or of the word P of silabisiri.yaml). Add a case = add a line.
#[rustfmt::skip]
const TRACES: &[(&str, &str, &str)] = &[
    ("neant", "", "2 hiat silabisiri.yaml:{silabisiri.yaml:neant}"),
    ("miază", "", "2 diftong silabisiri.yaml:{silabisiri.yaml:miază}"), // the first reading
    ("familie", "fa-mi-li-e", "6 hiat dexonline"),
    ("familie", "-li-e", "6 hiat parțial"),
    ("biologie", "-lo-gi-e", "2 diftong pasi.yaml:{pasi.yaml:restul}, 7 hiat parțial"),
    ("aripioară", "-pioa-", "5 diftong parțial, 6 diftong parțial"),
    ("real", "", "2 hiat hiat-ea.yaml:{hiat-ea.yaml:eal$}"),
    ("chiar", "", "3 diftong diftong-ia.yaml:{diftong-ia.yaml:chiar}"),
    ("bărbierit", "", "5 hiat hiat-ie.yaml:{hiat-ie.yaml:bier}"),
    ("protozoar", "", "7 hiat hiat-oa.yaml:{hiat-oa.yaml:coagul}"),
    ("piept", "", "2 diftong pasi.yaml:{pasi.yaml:ie după labială sau palatală}"),
    ("fier", "", "2 diftong pasi.yaml:{pasi.yaml:ie după labială sau palatală}"), // one syllable: not -ier
    ("caragialian", "", "6 diftong diftong-ia.yaml:{diftong-ia.yaml:chiar}, 9 hiat pasi.yaml:{pasi.yaml:ia după consoană}"),
    ("coastă", "", "2 diftong pasi.yaml:{pasi.yaml:nu e prefix}"), // not co- + astă
    ("preacurat", "", "3 diftong pasi.yaml:{pasi.yaml:restul}"), // prea + curat: no prefix
    ("preîncălzi", "", "3 hiat pasi.yaml:{pasi.yaml:prefix și un cuvânt cunoscut}"), // pre + încălzi: pre- is no prea-
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
    lines.extend(
        rules
            .pasi
            .iter()
            .map(|s| (format!("{PASI}:{}", s.pas.text), s.entry.line)),
    );
    let origin = |o: Origin| match o {
        Origin::Explicit(line) => format!("silabisiri.yaml:{line}"),
        Origin::Dexonline => "dexonline".into(),
        Origin::Partial => "parțial".into(),
        Origin::Rule(Rule::Step(i)) => format!("{PASI}:{}", step_name(i).1),
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
    // classes: C a consonant, V a vowel, and those of clase.yaml, with their digraphs (P: ch)
    (&["ie"], &["C"], &[], &[], &[], "prieten", 3, true),
    (&["ie"], &["C"], &[], &[], &[], "femeie", 5, false),
    (&["ie"], &["V"], &[], &[], &[], "femeie", 5, true),
    (&["ie"], &["L"], &[], &[], &[], "fier", 2, true),
    (&["ie"], &["L"], &[], &[], &[], "client", 3, false),
    (&["ie"], &["P"], &[], &[], &[], "chiem", 3, true),
    (&["ie"], &["P"], &[], &[], &[], "ghieț", 3, true),
    (&["ie"], &["PPP"], &[], &[], &[], "chghchie", 7, true), // three digraphs: six letters
    (&["ier"], &["^CC"], &[], &[], &[], "chier", 3, true),
    (&["ier"], &["^CC"], &[], &[], &[], "schier", 4, false), // three consonants from the start
    (&["ier"], &[], &["^C", "^CC"], &[], &[], "zbier", 3, false),
    (&["ier"], &[], &["^C", "^CC"], &[], &[], "plumier", 5, true),
    (&["ie"], &[], &[], &["CV$"], &[], "fiere", 2, true),
    (&["ie"], &[], &[], &["CV"], &[], "piept", 2, false),
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
