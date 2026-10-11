//! grafia-pipeline: builds the data file `grafia.dat` from dexonline.
//!
//! Run from the repository root (the defaults are under `data/`):
//!   cargo run --release -p grafia-pipeline -- load --download --client mariadb --port 3307
//!   cargo run --release -p grafia-pipeline -- build --mysql --port 3307
//!   cargo run --release -p grafia-pipeline -- build --text records.tsv [--known known.txt]
//!   cargo run --release -p grafia-pipeline -- export --port 3307
//!   cargo run --release -p grafia-pipeline -- seed
mod load;
mod records;
mod seed;

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, ensure};
use clap::{Args, Parser};
use grafia_concisa::build::{Stats, Tables, build};
use grafia_concisa::{RULES_HASH, datafile};

#[derive(Parser)]
#[command(version, about = "Builds the grafia concisă data file from dexonline.")]
enum Command {
    /// Load dexonline's database dump into a local MariaDB or MySQL server
    Load(load::LoadArgs),
    /// Build the data file from dexonline in MariaDB or MySQL, or from records in a text file
    Build(BuildArgs),
    /// Write dexonline's records from MariaDB or MySQL as text files, for build --text and the snapshot example
    Export {
        #[command(flatten)]
        server: Server,
        /// The directory for records.tsv and known.txt
        #[arg(long, default_value = "data")]
        out: PathBuf,
    },
    /// Build a small development data file, without MariaDB
    Seed {
        #[arg(long, default_value = "data/grafia.dat")]
        out: PathBuf,
    },
}

/// The MariaDB or MySQL server holding dexonline.
#[derive(Args)]
struct Server {
    #[arg(long, default_value = "127.0.0.1")]
    host: String,
    #[arg(long, default_value_t = 3306)]
    port: u16,
    #[arg(long, default_value = "root")]
    user: String,
    #[arg(long, default_value = "")]
    password: String,
    #[arg(long, default_value = "dexonline")]
    database: String,
}

#[derive(Args)]
#[group(id = "source", required = true, multiple = false, args = ["mysql", "text"])]
struct BuildArgs {
    /// Read dexonline from the server
    #[arg(long)]
    mysql: bool,
    /// Read the records from FILE, one per line, tab-separated: form, lemma, the lemma's
    /// hyphenation (complete, partial or empty), number (S/P), then optional flags in any
    /// order: V (a verb), D (DEX-MDA-DOOM), F (a lowercase feminine noun or unclassified).
    /// A line with one column is a syllabified headword; empty lines and # comments are skipped.
    #[arg(long, value_name = "FILE")]
    text: Option<PathBuf>,
    /// With --text: the words the prefix rule may find, one per line
    #[arg(long, value_name = "FILE", requires = "text")]
    known: Option<PathBuf>,
    #[command(flatten)]
    server: Server,
    #[arg(long, default_value = "data/grafia.dat")]
    out: PathBuf,
}

fn main() -> Result<()> {
    match Command::parse() {
        Command::Load(args) => load::load(&args),
        Command::Build(args) => {
            let (tables, stats, date) = match &args.text {
                Some(path) => {
                    let text = read(path)?;
                    let known = match &args.known {
                        Some(path) => records::known_from_text(&read(path)?),
                        None => HashSet::new(),
                    };
                    let (tables, stats) = build(records::from_text(&text)?, &known);
                    (tables, stats, String::new())
                }
                None => records::build_from_mysql(&args.server)?,
            };
            write(&args.out, &tables, &stats, &date)
        }
        Command::Export { server, out } => {
            let n = records::export_from_mysql(&server, &out)?;
            println!("{}: records.tsv ({n} records), known.txt", out.display());
            Ok(())
        }
        Command::Seed { out } => {
            let (tables, stats) = build(seed::records(), &HashSet::new());
            write(&out, &tables, &stats, "")
        }
    }
}

fn read(path: &Path) -> Result<String> {
    fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))
}

/// Writes the data file atomically, then prints its size and the build's stats.
fn write(out: &Path, tables: &Tables, stats: &Stats, date: &str) -> Result<()> {
    ensure!(stats["stored"] > 0, "no forms to store: {stats:?}");
    let bytes = datafile::write(tables, date)?;
    if let Some(dir) = out.parent() {
        fs::create_dir_all(dir).with_context(|| format!("creating {}", dir.display()))?;
    }
    let tmp = out.with_file_name(format!("{}.tmp", out.file_name().context("no file name")?.display()));
    let written = fs::write(&tmp, &bytes).and_then(|()| fs::rename(&tmp, out));
    if written.is_err() {
        let _ = fs::remove_file(&tmp);
    }
    written.with_context(|| format!("writing {}", out.display()))?;
    let date = if date.is_empty() { "no date" } else { date };
    let size = bytes.len();
    println!(
        "{}: {size} bytes, dexonline data of {date}, rules {RULES_HASH:016x}",
        out.display()
    );
    let stats: Vec<String> = stats.iter().map(|(k, v)| format!("{k} {v}")).collect();
    println!("{}", stats.join(", "));
    Ok(())
}
