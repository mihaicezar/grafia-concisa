//! Build the word database from inflected-form records.
//!
//! A record is (form, lemma, lemma_hyphenation, number, *flags):
//!   - lemma_hyphenation: dexonline's hyphenation of the lemma, complete, partial
//!     or empty; completed by the fallback rules (see rules::syllabify);
//!   - number: 'P' for plural, anything else singular;
//!   - flags, in any order (others, '' included, are ignored): 'V' if the lemma is a
//!     verb; 'D' if it is DEX-MDA-DOOM (a definition of its own in MDA*, DEX*, NODEX*
//!     or DOOM*), which picks the form an -ee spelling reverts to; 'F' if it is a
//!     lowercase feminine noun or unclassified (dexonline model F, T or IL), which
//!     marks a lemma in -iare as a long infinitive (rules::long_infinitive); 'I' and the
//!     form's inflection and variant (I52.0), which pair the forms of an obsolete lemma
//!     with those of its current lemma (rules::OBSOLETE); 'M' if it is a masculine noun or
//!     an adjective (dexonline model M or A), which makes a lemma in -eu the masculine of
//!     its -ee feminine for the -ee rule (zee <- zeu: zeẹle, not zeile).
//!
//! A record with fewer than four fields is a syllabified headword (dev/test input).
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::sync::LazyLock;

use fancy_regex::Regex;
use indexmap::IndexMap;

use crate::Dictionary;
use crate::rules::{
    AE_OE, CANDIDATE, EE_NOT_MASCULINE, EE_SPELLING, EURI, HIATUS_SUFFIX, NAMES, OBSOLETE, Row, Source, concise_ee,
    concise_from_reference, concise_from_syllables, ee_final, found, iere_spelling, long_infinitive, normalize,
    obsolete, revert_text, syllabifications, syllabify_all,
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
    /// a record of a lemma that is not obsolete
    current_word: bool,
    /// a record of a lowercase lemma: not only a name
    common: bool,
    /// the lemmas of its records
    lemmas: Vec<String>,
    /// (place in rules::OBSOLETE, current lemma, inflection, own spelling) of each obsolete
    /// lemma's first record
    obsolete: Vec<(usize, String, String, String)>,
    /// (its own spelling, its -iere form) of the first record of a long infinitive in -iare
    long_infinitive: Option<(String, String)>,
}

