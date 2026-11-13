//! A rule file's YAML, read with the position of every value: its keys, its lists and the
//! letters of a word, with the messages of what is wrong.
use std::collections::HashMap;

use marked_yaml::types::{MarkedMappingNode, MarkedScalarNode};
use marked_yaml::{LoadError, LoaderOptions, Marker, Node};
use unicode_normalization::is_nfc;

use crate::{Level, Message, Value};

/// The letters of the Romanian alphabet.
const ALPHABET: &str = "aăâbcdefghiîjklmnopqrsștțuvwxyz";
pub(crate) const VOWELS: &str = "aăâeiîou";

/// The parser's errors that a Romanian hint explains, by a part of their text.
const HINTS: [(&str, &str); 9] = [
    (
        "expected ',' or ']'",
        "lipsește o virgulă între două cuvinte, sau „]” la sfârșitul listei",
    ),
    (
        "did not find expected node content",
        "lipsește un cuvânt: două virgule la rând, sau o listă neterminată",
    ),
    (
        "mapping values are not allowed",
        "„:” nu poate sta aici; verifică indentarea, sau pune cuvântul între ghilimele",
    ),
    ("tabs disallowed", "un tab; indentează numai cu spații"),
    ("found unexpected end of stream", "ghilimelele nu sunt închise"),
    (
        "did not find expected '-' indicator",
        "indentarea nu se potrivește cu a rândurilor de deasupra",
    ),
    (
        "did not find expected key",
        "după valoare urmează altceva decât o cheie nouă; verifică parantezele și indentarea",
    ),
    (
        "unexpected character",
        "acest caracter nu poate începe o valoare; pune cuvântul între ghilimele",
    ),
    ("invalid trailing content", "text în plus după ghilimele"),
];

/// A value's line and column.
pub(crate) type At = (usize, usize);

impl Value {
    pub(crate) fn at(&self) -> At {
        (self.line, self.col)
    }
}

fn position(marker: Option<&Marker>) -> At {
    marker.map_or((0, 0), |m| (m.line(), m.column()))
}

/// Where a node starts.
pub(crate) fn node_at(node: &Node) -> At {
    position(node.span().start())
}

/// A scalar, as a value.
pub(crate) fn value(scalar: &MarkedScalarNode) -> Value {
    let (line, col) = position(scalar.span().start());
    Value {
        text: scalar.as_str().to_string(),
        line,
        col,
    }
}

/// Words joined with commas, the last with `last`: „a”, „b” și „c”.
pub(crate) fn join(words: &[&str], last: &str) -> String {
    match words {
        [] => String::new(),
        [one] => one.to_string(),
        [rest @ .., end] => format!("{} {last} {end}", rest.join(", ")),
    }
}

/// The pairs of `values` with the same key, the first and the repeated one.
pub(crate) fn duplicates(values: &[Value], key: impl Fn(&Value) -> String) -> Vec<(&Value, &Value)> {
    let mut first: HashMap<String, &Value> = HashMap::new();
    values
        .iter()
        .filter_map(|v| match first.get(&key(v)) {
            Some(&f) => Some((f, v)),
            None => {
                first.insert(key(v), v);
                None
            }
        })
        .collect()
}

/// One rule file being read, and the messages of the whole load.
pub(crate) struct File<'m> {
    pub name: &'static str,
    pub messages: &'m mut Vec<Message>,
}

