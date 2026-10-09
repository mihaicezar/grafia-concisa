//! Build the word database from inflected-form records (the port of `grafia_concisa/db.py`).
//!
//! A record is (form, lemma, lemma_hyphenation, number, *flags):
//!   - lemma_hyphenation: dexonline's hyphenation of the lemma, complete, partial
//!     or empty; completed by the fallback rules (see rules::syllabify);
//!   - number: 'P' for plural, anything else singular;
//!   - flags, in any order (others, '' included, are ignored): 'V' if the lemma is a
//!     verb; 'D' if it is DEX-MDA-DOOM (a definition of its own in MDA*, DEX*, NODEX*
//!     or DOOM*), which picks the form an -ee spelling reverts to; 'F' if it is a
//!     lowercase feminine noun or unclassified (dexonline model F, T or IL), which
//!     marks a lemma in -iare as a long infinitive (rules::long_infinitive).
//!
//! A record with fewer than four fields is a syllabified headword (dev/test input).
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::sync::LazyLock;

use fancy_regex::Regex;
use indexmap::IndexMap;

use crate::Dictionary;
use crate::rules::{
    EE_FINAL, EE_SPELLING, EURI, HIATUS_SUFFIX, Row, Source, concise_ee, concise_from_reference, found, has_candidate,
    iere_spelling, long_infinitive, normalize, revert_text, syllabifications, syllabify_all,
};

/// A record: its fields as read (see the module documentation).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Record(pub Vec<String>);

impl Record {
    pub fn new(fields: &[&str]) -> Record {
        Record(fields.iter().map(|f| f.to_string()).collect())
    }
}

/// The database: the tables of the spec's *Data* section.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Tables {
    /// normalized standard spelling -> its spellings
    pub forms: BTreeMap<String, Row>,
    /// normalized lemmas in -eu, for plurals missing from dexonline (bombeele <- bombeu)
    pub eu_lemmas: BTreeSet<String>,
    /// normalized lemmas in -ea, for genitives missing from dexonline (Andreei <- Andreea)
    pub ea_lemmas: BTreeSet<String>,
    /// normalized lemmas in -iesc and their syllabification, for adjective forms missing
    /// from dexonline (harabagiasca <- harabagi-esc)
    pub iesc_lemmas: BTreeMap<String, String>,
    /// a spelling with the -ee rule (aleẹ) -> the standard form it reverts to (alee, not
    /// aleie) and the other forms of a tie (bordeẹ: bordeie or bordee)
    pub ee_revert: BTreeMap<String, (String, Vec<String>)>,
}

impl Dictionary for Tables {
    fn form(&self, form: &str) -> Option<Row> {
        self.forms.get(form).cloned()
    }

    fn eu_lemma(&self, lemma: &str) -> bool {
        self.eu_lemmas.contains(lemma)
    }

    fn ea_lemma(&self, lemma: &str) -> bool {
        self.ea_lemmas.contains(lemma)
    }

    fn iesc_lemma(&self, lemma: &str) -> Option<String> {
        self.iesc_lemmas.get(lemma).cloned()
    }

    fn ee_revert(&self, concise: &str) -> Option<(String, Vec<String>)> {
        self.ee_revert.get(concise).cloned()
    }
}

/// Counts of what the build read and wrote, by name.
pub type Stats = BTreeMap<String, i64>;

fn add(stats: &mut Stats, key: &str, n: i64) {
    *stats.entry(key.to_string()).or_insert(0) += n;
}

static WORD_FULL: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\A[\p{L}\p{Nl}\p{No}]+\z").expect("valid pattern"));

// Romanian alphabetical order: ă, â after a, î after i, ș after s, ț after t
const RO_ALPHABET: &str = "aăâbcdefghiîjklmnopqrsștțuvwxyz";

fn ro_order(word: &str) -> Vec<u32> {
    word.chars()
        .map(|c| {
            RO_ALPHABET
                .chars()
                .position(|x| x == c)
                .map_or(c as u32, |i| 0x10000 + i as u32)
        })
        .collect()
}

fn clean(value: &str) -> String {
    normalize(value.trim())
}

/// A form's best reading so far: is the spelling the form's own, is it backed by
/// more than rules, is the form the lemma itself, and the source of the cuts.
type Priority = (bool, bool, bool, Source);

struct Entry {
    concise: String,
    priority: Priority,
    /// (plural, lemma) readings, for the -ee rule
    readings: Vec<(bool, String)>,
    /// spelling -> backed by more than rules
    spellings: IndexMap<String, bool>,
    /// DEX-MDA-DOOM
    dex: bool,
}

