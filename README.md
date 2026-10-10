# grafia concisă

A Romanian orthography that writes the diphthongs ie, ia, ea and oa as ẹ, ė, ē and ō when both vowels are in one syllable: *soare* → *sōre*, *iarbă* → *ėrbă*, but *real* and *vie* (a hiatus) stay as they are.

Rules: [`concise-writing-spec.md`](concise-writing-spec.md) (in Romanian: [`README.ro.md`](README.ro.md)); settled decisions and their reasons: [`DECISIONS.md`](DECISIONS.md).

## The web app

<https://mihaicezar.github.io/grafia-concisa/> (once GitHub Pages is enabled: see *Deploying*) converts a text in either direction:
- **Standard → concis**, with two switches: **î** writes â as î (on by default; *român* and its family keep â), and **-ee → -eẹ** applies the optional -ee rule (*idee* → *ideẹ*).
- **Concis → standard**: the reverse. With **-ee → -eẹ**, the text is read as written with the -ee rule.
- A word with two readings (*ia*: *ė* or *ia*) shows both on a switch; a click picks one. **Copiază** copies the text with the picked spellings.
- A word missing from the word data is converted by the rules alone and underlined with dots.

Everything runs in the browser: the rules are the Rust core compiled to WebAssembly, and the word data is a file the page downloads once. After the first visit the site works offline, and it can be installed as an app (a PWA).

## Stack

The project is one Rust core that every platform calls. It was first written in Python, then ported; the steps are in [`rust-port-plan.md`](rust-port-plan.md).