/// Build the tables.
///
/// `known`: all normalized dexonline word forms, for the rules' prefix check.
///
/// Only forms that can differ are stored (ie/ia/ea/oa/iăr, final -ee/-eea/-eei/-eele/
/// -eelor/-euri/-eurile/-eurilor/-oae, the forms of the lemmas in rules::AE_OE, or the
/// stem of an obsolete lemma).
/// A form of a long infinitive in -iare (flag 'F', rules::long_infinitive) is
/// converted as its -iere spelling (apreciare, apreciări -> apreciere, aprecieri),
/// unless another record gives the same form (conciliare: the adjective conciliar).
/// A form whose every record is of an obsolete lemma (rules::OBSOLETE) takes the
/// spelling of the current lemma's form with the same inflection, and its own spelling
/// (from its own syllabification) is its one alternative (alineează: aliniėză, or
/// alineēză); the row records the current form, for the footnote.
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
/// -ee form: computed from the final base form; a singular reading beats a plural. An -ee
/// lemma has a masculine when its -eu form is a lemma with the flag 'M' (zee <- zeu).
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
/// A DEX-MDA-DOOM plural in -eie beats the -ee plural of the same lemma (curcubeẹ: curcubeie).
/// The genitive and articulated plural of the names of rules::NAMES that the records list are
/// added when missing (Andreei, Andreele, Andreelor <- Andreea).
/// Except: a spelling of a word in -ae/-oe/-oae, or a genitive or plural that a kept feminine
/// writes with ẹ, whose character-map reading is a form too (Naẹ: naie, zeẹi: zeiei), reverts to
/// its source when that is a name (a capitalized lemma, not in -oae), with the other form on
/// the switch (Naẹ: Nae or naie), and otherwise to the character map's form (baẹ -> baie,
/// Fōẹ -> foaie, zeẹi -> zeiei).
pub fn build(records: impl IntoIterator<Item = Record>, known: &HashSet<String>) -> (Tables, Stats) {
    let mut stats: Stats = [
        "read",
        "invalid",
        "no_candidate",
        "derived",
        "ee_no_rule",
        "homographs",
        "obsolete",
    ]
    .into_iter()
    .map(|k| (k.to_string(), 0))
    .collect();
    // (lemma, hyphenation, verb) -> [(syllabified, source)], one per reading
    let mut references: HashMap<(String, String, bool), Vec<(String, Source)>> = HashMap::new();
    let mut rows: IndexMap<String, Entry> = IndexMap::new();
    // forms with no candidate, spelled like an -ee spelling (zeii) -> DEX-MDA-DOOM
    let mut unchanged: IndexMap<String, bool> = IndexMap::new();
    // (current lemma, inflection) -> its form, for the forms of obsolete lemmas
    let mut currents: HashMap<(String, String), String> = HashMap::new();
    // lemmas in -ee with a masculine: their -eu form is a masculine noun or an adjective (flag 'M'),
    // for the -ee rule (zee <- zeu; not coree <- coreu: EE_NOT_MASCULINE)
    let mut with_masculine: HashSet<String> = HashSet::new();
    // the names of rules::NAMES that dexonline lists, for their genitive and plural
    let mut names: HashSet<String> = HashSet::new();
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
        let inflection = flags.iter().find(|f| f.starts_with('I')).cloned();
        if let Some(i) = &inflection
            && OBSOLETE.iter().any(|(_, current, ..)| *current == lemma)
        {
            currents.entry((lemma.clone(), i.clone())).or_insert(form.clone());
        }
        if lemma.ends_with("eu") {
            tables.eu_lemmas.insert(lemma.clone());
            let feminine = format!("{}ee", &lemma[..lemma.len() - 2]);
            if flags.contains("M") && !EE_NOT_MASCULINE.contains(&feminine.as_str()) {
                with_masculine.insert(feminine);
            }
        }
        if lemma.ends_with("ea") {
            tables.ea_lemmas.insert(lemma.clone());
        }
        if NAMES.contains(&lemma.as_str()) {
            names.insert(lemma.clone());
        }
        if lemma.ends_with("iesc") && !tables.iesc_lemmas.contains_key(&lemma) {
            let reference = syllabify_all(&lemma, &hyph, known, verb).swap_remove(0).0;
            tables.iesc_lemmas.insert(lemma.clone(), reference);
        }
        // a form with no candidate digraph is stored only for an obsolete lemma (alineez,
        // not alinat)
        let ee_form = ee_final(&form);
        let candidate = found(&CANDIDATE, &form) || ee_form || obsolete(&lemma).is_some();
        if !candidate {
            add(&mut stats, "no_candidate", 1);
            if found(&EE_SPELLING, &form) {
                let entry = unchanged.entry(form).or_insert(false);
                *entry = *entry || dex;
            }
            continue;
        }

        // the spelling to convert, and its lemma
        let (mut spelled, mut reference) = (form.clone(), (lemma.clone(), hyph.clone(), verb));
        let mut long_inf = false;
        if syllabifications(&form).is_some() {
            // Cezar's syllabification of the form decides; it is its own reference, so the verb flag plays no role
            reference = (form.clone(), String::new(), false);
        } else if flags.contains("F") && long_infinitive(&lemma) {
            // apreciare -> apreciere; only the first hyphenation variant is used (rules::dexonline_cuts)
            let first = hyph.split(',').next().unwrap_or("").trim();
            spelled = iere_spelling(&form);
            reference = (iere_spelling(&lemma), iere_spelling(first), verb);
            long_inf = true;
            add(&mut stats, "long_infinitive", 1);
        }
        if !references.contains_key(&reference) {
            // an obsolete lemma's own syllabification gives its spelling on the switch
            let readings = match obsolete(&reference.0) {
                Some((_, syllabified, _)) => vec![(syllabified.to_string(), Source::Manual)],
                None => syllabify_all(&reference.0, &reference.1, known, verb),
            };
            add(&mut stats, &format!("lemma_{}", readings[0].1.name()), 1);
            references.insert(reference.clone(), readings);
        }
        // a long infinitive's own spelling, with the cuts of its -iere form (apreciare <-
        // a-pre-ci-e-re: apreciare, not aprecėre)
        let own = long_inf.then(|| concise_from_reference(&form, &references[&reference][0].0, verb));

        let current_lemma = obsolete(&lemma).map(|(i, _, current)| (i, current));
        let old = current_lemma.is_some();
        // the record's spelling of its first reading: an obsolete lemma's own spelling
        let mut first = None;

        for (syllabified, rank) in &references[&reference] {
            // a long infinitive or an obsolete lemma loses to any other record (conciliare:
            // the adjective; alinea: the imperfect of alini),
            // and rules alone to a backed syllabification (agreare <- agrea a-gre-a)
            let priority = (
                spelled == form && !old,
                *rank > Source::Rules,
                spelled == reference.0,
                *rank,
            );
            let concise = concise_from_reference(&spelled, syllabified, reference.2);
            first.get_or_insert_with(|| concise.clone());
            match rows.get_mut(&form) {
                None => {
                    add(&mut stats, "derived", i64::from(!priority.2));
                    let entry = Entry {
                        concise: concise.clone(),
                        priority,
                        readings: Vec::new(),
                        spellings: IndexMap::new(),
                        dex: false,
                        current_word: false,
                        common: false,
                        lemmas: Vec::new(),
                        obsolete: Vec::new(),
                        long_infinitive: None,
                    };
                    rows.insert(form.clone(), entry);
                }
                Some(row) if priority > row.priority => {
                    add(&mut stats, "derived", i64::from(row.priority.2) - i64::from(priority.2));
                    (row.concise, row.priority) = (concise.clone(), priority);
                }
                Some(_) => {}
            }
            // a long infinitive's -iere spelling changes letters, not cuts: no alternative;
            // nor is an obsolete lemma's spelling one on a current word's form (alinea <- alini)
            let backed = *rank > Source::Rules && spelled == form && !old;
            let spelling = rows[&form].spellings.entry(concise).or_insert(false);
            *spelling = *spelling || backed;
        }
        let row = &mut rows[&form];
        row.dex = row.dex || dex;
        // the form as read, before normalization: Nae, of a name (dexonline capitalizes the forms of
        // names), or lae
        row.common = row.common || !fields.first().is_some_and(|f| f.trim().starts_with(char::is_uppercase));
        if !row.lemmas.contains(&lemma) {
            row.lemmas.push(lemma.clone());
        }
        match (own, current_lemma) {
            (Some(own), _) => {
                if row.long_infinitive.is_none() {
                    row.long_infinitive = Some((own, spelled));
                }
            }
            (None, Some((place, current))) => {
                if let Some(i) = inflection
                    && !row.obsolete.iter().any(|o| o.0 == place)
                {
                    let own = first.expect("a reading");
                    row.obsolete.push((place, current.to_string(), i, own));
                }
            }
            (None, None) => row.current_word = true,
        }

        let reading = (number.to_uppercase() == "P", lemma);
        if ee_form && !row.readings.contains(&reading) {
            row.readings.push(reading);
        }
    }

    // spelling with the -ee rule -> [(form, -ee rule applied)]
    let mut by_spelling: IndexMap<String, Vec<(String, bool)>> = IndexMap::new();
    // a spelling like that of an -aie/-oaie/-eie word -> (its form, a name)
    let mut like_aie: HashMap<String, (String, bool)> = HashMap::new();
    for (form, row) in &rows {
        // an obsolete form: (its spellings, default first; its current forms)
        let obsolete = if row.current_word {
            None
        } else if let Some((own, current)) = &row.long_infinitive {
            Some((vec![row.concise.clone(), own.clone()], vec![current.clone()]))
        } else {
            // each obsolete lemma's reading, in the order of rules::OBSOLETE
            let mut readings: Vec<_> = row
                .obsolete
                .iter()
                .filter_map(|(place, lemma, inflection, own)| {
                    Some((*place, currents.get(&(lemma.clone(), inflection.clone()))?, own))
                })
                .collect();
            readings.sort_by_key(|(place, ..)| *place);
            let mut current: Vec<String> = Vec::new();
            for (_, c, _) in &readings {
                if !current.contains(c) {
                    current.push(c.to_string());
                }
            }
            let own = readings.iter().map(|(.., own)| own.to_string());
            match current.as_slice() {
                [] => None,
                // one current form: its spelling, then the obsolete ones (alineează: aliniėză, alineēză)
                [one] => {
                    let concise = rows.get(one).map_or(one, |r| &r.concise).clone();
                    Some((std::iter::once(concise).chain(own).collect(), current))
                }
                // several: only the obsolete spellings (alineam: alineam, alinēm)
                _ => Some((own.collect(), current)),
            }
        };
        if let Some((mut spellings, current)) = obsolete {
            let mut seen = HashSet::new();
            spellings.retain(|s| seen.insert(s.clone()));
            let concise = spellings.remove(0);
            add(&mut stats, "obsolete", 1);
            let row = Row {
                concise,
                concise_ee: None,
                alternatives: spellings,
                current,
            };
            tables.forms.insert(form.clone(), row);
            continue;
        }
        let mut readings = row.readings.clone();
        readings.sort_by_key(|(plural, _)| *plural);
        let ee = readings.iter().find_map(|(plural, lemma)| {
            concise_ee(form, &row.concise, lemma, *plural, with_masculine.contains(lemma)).map(|e| (e, lemma))
        });
        add(
            &mut stats,
            "ee_no_rule",
            i64::from(!readings.is_empty() && ee.is_none()),
        );
        // the spelling of a word in -ae/-oe/-oae, or of the genitive or plural that a kept
        // feminine writes with ẹ (not an -eu lemma's plural, as before: bombeẹle), reads by
        // character map as a word in -aie/-oaie/-eie, which may exist too (Naẹ: naie, zeẹi: zeiei)
        if let Some((e, lemma)) = &ee
            && (form.ends_with("oae")
                || AE_OE.contains(&lemma.as_str())
                || e.ends_with("ẹi")
                || (e.ends_with("ẹle") || e.ends_with("ẹlor")) && !lemma.ends_with("eu"))
        {
            // a name, unless in -oae (Foae: foaie)
            let name = !row.common && !form.ends_with("oae");
            like_aie.insert(e.clone(), (form.clone(), name));
        }
        let ee = ee.map(|(e, ..)| e);
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
                current: Vec::new(),
            },
        );
    }
    // the genitive and articulated plural of the names of rules::NAMES that dexonline lists
    // without them (Andreei, Andreele, Andreelor <- Andreea; Zoei <- Zoe)
    let mut generated: Vec<String> = Vec::new();
    for name in NAMES.iter().filter(|n| names.contains(**n)) {
        let base = name.strip_suffix('a').unwrap_or(name);
        for (end, plural) in [("i", false), ("le", true), ("lor", true)] {
            let form = format!("{base}{end}");
            if tables.forms.contains_key(&form) {
                continue;
            }
            let concise = concise_from_syllables(&syllabify_all(&form, "", known, false).swap_remove(0).0);
            let ee = concise_ee(&form, &concise, name, plural, false).filter(|e| *e != concise);
            if let Some(e) = &ee {
                like_aie.insert(e.clone(), (form.clone(), true));
            }
            let spelling = ee.clone().unwrap_or_else(|| concise.clone());
            by_spelling
                .entry(spelling)
                .or_default()
                .push((form.clone(), ee.is_some()));
            let row = Row {
                concise,
                concise_ee: ee,
                alternatives: Vec::new(),
                current: Vec::new(),
            };
            tables.forms.insert(form.clone(), row);
            generated.push(form);
        }
    }
    for form in unchanged.keys() {
        by_spelling.entry(form.clone()).or_default().push((form.clone(), false));
    }
    let dex: HashMap<&str, bool> = rows
        .iter()
        .map(|(f, row)| (f.as_str(), row.dex))
        .chain(unchanged.iter().map(|(f, &d)| (f.as_str(), d)))
        .chain(generated.iter().map(|f| (f.as_str(), false)))
        .collect();
    let lemmas = |f: &str| rows.get(f).map_or(&[][..], |r| r.lemmas.as_slice());
    for (spelling, forms) in &by_spelling {
        if !forms.iter().any(|&(_, changed)| changed) {
            continue;
        }
        let plain = revert_text(spelling);
        // the -aie/-oaie/-eie word of a spelling like_aie exists: a name is the default, with
        // that word on the switch (Naẹ: Nae or naie); a common word is an old spelling of it,
        // which the character map gives (baẹ -> baie, Fōẹ -> foaie, zeẹi -> zeiei)
        if let Some((form, name)) = like_aie.get(spelling)
            && forms.iter().any(|(f, _)| *f == plain)
        {
            if *name {
                tables.ee_revert.insert(spelling.clone(), (form.clone(), vec![plain]));
            }
            continue;
        }
        let rank = |f: &str| (dex[f], dex[f] && !found(&EURI, f));
        let top = forms.iter().map(|(f, _)| rank(f)).max().expect("a form");
        let mut tied: Vec<&String> = forms.iter().map(|(f, _)| f).filter(|f| rank(f) == top).collect();
        tied.sort_by_key(|f| ro_order(f));
        // a DEX-MDA-DOOM plural in -eie beats the -ee plural of its own lemma (curcubeẹ: curcubeie,
        // not curcubee), but not another lemma's (bordeẹ: bordeie of bordei, or bordee of bordeu)
        if ["eie", "eiele", "eielor"].iter().any(|end| plain.ends_with(end))
            && tied.contains(&&plain)
            && dex[plain.as_str()]
            && tied
                .iter()
                .all(|f| **f == plain || lemmas(f).iter().any(|l| lemmas(&plain).contains(l)))
        {
            tied.retain(|f| **f == plain);
        }
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
    add(&mut stats, "names_generated", count(generated.len()));
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