/// Build the tables.
///
/// `known`: all normalized dexonline word forms, for the rules' prefix check.
///
/// Only forms that can differ are stored (ie/ia/ea/oa/iăr, or final -ee/-eea/-eei/-eele/
/// -eelor/-euri/-eurile/-eurilor).
/// A form of a long infinitive in -iare (flag 'F', rules::long_infinitive) is
/// converted as its -iere spelling (apreciare, apreciări -> apreciere, aprecieri),
/// unless another record gives the same form (conciliare: the adjective conciliar).
/// A form in rules::SYLLABIFICATIONS takes all its spellings from there, whatever
/// its lemma (piei: pẹi only, not also piei <- pia).
/// All lemmas in -eu and -ea are stored too (tables eu_lemmas, ea_lemmas), for
/// the -ee rule on unknown words: plurals in -eele/-eelor (bombeele <- bombeu)
/// and genitives in -eei (Andreei <- Andreea). So are the lemmas in -iesc, with
/// their syllabification (iesc_lemmas), for the forms of adjectives missing from
/// dexonline (harabagiasca <- harabagi-esc).
/// Base form: any record beats one of a long infinitive (above); then a record
/// backed by an explicit, dexonline or partial syllabification beats one from
/// rules alone (agreare <- agrea a-gre-a, not the headword agreare by rules);
/// then a record where the form is the lemma itself (its own syllabification)
/// beats one derived from another lemma; then an explicit syllabification beats
/// dexonline, which beats a partial one; otherwise the first record wins. The
/// winner is the default spelling; the others become alternatives (see below).
/// -ee form: computed from the final base form; a singular reading beats a plural.
///
/// Alternatives: the other spellings a form gets from its records (a
/// homograph: ia = the verb lua or the articulated ie). Only spellings backed
/// by an explicit, dexonline or partial syllabification count; one from rules
/// alone is likely a rules error, not a second reading. Except: when one
/// record gives the -ėză/-ėscă/-ėsca suffix, every spelling counts (liază:
/// the noun liază or the verb lia, liėză).
///
/// Reverse -ee (table ee_revert): a spelling with the -ee rule reverts to one of
/// the forms that give it, with the rule or unchanged (zeii: the form zeii, or
/// zeei by the rule). A DEX-MDA-DOOM form beats one without (alee, not aleie);
/// among DEX-MDA-DOOM forms, a plural in -ee beats one in -euri (apogee, not
/// apogeuri). The forms still tied are all offered (bordeẹ: bordeie or bordee),
/// the default being the character map's result, else the first in Romanian
/// alphabetical order. Stored only where the character map alone is not the answer.
pub fn build(records: impl IntoIterator<Item = Record>, known: &HashSet<String>) -> (Tables, Stats) {
    let mut stats: Stats = ["read", "invalid", "no_candidate", "derived", "ee_no_rule", "homographs"]
        .into_iter()
        .map(|k| (k.to_string(), 0))
        .collect();
    // (lemma, hyphenation, verb) -> [(syllabified, source)], one per reading
    let mut references: HashMap<(String, String, bool), Vec<(String, Source)>> = HashMap::new();
    let mut rows: IndexMap<String, Entry> = IndexMap::new();
    // forms with no candidate, spelled like an -ee spelling (zeii) -> DEX-MDA-DOOM
    let mut unchanged: IndexMap<String, bool> = IndexMap::new();
    let mut tables = Tables::default();

    for Record(fields) in records {
        add(&mut stats, "read", 1);
        let (form, lemma, hyph, number, flags) = if fields.len() < 4 {
            let hyph = clean(fields.first().map_or("", String::as_str));
            let form = hyph.replace('-', "");
            (form.clone(), form, hyph, String::new(), HashSet::new())
        } else {
            let flags: HashSet<String> = fields[4..].iter().map(|x| clean(x).to_uppercase()).collect();
            (
                clean(&fields[0]),
                clean(&fields[1]),
                clean(&fields[2]),
                clean(&fields[3]),
                flags,
            )
        };

        if !found(&WORD_FULL, &form) || !found(&WORD_FULL, &lemma) {
            add(&mut stats, "invalid", 1);
            continue;
        }
        let (verb, dex) = (flags.contains("V"), flags.contains("D"));
        if lemma.ends_with("eu") {
            tables.eu_lemmas.insert(lemma.clone());
        }
        if lemma.ends_with("ea") {
            tables.ea_lemmas.insert(lemma.clone());
        }
        if lemma.ends_with("iesc") && !tables.iesc_lemmas.contains_key(&lemma) {
            let reference = syllabify_all(&lemma, &hyph, known, verb).swap_remove(0).0;
            tables.iesc_lemmas.insert(lemma.clone(), reference);
        }
        if !has_candidate(&form, true) {
            add(&mut stats, "no_candidate", 1);
            if found(&EE_SPELLING, &form) {
                let entry = unchanged.entry(form).or_insert(false);
                *entry = *entry || dex;
            }
            continue;
        }

        // the spelling to convert, and its lemma
        let (mut spelled, mut reference) = (form.clone(), (lemma.clone(), hyph.clone(), verb));
        if syllabifications(&form).is_some() {
            // Cezar's syllabification of the form decides; the verb flag only matters to the rules
            reference = (form.clone(), String::new(), false);
        } else if flags.contains("F") && long_infinitive(&lemma) {
            // apreciare -> apreciere; only the first hyphenation variant is used (rules::dexonline_cuts)
            let first = hyph.split(',').next().unwrap_or("").trim();
            spelled = iere_spelling(&form);
            reference = (iere_spelling(&lemma), iere_spelling(first), verb);
            add(&mut stats, "long_infinitive", 1);
        }
        if !references.contains_key(&reference) {
            let readings = syllabify_all(&reference.0, &reference.1, known, verb);
            add(&mut stats, &format!("lemma_{}", readings[0].1.name()), 1);
            references.insert(reference.clone(), readings);
        }

        for (syllabified, rank) in &references[&reference] {
            // a long infinitive loses to any other record (conciliare: the adjective),
            // and rules alone to a backed syllabification (agreare <- agrea a-gre-a)
            let priority = (spelled == form, *rank > Source::Rules, spelled == reference.0, *rank);
            let concise = concise_from_reference(&spelled, syllabified);
            match rows.get_mut(&form) {
                None => {
                    add(&mut stats, "derived", i64::from(!priority.2));
                    let entry = Entry {
                        concise: concise.clone(),
                        priority,
                        readings: Vec::new(),
                        spellings: IndexMap::new(),
                        dex: false,
                    };
                    rows.insert(form.clone(), entry);
                }
                Some(row) if priority > row.priority => {
                    add(&mut stats, "derived", i64::from(row.priority.2) - i64::from(priority.2));
                    (row.concise, row.priority) = (concise.clone(), priority);
                }
                Some(_) => {}
            }
            // a long infinitive's -iere spelling changes letters, not cuts: no alternative
            let backed = *rank > Source::Rules && spelled == form;
            let spelling = rows[&form].spellings.entry(concise).or_insert(false);
            *spelling = *spelling || backed;
        }
        let row = &mut rows[&form];
        row.dex = row.dex || dex;

        let reading = (number.to_uppercase() == "P", lemma);
        if found(&EE_FINAL, &form) && !row.readings.contains(&reading) {
            row.readings.push(reading);
        }
    }

    // spelling with the -ee rule -> [(form, -ee rule applied)]
    let mut by_spelling: IndexMap<String, Vec<(String, bool)>> = IndexMap::new();
    for (form, row) in &rows {
        let mut readings = row.readings.clone();
        readings.sort_by_key(|(plural, _)| *plural);
        let ee = readings
            .iter()
            .find_map(|(plural, lemma)| concise_ee(form, &row.concise, lemma, *plural));
        add(
            &mut stats,
            "ee_no_rule",
            i64::from(!readings.is_empty() && ee.is_none()),
        );
        let suffix = row.spellings.keys().any(|x| found(&HIATUS_SUFFIX, x));
        let alts: Vec<String> = row
            .spellings
            .iter()
            .filter(|&(x, &backed)| (backed || suffix) && *x != row.concise)
            .map(|(x, _)| x.clone())
            .collect();
        add(&mut stats, "homographs", i64::from(!alts.is_empty()));
        let ee = ee.filter(|e| *e != row.concise);
        let spelling = ee.clone().unwrap_or_else(|| row.concise.clone());
        by_spelling
            .entry(spelling)
            .or_default()
            .push((form.clone(), ee.is_some()));
        tables.forms.insert(
            form.clone(),
            Row {
                concise: row.concise.clone(),
                concise_ee: ee,
                alternatives: alts,
            },
        );
    }
    for form in unchanged.keys() {
        by_spelling.entry(form.clone()).or_default().push((form.clone(), false));
    }
    let dex: HashMap<&str, bool> = rows
        .iter()
        .map(|(f, row)| (f.as_str(), row.dex))
        .chain(unchanged.iter().map(|(f, &d)| (f.as_str(), d)))
        .collect();
    for (spelling, forms) in &by_spelling {
        if !forms.iter().any(|&(_, changed)| changed) {
            continue;
        }
        let plain = revert_text(spelling);
        let rank = |f: &str| (dex[f], dex[f] && !found(&EURI, f));
        let top = forms.iter().map(|(f, _)| rank(f)).max().expect("a form");
        let mut tied: Vec<&String> = forms.iter().map(|(f, _)| f).filter(|f| rank(f) == top).collect();
        tied.sort_by_key(|f| ro_order(f));
        let best = if tied.contains(&&plain) {
            plain.clone()
        } else {
            tied[0].clone()
        };
        let alts: Vec<String> = tied.into_iter().filter(|f| **f != best).cloned().collect();
        if best != plain || !alts.is_empty() {
            tables.ee_revert.insert(spelling.clone(), (best, alts));
        }
    }

    let count = |n: usize| i64::try_from(n).expect("a count");
    add(&mut stats, "stored", count(rows.len()));
    add(
        &mut stats,
        "ee",
        count(tables.forms.values().filter(|r| r.concise_ee.is_some()).count()),
    );
    add(&mut stats, "ee_revert", count(tables.ee_revert.len()));
    add(
        &mut stats,
        "ee_revert_ties",
        count(tables.ee_revert.values().filter(|(_, a)| !a.is_empty()).count()),
    );
    add(&mut stats, "eu_lemmas", count(tables.eu_lemmas.len()));
    add(&mut stats, "ea_lemmas", count(tables.ea_lemmas.len()));
    add(&mut stats, "iesc_lemmas", count(tables.iesc_lemmas.len()));
    (tables, stats)
}
