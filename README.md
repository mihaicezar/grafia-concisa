# grafia concisă

A Romanian orthography that writes the diphthongs ie, ia, ea and oa as ẹ, ė, ē and ō when both vowels are in one syllable: *soare* → *sōre*, *iarbă* → *ėrbă*, but *real* and *vie* (a hiatus) stay as they are.

Rules: [`concise-writing-spec.md`](concise-writing-spec.md) (in Romanian: [`README.ro.md`](README.ro.md)); settled decisions and their reasons: [`DECISIONS.md`](DECISIONS.md).

## The web app

<https://mihaicezar.github.io/grafia-concisa/> (once GitHub Pages is enabled: see *Deploying*) converts a text in either direction:
- **Standard → concis**, with two switches: **î** writes â as î (on by default; *român* and its family keep â), and **-eẹ/aẹ/oẹ** applies the optional rule of that name (*idee* → *ideẹ*, *Nae* → *Naẹ*).
- **Concis → standard**: the reverse. With **-eẹ/aẹ/oẹ**, the text is read as written with that rule.
- A word with two readings (*ia*: *ė* or *ia*) shows both on a switch; a click picks one. **Copiază** copies the text with the picked spellings.
- A word missing from the word data is converted by the rules alone and underlined with dots.

Everything runs in the browser: the rules are the Rust core compiled to WebAssembly, and the word data is a file the page downloads once. After the first visit the site works offline, and it can be installed as an app (a PWA).

## Stack

The project is one Rust core that every platform calls. It was first written in Python, then ported; the steps are in [`rust-port-plan.md`](rust-port-plan.md).