| Part | Where | What | Status |
|---|---|---|---|
| Core | `crates/core` (crate `grafia-concisa`) | the rules, conversion and reverse conversion, the database builder, the data file | done |
| Data file | `web/grafia.dat` | the word database: [`fst`](https://crates.io/crates/fst) maps read in place, 0.8 MB (0.56 MB gzipped) | done |
| Pipeline | `crates/pipeline` (binary `grafia-pipeline`) | loads dexonline into MariaDB and builds `grafia.dat` | done |
| Web app | `crates/wasm`, `web/`, `xtask/` | the core in WASM, in a static page that works offline, on GitHub Pages | done |
| iOS | `crates/ffi`, `ios/` | a keyboard: converts as you type, or types ẹ ė ē ō | phase 4 |
| Android | `crates/ffi`, `android/` | *grafia concisă* in the text selection menu | phase 5 |

- **The core** uses `fancy-regex` (the rules' patterns need lookbehind), `unicode-normalization` (NFC), `indexmap` (ordered maps, for the order of the alternatives), `fst` and `crc32fast` (the data file's checksum), and `serde` (the answers' JSON). It has no I/O: the platforms pass it the data file's bytes.
- **The data file** is built by the pipeline from [dexonline](https://dexonline.ro)'s public database dump. The web page loads it into memory; the mobile apps will memory-map it, so it costs the iOS keyboard little memory. Its layout is in [`crates/core/src/datafile.rs`](crates/core/src/datafile.rs).
- **The pipeline** uses `mysql` (a pure Rust client), `flate2` (to filter the gzipped dump), `ureq` with the system's TLS (to download it) and `clap`.
- **The web app** is one page with plain JavaScript, no framework and no bundler. `crates/wasm` wraps the core with `wasm-bindgen` and `serde-wasm-bindgen`. `cargo xtask web` builds the site with `wasm-bindgen-cli-support`, which must be the same version as `wasm-bindgen`: both are pinned side by side in the workspace's [`Cargo.toml`](Cargo.toml), the one place for that version. The page's font, [Literata](https://github.com/googlefonts/literata), is served from the site itself.
- **CI** (GitHub Actions, [`.github/workflows/ci.yml`](.github/workflows/ci.yml)) runs `cargo fmt --check`, `cargo clippy -D warnings`, the tests and the WASM tests, and builds the web app. On `main`, it then publishes the web app on GitHub Pages.

## Development

Tools:
- Rust stable, through [rustup](https://rustup.rs), with the WASM target: `rustup target add wasm32-unknown-unknown`. On Windows, Rust needs the Visual Studio Build Tools with the C++ workload (for the MSVC linker).
- [Node](https://nodejs.org) (a current LTS), which runs the WASM tests and `npx serve`.
- `wasm-bindgen-test-runner`, for the WASM tests. Its version must be `wasm-bindgen`'s, which this command reads from `Cargo.lock`; run it again after a change of that version:
  `cargo install wasm-bindgen-cli --locked --version "$(cargo pkgid wasm-bindgen | sed 's/.*@//')"`
- A MariaDB or MySQL server, only to build the full data file (see *Building the data file from dexonline*).

```
cargo test --workspace                                      # tests
cargo test -p grafia-wasm --target wasm32-unknown-unknown   # WASM tests, under Node
cargo xtask web                                             # build the web app into web/dist/
npx serve web/dist                                          # serve it on http://localhost:3000
cargo run --release -p grafia-pipeline -- seed              # small dev data file, data/grafia.dat (no MariaDB)
```

`cargo xtask` runs from anywhere in the repository; the pipeline runs from its root, since its default paths are under `data/`, which git ignores. `cargo run --release -p grafia-pipeline -- help` lists the pipeline's commands and options.

### Tests

The tests must pass before every commit, and the WASM tests too when a commit changes `crates/` or `web/grafia.dat`. CI runs both on every push.
- **The rules** are tested by tables in [`crates/core/tests/tables.rs`](crates/core/tests/tables.rs): a new case is one line. The tables that need a database run twice, on the built tables in memory and through the data file.
- **The WASM tests** ([`crates/wasm/tests/node.rs`](crates/wasm/tests/node.rs)) run the engine under Node on `web/grafia.dat`, the data file the site ships: the answers as the page receives them, the switches, the date of the data, and a bad file. `cargo test --target wasm32-unknown-unknown` runs them with `wasm-bindgen-test-runner` (set in [`.cargo/config.toml`](.cargo/config.toml)); `cargo test --workspace` skips them.
- **The page** has no automated test. After a change to `web/`, check it in a browser, as in step 4 of *Deploying*.

### The web app locally

`cargo xtask web`, then `npx serve web/dist`, and open <http://localhost:3000>. The site must be served over HTTP: browsers don't run its module script or its service worker from a `file://` page. Service workers also need HTTPS, except on `localhost`.

The service worker keeps serving the version it cached. After a rebuild, close every tab of the site and open it again, or tick *Update on reload* in the browser's developer tools (*Application* → *Service workers*).

## How the web app works

- `web/index.html` imports `grafia.js`, which wasm-bindgen generates and which loads `grafia_bg.wasm`. In parallel, the page fetches `grafia.dat.gz` and unpacks it with the browser's `DecompressionStream`, so the host needs no compression settings. `new Engine(bytes)` then checks the data file.
- `engine.convert(text, ee, ai)` and `engine.revert(text, ee)` return `{text, segments, unknown}` and `{text, segments}`. A segment is `{t}`, plus `alts` (all the spellings of a homograph, the default first) and `unknown` (a word missing from the data) when they apply. `engine.date` is the date of the dexonline data, which the footer shows.
- `web/sw.js` is the service worker. It caches every file of the site under a name that holds the build's version, `grafia-<version>`, and serves them from the cache first. A new version is cached beside the old one, with files fetched past the browser's HTTP cache so that none of the old ones slip in, and takes over once no page uses the old one. The old `grafia-` caches are then deleted, and only those: the other sites of `mihaicezar.github.io` share the same cache storage.
- `cargo xtask web` writes `web/dist/`:
  1. it checks that `web/grafia.dat` opens, so a data file in an old format stops the build;
  2. it builds `crates/wasm` with the `wasm` profile (optimized for size) and runs wasm-bindgen on it;
  3. it copies the files of `web/`, and gzips the data file;
  4. it writes `sw.js` with the list of files and the version, a hash of all of them.

For the 2026-10-04 dump, the site is 2.1 MB, of which `grafia_bg.wasm` 1.34 MB (0.48 MB gzipped), `grafia.dat.gz` 0.56 MB and the fonts 0.17 MB.

## Deploying

The web app is hosted on [GitHub Pages](https://docs.github.com/en/pages), which hosts static sites for free from a public repository.

1. Once, in the repository's settings: *Pages* → *Build and deployment* → *Source*: **GitHub Actions**.
2. On each push to `main`, the `ci` workflow runs the tests and `cargo xtask web`, then its `deploy` job publishes `web/dist/`. A failing test stops the deployment.
3. The site is at <https://mihaicezar.github.io/grafia-concisa/>, over HTTPS, which the service worker needs. Every URL in the site is relative, so it works under `/grafia-concisa/`. A custom domain can be set under *Pages*.
4. After a deployment, check the site in a browser: convert a text with a homograph (*Soare, ia!*) and switch it, copy, revert with the -ee switch (*Bordeẹ, aleẹ*), then go offline (developer tools, *Network* → *Offline*), reload, and convert again. The browser should offer to install the app.

CI can't build the data file, since it has neither MariaDB nor the dexonline dump: `web/grafia.dat` is built locally and committed (see *Updating the data*).

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
cargo run --release -p grafia-pipeline -- build --mysql --port 3307 --out web/grafia.dat
```

This writes the web app's data file (without `--out`, `data/grafia.dat`), in about 2 minutes, most of it in MariaDB. It prints the file's size, the date of the data and the build's stats. The date is the day of dexonline's latest change to a lexeme or a definition, so it always matches the data. For the 2026-10-04 dump (2026-10-10 rules): 816,260 bytes, data of 2026-10-03; 292,128 forms stored, of which 51 homographs and 2,749 with an -ee spelling, plus 2,613 reverse -ee spellings (`ee_revert`, 129 of them ties), 1,066 lemmas in -eu (`eu_lemmas`), 3,434 in -ea (`ea_lemmas`) and 65 in -iesc (`iesc_lemmas`). Lemma syllabifications came from dexonline (4,165 complete, 6,601 partial), Cezar's explicit list (18) and the rules alone (80,859).

What the build reads:
- the inflected forms, with their lemma and number (`InflectedForm`, `Inflection`);
- each lemma's dexonline syllabification (`Lexeme.hyphenations`; complete or partial, completed by the rules);
- the frequent common words, which the prefix rule checks against (`Lexeme.frequency`);
- which lexemes are DEX-MDA-DOOM, with a definition of their own (`Definition.lexicon`) in a source named MDA\*, DEX\*, NODEX\* or DOOM\* (`EntryLexeme`, `EntryDefinition`, `Definition`, `Source`), for reverse -ee.

How these are turned into spellings is in the spec. The records are read in a fixed order (by lexeme, then form id), since the first record wins a tie, so the same data always gives the same file, byte for byte.

`build --text FILE` reads the records from a text file instead, one per line (`build --help` gives the format), with `--known FILE` for the words the prefix rule checks against. Its data file has no date. `export --port 3307` writes the records and the known words that `build --mysql` reads to `data/records.tsv` and `data/known.txt` (`--out` for another directory). They are the input of `build --text` and of `crates/core/examples/snapshot.rs`, which writes every table and the conversions of every form, to compare two versions of the rules or of the data.

### Updating the data

`web/grafia.dat` holds the rules' output for every word, so it is rebuilt after a dexonline update and after every rule change:
1. Start the server.
2. For a dexonline update, run step 2 with `--download`.
3. Run step 3, which writes `web/grafia.dat`.
4. Run the tests and the WASM tests.
5. Compare the build's stats with the previous ones (forms stored, homographs, lemma sources). A large jump points to a change in dexonline's data or schema worth looking at before deploying.
6. Commit `web/grafia.dat`, with the rule change if there is one. Pushing to `main` deploys it.

Updates are picked up automatically wherever dexonline's syllabifications changed or new words appeared. Cezar's explicit syllabifications (`rules::SYLLABIFICATIONS`) still prevail.

### Pitfalls

- **Table names on Windows.** A Windows server stores table names in lowercase (`lexeme`). Queries are case-insensitive there, so nothing needs to change.
- **`WARNING: option --ssl-verify-server-cert is disabled`.** The `mariadb` client prints it for a local server with no password. It is harmless.

### This machine (as of 2026-10-10)

- Rust 1.99 (MSVC) through rustup, in `%USERPROFILE%\.cargo\bin`, with the `wasm32-unknown-unknown` target and `wasm-bindgen-cli`; Node 22.
- `data/word_db.sqlite` is the last database of the Python version (2026-10-04 dump), kept outside git for reference. Nothing uses it.
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

The code is licensed under the [GNU GPL v3 or later](LICENSE). The word data comes from [dexonline](https://dexonline.ro) under the GPL, and its attribution must stay in the UI. The Literata font is licensed under the [SIL Open Font License 1.1](web/fonts/OFL.txt).
