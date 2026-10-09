# grafia concisă — Rust port plan

**Status:** step 0 and phase 1 done on 2026-10-10: Rust matches Python on the 2026-10-04 dump, and the Python rules are frozen.

## Context

DECISIONS.md (*Platforms and stack*, 2026-10-09) settles the following:
- one Rust core for the rules, conversion, reverse conversion and pipeline;
- a serverless WASM PWA;
- an iOS keyboard;
- an Android text-processing action.

CLAUDE.md task 2 gives the order, with iOS before Android. This plan details each phase from the code as it stands:
- `grafia_concisa/rules.py`, 649 lines;
- `grafia_concisa/db.py`, 242 lines;
- `server/server.py`, 170 lines;
- three pipeline scripts;
- `server/test_server.py`, with 15 test tables.

As of 2026-10-09, none of the build tools is installed yet: no Rust, no MSVC Build Tools, no Android SDK or JDK. Node is installed. The portable MariaDB is still there (`…\9fe3f075-…\scratchpad\mariadb-11.4.13-winx64`, `mdata`), and `data/dex-database.sql.gz` dates from 2026-10-04.

**Outcome:** one Rust crate implements the spec, and the web, iOS and Android all call it. Python is gone once the PWA replaces the Flask site.

## Decisions for the port (2026-10-09)

Logged in `DECISIONS.md` (*Platforms and stack*), which prevails over this list.

1. **Freeze of the Python rules.** Once Rust matches Python on the full data (the phase 1 gate), `rules.py`, `db.py` and `server/` are frozen.
   - After that, rule changes go into Rust only. The Flask app is not deployed, so nothing depends on the Python version; it keeps the last Python rules until the PWA replaces it.
   - These files are deleted when the PWA replaces the Flask app.
   - A rule change made *before* the gate goes into Python first, then Rust, and the gate is rerun.
2. **Python pipeline.** `load_dexonline.py` and `build_db.py`'s queries stay editable until the Rust pipeline replaces them. They are deleted at the end of phase 2, and the frozen Flask app keeps its last `word_db.sqlite`.
3. **Phase 1 also ports `db.py`'s build logic**, not only `rules.py`. CLAUDE.md first listed only `rules.py`, but `db.py` holds spec rules: the default and alternatives priority and the reverse -ee ranking. The test tables that go through the database (`TEXTS`, `HOMOGRAPHS`, `UNKNOWN_EE`, `REVERTS`, …) also need a built test database.
4. **The data file format is chosen in phase 2, not phase 3.** The pipeline writes the file, and its constraints (WASM, memory-mapped on iOS) are already known.
5. **iOS comes before Android**, because Cezar's own phone is an iPhone, so the iOS app can be tested in daily use first.
6. **iOS:**
   - One keyboard with a mode key that remembers its mode.
   - Predictions come from `UITextChecker`. The first step of phase 4 is a check on a device that it has Romanian. This closed the open question of where the predictions come from.
   - During development the app is installed on Cezar's own iPhone with a free Apple ID. Apple Developer Program membership is needed only for the first TestFlight or App Store release.
7. **Android:** the selection is replaced at once when every word has one known spelling. With a homograph or an unknown word, a dialog first shows the same switches as the web page.
8. **Hosting (2026-10-10):** the PWA goes on GitHub Pages. No site was deployed before it.

## Target layout

```
Cargo.toml              workspace (edition 2024)
crates/core/            package grafia-concisa: rules.rs (= rules.py), build.rs (= db.py),
                        datafile.rs (phase 2), lib.rs (Dictionary trait, public API)
crates/core/tests/tables.rs   the test tables (= server/test_server.py)
crates/pipeline/        binary grafia-pipeline: load | build | seed          (phase 2)
crates/wasm/            wasm-bindgen wrapper                                 (phase 3)
crates/ffi/             UniFFI wrapper: Swift (phase 4), Kotlin (phase 5)
xtask/                  cargo xtask web | ios | android
web/                    PWA (index.html from server/static/)                 (phase 3)
ios/                    XcodeGen project.yml: app + keyboard extension       (phase 4)
android/                Gradle project                                       (phase 5)
```

## Step 0: docs and toolchain

- A doc commit (`doc(project) : …`):
  - DECISIONS.md entries for the seven decisions above;
  - this plan committed as `rust-port-plan.md`, so later sessions have it, and deleted when the port ends;
  - in CLAUDE.md task 2, iOS moved before Android and a pointer to that file.
