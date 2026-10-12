//! The build's input: inflected-form records and the words the prefix rule may
//! find, from dexonline in MariaDB or MySQL, or from text files, which `export`
//! writes from the server.
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;

use anyhow::{Context, Result, bail};
use grafia_concisa::build::{Record, Stats, Tables, build};
use grafia_concisa::rules::normalize;
use mysql::prelude::Queryable;
use mysql::{Conn, OptsBuilder, Value, from_value_opt};

use crate::Server;

// One row per inflected form with nine columns:
//   form ('ideea'), lemma ('idee'), the lemma's dexonline hyphenation
//   (complete, partial such as '-li-e', or ''), number ('S' or 'P'),
//   'V' if the lemma is a verb, else '',
//   'D' if the lexeme is DEX-MDA-DOOM, else '': it has an active definition of its own
//   (headword = the lexeme, not just mentioned in another word's definition: acneie
//   under coș) from a source named MDA*, DEX*, NODEX* or DOOM* (MDA2, DEX '09, NODEX,
//   DOOM 3; not 'dexonline'),
//   'F' if the lexeme is lowercase and of model F, T or IL (a feminine noun or
//   unclassified), else '': a lemma in -iare is then a long infinitive (apreciare),
//   'I' + the form's inflection and variant ('I52.0'), which pair the forms of an obsolete
//   lemma with those of its current lemma (rules::OBSOLETE: alineează -> aliniază),
//   'M' if the lexeme is of model M or A (a masculine noun or an adjective), else '': a lemma
//   in -eu is then the masculine of its -ee feminine for the -ee rule (zee <- zeu).
// Partial and missing hyphenations are completed by rules::syllabify.
// The order is fixed, since the first record wins a tie (the spec's priority list):
// the oldest lexeme first, then its oldest form.
const SOURCE_QUERY: &str = "
SELECT f.formNoAccent, l.formNoAccent, IFNULL(l.hyphenations, ''), IF(i.number = 2, 'P', 'S'),
       IF(l.modelType IN ('V', 'VT'), 'V', ''), IF(dex.lexemeId IS NULL, '', 'D'),
       IF(l.modelType IN ('F', 'T', 'IL') AND l.formNoAccent COLLATE utf8mb4_bin REGEXP '^[a-zăâîșț]',
          'F', ''),
       CONCAT('I', f.inflectionId, '.', f.variant), IF(l.modelType IN ('M', 'A'), 'M', '')
FROM InflectedForm f
JOIN Lexeme l ON l.id = f.lexemeId
JOIN Inflection i ON i.id = f.inflectionId
LEFT JOIN (
    SELECT DISTINCT el.lexemeId
    FROM EntryLexeme el
    JOIN Lexeme dl ON dl.id = el.lexemeId
    JOIN EntryDefinition ed ON ed.entryId = el.entryId
    JOIN Definition d ON d.id = ed.definitionId AND d.status = 0 AND d.lexicon = dl.formNoAccent
    JOIN Source s ON s.id = d.sourceId
    WHERE s.shortName COLLATE utf8mb4_bin REGEXP '^(MDA|DEX|NODEX|DOOM)'
) dex ON dex.lexemeId = l.id
WHERE f.formNoAccent REGEXP '^[a-zăâîșț]+$'
ORDER BY l.id, f.id
";

// The words the prefix rule may find: every form of a common (lowercase),
// frequent lexeme. The cutoff drops rare words like 'agră' (ne-agră) and 'azim'.
const KNOWN_QUERY: &str = "
SELECT DISTINCT f.formNoAccent
FROM InflectedForm f
JOIN Lexeme l ON l.id = f.lexemeId
WHERE l.formNoAccent COLLATE utf8mb4_bin REGEXP '^[a-zăâîșț]' AND l.frequency >= 0.9
";

// The date of the data: the day (UTC) of dexonline's latest change to a lexeme or a definition.
const DATE_QUERY: &str = "
SELECT DATE(FROM_UNIXTIME(GREATEST((SELECT MAX(modDate) FROM Lexeme), (SELECT MAX(modDate) FROM Definition))))
";

/// The records of a text file (see `build --help`).
pub fn from_text(text: &str) -> Result<impl Iterator<Item = Record> + '_> {
    let records = || {
        text.lines()
            .enumerate()
            .filter(|(_, line)| !line.trim().is_empty() && !line.starts_with('#'))
            .map(|(i, line)| (i + 1, line.split('\t').collect::<Vec<_>>()))
    };
    if let Some((n, _)) = records().find(|(_, fields)| matches!(fields.len(), 2 | 3)) {
        bail!("line {n}: a record has one field (a syllabified headword) or at least four");
    }
    Ok(records().map(|(_, fields)| Record::new(&fields)))
}

/// The known words of a text file, one per line; empty lines and # comments are skipped.
pub fn known_from_text(text: &str) -> HashSet<String> {
    text.lines()
        .map(str::trim)
        .filter(|word| !word.is_empty() && !word.starts_with('#'))
        .map(normalize)
        .collect()
}

