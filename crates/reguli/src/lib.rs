//! The rule files of `reguli/`, which a linguist edits (`rule-files-plan.md`): reading them,
//! with the line and column of every value, the loading checks, whose messages are in
//! Romanian, and the Rust code the rules compile to. The core's build script
//! (`crates/core/codegen.rs`) writes that code into the core, and `cargo run -p grafia-reguli`
//! prints the messages (`src/main.rs`). The spec's *Rule files* section gives the format.
//!
//! - `yaml`: a file's YAML, read with positions: its keys, its lists, the letters of a word;
//! - `files`: each file, and the checks of its lists and tables;
//! - `rust`: the Rust code.
mod files;
mod rust;
mod yaml;

use std::collections::BTreeMap;
use std::fmt::{self, Display};
use std::fs;
use std::path::{Path, PathBuf};

pub use rust::{meta, rust};

/// The file of Cezar's syllabifications, which maps words to their syllabifications.
pub const SILABISIRI: &str = "silabisiri.yaml";
/// The obsolete lemmas: their file and key.
pub const LEME: (&str, &str) = ("invechite.yaml", "leme");
/// The classes of letters of the contexts, which map a capital to its letters and digraphs.
pub const CLASE: &str = "clase.yaml";
/// The steps of the fallback rules, in order.
pub const PASI: &str = "pasi.yaml";
/// The classes that are given, not defined in `CLASE`: C, a consonant (any letter that is not a
/// vowel), and V, a vowel.
pub const GIVEN_CLASSES: [char; 2] = ['C', 'V'];

/// A list of words of a rule file: its file and key, the constant of the core's `rules` it
/// compiles to (`&[&str]`), what each of its words must be, and the words of another list that
/// it holds without writing them.
#[derive(Clone, Copy, Debug)]
pub struct List {
    pub file: &'static str,
    pub key: &'static str,
    pub constant: &'static str,
    pub shape: Shape,
    /// The key of another list of the same file and an ending: that list's words in it count
    /// as words of this one too, and this one doesn't write them again.
    pub plus: Option<(&'static str, &'static str)>,
}

/// What each word of a list must be, besides its letters.
#[derive(Clone, Copy, Debug)]
pub enum Shape {
    Any,
    /// ends in one of these
    Ending(&'static [&'static str]),
    /// ends in a vowel + -ian
    VowelIan,
    /// holds â
    WithA,
}

/// The lists of words, in the order of their files and of their keys: a new list is a line
/// here, and the core's code that reads its constant. The names in -oe of `nume` follow the
/// -ae/-oe rule, so they count as words of `ae_oe`.
#[rustfmt::skip]
pub const LISTS: [List; 8] = [
    List { file: "invechite.yaml", key: "iare_pastrate", constant: "IARE_KEPT", shape: Shape::Ending(&["iare"]), plus: None },
    List { file: "ee.yaml", key: "nume", constant: "NAMES", shape: Shape::Ending(&["eea", "ee", "oe"]), plus: None },
    List { file: "ee.yaml", key: "ae_oe", constant: "AE_OE", shape: Shape::Ending(&["ae", "oe"]), plus: Some(("nume", "oe")) },
    List { file: "ee.yaml", key: "eea_neschimbate", constant: "EEA_UNCHANGED", shape: Shape::Ending(&["eea"]), plus: None },
    List { file: "ee.yaml", key: "fara_masculin", constant: "EE_NOT_MASCULINE", shape: Shape::Ending(&["ee"]), plus: None },
    List { file: "neconvertite.yaml", key: "nume_latine", constant: "NO_CONVERT", shape: Shape::VowelIan, plus: None },
    List { file: "a-i.yaml", key: "radacini_cu_a", constant: "A_ROOTS", shape: Shape::WithA, plus: None },
    List { file: "necunoscute.yaml", key: "iesc", constant: "IESC_ENDINGS", shape: Shape::Any, plus: None },
];

/// An exception table of the fallback rules, a file of entries: its file, the constant of the
/// core's `rules` it compiles to (a `Table`), the pair of vowels its step consults it for, and
/// whether its entries make that pair a hiatus or a diphthong. The step of `PASI` that consults
/// it names it in its `tabel`.
#[derive(Clone, Copy, Debug)]
pub struct Table {
    pub file: &'static str,
    pub constant: &'static str,
    pub pair: &'static str,
    pub hiat: bool,
}

/// The exception tables.
#[rustfmt::skip]
pub const TABLES: [Table; 4] = [
    Table { file: "diftong-ia.yaml", constant: "IA_DIPHTHONG", pair: "ia", hiat: false },
    Table { file: "hiat-ea.yaml", constant: "EA_HIATUS", pair: "ea", hiat: true },
    Table { file: "hiat-ie.yaml", constant: "IE_HIATUS", pair: "ie", hiat: true },
    Table { file: "hiat-oa.yaml", constant: "OA_HIATUS", pair: "oa", hiat: true },
];