| Part | Where | What | Status |
|---|---|---|---|
| Core | `crates/core` (crate `grafia-concisa`) | the rules, conversion and reverse conversion, the database builder, the data file | done |
| Rule files | `reguli/`, `crates/reguli` (crate `grafia-reguli`) | the hand lists of the rules, in YAML files a linguist edits, which the core's build compiles in ([`rule-files-plan.md`](rule-files-plan.md)) | stage 2 |
| Data file | `web/grafia.dat` (built, not committed) | the word database: [`fst`](https://crates.io/crates/fst) maps read in place, 0.8 MB (0.56 MB gzipped) | done |
| Pipeline | `crates/pipeline` (binary `grafia-pipeline`) | loads dexonline into MariaDB, exports its records, and builds `grafia.dat` | done |
| Web app | `crates/wasm`, `web/`, `xtask/` | the core in WASM, in a static page that works offline, on GitHub Pages | done |
| iOS | `crates/ffi`, `ios/` | a keyboard: converts as you type, or types ẹ ė ē ō | phase 4 |
| Android | `crates/ffi`, `android/` | *grafia concisă* in the text selection menu | phase 5 |

- **The core** uses `fancy-regex` (the rules' patterns need lookbehind), `unicode-normalization` (NFC), `indexmap` (ordered maps, for the order of the alternatives), `fst` and `crc32fast` (the data file's checksum), and `serde` (the answers' JSON). It has no I/O: the platforms pass it the data file's bytes.
- **The rule files** are read by `grafia-reguli`, with `marked-yaml` (YAML 1.2 with the line and column of every value) and `unicode-normalization`. It is only a build dependency of the core: its build script (`crates/core/codegen.rs`) turns the files into Rust constants, so no YAML parser ships. Their format and checks are in the spec's *Rule files*.
- **The data file** is built by the pipeline from an export of [dexonline](https://dexonline.ro)'s records, pinned in [`dexonline.lock`](dexonline.lock) (see *The data file*). The web page loads it into memory; the mobile apps will memory-map it, so it costs the iOS keyboard little memory. Its layout is in [`crates/core/src/datafile.rs`](crates/core/src/datafile.rs).
- **The pipeline** uses `mysql` (a pure Rust client), `flate2` (to filter the gzipped dump), `ureq` with the system's TLS (to download it) and `clap`.
- **`cargo xtask data`** downloads the pinned export with `ureq` and checks it with `sha2`; `tar` and `flate2` pack and unpack it.
- **The web app** is one page with plain JavaScript, no framework and no bundler. `crates/wasm` wraps the core with `wasm-bindgen` and `serde-wasm-bindgen`. `cargo xtask web` builds the site with `wasm-bindgen-cli-support`, which must be the same version as `wasm-bindgen`: both are pinned side by side in the workspace's [`Cargo.toml`](Cargo.toml), the one place for that version. The page's font, [Literata](https://github.com/googlefonts/literata), is served from the site itself.
- **CI** (GitHub Actions, [`.github/workflows/ci.yml`](.github/workflows/ci.yml)) checks the rule files, showing each message on its line of the pull request, builds the data file from the pinned export, runs `cargo fmt --check`, `cargo clippy -D warnings`, the tests and the WASM tests, and builds the web app. On a pull request, it also lists the words whose spelling the pull request changes, in Romanian, in a comment. On `main`, it then publishes the web app on GitHub Pages.

## Development

Tools:
- Rust stable, through [rustup](https://rustup.rs), with the WASM target: `rustup target add wasm32-unknown-unknown`. On Windows, Rust needs the Visual Studio Build Tools with the C++ workload (for the MSVC linker).
- [Node](https://nodejs.org) (a current LTS), which runs the WASM tests and `npx serve`.
- `wasm-bindgen-test-runner`, for the WASM tests. Its version must be `wasm-bindgen`'s, which this command reads from `Cargo.lock`; run it again after a change of that version:
  `cargo install wasm-bindgen-cli --locked --version "$(cargo pkgid wasm-bindgen | sed 's/.*@//')"`
- A MariaDB or MySQL server, only to bring in a dexonline update (see *Updating dexonline*).

```
cargo run -p grafia-reguli                                  # check the rule files of reguli/
cargo xtask data                                            # build web/grafia.dat from the pinned export of dexonline
cargo test --workspace                                      # tests
cargo test -p grafia-wasm --target wasm32-unknown-unknown   # WASM tests, under Node, on web/grafia.dat
cargo xtask web                                             # build the web app into web/dist/
npx serve web/dist                                          # serve it on http://localhost:3000
cargo xtask diff main                                       # the words whose output changes against main
cargo run --release -p grafia-pipeline -- seed              # small dev data file, data/grafia.dat (no MariaDB)
```

`cargo xtask` runs from anywhere in the repository; the pipeline runs from its root, since its default paths are under `data/`, which git ignores. `cargo run --release -p grafia-pipeline -- help` lists the pipeline's commands and options.

### Tests

The tests must pass before every commit, and the WASM tests too when a commit changes `crates/` or `dexonline.lock`. CI runs both on every push.
- **The rules** are tested by tables in [`crates/core/tests/tables.rs`](crates/core/tests/tables.rs): a new case is one line. The tables that need a database run twice, on the built tables in memory and through the data file.
- **The loading checks of the rule files** are tested by a table of broken files, each with the messages it gives, in [`crates/reguli/tests/broken.rs`](crates/reguli/tests/broken.rs).
- **The WASM tests** ([`crates/wasm/tests/node.rs`](crates/wasm/tests/node.rs)) run the engine under Node on `web/grafia.dat`, the data file the site ships, which `cargo xtask data` builds: the answers as the page receives them, the switches, the date of the data, and a bad file. `cargo test --target wasm32-unknown-unknown` runs them with `wasm-bindgen-test-runner` (set in [`.cargo/config.toml`](.cargo/config.toml)); `cargo test --workspace` skips them.
- **The page** has no automated test. After a change to `web/`, check it in a browser, as in step 4 of *Deploying*.

### The web app locally

`cargo xtask web`, then `npx serve web/dist`, and open <http://localhost:3000>. The site must be served over HTTP: browsers don't run its module script or its service worker from a `file://` page. Service workers also need HTTPS, except on `localhost`.

To access the app from other devices on your local WiFi:

```bash
npx serve web/dist -p 5000
```

Then find your computer's local IP address with `ipconfig` (look for "IPv4 Address") and visit `http://<your-ip>:5000` from another device.

The service worker keeps serving the version it cached. After a rebuild, close every tab of the site and open it again, or tick *Update on reload* in the browser's developer tools (*Application* → *Service workers*).

## How the web app works

- `web/index.html` imports `grafia.js`, which wasm-bindgen generates and which loads `grafia_bg.wasm`. In parallel, the page fetches `grafia.dat.gz` and unpacks it with the browser's `DecompressionStream`, so the host needs no compression settings. `new Engine(bytes)` then checks the data file.
- `engine.convert(text, ee, ai)` and `engine.revert(text, ee)` return `{text, segments, unknown}` and `{text, segments}`. A segment is `{t}`, plus `alts` (all the spellings of a homograph, the default first) and `unknown` (a word missing from the data) when they apply. `engine.date` is the date of the dexonline data, which the footer shows.
- `web/sw.js` is the service worker. It caches every file of the site under a name that holds the build's version, `grafia-<version>`, and serves them from the cache first. A new version is cached beside the old one, with files fetched past the browser's HTTP cache so that none of the old ones slip in, and takes over once no page uses the old one. The old `grafia-` caches are then deleted, and only those: the other sites of `mihaicezar.github.io` share the same cache storage.
- `cargo xtask web` writes `web/dist/`:
  1. it checks that `web/grafia.dat` opens and was built with the current rules, so a data file in an old format, or not rebuilt after a rule change, stops the build (`cargo xtask data` rebuilds it);
  2. it builds `crates/wasm` with the `wasm` profile (optimized for size) and runs wasm-bindgen on it;
  3. it copies the files of `web/`, and gzips the data file;
  4. it writes `sw.js` with the list of files and the version, a hash of all of them.

For the 2026-10-04 dump, the site is 2.1 MB, of which `grafia_bg.wasm` 1.34 MB (0.48 MB gzipped), `grafia.dat.gz` 0.56 MB and the fonts 0.17 MB.

## Deploying

The web app is hosted on [GitHub Pages](https://docs.github.com/en/pages), which hosts static sites for free from a public repository.

1. Once, in the repository's settings: *Pages* → *Build and deployment* → *Source*: **GitHub Actions**.
2. On each push to `main`, the `ci` workflow runs `cargo xtask data`, the tests and `cargo xtask web`, then its `deploy` job publishes `web/dist/`. A failing test stops the deployment.
3. The site is at <https://mihaicezar.github.io/grafia-concisa/>, over HTTPS, which the service worker needs. Every URL in the site is relative, so it works under `/grafia-concisa/`. A custom domain can be set under *Pages*.
4. After a deployment, check the site in a browser: convert a text with a homograph (*Soare, ia!*) and switch it, copy, revert with the -eẹ/aẹ/oẹ switch (*Bordeẹ, aleẹ*), then go offline (developer tools, *Network* → *Offline*), reload, and convert again. The browser should offer to install the app.

CI builds the data file from the pinned export of dexonline (see *The data file*), so the deployed file is always built with the rules it ships with.

## The data file

`grafia.dat` is built from [dexonline](https://dexonline.ro)'s public database dump. The data is GPL, and the attribution must stay in the UI.

It comes in two steps:
1. **An export** of dexonline's records, made from the dump in a local MariaDB (or MySQL) server and published as an archive on a release of this repository. [`dexonline.lock`](dexonline.lock) pins it. Only a dexonline update makes a new one (see *Updating dexonline*).
2. **The build** of `grafia.dat` from the pinned export: `cargo xtask data`, by anyone and with no MariaDB, and by CI on every push.

### Building `grafia.dat`

```bash
cargo xtask data
```

- `dexonline.lock` gives the URL of the archive, `dexonline-<date>.tar.gz`, an asset of the release `dexonline-<date>` named after the date of the data, and its SHA-256.
- The archive is downloaded into `data/`, unless it is there already with that checksum. A wrong checksum stops the build. Its files, `records.tsv`, `known.txt` and `date.txt`, are unpacked into `data/dexonline-<date>/`.
- `grafia-pipeline build --text` builds `web/grafia.dat` from them, in about 15 seconds once the pipeline is compiled. It prints the file's size, the date of the data, the rules (`RULES_HASH`, see *Rule changes*), the build's stats and the file's SHA-256. The same export and rules give the same file, byte for byte, on any machine, so a local build can be compared with CI's log.

For the 2026-11-11 data (rules `24ffedf6588957a6`): 819,889 bytes; 292,963 forms stored, of which 50 homographs and 3,007 with an -ee spelling, plus 2,845 reverse -ee spellings (`ee_revert`, 61 of them ties), 1,066 lemmas in -eu (`eu_lemmas`), 3,436 in -ea (`ea_lemmas`) and 65 in -iesc (`iesc_lemmas`). Lemma syllabifications came from dexonline (4,165 complete, 6,601 partial), Cezar's explicit list (23) and the rules alone (81,112).

What the export holds, from dexonline's tables:
- the inflected forms, with their lemma and number (`InflectedForm`, `Inflection`);
- each lemma's dexonline syllabification (`Lexeme.hyphenations`; complete or partial, completed by the rules);
- the frequent common words, which the prefix rule checks against (`Lexeme.frequency`);
- which lexemes are DEX-MDA-DOOM, with a definition of their own (`Definition.lexicon`) in a source named MDA\*, DEX\*, NODEX\* or DOOM\* (`EntryLexeme`, `EntryDefinition`, `Definition`, `Source`), for reverse -ee;
- the date of the data: the day of dexonline's latest change to a lexeme or a definition, so it always matches the data.

How these are turned into spellings is in the spec. The records are read in a fixed order (by lexeme, then form id), since the first record wins a tie, so the same data always gives the same file, byte for byte.

`records.tsv` holds one record per line (`grafia-pipeline build --help` gives the format), `known.txt` the words the prefix rule checks against, and `date.txt` the date. They are also the input of `crates/core/examples/snapshot.rs`, which writes every table and the conversions of every form, to compare two versions of the rules or of the data (see `cargo xtask diff`, below).

### Rule changes

`web/grafia.dat` holds the rules' output for every word. It records the rules it was built with, `RULES_HASH`: a hash of the code of the files of `crates/core/src/rules/` and of `build.rs`, and of the values of the rule files of `reguli/`, without comments or whitespace. `cargo xtask web` refuses a data file built with other rules, so after any change to the code of these files or to a value of a rule file, even one that changes no spelling, run `cargo xtask data` again; a change to their comments alone doesn't need it. No data file is committed: CI builds it on every push, from the pinned export and with the rules of the commit.

`cargo xtask diff BASE` lists, in Romanian, the words whose output the working tree changes against the commit BASE (`main`, for a branch):
- it writes the snapshots of BASE, in a worktree (`target/diff-base`), and of the working tree into `data/snapshot/before` and `data/snapshot/after`, on the pinned export, or each on its own export when `dexonline.lock` changed;
- it writes `data/snapshot/diff.md`: the forms whose default spelling, alternatives, -eẹ/aẹ/oẹ spelling or current form changed, the forms added or removed, then the rows of the snapshot's other files that changed. `--max N` keeps at most N rows per section.

On a pull request, CI runs it against the base branch with `--max 50`, and posts `diff.md` in the job's summary and in one comment on the pull request, replaced on each push. A pull request from a fork gets the summary only, since its token can't write.

## Updating dexonline

A dexonline update is the only step that needs dexonline's database: someone with a MariaDB server loads the latest dump, exports its records and publishes the export, then a pull request pins it.

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
- A load takes about 3 minutes and ends by printing the row counts. For the dump of 2026-11-12: `Lexeme 321278`, `InflectedForm 2279620`.

### 3. Export and pin

```bash
cargo run --release -p grafia-pipeline -- export --port 3307 --out data/export
cargo xtask pin data/export
```

- `export` writes `records.tsv`, `known.txt` and `date.txt` into `data/export`, in about 3 minutes, most of it in MariaDB.
- `pin` packs them into `data/dexonline-<date>.tar.gz`, named after the date of the data, and writes `dexonline.lock` with the archive's URL and SHA-256. The archive holds only the files' bytes (no times, no owners), so the same export always gives the same archive: 12.7 MB for the 2026-11-11 data. Since the archive is then in `data/` with its checksum, `cargo xtask data` builds from it without downloading.
- `build --mysql --port 3307 --out FILE` still builds the data file straight from the server, in about 3 minutes. On the same dump, it gives the same file as `cargo xtask data` on its export.

### 4. Publish the export

The archive is published as the release `dexonline-<date>` of this repository, with dexonline's attribution in its notes. `pin` writes the notes into `data/dexonline-<date>.md` and prints the command for the [GitHub CLI](https://cli.github.com), to run from the repository's root:

```bash
gh release create dexonline-<date> data/dexonline-<date>.tar.gz --title "dexonline <date>" --notes-file data/dexonline-<date>.md
```

The asset is the archive, `data/dexonline-<date>.tar.gz`, not the folder `data/dexonline-<date>/` beside it, which `cargo xtask data` unpacks it into. Or on GitHub: *Releases* → *Draft a new release*, with a new tag `dexonline-<date>`, the archive as its asset, and the notes of `data/dexonline-<date>.md`:

> The records of [dexonline](https://dexonline.ro)'s database, data of <date>, as `grafia-pipeline export` writes them: `records.tsv` (the inflected forms), `known.txt` (the words the prefix rule checks against) and `date.txt` (the date of the data). `cargo xtask data` builds `grafia.dat` from them.
>
> The data is dexonline's (© 2004-<year> dexonline, https://dexonline.ro), under the GNU General Public License, version 2 or later.

### 5. Pin it in a pull request

Commit `dexonline.lock` alone, in a pull request of its own. CI builds the data file from the new export, and its comment lists the words the update changes, against the previous export. Compare the build's stats in CI's log with the previous ones (forms stored, homographs, lemma sources): a large jump points to a change in dexonline's data or schema worth looking at before merging. Merging deploys it.

Updates are picked up automatically wherever dexonline's syllabifications changed or new words appeared. Cezar's explicit syllabifications (`reguli/silabisiri.yaml`) still prevail.

### Pitfalls

- **A corrupt download.** `load` checks the dump only as it reads it. A download once had the right size but was corrupt, and the load stopped in the middle of the SQL (`ERROR at line 301: Unknown command '\''`); `gzip -t data/dex-database.sql.gz` showed it. Download it again.
- **`read data\dexonline-<date>\: Incorrect function`.** `gh release create` was given the folder, not the archive. When an upload fails, `gh` deletes the draft release it created, so nothing is published: run the command again with `data/dexonline-<date>.tar.gz`.
- **Table names on Windows.** A Windows server stores table names in lowercase (`lexeme`). Queries are case-insensitive there, so nothing needs to change.
- **`WARNING: option --ssl-verify-server-cert is disabled`.** The `mariadb` client prints it for a local server with no password. It is harmless.

### This machine (as of 2026-11-11)

- macOS (Apple Silicon): Rust 1.99.1 through rustup in `~/.cargo/bin`, with rustfmt, Clippy and the `wasm32-unknown-unknown` target. Run `source ~/.cargo/env` if Cargo is not on the shell's PATH.
- `wasm-bindgen-cli` 0.2.129 includes `wasm-bindgen-test-runner`; Node is installed through Homebrew.
- MariaDB 12.2.2 is installed through Homebrew. Its persistent data directory is `/opt/homebrew/var/mysql`; the `dexonline` database holds the dump downloaded on 2026-11-11. The local `grafia` account has access only to this database, without a password.
- Start the server for a dexonline update (it is not configured to start at login):

```bash
/opt/homebrew/opt/mariadb/bin/mariadbd --no-defaults --datadir=/opt/homebrew/var/mysql --bind-address=127.0.0.1 --port=3307 --socket=/tmp/grafia-mariadb.sock --pid-file=/tmp/grafia-mariadb.pid --max-allowed-packet=1G
```

In another terminal, from the repository root (see *Updating dexonline*):

```bash
source ~/.cargo/env
cargo run --release -p grafia-pipeline -- load --download --port 3307 --user grafia
cargo run --release -p grafia-pipeline -- export --port 3307 --user grafia --out data/export
cargo xtask pin data/export
```

Stop the server with `mariadb-admin --no-defaults --socket=/tmp/grafia-mariadb.sock --user=cezar shutdown`.

## License

The code is licensed under the [GNU GPL v3 or later](LICENSE). The word data comes from [dexonline](https://dexonline.ro) under the GPL, and its attribution must stay in the UI. The Literata font is licensed under the [SIL Open Font License 1.1](web/fonts/OFL.txt).
