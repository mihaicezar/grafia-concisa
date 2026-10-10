//! The data file `grafia.dat`: the tables of the spec's *Data* section, read in
//! place (from a `Vec<u8>` in WASM, memory-mapped on iOS and Android).
//!
//! Layout, integers little-endian:
//!   - the magic `GRAFIAC\0`, the format version (u32) and each section's length (u32);
//!   - the sections, in this order:
//!     - `date`: the date of the dexonline data (UTF-8, empty if unknown);
//!     - `forms`: an `fst::Map` from the normalized form to its entry;
//!     - `eu_lemmas`, `ea_lemmas`: `fst::Set`s;
//!     - `iesc_lemmas`, `ee_revert`: `fst::Map`s from the key to its entry;
//!     - `entries`: the entries, at the offsets the maps give;
//!   - the CRC-32 (u32) of all the bytes before it.
//!
//! An entry's words are each written as an edit of its key: three bytes p, d and
//! n, then n bytes, for `key[..p]` + those bytes + `key[p + d..]` (soare -> sōre:
//! 1, 2, "ō"). Many keys share an edit (boare -> bōre too), and identical entries
//! are stored once.
//!   - `forms`: a byte (bit 0: has an -ee spelling; bits 1-7: the number of
//!     alternatives), the concise spelling, the -ee spelling, then the alternatives;
//!   - `ee_revert`: the number of alternatives, the form, then the alternatives;
//!   - `iesc_lemmas`: the syllabification.
//!
//! The writer sorts everything, so the same tables always give the same bytes.
use std::collections::HashMap;
use std::fmt;
use std::ops::Range;

use fst::{Map, MapBuilder, Set, SetBuilder};

use crate::build::Tables;
use crate::{Dictionary, Row};

const MAGIC: &[u8; 8] = b"GRAFIAC\0";
/// The format version, changed with the layout.
pub const VERSION: u32 = 1;

const DATE: usize = 0;
const FORMS: usize = 1;
const EU_LEMMAS: usize = 2;
const EA_LEMMAS: usize = 3;
const IESC_LEMMAS: usize = 4;
const EE_REVERT: usize = 5;
const ENTRIES: usize = 6;
const SECTIONS: usize = 7;
const HEADER: usize = MAGIC.len() + 4 + 4 * SECTIONS;

/// A data file that can't be written or read.
#[derive(Debug)]
pub struct Error(String);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Error {}

impl From<fst::Error> for Error {
    fn from(e: fst::Error) -> Error {
        Error(format!("data file: {e}"))
    }
}

/// A length within a word: one byte.
fn byte(n: usize, word: &str) -> Result<u8, Error> {
    u8::try_from(n).map_err(|_| Error(format!("data file: {word} is over 255 bytes")))
}

/// The number of alternatives of `key`, at most `max`.
fn count(alternatives: &[String], max: u8, key: &str) -> Result<u8, Error> {
    u8::try_from(alternatives.len())
        .ok()
        .filter(|&n| n <= max)
        .ok_or_else(|| Error(format!("data file: {key} has over {max} alternatives")))
}

/// Writes `word` as an edit of `key` (see the module documentation).
fn push_word(entry: &mut Vec<u8>, key: &str, word: &str) -> Result<(), Error> {
    let common = |a: &mut dyn Iterator<Item = char>, b: &mut dyn Iterator<Item = char>| -> usize {
        a.zip(b).take_while(|(x, y)| x == y).map(|(x, _)| x.len_utf8()).sum()
    };
    let p = common(&mut key.chars(), &mut word.chars());
    let s = common(&mut key[p..].chars().rev(), &mut word[p..].chars().rev());
    let (d, middle) = (key.len() - p - s, &word[p..word.len() - s]);
    // an unchanged word is always written 0, 0, 0
    let p = if d == 0 && middle.is_empty() { 0 } else { p };
    entry.extend([byte(p, key)?, byte(d, key)?, byte(middle.len(), word)?]);
    entry.extend_from_slice(middle.as_bytes());
    Ok(())
}

/// The entries section: each distinct entry once, at its offset.
#[derive(Default)]
struct Entries {
    bytes: Vec<u8>,
    offsets: HashMap<Vec<u8>, u64>,
}