impl File<'_> {
    fn message(&mut self, (line, col): At, level: Level, text: String) {
        self.messages.push(Message {
            dir: String::new(), // filled in by files::read
            file: self.name.to_string(),
            line,
            col,
            level,
            text,
        });
    }

    pub fn error(&mut self, at: At, text: String) {
        self.message(at, Level::Error, text);
    }

    pub fn warning(&mut self, at: At, text: String) {
        self.message(at, Level::Warning, text);
    }

    /// The mapping at the top of `text`, or None after an error.
    pub fn mapping(&mut self, text: &str) -> Option<MarkedMappingNode> {
        // the byte order mark of a file saved "UTF-8 with BOM": invisible, and the parser would
        // read it as part of the first key
        let text = text.strip_prefix('\u{feff}').unwrap_or(text);
        // the parser keeps the first YAML document and drops the others without a word
        for (i, line) in text.lines().enumerate() {
            let line = line.trim_end();
            if let Some(marker) = ["---", "..."]
                .iter()
                .find(|m| line == **m || line.starts_with(&format!("{m} ")))
            {
                self.error(
                    (i + 1, 1),
                    format!(
                        "„{marker}” desparte documente YAML, iar un fișier de reguli are unul singur; șterge rândul."
                    ),
                );
                return None;
            }
        }
        let options = LoaderOptions::default()
            .error_on_duplicate_keys(true)
            .prevent_coercion(true);
        match marked_yaml::parse_yaml_with_options(0, text, options) {
            Ok(Node::Mapping(map)) => Some(map),
            // the loader gives TopLevelMustBeMapping instead; never a file dropped in silence
            Ok(node) => {
                self.error(node_at(&node), "fișierul e format din perechi „cheie: valoare”.".into());
                None
            }
            Err(e) => {
                let (at, text) = load_error(&e);
                self.error(at, text);
                None
            }
        }
    }

    /// The values of the keys of `map`, by key. A key not in `required` or `optional` is an
    /// error, and so is a missing required key, reported at `at`.
    pub fn fields<'n>(
        &mut self,
        map: &'n MarkedMappingNode,
        required: &[&'static str],
        optional: &[&'static str],
        at: At,
    ) -> HashMap<&'static str, (Value, &'n Node)> {
        let all: Vec<&str> = required.iter().chain(optional).copied().collect();
        let mut fields = HashMap::new();
        for (key, node) in map.iter() {
            let key = value(key);
            match all.iter().find(|&&k| k == key.text) {
                Some(&k) => {
                    fields.insert(k, (key, node));
                }
                None => self.error(
                    key.at(),
                    format!(
                        "„{}” nu e o cheie cunoscută aici; cheile sunt {}.",
                        key.text,
                        join(&all, "și")
                    ),
                ),
            }
        }
        for k in required {
            if !fields.contains_key(k) {
                self.error(at, format!("lipsește cheia „{k}”."));
            }
        }
        fields
    }

    /// The scalar value of `key`, or None after an error. An empty value starts where the next
    /// one does, so it is reported at its key.
    pub fn scalar(&mut self, key: &Value, node: &Node) -> Option<Value> {
        match node {
            Node::Scalar(s) if s.as_str().is_empty() => {
                self.error(key.at(), format!("„{}” nu are nicio valoare.", key.text));
                None
            }
            Node::Scalar(s) => Some(value(s)),
            _ => {
                self.error(key.at(), format!("„{}” are un singur cuvânt, nu o listă.", key.text));
                None
            }
        }
    }

    /// The words of the list of `key`, each a scalar.
    pub fn list(&mut self, key: &Value, node: &Node) -> Vec<Value> {
        let Some(seq) = node.as_sequence() else {
            self.error(
                key.at(),
                format!("„{0}” e o listă de cuvinte: {0}: [cuvânt, cuvânt].", key.text),
            );
            return Vec::new();
        };
        seq.iter()
            .filter_map(|item| match item {
                // like an empty value, an empty element is reported at its key
                Node::Scalar(s) if s.as_str().is_empty() => {
                    self.error(key.at(), format!("„{}” are un element gol.", key.text));
                    None
                }
                Node::Scalar(s) => Some(value(s)),
                _ => {
                    self.error(node_at(item), format!("un element din „{}” nu e un cuvânt.", key.text));
                    None
                }
            })
            .collect()
    }

    /// Checks the letters of a word, or of a syllabification with `hyphens`: an error for a
    /// character the standard orthography doesn't write. False after an error.
    pub fn letters(&mut self, v: &Value, hyphens: bool) -> bool {
        let w = &v.text;
        if w.is_empty() {
            self.error(v.at(), "un cuvânt gol.".into());
            return false;
        }
        if !is_nfc(w) {
            self.error(
                v.at(),
                format!("„{w}” are o literă descompusă: o literă urmată de un semn diacritic separat; scrie-o ca un singur caracter."),
            );
            return false;
        }
        for c in w.chars() {
            let error = match c {
                '-' if hyphens => continue,
                c if ALPHABET.contains(c) => continue,
                '-' => format!("„{w}” are o cratimă; cratimele despart doar silabele unei silabisiri."),
                'ş' => format!("„{w}” are ş cu sedilă; scrie ș, cu virgulă."),
                'ţ' => format!("„{w}” are ţ cu sedilă; scrie ț, cu virgulă."),
                'ã' => format!("„{w}” are ã; scrie ă."),
                c if c.is_whitespace() => format!("„{w}” are un spațiu; un cuvânt nu are spații."),
                c if c.is_uppercase() => format!("„{w}” are majuscula „{c}”; scrie cuvintele cu litere mici."),
                c => format!("„{w}” are „{c}”, care nu e o literă a alfabetului românesc."),
            };
            self.error(v.at(), error);
            return false;
        }
        true
    }

    /// A warning for î inside a word, or inside a syllabified word: the standard orthography
    /// writes â there, except in compounds (neînțeles). Checked once per word: on a key of
    /// `silabisiri.yaml`, not on its syllabifications, and not on a stem.
    pub fn inner_i(&mut self, v: &Value) {
        let letters: Vec<char> = v.text.chars().filter(|&c| c != '-').collect();
        // the letters between the first and the last (none in a word of one or two letters)
        let inside = letters.get(1..letters.len().saturating_sub(1)).unwrap_or_default();
        if inside.contains(&'î') {
            self.warning(
                v.at(),
                format!(
                    "„{}” are î în interiorul cuvântului; în grafia standard acolo se scrie â (câine), afară de compuse ca neînțeles.",
                    v.text
                ),
            );
        }
    }
}