/// The keys of an entry of a table: its patterns, its contexts, and its words.
pub const TIPAR: &str = "tipar";
pub const CONTEXTS: [&str; 4] = ["ante", "non_ante", "post", "non_post"];
pub const EXEMPLE: &str = "exemple";
pub const CONTRAEXEMPLE: &str = "contraexemple";

/// The rule files, in the order of their messages, which is also the order they are read in:
/// `SILABISIRI`, `LEME`'s file, the files of `LISTS`, `CLASE` (which the contexts of the steps
/// and of the tables use), `PASI`, then the files of `TABLES`.
pub fn files() -> Vec<&'static str> {
    let mut files = vec![SILABISIRI, LEME.0];
    let lists = LISTS.iter().map(|l| l.file);
    for file in lists.chain([CLASE, PASI]).chain(TABLES.iter().map(|t| t.file)) {
        if !files.contains(&file) {
            files.push(file);
        }
    }
    files
}

/// The list of `LISTS` with this file and key.
pub fn list(file: &str, key: &str) -> Option<&'static List> {
    LISTS.iter().find(|l| l.file == file && l.key == key)
}

/// The table of `TABLES` in this file.
pub fn table(file: &str) -> Option<&'static Table> {
    TABLES.iter().find(|t| t.file == file)
}

/// The workspace's `reguli/`.
pub fn default_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../reguli")
}

/// How the messages name `dir`: `reguli` for the workspace's, or else the path as given.
pub fn shown(dir: &Path) -> String {
    let canonical = |dir: &Path| fs::canonicalize(dir).ok();
    match canonical(dir) {
        Some(d) if Some(&d) == canonical(&default_dir()).as_ref() => "reguli".into(),
        _ => dir.display().to_string().trim_end_matches(['/', '\\']).to_string(),
    }
}

/// A value of a rule file: its text, and the line and column where it starts (from 1).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Value {
    pub text: String,
    pub line: usize,
    pub col: usize,
}

/// An obsolete lemma, from `leme` in `invechite.yaml`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Obsolete {
    /// The lemma, syllabified: a-li-ne-a.
    pub lema: Value,
    /// Its current lemma: alinia.
    pub actuala: Value,
    /// The stem its forms start with: aline.
    pub tulpina: Value,
    /// dexonline lists it without its forms.
    pub fara_forme: bool,
}

/// An entry of an exception table: where it starts (its first key), its patterns, its contexts
/// and its words. A key it doesn't have is an empty list.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Entry {
    pub line: usize,
    pub col: usize,
    /// One or more patterns, each holding the table's pair: letters, with `^` for the word's
    /// start and `$` for its end.
    pub tipar: Vec<Value>,
    /// What is just before a pattern (`ante`: one of these; `non_ante`: none of them) and just
    /// after it (`post`, `non_post`), in the order of `CONTEXTS`.
    pub contexts: [Vec<Value>; 4],
    /// The words it decides.
    pub exemple: Vec<Value>,
    /// Words that hold a pattern, but which its contexts or anchors exclude.
    pub contraexemple: Vec<Value>,
}

/// A step of `pasi.yaml`: where it starts (its first key), its name, its patterns, contexts and
/// words (as an entry of a table has them), its prefixes, and what decides it. A key it doesn't
/// have is an empty list, 0 (`rest_cunoscut`), false (`doar: verb`) or None (`tabel`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Step {
    /// Its name, for the trace and the messages.
    pub pas: Value,
    /// Its patterns, contexts and words, and where it starts.
    pub entry: Entry,
    /// The word starts with one of these, and the pair is the prefix's last vowel and the next
    /// letter.
    pub prefixe: Vec<Value>,
    /// With `prefixe`: the rest of the word is a known word of at least this many letters.
    pub rest_cunoscut: usize,
    /// Not when the word starts with one of these, followed by a known word.
    pub non_prefixe: Vec<Value>,
    /// `doar: verb`: only in a verb infinitive.
    pub verb: bool,
    /// An exception table, by its file: the step applies only where one of its entries does.
    pub tabel: Option<Value>,
    /// Whether it makes the pair a hiatus (`rezultat: hiat`) or a diphthong.
    pub hiat: bool,
}