impl Entries {
    fn add(&mut self, entry: Vec<u8>) -> u64 {
        let bytes = &mut self.bytes;
        *self.offsets.entry(entry).or_insert_with_key(|entry| {
            let offset = bytes.len() as u64;
            bytes.extend_from_slice(entry);
            offset
        })
    }
}

fn set<'a>(keys: impl IntoIterator<Item = &'a String>) -> Result<Vec<u8>, Error> {
    let mut set = SetBuilder::memory();
    set.extend_iter(keys)?;
    Ok(set.into_inner()?)
}

/// The data file of `tables`, with the `date` of the dexonline data.
pub fn write(tables: &Tables, date: &str) -> Result<Vec<u8>, Error> {
    let mut entries = Entries::default();

    let mut forms = MapBuilder::memory();
    for (form, row) in &tables.forms {
        let alts = count(&row.alternatives, 127, form)?;
        let mut entry = vec![alts << 1 | u8::from(row.concise_ee.is_some())];
        for word in std::iter::once(&row.concise)
            .chain(&row.concise_ee)
            .chain(&row.alternatives)
        {
            push_word(&mut entry, form, word)?;
        }
        forms.insert(form, entries.add(entry))?;
    }

    let mut iesc = MapBuilder::memory();
    for (lemma, reference) in &tables.iesc_lemmas {
        let mut entry = Vec::new();
        push_word(&mut entry, lemma, reference)?;
        iesc.insert(lemma, entries.add(entry))?;
    }

    let mut ee_revert = MapBuilder::memory();
    for (concise, (form, alts)) in &tables.ee_revert {
        let mut entry = vec![count(alts, u8::MAX, concise)?];
        for word in std::iter::once(form).chain(alts) {
            push_word(&mut entry, concise, word)?;
        }
        ee_revert.insert(concise, entries.add(entry))?;
    }

    let sections: [Vec<u8>; SECTIONS] = [
        date.as_bytes().to_vec(),
        forms.into_inner()?,
        set(&tables.eu_lemmas)?,
        set(&tables.ea_lemmas)?,
        iesc.into_inner()?,
        ee_revert.into_inner()?,
        entries.bytes,
    ];
    let mut out = Vec::with_capacity(HEADER + sections.iter().map(Vec::len).sum::<usize>());
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&VERSION.to_le_bytes());
    for section in &sections {
        let len = u32::try_from(section.len()).map_err(|_| Error("data file: a section over 4 GB".into()))?;
        out.extend_from_slice(&len.to_le_bytes());
    }
    for section in &sections {
        out.extend_from_slice(section);
    }
    let checksum = crc32fast::hash(&out);
    out.extend_from_slice(&checksum.to_le_bytes());
    Ok(out)
}

/// A data file, read in place from its bytes.
pub struct DataFile<B> {
    bytes: B,
    sections: [Range<usize>; SECTIONS],
}

fn u32_at(bytes: &[u8], at: usize) -> usize {
    u32::from_le_bytes(bytes[at..at + 4].try_into().expect("4 bytes")) as usize
}

impl<B: AsRef<[u8]>> DataFile<B> {
    /// Checks the header, the checksum and the sections, so that lookups can't fail.
    pub fn new(bytes: B) -> Result<DataFile<B>, Error> {
        let data = bytes.as_ref();
        if data.len() < HEADER + 4 || &data[..MAGIC.len()] != MAGIC {
            return Err(Error("not a grafia concisă data file".into()));
        }
        let version = u32_at(data, MAGIC.len()) as u32;
        if version != VERSION {
            return Err(Error(format!("data file version {version}, expected {VERSION}")));
        }
        let (body, checksum) = data.split_at(data.len() - 4);
        if crc32fast::hash(body) as usize != u32_at(checksum, 0) {
            return Err(Error("data file corrupted: bad checksum".into()));
        }
        let mut sections: [Range<usize>; SECTIONS] = Default::default();
        let mut at = HEADER;
        for (i, section) in sections.iter_mut().enumerate() {
            let end = at.checked_add(u32_at(data, MAGIC.len() + 4 + 4 * i));
            *section = at..end.ok_or(Error("data file: sections over its size".into()))?;
            at = section.end;
        }
        if at != body.len() {
            return Err(Error("data file: sections not matching its size".into()));
        }
        std::str::from_utf8(&data[sections[DATE].clone()]).map_err(|_| Error("data file: bad date".into()))?;
        for i in [FORMS, IESC_LEMMAS, EE_REVERT] {
            Map::new(&data[sections[i].clone()])?;
        }
        for i in [EU_LEMMAS, EA_LEMMAS] {
            Set::new(&data[sections[i].clone()])?;
        }
        Ok(DataFile { bytes, sections })
    }

