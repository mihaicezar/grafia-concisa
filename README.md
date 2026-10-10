# grafia concisă

A Romanian orthography that writes the diphthongs ie, ia, ea and oa as ẹ, ė, ē and ō when both vowels are in one syllable: *soare* → *sōre*, *iarbă* → *ėrbă*, but *real* and *vie* (a hiatus) stay as they are.

Rules: [`concise-writing-spec.md`](concise-writing-spec.md) (in Romanian: [`README.ro.md`](README.ro.md)); settled decisions and their reasons: [`DECISIONS.md`](DECISIONS.md).

## Stack

The project is moving from Python to one Rust core that every platform calls (see [`rust-port-plan.md`](rust-port-plan.md)).

| Part | Where | What | Status |
|---|---|---|---|
| Core | `crates/core` (crate `grafia-concisa`) | Rust: the rules, conversion and reverse conversion, the database builder, the data file | done |
| Data file | `grafia.dat` | the word database: [`fst`](https://crates.io/crates/fst) maps read in place, 0.8 MB (0.56 MB gzipped) | done |
| Pipeline | `crates/pipeline` (binary `grafia-pipeline`) | Rust: loads dexonline into MariaDB and builds `grafia.dat` | done |
| Web app, today | `server/`, `grafia_concisa/` | Python: a Flask API and a static page, in Docker, on the Python rules (frozen since 2026-10-10) and `word_db.sqlite` | not deployed |
| Web app, PWA | `crates/wasm`, `web/` | the core in WASM, offline, on GitHub Pages; replaces the Flask app and the rest of Python | phase 3 |
| iOS | `crates/ffi`, `ios/` | a keyboard: converts as you type, or types ẹ ė ē ō | phase 4 |
| Android | `crates/ffi`, `android/` | *grafia concisă* in the text selection menu | phase 5 |

- **The core** uses `fancy-regex` (the rules' patterns need lookbehind), `unicode-normalization` (NFC), `indexmap` (ordered maps, for the order of the alternatives), `fst` and `crc32fast` (the data file's checksum). It has no I/O: the platforms pass it the data file's bytes.
- **The data file** is built by the pipeline from [dexonline](https://dexonline.ro)'s public database dump. A web page loads it into memory; the mobile apps will memory-map it, so it costs the iOS keyboard little memory. Its layout is in [`crates/core/src/datafile.rs`](crates/core/src/datafile.rs).
- **The pipeline** uses `mysql` (a pure Rust client), `flate2` (to filter the gzipped dump), `ureq` with the system's TLS (to download it) and `clap`.
- **The Python part** is frozen: rule changes go into Rust only. The Flask app keeps the last Python rules until the PWA replaces it. Its tests still run in CI.
- **CI** (GitHub Actions, [`.github/workflows/tests.yml`](.github/workflows/tests.yml)) runs the Python tests, and `cargo fmt --check`, `cargo clippy -D warnings` and `cargo test` on the Rust workspace.

## Development

Tools:
- Rust stable, through [rustup](https://rustup.rs). On Windows it needs the Visual Studio Build Tools with the C++ workload (for the MSVC linker).
- [uv](https://docs.astral.sh/uv/) for the Python part, never pip: `uv sync` sets up the environment from `uv.lock`, and every Python command runs with `uv run`.
- A MariaDB or MySQL server, only to build the full data file (see below).

```
cargo test --workspace                                  # Rust tests
uv run python -m unittest discover -s server            # Python tests
cargo run --release -p grafia-pipeline -- seed          # small dev data file, data/grafia.dat (no MariaDB)
uv run flask --app server/server.py run                 # Flask dev server, on word_db.sqlite
```

Both test suites must pass before every commit. The tests are tables in [`crates/core/tests/tables.rs`](crates/core/tests/tables.rs): a new case is one line. The tables that need a database run twice, on the built tables in memory and through the data file.

The pipeline runs from the repository root: its default paths are under `data/`, which git ignores. `cargo run --release -p grafia-pipeline -- help` lists its commands and options.

## Deploying the web app

The web app is not deployed yet. It will be hosted on [GitHub Pages](https://docs.github.com/en/pages), which hosts static sites for free from a public repository. That needs the PWA (phase 3 of [`rust-port-plan.md`](rust-port-plan.md)); until then, the app needs a server.

### Today: the Flask app, in Docker

The page (`server/static/index.html`) sends the text to the Flask API (`server/server.py`), which converts it with the Python rules and `word_db.sqlite`. So GitHub Pages can't host it, but any machine with Docker can run it:

1. Get a `word_db.sqlite` into the repository root (see below).
2. Build the image with `docker build -t grafia-concisa .`. It copies `word_db.sqlite` from the working directory, since the database is not in git.
3. Run it with `docker run -p 10000:10000 grafia-concisa`, then open <http://localhost:10000>. The container listens on `$PORT` (10000 by default), and `/healthz` answers once the database is readable.

`word_db.sqlite` was built by the Python pipeline, which the Rust pipeline replaced on 2026-10-10. The last commit that has it is `5a94a10`, and its database works with today's Flask app, since the Python rules haven't changed since. Without a `word_db.sqlite` at hand:

```bash
git worktree add ../grafia-concisa-py 5a94a10
cd ../grafia-concisa-py
uv run python pipeline/seed_db.py    # the small dev database, or the full one as in that commit's README
cp word_db.sqlite ../grafia-concisa/
```

The Flask app is not deployed anywhere: the PWA replaces it.

### With the PWA: GitHub Pages

The rules then run in the browser (WASM), so the site is only static files. `web/`, `cargo xtask web` and the workflow come with phase 3; the steps will be:

1. Once, in the repository's settings: *Pages* → *Build and deployment* → *Source*: **GitHub Actions**.
2. The data file is built locally and committed as `web/grafia.dat`, since CI has neither MariaDB nor the dexonline dump: `cargo run --release -p grafia-pipeline -- build --mysql --port 3307 --out web/grafia.dat` (see *Building the data file from dexonline*). After a dexonline update, rebuild it, run the tests and commit it.
3. On each push to `main`, a GitHub Actions workflow runs `cargo xtask web` and publishes `web/dist/` to GitHub Pages.
4. The site is then at <https://mihaicezar.github.io/grafia-concisa/>, over HTTPS, which the offline cache (service worker) needs. A custom domain can be set under *Pages*.

## Building the data file from dexonline

`grafia.dat` is built from [dexonline](https://dexonline.ro)'s public database dump. The data is GPL, and the attribution must stay in the UI.

The steps are:
1. A local MariaDB (or MySQL) server.
2. Load the dump's tables into it (`grafia-pipeline load`).
3. Build `grafia.dat` from them (`grafia-pipeline build`).

All of it is repeatable. Steps 2 and 3 are also how dexonline updates come in.

### 1. A MariaDB server (once)

Any MariaDB or MySQL works. On Windows, the portable zip needs no installer and no service:

```bash
# from https://mariadb.org/download/ : the "ZIP file" package, e.g. mariadb-11.4.13-winx64.zip
M=/path/to/mariadb-11.4.13-winx64/bin        # where the zip was unpacked
D=/path/to/mariadb-data                      # data directory (about 800 MB once loaded)
"$M/mariadb-install-db.exe" --datadir="$D" --port=3307   # once: creates the data dir and $D/my.ini
"$M/mariadbd.exe" --defaults-file="$D/my.ini" --console  # start (runs in the foreground)
"$M/mariadb-admin.exe" --user=root --port=3307 --host=127.0.0.1 shutdown  # stop
```

The server then listens on `127.0.0.1:3307`, as user `root` with no password. Port 3307 avoids clashing with any other MySQL.

On Linux or macOS, install MariaDB with the package manager instead; the default port is 3306.

### 2. Load dexonline

```bash
cargo run --release -p grafia-pipeline -- load --download --client "$M/mariadb.exe" --port 3307
```

- `--download` fetches `https://dexonline.ro/static/download/dex-database.sql.gz` into `data/` (about 380 MB), and prints the dump's `Last-Modified` date.
- The dump is gzipped SQL from `mysqldump`, regenerated by dexonline about daily. Without `--download`, `load` loads the dump already in `data/`.
- Only the tables the build reads are imported: `Lexeme`, `InflectedForm`, `Inflection`, `EntryLexeme`, `EntryDefinition`, `Definition` and `Source`. They go into the database `dexonline` (utf8mb4, Romanian collation), and each run replaces them. The rest of the dump is skipped. The `mariadb` client does the import.
- A load takes about 3 minutes and ends by printing the row counts. For the 2026-10-04 dump: `Lexeme 320935`, `InflectedForm 2274627`.

### 3. Build `grafia.dat`

```bash
cargo run --release -p grafia-pipeline -- build --mysql --port 3307
```

This writes `data/grafia.dat` (`--out` for another path), in about 2 minutes, most of it in MariaDB. It prints the file's size, the date of the data and the build's stats. The date is the day of dexonline's latest change to a lexeme or a definition, so it always matches the data. For the 2026-10-04 dump (2026-10-10 rules): 816,260 bytes, data of 2026-10-03; 292,128 forms stored, of which 51 homographs and 2,749 with an -ee spelling, plus 2,613 reverse -ee spellings (`ee_revert`, 129 of them ties), 1,066 lemmas in -eu (`eu_lemmas`), 3,434 in -ea (`ea_lemmas`) and 65 in -iesc (`iesc_lemmas`). Lemma syllabifications came from dexonline (4,165 complete, 6,601 partial), Cezar's explicit list (18) and the rules alone (80,859).

What the build reads:
- the inflected forms, with their lemma and number (`InflectedForm`, `Inflection`);
- each lemma's dexonline syllabification (`Lexeme.hyphenations`; complete or partial, completed by the rules);
- the frequent common words, which the prefix rule checks against (`Lexeme.frequency`);
- which lexemes are DEX-MDA-DOOM, with a definition of their own (`Definition.lexicon`) in a source named MDA\*, DEX\*, NODEX\* or DOOM\* (`EntryLexeme`, `EntryDefinition`, `Definition`, `Source`), for reverse -ee.

How these are turned into spellings is in the spec. The records are read in a fixed order (by lexeme, then form id), since the first record wins a tie, so the same data always gives the same file, byte for byte.

`build --text FILE` reads the records from a text file instead, one per line (`build --help` gives the format), with `--known FILE` for the words the prefix rule checks against. Its data file has no date.

### Integrating a dexonline update

1. Start the server.
2. Run step 2 with `--download`, then step 3.
3. Run the tests.
4. Compare the build's stats with the previous ones (forms stored, homographs, lemma sources). A large jump points to a change in dexonline's data or schema worth looking at before deploying.
5. With the PWA, copy the file to `web/grafia.dat` and commit it (see *Deploying the web app*).

Updates are picked up automatically wherever dexonline's syllabifications changed or new words appeared. Cezar's explicit syllabifications (`rules::SYLLABIFICATIONS`) still prevail.

### Pitfalls

- **Table names on Windows.** A Windows server stores table names in lowercase (`lexeme`). Queries are case-insensitive there, so nothing needs to change.
- **`WARNING: option --ssl-verify-server-cert is disabled`.** The `mariadb` client prints it for a local server with no password. It is harmless.

### This machine (as of 2026-10-10)

- Rust 1.99 (MSVC) through rustup, in `%USERPROFILE%\.cargo\bin`, with the `wasm32-unknown-unknown` target.
- The builds so far used a portable MariaDB 11.4.13 in a temporary Claude Code scratchpad, which can be cleaned up at any time:

| | |
|---|---|
| scratchpad | `C:\Users\user\AppData\Local\Temp\claude\d--Projects-grafia-concisa\9fe3f075-ee4f-4681-b644-3980cad05de6\scratchpad` |
| server | `mariadb-11.4.13-winx64\bin\mariadbd.exe`, `--defaults-file=mdata\my.ini` |
| data | `mdata\`, database `dexonline` (2026-10-04 dump) |
| connection | `127.0.0.1:3307`, user `root`, no password |

This server crashes on `DESCRIBE` and `SHOW COLUMNS` (an access violation in MariaDB 11.4.13; the data survives), so look at the schema some other way, or restart it afterwards with the command of step 1.

If it is gone, redo steps 1 and 2. For a lasting setup, keep the server and data outside the scratchpad.

## License

The code is licensed under the [GNU GPL v3 or later](LICENSE). The word data comes from [dexonline](https://dexonline.ro) under the GPL, and its attribution must stay in the UI.
