//! Compare the Rust port with Python on the full data (temporary: see rust-port-plan.md).
//!
//! Writes the same files as pipeline/parity.py, which documents them.
//! Run: cargo run --release -p grafia-concisa --example parity -- [DATA [OUT]]
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use grafia_concisa::build::{Record, build};
use grafia_concisa::rules::{concise_by_rules, convert_text, normalize, revert_segments, syllabify_all};
use grafia_concisa::{Dictionary, Segment};

const SWITCHES: [(bool, bool); 4] = [(false, true), (true, true), (false, false), (true, false)]; // (ee, ai)

/// One line per text: each segment's text, <its alternatives>, ?its unknown word.
fn render(segments: &[Segment]) -> String {
    let rendered: Vec<String> = segments
        .iter()
        .map(|s| {
            let mut x = s.t.clone();
            if !s.alts.is_empty() {
                x += &format!("<{}>", s.alts.join("/"));
            }
            if let Some(unknown) = &s.unknown {
                x += &format!("?{unknown}");
            }
            x
        })
        .collect();
    rendered.join("¦")
}

fn write(path: &Path, lines: impl IntoIterator<Item = String>) {
    let text: String = lines.into_iter().map(|x| x + "\n").collect();
    fs::write(path, text).expect("write an output file");
}

fn clean(value: &str) -> String {
    normalize(value.trim())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let data = PathBuf::from(args.first().map_or("data", String::as_str));
    let out = args
        .get(1)
        .map_or_else(|| data.join("parity").join("rust"), PathBuf::from);
    fs::create_dir_all(&out).expect("create the output directory");
    let records: Vec<Record> = fs::read_to_string(data.join("records.tsv"))
        .expect("read records.tsv")
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
        .map(|line| {
            let parts: Vec<&str> = line.split('\t').collect();
            Record::new(if parts.len() >= 4 { &parts } else { &parts[..1] })
        })
        .collect();
    let known: HashSet<String> = fs::read_to_string(data.join("known.txt"))
        .expect("read known.txt")
        .lines()
        .map(String::from)
        .collect();

    let mut refs = BTreeSet::new();
    for Record(fields) in &records {
        if fields.len() >= 4 {
            let verb = fields[4..].iter().any(|x| clean(x).to_uppercase() == "V");
            refs.insert((clean(&fields[1]), clean(&fields[2]), verb));
        }
    }
    let mut lines: Vec<String> = refs
        .iter()
        .map(|(lemma, hyph, verb)| {
            let readings: Vec<String> = syllabify_all(lemma, hyph, &known, *verb)
                .into_iter()
                .map(|(s, source)| format!("{s}/{}", source.name()))
                .collect();
            format!(
                "{lemma}\t{hyph}\t{}\t{}",
                if *verb { "V" } else { "" },
                readings.join(" ")
            )
        })
        .collect();
    lines.sort();
    write(&out.join("references.tsv"), lines);

    let (tables, stats) = build(records, &known);
    write(&out.join("stats.tsv"), stats.iter().map(|(k, v)| format!("{k}\t{v}")));
    write(
        &out.join("forms.tsv"),
        tables.forms.iter().map(|(f, r)| {
            format!(
                "{f}\t{}\t{}\t{}",
                r.concise,
                r.concise_ee.as_deref().unwrap_or(""),
                r.alternatives.join(" ")
            )
        }),
    );
    write(&out.join("eu_lemmas.txt"), tables.eu_lemmas.iter().cloned());
    write(&out.join("ea_lemmas.txt"), tables.ea_lemmas.iter().cloned());
    write(
        &out.join("iesc_lemmas.tsv"),
        tables.iesc_lemmas.iter().map(|(k, v)| format!("{k}\t{v}")),
    );
    write(
        &out.join("ee_revert.tsv"),
        tables
            .ee_revert
            .iter()
            .map(|(k, (f, a))| format!("{k}\t{f}\t{}", a.join(" "))),
    );

    let dict: &dyn Dictionary = &tables;
    write(
        &out.join("convert.tsv"),
        tables.forms.keys().flat_map(|form| {
            let mut c = form.chars();
            let initial = c
                .next()
                .map(|x| x.to_uppercase().collect::<String>())
                .unwrap_or_default()
                + c.as_str();
            let text = format!("{form} {initial} {}", form.to_uppercase());
            SWITCHES.map(|(ee, ai)| {
                format!(
                    "{form}\t{}{}\t{}",
                    u8::from(ee),
                    u8::from(ai),
                    render(&convert_text(&text, dict, ee, ai))
                )
            })
        }),
    );
    write(
        &out.join("unknown.tsv"),
        tables.forms.keys().flat_map(|form| {
            [false, true].map(|ee| {
                format!(
                    "{form}\t{}\t{}",
                    u8::from(ee),
                    concise_by_rules(form, ee, &HashMap::new()).join(" ")
                )
            })
        }),
    );
    let spellings: BTreeSet<&String> = tables
        .forms
        .values()
        .flat_map(|r| std::iter::once(&r.concise).chain(&r.concise_ee).chain(&r.alternatives))
        .collect();
    write(
        &out.join("revert.tsv"),
        spellings.into_iter().flat_map(|x| {
            [false, true].map(|ee| {
                format!(
                    "{x}\t{}\t{}",
                    u8::from(ee),
                    render(&revert_segments(x, if ee { Some(dict) } else { None }))
                )
            })
        }),
    );
    println!("{}: {stats:?}", out.display());
}