    /// The date of the dexonline data, empty if unknown.
    pub fn date(&self) -> &str {
        std::str::from_utf8(self.section(DATE)).expect("checked in new")
    }

    fn section(&self, i: usize) -> &[u8] {
        &self.bytes.as_ref()[self.sections[i].clone()]
    }

    fn contains(&self, i: usize, key: &str) -> bool {
        Set::new(self.section(i)).expect("checked in new").contains(key)
    }

    /// The entry of `key` in the map `i`.
    fn entry<'a>(&'a self, i: usize, key: &'a str) -> Option<Entry<'a>> {
        let offset = Map::new(self.section(i)).expect("checked in new").get(key)?;
        let bytes = self.section(ENTRIES).get(usize::try_from(offset).ok()?..)?;
        Some(Entry { key, bytes })
    }
}

/// An entry being read. Malformed bytes read as None.
struct Entry<'a> {
    key: &'a str,
    bytes: &'a [u8],
}

impl Entry<'_> {
    fn byte(&mut self) -> Option<u8> {
        let (&b, rest) = self.bytes.split_first()?;
        self.bytes = rest;
        Some(b)
    }

    fn word(&mut self) -> Option<String> {
        let (p, d, n) = (self.byte()? as usize, self.byte()? as usize, self.byte()? as usize);
        let middle = std::str::from_utf8(self.bytes.get(..n)?).ok()?;
        self.bytes = &self.bytes[n..];
        Some([self.key.get(..p)?, middle, self.key.get(p.checked_add(d)?..)?].concat())
    }

    fn words(&mut self, n: u8) -> Option<Vec<String>> {
        (0..n).map(|_| self.word()).collect()
    }
}

impl<B: AsRef<[u8]>> Dictionary for DataFile<B> {
    fn form(&self, form: &str) -> Option<Row> {
        let mut entry = self.entry(FORMS, form)?;
        let header = entry.byte()?;
        let concise = entry.word()?;
        let concise_ee = if header & 1 == 1 { Some(entry.word()?) } else { None };
        let alternatives = entry.words(header >> 1)?;
        Some(Row {
            concise,
            concise_ee,
            alternatives,
        })
    }

    fn eu_lemma(&self, lemma: &str) -> bool {
        self.contains(EU_LEMMAS, lemma)
    }

    fn ea_lemma(&self, lemma: &str) -> bool {
        self.contains(EA_LEMMAS, lemma)
    }

    fn iesc_lemma(&self, lemma: &str) -> Option<String> {
        self.entry(IESC_LEMMAS, lemma)?.word()
    }

    fn ee_revert(&self, concise: &str) -> Option<(String, Vec<String>)> {
        let mut entry = self.entry(EE_REVERT, concise)?;
        let alts = entry.byte()?;
        Some((entry.word()?, entry.words(alts)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn edit(key: &str, word: &str) -> Vec<u8> {
        let mut entry = Vec::new();
        push_word(&mut entry, key, word).unwrap();
        entry
    }

    #[test]
    fn edits() {
        // the documentation's example; an unchanged word is the same edit for every key
        assert_eq!(edit("soare", "sōre"), [&[1, 2, 2], "ō".as_bytes()].concat());
        assert_eq!(edit("soare", "soare"), [0, 0, 0]);
        for (key, word) in [
            ("ia", "ė"),
            ("aleẹ", "alee"),
            ("harabagiesc", "harabagi-esc"),
            ("logoree", "logorei"),
            ("aa", "a"),
            ("a", "aa"),
        ] {
            let bytes = edit(key, word);
            let mut entry = Entry { key, bytes: &bytes };
            assert_eq!(entry.word().as_deref(), Some(word), "{key} -> {word}");
            assert!(entry.bytes.is_empty(), "{key} -> {word}");
        }
        assert!(push_word(&mut Vec::new(), &"a".repeat(256), "b").is_err());
        assert!(push_word(&mut Vec::new(), "a", &"b".repeat(256)).is_err());
    }
}