- Install:
  - Visual Studio Build Tools with the C++ workload (rustc's MSVC linker);
  - rustup (stable, MSVC);
  - `rustup target add wasm32-unknown-unknown`.
- Set up the Cargo workspace and add `target/` to `.gitignore`.
- Add a CI job with `cargo fmt --check`, `cargo clippy --workspace -- -D warnings` and `cargo test --workspace`, next to the Python job.
- In CLAUDE.md: `cargo test --workspace` must also pass before every commit.

## Phase 1: core (`rules.py` + `db.py` → `crates/core`)

**Port it file for file and name for name**, so that a Python diff can be read against the Rust code:
- `rules.rs` mirrors `rules.py`: `syllabify`, `syllabify_all`, `rule_cut`, `dexonline_cuts`, `concise_from_reference`, `concise_ee`, `concise_by_rules`, `lemma_candidates`, `resolve`, `convert_text`, `revert_segments`, `a_to_i`, `apply_case`, `normalize`, `has_candidate`, `long_infinitive`, `iere_spelling`.
- The constants keep their names: `SYLLABIFICATIONS`, `IA_DIPHTHONG`, `EA_HIATUS`, `IE_HIATUS`, `OA_HIATUS`, `PREFIXES`, `IERE_DIPHTHONG`, `IARE_KEPT`, `EEA_UNCHANGED`.
- `build.rs` mirrors `db.build`: `build(records, known) -> (Tables, Stats)`, where `Tables` holds the five tables in memory. It writes no file; the data file comes in phase 2.
- **The Python glue in `server.py` also moves into the core:**
  - the routing of `lookup_known`: -eu → `eu_lemmas`, -ea → `ea_lemmas`, -iesc → `iesc_lemmas`, anything else → `forms`;
  - the `/api/lookup` logic;
  - the response shapes: `convert(text, &dict, Options{ee, ai}) -> {text, segments, unknown}`, `revert(text, Option<&dict>) -> {text, segments}` (the dictionary only with the -ee switch) and `lookup(words, &dict, opts)`.
  - `Segment` is serialized with serde into the same JSON as `/api/convert` today (`t`, `alts`, `unknown`), so the web UI's rendering code stays as it is.
- A `Dictionary` trait has three methods: `form(key)`, `known(word)` and `ee_revert(key)`.
  - Lookups are per key; `server.py` batches them only because of SQL.
  - `Tables` implements the trait in phase 1, and `DataFile` in phase 2.

**Crates:**
- `fancy-regex`, because the patterns use lookbehind (`(?<!s)chiag`, `(?<=[^zs])bier`, `HIATUS_SUFFIX`). They are kept verbatim, so they stay comparable with Python and the spec.
- `unicode-normalization` for NFC.
- `indexmap` for Python's dict insertion order: it fixes the order of the alternatives.
- `serde`.
- `std::sync::LazyLock` for the compiled patterns.

**Porting pitfalls,** each covered by the tests or the gate:
- Python indexes strings by code point, Rust by byte. The word functions work on a `Vec<char>`, so they mirror Python line for line; byte offsets appear only around regex calls (`_inside`, and the position-anchored `IENT.match`).
- `WORD = [^\W\d_]+` becomes `[\p{L}\p{Nl}\p{No}]+`, not Rust's `\w`, which also matches combining marks.
- `str.isupper()` needs a small helper: at least one cased character and no lowercase ones.
- `sorted()` is stable, like `sort_by`. `_RO_ORDER` becomes the same sort key.
- The build stats (`Counter`) must match Python's, key for key.

**Tests:** `crates/core/tests/tables.rs` ports every table verbatim: `WORDS`, `DERIVED`, `EE_WORDS`, `SYLLABIFY`, `HOMOGRAPH_RECORDS`/`HOMOGRAPHS`, `LONG_INFINITIVE_RECORDS`, `UNKNOWN_EE`, `EE_REVERT_RECORDS`/`REVERT_HOMOGRAPHS`, `LEMMAS`, `KNOWN`, `EE_RULE`, `TEXTS` and `REVERTS`.
- The test database is built with `build()` from the same records, as `setUpClass` does.
- `test_round_trip_whole_db` is ported, and `test_lookup` calls `lookup()`.
- The HTTP-only tests (`test_bad_requests`, `test_health_and_index`) are not ported.
- A new case is still one line in a table.

**Pipeline change in Python** (decision 2): `build_db.py --export DIR` writes `records.tsv` (the rows of `SOURCE_QUERY`, in the `--text` format) and `known.txt` (`KNOWN_QUERY`) into `data/`.

**Parity gate.** The temporary tools `pipeline/parity.py` and `crates/core/examples/parity.rs` both read `data/records.tsv` and `known.txt`, and each writes the same files to `data/parity/{python,rust}/`:
1. `references.tsv`: `syllabify_all` of every distinct (lemma, hyphenation, verb). This narrows a difference down to the syllabifier.
2. The five tables, sorted, plus the build stats.
3. `convert.tsv`: every stored form, as the text "form Form FORM", with all four ee/ai switch combinations. Segments are written in one canonical line format that both sides produce.
4. `unknown.tsv`: `concise_by_rules(form)` of every form, with ee off and on. This runs the fallback rules alone on about 292,000 real words.
5. `revert.tsv`: every `concise_form`, `concise_ee` and alternative, with and without the -ee lookup.

The gate passes when:
- `git diff --no-index --stat data/parity/python data/parity/rust` shows nothing;
- both test suites pass.

**The gate commit starts the freeze (decision 1).** From then on, the rules live in Rust, so in the same commit:
- the spec's and `README.ro.md`'s `rules.X` names become `rules::X`;
- CLAUDE.md's *Layout* puts the rule logic in `crates/core`, and task 1 refers to `rules::rule_cut`.

## Phase 2: pipeline and data file

**`crates/pipeline`**, the binary `grafia-pipeline`, with clap subcommands:
- `load`: ports `load_dexonline.py`.
  - The download prints the dump's Last-Modified date and saves it as `data/dex-database.date`.
  - flate2 filters the gzip down to the tables in `TABLES`, and the result is piped into the mariadb client.
- `build --mysql [conn] | --text FILE [--known FILE] --out PATH`: runs `SOURCE_QUERY` and `KNOWN_QUERY` verbatim through the `mysql` crate (no TLS: the server is local), then `core::build`, then `datafile::write`. The dump date in the header is read from `data/dex-database.date` when that file exists.
- `seed`: `seed_db.py`'s `SEED` list → a development data file.
- For HTTPS, prefer native-tls (Schannel) over rustls with aws-lc, which needs cmake and NASM on Windows.

**Data file `grafia.dat` (`crates/core/src/datafile.rs`):**
- A header: magic, format version, and the dexonline dump date (shown in the apps' attribution).
- Sections:
  - `forms`: an `fst::Map` from the normalized form to an entry in a string table. The entry holds the concise spelling (empty when it equals the form, as in about 40% of forms), the -ee spelling and the alternatives.
  - `eu_lemmas` and `ea_lemmas`: `fst::Set`.
  - `iesc_lemmas` and `ee_revert`: `fst::Map` into the string table.
- The writer sorts everything, so the same input always gives the same bytes.
- `DataFile<B: AsRef<[u8]>>` implements `Dictionary`:
  - over a `Vec<u8>` in WASM;
  - over a `memmap2::Mmap` (feature `mmap`) on iOS and Android. Clean, file-backed pages don't count against the keyboard's memory limit the way heap memory does.
- **Why not SQLite:** in WASM it needs a C build and a VFS, and only exact-key lookups are needed. The `fst` crate is pure Rust and made for this.
- **Size target,** measured today:
  - the forms as sorted text are 1.59 MB gzipped (1.34 MB with the unchanged spellings left out);
  - the SQLite file is 2.54 MB gzipped (9.1 MB raw);
  - target: `grafia.dat` at most 1.5 MB gzipped and 6 MB raw.

**Tests:** the database-backed tables run against both `Tables` and the round trip `Tables` → bytes → `DataFile`.

**Gate:**
- the build from MySQL is byte-identical to the build from the Python `--export`;
- the stats are equal.

**End of phase 2:** delete `pipeline/*.py` (including `parity.py`), `examples/parity.rs` and the `pipeline` extra in `pyproject.toml`.

**Docs:**
- the spec's *Data* section describes `grafia.dat`, and `README.ro.md` follows it;
- README.md and CLAUDE.md (*Layout*, *Commands*) describe `cargo run -p grafia-pipeline -- load|build|seed`.

## Phase 3: PWA, and the rest of Python removed

- **`crates/wasm`** (wasm-bindgen + serde-wasm-bindgen): `new Engine(bytes)`, `engine.convert(text, ee, ai)` and `engine.revert(text, ee)`, returning the core's shapes, which are the current API's JSON.
- **`web/`:**
  - `index.html` is moved from `server/static/`. `run()` calls the engine instead of `fetch`, and a status shows while the WASM and data load.
  - Literata (OFL) is self-hosted, so the page works offline.
  - `manifest.webmanifest` and icons make it installable.
  - `sw.js` serves the shell, WASM and data cache-first, under a cache name taken from the build hash.
  - The dexonline attribution stays.
- **Data delivery:** `grafia.dat.gz` is decompressed in the browser with `DecompressionStream('gzip')`, so the static host needs no compression settings.
- **`cargo xtask web`:**
  - builds `crates/wasm` for `wasm32-unknown-unknown`;
  - runs wasm-bindgen through `wasm-bindgen-cli-support` (the version always matches, and there is no CLI to install);
  - gzips the data file and stamps the build hash into `sw.js`;
  - writes `web/dist/`.
- **Tests:** a `wasm-bindgen-test` smoke test under Node converts and reverts some `TEXTS` lines against the seed data. CI builds `web/dist`.
- **Hosting on GitHub Pages** (decision 8):
  - once, in the repository settings: *Pages* → *Source*: GitHub Actions; free hosting needs a public repository;
  - CI has neither MariaDB nor the dexonline dump, so the pipeline's data file is committed (`web/grafia.dat`), and rebuilt and committed after each dexonline update;
  - a workflow runs `cargo xtask web` on each push to `main` and publishes `web/dist/` with `actions/deploy-pages`;
  - the site is served under `/grafia-concisa/`, so every URL (manifest, service worker scope, WASM, data file) is relative.
- **Python removal,** in the commit that adds the Pages workflow:
  - delete `grafia_concisa/`, `server/`, `pyproject.toml`, `uv.lock`, `Dockerfile`, `.dockerignore` and the Python CI job;
  - update CLAUDE.md (*Layout*, *Commands*; the "uv only" rule goes);
  - update README.md;
  - in DECISIONS.md, date the removal and replace "the `/api/revert` endpoint … kept for now" (revert is now in the engine, and the round-trip test is in Rust).

## Phase 4: iOS keyboard

**Development without a Mac and without a paid account:**
- CI on a GitHub Actions macOS runner builds an unsigned `.ipa` (`xcodebuild … CODE_SIGNING_ALLOWED=NO`).
- On Windows, Sideloadly signs it with a free Apple ID and installs it on the iPhone. This needs:
  - the non-Store versions of iTunes and iCloud;
  - Developer Mode on the iPhone (iOS 16 and later).
- A free Apple ID has limits:
  - the signature lasts 7 days (Sideloadly's daemon can refresh it);
  - at most 3 sideloaded apps can be active;
  - the app and its extension each take an App ID.
- The keyboard needs no capabilities (no App Group, no network), so free signing covers it. The first install checks this.

**Check first:** a sideloaded debug build checks two things on the iPhone:
- `UITextChecker.availableLanguages` includes Romanian;
- `completions(forPartialWordRange:in:language:)` works in the keyboard without Full Access.

If not, the source of predictions goes back to Cezar for a decision. The alternative was a dexonline word list built by the pipeline.

**`crates/ffi`** (UniFFI proc macros), shared with Android in phase 5:
- `Engine::open(path)`, which memory-maps the file;
- `convert(text, Options)`, `revert(text, ee)` and `lookup(word, Options)` (one word: the core's `lookup`);
- its own records `Options {ee, ai}` and `Segment {text, alts, unknown}`, converted from the core's types, and an error enum for a bad file.

**`cargo xtask ios`** (macOS only, run in CI): builds `crates/ffi` for `aarch64-apple-ios`, makes `GrafiaCore.xcframework` and generates the Swift bindings.

**`ios/project.yml`** (XcodeGen, so the `.xcodeproj` is generated in CI and never edited by hand on Windows), with two targets:
- `GrafiaConcisa`, the container app: how to enable the keyboard, the web page's screen and attribution, plus ads from the first App Store release (DECISIONS: *Licensing*);
- `GrafiaKeyboard`, with `RequestsOpenAccess = NO`, so no network and no ads in the keyboard.

**The keyboard UI uses open-source KeyboardKit (MIT),** for the layout, shift and caps lock, long-press callouts and the suggestion bar.
- KeyboardKit Pro is excluded: it is licensed, closed source and doesn't fit a GPL app. Its localized layouts and autocomplete are not needed.
- The Romanian layout is our own input set: QWERTY with ă â î ș ț on long press.
- Check at the start that the open-source core allows custom callouts. If it doesn't, write the keyboard view in SwiftUI.

**Data and settings:**
- `grafia.dat` sits in the extension's bundle, memory-mapped, and is loaded only in convert mode.
- The container app reads the same file from inside the extension (`PlugIns/GrafiaKeyboard.appex`), so there is one copy. If that doesn't work, it ships its own copy.
- Without Full Access the keyboard can't read the app's settings, so the mode, -ee and î switches live in the keyboard, in its own UserDefaults (a long press on the mode key).

**Convert mode:**
- On a word separator, the word before the cursor (`documentContextBeforeInput`) goes through `lookup`. If its spelling differs, it is replaced (`deleteBackward` × n, then `insertText`) before the separator is inserted.
- A homograph gets its default spelling, and the suggestion bar offers its other spellings and the word as typed, to swap the last word.
- An unknown word is converted by the rules alone, as on the web.
- The bar shows `UITextChecker` completions of the current prefix, in concise spelling (each passed through `lookup`).

**Letters mode:** ẹ ė ē ō as dedicated keys, and also on long press (ẹ ė ē on e, ō on o). It converts nothing, loads no data and shows no predictions.

**Memory:** with no Mac there is no console, so a debug build shows `os_proc_available_memory()` in the suggestion bar, to check the margin on the iPhone.

**Release** (once the keyboard is ready, not before):
- the Apple Developer Program membership;
- an App Store Connect API key in the CI secrets, so CI signs, archives and uploads to TestFlight;
- the GPL check (see *Risks*).

## Phase 5: Android

- **`crates/ffi`** is reused as is. `cargo xtask android`:
  - builds `libgrafia_ffi.so` with cargo-ndk for arm64-v8a, armeabi-v7a and x86_64 into `jniLibs`;
  - generates the Kotlin bindings with uniffi-bindgen in library mode. They need JNA (`@aar`).
- **`android/`:** a Kotlin `app` module with minSdk 23 (`PROCESS_TEXT` exists from API 23).
  - `grafia.dat` ships in the app's assets. It is copied to `filesDir` once per app version, then memory-mapped.
- **`ProcessTextActivity`:**
  - an intent filter for `android.intent.action.PROCESS_TEXT` and `text/plain`, labelled "grafia concisă", with a translucent theme;
  - it converts with the saved switches;
  - when every word has one known spelling, it returns `EXTRA_PROCESS_TEXT` at once, which replaces the selection;
  - otherwise a dialog shows the text with the homograph switches and unknown words marked as on the web, and an «Înlocuiește» button (or «Copiază» when `EXTRA_PROCESS_TEXT_READONLY` is set).
  - The menu has one entry, which converts; revert is on the main screen.
- **`MainActivity`:**
  - the web page's screen: text, direction, the -ee and î switches (also used by the action) and copy;
  - the dexonline (GPL) attribution and a link to the source code.
- **CI** (Ubuntu): Rust Android targets with the runner's NDK, then `./gradlew assembleRelease`. A signing keystore goes into secrets for Play releases.
- **Local setup:** Android Studio (SDK, NDK, JDK), `cargo install cargo-ndk`, and the Rust Android targets. Distribution is the Play Store, with no ads.

## Risks

- **GPL data on the App Store.** Apple's usage rules add restrictions that the FSF considers incompatible with the GPL; VLC was removed in 2011 over this. Cezar owns the code, but the dexonline data is third-party GPL. Before the first App Store submission, set a custom EULA in App Store Connect and/or ask dexonline for permission. Sideloading one's own build is not affected.
- **Python and Rust Unicode details** (`\w`, case mapping) are covered by the parity gate on real words. Exotic input beyond that is accepted as is.
- **`UITextChecker` without Romanian:** see the check at the start of phase 4.
- **Free signing:** a keyboard extension signed by Sideloadly is checked on the first install. If it fails, the membership is needed earlier than planned.

## Verification

| Phase | Check |
|---|---|
| 1 | `cargo test --workspace` and `uv run python -m unittest discover -s server` pass. The parity diff on the 2026-10-04 dump is empty (MariaDB on port 3307, then `build_db.py --export`). |
| 2 | Data file round-trip tests pass. The build from MySQL is byte-identical to the build from the export. `grafia.dat` is within the size targets, and the stats equal Python's. |
| 3 | The Node smoke test passes. `cargo xtask web`, then serve `web/dist` locally (`npx serve`): convert the `TEXTS` samples, switch a homograph, copy, go offline in devtools and reload. Lighthouse reports the PWA as installable. The same checks pass on the GitHub Pages URL. |
| 4 | Sideload a CI build on the iPhone and enable the keyboard without Full Access. Typing "soare " gives "sōre ". "ia " gives "ė " with "ia" in the bar. Romanian completions appear. The memory margin shows in the debug bar. |
| 5 | The APK builds in CI. On a device or emulator, selecting "soare iarba" in another app's text field replaces it with "sōre ėrba". "ia" opens the dialog with the switch. Read-only text offers copy. |

Each phase ends with its own commits in the CLAUDE.md format, with all tests passing.
