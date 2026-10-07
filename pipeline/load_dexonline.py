"""Load dexonline's public database dump into a local MariaDB/MySQL server.

Only the tables build_db.py reads are imported (Lexeme, InflectedForm,
Inflection, and EntryLexeme, EntryDefinition, Definition, Source for the
DEX-MDA-DOOM flag). Each import replaces those tables, so rerunning it with a newer
dump is how dexonline updates come in.

Usage:
  uv run python pipeline/load_dexonline.py --download          # fetch the latest dump, then load it
  uv run python pipeline/load_dexonline.py                     # load data/dex-database.sql.gz
  uv run python pipeline/load_dexonline.py --client "C:/mariadb/bin/mariadb.exe" --port 3307
"""
import argparse
import gzip
import re
import subprocess
import sys
import urllib.request
from collections.abc import Iterator
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DUMP_URL = "https://dexonline.ro/static/download/dex-database.sql.gz"
TABLES = {"Lexeme", "InflectedForm", "Inflection",  # everything build_db.py reads
          "EntryLexeme", "EntryDefinition", "Definition", "Source"}
_TABLE_START = re.compile(rb"-- Table structure for table `(\w+)`")


def download(url: str, path: Path) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_name(path.name + ".part")
    with urllib.request.urlopen(url) as r, tmp.open("wb") as f:
        print(f"{url}: Last-Modified {r.headers.get('Last-Modified')}, {r.headers.get('Content-Length')} bytes")
        while chunk := r.read(1 << 20):
            f.write(chunk)
    tmp.replace(path)


def filtered(dump: Path) -> Iterator[bytes]:
    """The dump's header (session settings) and the sections of TABLES."""
    table = None
    with gzip.open(dump, "rb") as f:
        for line in f:
            if m := _TABLE_START.match(line):
                table = m.group(1).decode()
            if table is None or table in TABLES:
                yield line


def main() -> int:
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("--download", action="store_true", help=f"fetch {DUMP_URL} first")
    p.add_argument("--dump", type=Path, default=ROOT / "data" / "dex-database.sql.gz")
    p.add_argument("--client", default="mariadb", help="MariaDB/MySQL command-line client")
    p.add_argument("--host", default="127.0.0.1")
    p.add_argument("--port", type=int, default=3306)
    p.add_argument("--user", default="root")
    p.add_argument("--password", default="")
    p.add_argument("--database", default="dexonline")
    args = p.parse_args()

    if args.download:
        download(DUMP_URL, args.dump)
    client = [args.client, f"--host={args.host}", f"--port={args.port}", f"--user={args.user}",
              "--default-character-set=utf8mb4", "--max-allowed-packet=1G"]
    if args.password:
        client.append(f"--password={args.password}")

    subprocess.run(
        [*client, "-e", f"CREATE DATABASE IF NOT EXISTS `{args.database}` "
                        "CHARACTER SET utf8mb4 COLLATE utf8mb4_romanian_ci"],
        check=True,
    )
    with subprocess.Popen([*client, args.database], stdin=subprocess.PIPE) as proc:
        for line in filtered(args.dump):
            proc.stdin.write(line)
        proc.stdin.close()
    if proc.returncode:
        return proc.returncode

    counts = subprocess.run(
        [*client, "--batch", "--skip-column-names", args.database, "-e",
         "SELECT 'Lexeme', COUNT(*) FROM Lexeme UNION ALL SELECT 'InflectedForm', COUNT(*) FROM InflectedForm"],
        check=True, capture_output=True, text=True,
    )
    print(counts.stdout.strip())
    return 0


if __name__ == "__main__":
    sys.exit(main())