fn connect(server: &Server) -> Result<Conn> {
    let opts = OptsBuilder::new()
        .ip_or_hostname(Some(&server.host))
        .tcp_port(server.port)
        .user(Some(&server.user))
        .pass(Some(&server.password).filter(|p| !p.is_empty()))
        .db_name(Some(&server.database))
        .prefer_socket(false)
        .init(vec!["SET NAMES utf8mb4", "SET time_zone = '+00:00'"]);
    Conn::new(opts).with_context(|| format!("connecting to {}:{}", server.host, server.port))
}

fn known_words(conn: &mut Conn) -> Result<HashSet<String>> {
    conn.query_iter(KNOWN_QUERY)?
        .map(|row| Ok(normalize(&field(row?.unwrap().swap_remove(0))?)))
        .collect()
}

/// The records, one per row of SOURCE_QUERY, in its order.
fn records(conn: &mut Conn) -> Result<impl Iterator<Item = Result<Record>> + '_> {
    Ok(conn.query_iter(SOURCE_QUERY)?.map(|row| {
        let fields = row?.unwrap().into_iter().map(field).collect::<Result<_>>()?;
        Ok(Record(fields))
    }))
}

/// Builds the tables from dexonline on the server; also returns the date of its data.
pub fn build_from_mysql(server: &Server) -> Result<(Tables, Stats, String)> {
    let mut conn = connect(server)?;
    let date: Option<Option<String>> = conn.query_first(DATE_QUERY)?;
    let known = known_words(&mut conn)?;
    // the records are streamed into the build; the first error stops them
    let mut failed = None;
    let records = records(&mut conn)?.map_while(|record| record.map_err(|e| failed = Some(e)).ok());
    let (tables, stats) = build(records, &known);
    match failed {
        Some(e) => Err(e.context("reading the records")),
        None => Ok((tables, stats, date.flatten().unwrap_or_default())),
    }
}

/// Writes dexonline's records and known words from the server into `dir`:
/// `records.tsv` and `known.txt`, in the formats of `build --text` and `--known`.
/// Returns the number of records.
pub fn export_from_mysql(server: &Server, dir: &Path) -> Result<usize> {
    let mut conn = connect(server)?;
    let mut known: Vec<String> = known_words(&mut conn)?.into_iter().collect();
    known.sort();
    fs::create_dir_all(dir).with_context(|| format!("creating {}", dir.display()))?;
    let path = dir.join("records.tsv");
    let mut out = BufWriter::new(File::create(&path).with_context(|| format!("writing {}", path.display()))?);
    let mut n = 0;
    for record in records(&mut conn)? {
        out.write_all(text_line(&record?)?.as_bytes())?;
        n += 1;
    }
    out.flush()?;
    let known: String = known.iter().map(|word| format!("{word}\n")).collect();
    fs::write(dir.join("known.txt"), known).with_context(|| format!("writing {}", dir.display()))?;
    Ok(n)
}

/// A record as a line of a text file (see `from_text`).
fn text_line(Record(fields): &Record) -> Result<String> {
    if let Some(field) = fields.iter().find(|f| f.contains(['\t', '\n', '\r'])) {
        bail!("a field holds a tab or a line break: {field:?}");
    }
    Ok(fields.join("\t") + "\n")
}

/// A column's text; NULL reads as ''.
fn field(value: Value) -> Result<String> {
    Ok(match value {
        Value::NULL => String::new(),
        value => from_value_opt(value)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_records() {
        let records: Vec<Record> = from_text("soa-re\n\n# a comment\nideea\tidee\ti-de-e\tS\tD\n")
            .unwrap()
            .collect();
        assert_eq!(
            records,
            [
                Record::new(&["soa-re"]),
                Record::new(&["ideea", "idee", "i-de-e", "S", "D"])
            ]
        );
        let error = from_text("soa-re\nideea\tidee\ti-de-e\n").err().unwrap();
        assert_eq!(
            error.to_string(),
            "line 2: a record has one field (a syllabified headword) or at least four"
        );
        let known = known_from_text(" Abilitat \n\n# a comment\n");
        assert_eq!(known, HashSet::from(["abilitat".to_string()]));
    }

    #[test]
    fn exported_records() {
        // as SOURCE_QUERY gives them: nine fields, some empty
        let records = [
            Record::new(&["ideea", "idee", "i-de-e", "S", "", "D", "", "I13.0", ""]),
            Record::new(&["zeii", "zeu", "", "P", "", "D", "", "I37.0", "M"]),
        ];
        let text: String = records.iter().map(|r| text_line(r).unwrap()).collect();
        assert_eq!(
            text,
            "ideea\tidee\ti-de-e\tS\t\tD\t\tI13.0\t\nzeii\tzeu\t\tP\t\tD\t\tI37.0\tM\n"
        );
        assert_eq!(from_text(&text).unwrap().collect::<Vec<_>>(), records);
        assert!(text_line(&Record::new(&["a\tb", "a", "", "S"])).is_err());
    }
}
