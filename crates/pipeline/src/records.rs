//! The build's input: inflected-form records and the words the prefix rule may
//! find, from dexonline in MariaDB or MySQL, or from text files.
use std::collections::HashSet;

use anyhow::{Context, Result, bail};
use grafia_concisa::build::{Record, Stats, Tables, build};
use grafia_concisa::rules::normalize;
use mysql::prelude::Queryable;
use mysql::{Conn, OptsBuilder, Value, from_value_opt};

use crate::Server;

// One row per inflected form with seven columns:
//   form ('ideea'), lemma ('idee'), the lemma's dexonline hyphenation
//   (complete, partial such as '-li-e', or ''), number ('S' or 'P'),
//   'V' if the lemma is a verb, else '',
//   'D' if the lexeme is DEX-MDA-DOOM, else '': it has an active definition of its own
//   (headword = the lexeme, not just mentioned in another word's definition: acneie
//   under coș) from a source named MDA*, DEX*, NODEX* or DOOM* (MDA2, DEX '09, NODEX,
//   DOOM 3; not 'dexonline'),
//   'F' if the lexeme is lowercase and of model F, T or IL (a feminine noun or
//   unclassified), else '': a lemma in -iare is then a long infinitive (apreciare).
// Partial and missing hyphenations are completed by rules::syllabify.
// The order is fixed, since the first record wins a tie (the spec's priority list):
// the oldest lexeme first, then its oldest form.
const SOURCE_QUERY: &str = "
SELECT f.formNoAccent, l.formNoAccent, IFNULL(l.hyphenations, ''), IF(i.number = 2, 'P', 'S'),
       IF(l.modelType IN ('V', 'VT'), 'V', ''), IF(dex.lexemeId IS NULL, '', 'D'),
       IF(l.modelType IN ('F', 'T', 'IL') AND l.formNoAccent COLLATE utf8mb4_bin REGEXP '^[a-zăâîșț]',
          'F', '')
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

/// Builds the tables from dexonline on the server; also returns the date of its data.
pub fn build_from_mysql(server: &Server) -> Result<(Tables, Stats, String)> {
    let opts = OptsBuilder::new()
        .ip_or_hostname(Some(&server.host))
        .tcp_port(server.port)
        .user(Some(&server.user))
        .pass(Some(&server.password).filter(|p| !p.is_empty()))
        .db_name(Some(&server.database))
        .prefer_socket(false)
        .init(vec!["SET NAMES utf8mb4", "SET time_zone = '+00:00'"]);
    let mut conn = Conn::new(opts).with_context(|| format!("connecting to {}:{}", server.host, server.port))?;

    let date: Option<Option<String>> = conn.query_first(DATE_QUERY)?;
    let known = conn
        .query_iter(KNOWN_QUERY)?
        .map(|row| Ok(normalize(&field(row?.unwrap().swap_remove(0))?)))
        .collect::<Result<HashSet<String>>>()?;
    // the records are streamed into the build; the first error stops them
    let mut failed = None;
    let records = conn.query_iter(SOURCE_QUERY)?.map_while(|row| {
        let record = row.map_err(anyhow::Error::from).and_then(|row| {
            let fields = row.unwrap().into_iter().map(field).collect::<Result<_>>()?;
            Ok(Record(fields))
        });
        record.map_err(|e| failed = Some(e)).ok()
    });
    let (tables, stats) = build(records, &known);
    match failed {
        Some(e) => Err(e.context("reading the records")),
        None => Ok((tables, stats, date.flatten().unwrap_or_default())),
    }
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
}
