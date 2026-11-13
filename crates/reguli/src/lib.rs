//! The rule files of `reguli/`, which a linguist edits (`rule-files-plan.md`): reading them,
//! with the line and column of every value, the loading checks, whose messages are in
//! Romanian, and the Rust code the rules compile to. The core's build script
//! (`crates/core/codegen.rs`) writes that code into the core, and `cargo run -p grafia-reguli`
//! prints the messages (`src/main.rs`). The spec's *Rule files* section gives the format.
//!
//! - `yaml`: a file's YAML, read with positions: its keys, its lists, the letters of a word;
//! - `files`: each file, and the checks of its lists;
//! - `rust`: the Rust code.
mod files;
mod rust;
mod yaml;

use std::collections::BTreeMap;
use std::fmt::{self, Display};
use std::fs;
use std::path::{Path, PathBuf};

pub use rust::rust;

/// The file of Cezar's syllabifications, which maps words to their syllabifications.
pub const SILABISIRI: &str = "silabisiri.yaml";
/// The obsolete lemmas: their file and key.
pub const LEMME: (&str, &str) = ("invechite.yaml", "lemme");

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

/// The rule files, in the order of their messages: `SILABISIRI`, `LEMME`'s file, then the files
/// of `LISTS`.
pub fn files() -> Vec<&'static str> {
    let mut files = vec![SILABISIRI, LEMME.0];
    for list in &LISTS {
        if !files.contains(&list.file) {
            files.push(list.file);
        }
    }
    files
}

/// The list of `LISTS` with this file and key.
pub fn list(file: &str, key: &str) -> Option<&'static List> {
    LISTS.iter().find(|l| l.file == file && l.key == key)
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

/// An obsolete lemma, from `lemme` in `invechite.yaml`.
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

/// The rules of the files.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Rules {
    /// `silabisiri.yaml`: each word, with its syllabifications, the default first.
    pub silabisiri: Vec<(Value, Vec<Value>)>,
    /// `lemme` in `invechite.yaml`.
    pub lemme: Vec<Obsolete>,
    /// The words written in each list of `LISTS`, by its constant.
    pub lists: BTreeMap<&'static str, Vec<Value>>,
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