/// The position and the message of a parser error.
fn load_error(e: &LoadError) -> (At, String) {
    let at = |m: &Marker| position(Some(m));
    match e {
        LoadError::TopLevelMustBeMapping(m) => (at(m), "fișierul e format din perechi „cheie: valoare”.".into()),
        LoadError::TopLevelMustBeSequence(m) => {
            (at(m), "fișierul e o listă de intrări, fiecare începând cu „- ”.".into())
        }
        LoadError::UnexpectedAnchor(m) => (
            at(m),
            "ancorele YAML (&) nu se folosesc în fișierele de reguli; scrie valoarea întreagă.".into(),
        ),
        LoadError::UnexpectedTag(m) => (
            at(m),
            "etichetele YAML (!) nu se folosesc în fișierele de reguli.".into(),
        ),
        LoadError::MappingKeyMustBeScalar(m) => (at(m), "o cheie e un singur cuvânt, nu o listă.".into()),
        LoadError::ScanError(m, scan) => {
            let info = scan.info();
            let hint = HINTS
                .iter()
                .find(|(en, _)| info.contains(en))
                .map(|(_, ro)| ro.to_string());
            (
                at(m),
                format!("YAML greșit: {}.", hint.unwrap_or_else(|| format!("„{info}”"))),
            )
        }
        LoadError::DuplicateKey(d) => {
            let (key, first) = (value(&d.key), value(&d.prev_key));
            (
                key.at(),
                format!(
                    "„{}” apare de două ori ca cheie (întâi la rândul {}); păstreaz-o o singură dată.",
                    key.text, first.line
                ),
            )
        }
    }
}