/// The rules of the files.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Rules {
    /// `silabisiri.yaml`: each word, with its syllabifications, the default first.
    pub silabisiri: Vec<(Value, Vec<Value>)>,
    /// `leme` in `invechite.yaml`.
    pub leme: Vec<Obsolete>,
    /// The words written in each list of `LISTS`, by its constant.
    pub lists: BTreeMap<&'static str, Vec<Value>>,
    /// The entries of each table of `TABLES`, in order, by its file.
    pub tables: BTreeMap<&'static str, Vec<Entry>>,
    /// `clase.yaml`: each class, a capital, with its letters and digraphs.
    pub clase: Vec<(Value, Vec<Value>)>,
    /// `pasi.yaml`: the steps, in order.
    pub pasi: Vec<Step>,
}

/// An error stops the build; a warning doesn't.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Level {
    Error,
    Warning,
}

/// A message on a rule file, at a line and a column (both 0 for the whole file), or on the
/// directory itself (no file).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    /// The directory, as `shown` names it.
    pub dir: String,
    /// The file's name, empty for the directory.
    pub file: String,
    pub line: usize,
    pub col: usize,
    pub level: Level,
    pub text: String,
}

/// reguli/ee.yaml:3:30: eroare: „zoş” are ş cu sedilă; scrie ș, cu virgulă.
impl Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.dir, self.file)?;
        if self.line > 0 {
            write!(f, ":{}:{}", self.line, self.col)?;
        }
        let level = match self.level {
            Level::Error => "eroare",
            Level::Warning => "avertisment",
        };
        write!(f, ": {level}: {}", self.text)
    }
}

impl Message {
    /// The message as a GitHub Actions workflow command, which GitHub shows on that line of a
    /// pull request's files: `::error file=reguli/ee.yaml,line=3,col=30::…`; with no file
    /// for the directory itself.
    pub fn annotation(&self) -> String {
        let level = match self.level {
            Level::Error => "error",
            Level::Warning => "warning",
        };
        let mut place = String::new();
        if !self.file.is_empty() {
            place = format!(" file={}/{}", self.dir, self.file);
            if self.line > 0 {
                place += &format!(",line={},col={}", self.line, self.col);
            }
        }
        let text = self.text.replace('%', "%25").replace('\r', "%0D").replace('\n', "%0A");
        format!("::{level}{place}::{text}")
    }
}

/// True if the file `name` of `reguli/` is read as a rule file: a file in .yaml or .yml that is
/// not hidden. The others (.DS_Store, an editor's backup or swap file, a note) are left alone.
pub fn is_yaml(name: &str) -> bool {
    !name.starts_with('.') && (name.ends_with(".yaml") || name.ends_with(".yml"))
}

/// The texts of rule files, by name: None for a file that couldn't be read.
pub type Texts = BTreeMap<String, Option<String>>;

/// The texts of the YAML files of `dir` (`is_yaml`), by name; a file that can't be read, or
/// isn't in UTF-8, has none, and a message. Err with the message of a directory that can't
/// be read.
pub fn read_texts(dir: &Path) -> Result<(Texts, Vec<Message>), Message> {
    let message = |file: &str, text: String| Message {
        dir: shown(dir),
        file: file.to_string(),
        line: 0,
        col: 0,
        level: Level::Error,
        text,
    };
    let entries = fs::read_dir(dir)
        .and_then(|entries| entries.collect::<Result<Vec<_>, _>>())
        .map_err(|e| message("", format!("dosarul nu poate fi citit ({e}).")))?;
    let (mut texts, mut messages) = (BTreeMap::new(), Vec::new());
    for entry in entries {
        let name = entry.file_name().to_string_lossy().into_owned();
        if !is_yaml(&name) {
            continue;
        }
        let text = match fs::read(entry.path()) {
            Ok(bytes) => String::from_utf8(bytes)
                .inspect_err(|_| {
                    messages.push(message(
                        &name,
                        "fișierul nu e scris în UTF-8; salvează-l ca UTF-8.".into(),
                    ))
                })
                .ok(),
            Err(e) => {
                messages.push(message(&name, format!("fișierul nu poate fi citit ({e}).")));
                None
            }
        };
        texts.insert(name, text);
    }
    Ok((texts, messages))
}

/// Reads the rule files in `dir` and runs the loading checks: the rules, unless a message is
/// an error, and the messages, by file and line.
pub fn load(dir: &Path) -> (Option<Rules>, Vec<Message>) {
    match read_texts(dir) {
        Ok((texts, messages)) => files::read(&texts, messages, &shown(dir)),
        Err(message) => (None, vec![message]),
    }
}

/// The same as `load`, on the texts of the files, by name, named as in the workspace's `reguli/`.
pub fn load_texts(texts: &BTreeMap<String, String>) -> (Option<Rules>, Vec<Message>) {
    let texts = texts
        .iter()
        .map(|(name, text)| (name.clone(), Some(text.clone())))
        .collect();
    files::read(&texts, Vec::new(), "reguli")
}
