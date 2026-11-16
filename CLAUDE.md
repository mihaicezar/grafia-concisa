# grafia concisă

Romanian orthography that writes ie/ia/ea/oa diphthongs (no hiatus) as ẹ/ė/ē/ō. The name is always "grafia concisă" / `grafia-concisa`.

@DECISIONS.md
@concise-writing-spec.md

## Layout
- `reguli/`: the rule files, which a linguist edits: the hand lists of the rules, the steps of the fallback rules (`pasi.yaml`), their classes of letters (`clase.yaml`) and their exception tables, in YAML, with Romanian comments (the spec's *Rule files*)
- `crates/reguli/`: the package `grafia-reguli`: reads `reguli/` with the line and column of every value, runs the loading checks (messages in Romanian) and writes the Rust code of the lists, the exception tables, the classes and the steps; each list of words is declared once, in `LISTS` (`src/lib.rs`: its file, key, constant and the shape of its words), and each table in `TABLES` (its file, constant, pair of vowels and result), which a step of `pasi.yaml` names; `cargo run -p grafia-reguli` prints the messages, and in CI also as annotations on the lines of the pull request; tests of broken files (`tests/broken.rs`)
- `crates/core/`: the Rust core: rules (`rules/`, one file per concern: see its `mod.rs`), the DB builder (`build.rs`), the data file `grafia.dat` (`datafile.rs`), lookups and conversion (`lib.rs`), tests (`tests/tables.rs`; the rules checks of the examples of the steps and of the tables, `tests/reguli.rs`), and the build script `codegen.rs`, which compiles `reguli/` into `$OUT_DIR/reguli.rs` (`rules/lists.rs` includes it) and the lines of its rules and the names of its steps, for the trace of the syllabification, into `$OUT_DIR/reguli_meta.rs` (`rules/tables.rs` includes it; not in `RULES_HASH`). Put rule logic here only.
- `crates/pipeline/`: the binary `grafia-pipeline`: `load` (dexonline dump → local MariaDB), `build` (MariaDB or an export → `grafia.dat`), `export` (MariaDB → the export: text records, known words and the date, for `build --text` and `crates/core/examples/snapshot.rs`), `seed` (dev data file, no MariaDB)
- `crates/wasm/`: the core in WASM for the web app (`Engine`), tests under Node (`tests/node.rs`)
- `web/`: the web app (a PWA): the page (`index.html`), the service worker (`sw.js`), the manifest, icons and fonts, and the data file it ships, `grafia.dat` (not committed: `cargo xtask data` builds it, in CI too)
- `xtask/`: `cargo xtask web` (the web app → `web/dist/`), `data` (the pinned export → `web/grafia.dat`), `pin` (an export → its archive and `dexonline.lock`), `diff` (the words whose output changes, in Romanian), `reguli` (the checks of the rule files)
- `dexonline.lock`: the pinned export of dexonline: the URL of its archive, a release asset, and its SHA-256

## Commands
- `cargo test --workspace`: run the tests (must pass before any commit).
- `cargo run -p grafia-reguli`: the loading checks of the rule files of `reguli/` (the core's build runs them too, and stops on an error).
- `cargo xtask reguli`: the loading checks, then the rules checks: each example and counterexample of an exception table decided by the rules as its entry says (`cargo test` runs them too). CI runs it, and shows each message on its line of the pull request.
- `cargo xtask data`: fetch the export of dexonline that `dexonline.lock` pins into `data/`, and build `web/grafia.dat` from it (no MariaDB). The WASM tests and `cargo xtask web` read that file; rerun it after any change to the code of a file of `rules/` or of `build.rs`, or to a value of a file of `reguli/` (not only their comments, the examples of a step or a table, or the name of a step): the file records `RULES_HASH`, and `cargo xtask web` refuses it otherwise.
- `cargo test -p grafia-wasm --target wasm32-unknown-unknown`: the WASM tests, under Node, on `web/grafia.dat` (must pass before a commit that changes `crates/` or `dexonline.lock`). Needs `wasm-bindgen-test-runner`: see README.md.
- `cargo xtask diff BASE`: the words whose output changes against the commit BASE, on every form of the database, in Romanian, in `data/snapshot/diff.md` (the `snapshot-diff` skill). CI comments it on every pull request.
- `cargo xtask web`: build the web app into `web/dist/`; `npx serve web/dist` serves it on <http://localhost:3000>.
- `cargo run --release -p grafia-pipeline -- seed`: build a small dev data file, `data/grafia.dat`.
- A dexonline update (README.md, *Updating dexonline*) is the only step that needs a MariaDB server, on port 3307 (on Windows a portable one, see README.md; on the Mac, "This machine" there):
  - `cargo run --release -p grafia-pipeline -- load --download --client <mariadb> --port 3307`: load the latest dump;
  - `cargo run --release -p grafia-pipeline -- export --port 3307 --out data/export`, then `cargo xtask pin data/export`: the export, its archive `data/dexonline-<date>.tar.gz` and a new `dexonline.lock`;
  - Cezar publishes the archive as the release `dexonline-<date>`, then `dexonline.lock` goes in a pull request of its own, whose CI comment lists what the update changes.

## Conventions
- Simple over clever, with best practices, and no speculative features.
- Rules go in `reguli/`: a word of a hand list, an explicit syllabification, an obsolete lemma, a step of the fallback rules or an entry of an exception table is edited in its rule file, never in Rust. `crates/core` keeps the mechanisms, and the lists that have not moved yet (`rule-files-plan.md`, *Inventory*).
- Tests are table-driven: a new case is one line in `WORDS`, `DERIVED`, `SYLLABIFY`, `EE_WORDS`, `TEXTS` or `REVERTS` (in `crates/core/tests/tables.rs`), or, for a loading check of the rule files, in `BROKEN` (in `crates/reguli/tests/broken.rs`), or, for the rules checks of the steps and of the exception tables, in `CHECKS`, `TRACES` or `MATCHES` (in `crates/core/tests/reguli.rs`). An example of a step or of an entry of an exception table is a word of its `exemple` or `contraexemple`, not a line of `SYLLABIFY`.
- Docs have separate roles:
  - `DECISIONS.md` logs each settled decision: date, principle, why, an example and the spec section. It holds no mechanics, and it is the authority on intent.
  - `concise-writing-spec.md` holds the complete current rules, with no history. A rule change updates it in the same commit; a disagreement with `DECISIONS.md` is fixed there.
  - `README.ro.md` is the Romanian rendering of the spec, updated with it. It never contains anything that is not in the spec.
  - `rust-port-plan.md` holds the steps of the Rust port, and is deleted when the port ends. `DECISIONS.md` prevails over it.
  - `rule-files-plan.md` holds the stages that move the rules to YAML files a linguist edits (`reguli/`); its *Status* says which stage is next. It is deleted when its last stage is done. `DECISIONS.md` prevails over it.
- Linguistic edge cases: ask Cezar and don't guess. They are discussed in Romanian.
- Keep the dexonline attribution (GPL) in the UI.
- Commit messages: a one-line subject `type(domain) : summary`, with type one of `feat`, `fix`, `doc`, `chore`, `refactor`, `ci`; then a blank line and a `- ` list of the changes. For a change to the rules, the conversion or the spec, add a blank line, `Rezumat:` and a short `- ` list in Romanian. No line wrapping.

## Open tasks
1. Refine the fallback syllabifier (the steps of `reguli/pasi.yaml`). dexonline prevails wherever it has data. The exception tables (`reguli/diftong-ia.yaml`, `hiat-ea.yaml`, `hiat-ie.yaml`, `hiat-oa.yaml`) were reviewed by Cezar on 2026-10-09 against dexonline's own syllabifications of similar words. Extend them as new cases appear.
2. Port to Rust (2026-10-09, see *Platforms and stack* in `DECISIONS.md`) following `rust-port-plan.md`, in this order:
   - the core (all of `rules.py`, and the builder in `db.py`) with the test tables, then check that it matches Python on every form in the database; the Python rules are then frozen (done 2026-10-10);
   - the pipeline, and the data file format: it must work in WASM and be memory-mapped on iOS, where keyboards are killed above a few tens of MB (done 2026-10-10: `grafia.dat`, and the Python pipeline is removed);
   - the web as a WASM PWA on GitHub Pages (done 2026-10-10: `crates/wasm`, `web/`, `cargo xtask web`, and the rest of Python is removed);
   - iOS: one keyboard with two modes and a mode key, with the data file inside the keyboard extension (it can't read the App Group container without Full Access), and predictions from `UITextChecker`. iOS builds need macOS (a cloud CI build is fine); during development they are sideloaded with a free Apple ID;
   - Android: an `ACTION_PROCESS_TEXT` activity, with bindings generated by UniFFI.
3. Move the rules to files a linguist edits, following `rule-files-plan.md` (2026-11-12, see *Platforms and stack* in `DECISIONS.md`).
