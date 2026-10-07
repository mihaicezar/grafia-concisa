"""Build word_db.sqlite from word-form records.

Sources:
  --text FILE   one record per line, tab-separated:
                  form, lemma, lemma hyphenation (complete, partial or empty), number (S/P),
                  then optional flags in any order: V if the lemma is a verb, D if it is
                  DEX-MDA-DOOM (below)
                e.g. 'ideea\tidee\ti-de-e\tS'. A single column = a syllabified headword.
  --mysql       dexonline MySQL dump loaded locally (needs the `pipeline` extra)

Usage:
  uv run python pipeline/build_db.py --text forms.txt
  uv run --extra pipeline python pipeline/build_db.py --mysql --user root --port 3306 --database dexonline
"""
import argparse
import sys
from collections.abc import Iterator
from pathlib import Path

from grafia_concisa.db import Record, build
from grafia_concisa.rules import normalize

ROOT = Path(__file__).resolve().parent.parent

# One row per inflected form with six columns:
#   form ('ideea'), lemma ('idee'), the lemma's dexonline hyphenation
#   (complete, partial such as '-li-e', or ''), number ('S' or 'P'),
#   'V' if the lemma is a verb, else '',
#   'D' if the lexeme is DEX-MDA-DOOM, else '': it has an active definition of its own
#   (headword = the lexeme, not just mentioned in another word's definition: acneie
#   under coș) from a source named MDA*, DEX*, NODEX* or DOOM* (MDA2, DEX '09, NODEX,
#   DOOM 3; not 'dexonline').
# Partial and missing hyphenations are completed by rules.syllabify.
SOURCE_QUERY = """
SELECT f.formNoAccent, l.formNoAccent, IFNULL(l.hyphenations, ''), IF(i.number = 2, 'P', 'S'),
       IF(l.modelType IN ('V', 'VT'), 'V', ''), IF(dex.lexemeId IS NULL, '', 'D')
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
"""

# The words the prefix rule may find: every form of a common (lowercase),
# frequent lexeme. The cutoff drops rare words like 'agră' (ne-agră) and 'azim'.
KNOWN_QUERY = """
SELECT DISTINCT f.formNoAccent
FROM InflectedForm f
JOIN Lexeme l ON l.id = f.lexemeId
WHERE l.formNoAccent COLLATE utf8mb4_bin REGEXP '^[a-zăâîșț]' AND l.frequency >= 0.9
"""


def from_text(path: Path) -> Iterator[Record]:
    with path.open(encoding="utf-8") as f:
        for line in f:
            if line.strip() and not line.startswith("#"):
                parts = line.rstrip("\n").split("\t")
                yield tuple(parts) if len(parts) >= 4 else parts[0]


def known_forms(args: argparse.Namespace) -> set[str]:
    with _connect(args) as conn:
        cur = conn.cursor()
        cur.execute(KNOWN_QUERY)
        return {normalize(form) for (form,) in cur}


def from_mysql(args: argparse.Namespace) -> Iterator[Record]:
    with _connect(args) as conn:
        cur = conn.cursor()
        cur.execute(SOURCE_QUERY)
        yield from cur


def _connect(args: argparse.Namespace):
    import mysql.connector

    return mysql.connector.connect(
        host=args.host, port=args.port, user=args.user, password=args.password, database=args.database
    )


def main() -> int:
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    src = p.add_mutually_exclusive_group(required=True)
    src.add_argument("--text", type=Path)
    src.add_argument("--mysql", action="store_true")
    p.add_argument("--host", default="localhost")
    p.add_argument("--port", type=int, default=3306)
    p.add_argument("--user", default="root")
    p.add_argument("--password", default="")
    p.add_argument("--database", default="dexonline")
    p.add_argument("--out", type=Path, default=ROOT / "word_db.sqlite")
    args = p.parse_args()

    if args.text:
        stats = build(args.out, from_text(args.text))
    else:
        stats = build(args.out, from_mysql(args), known_forms(args))
    print(f"{args.out}: {stats}")
    return 0 if stats["stored"] else 1


if __name__ == "__main__":
    sys.exit(main())
