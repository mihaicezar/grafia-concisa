# grafia concisă — rule files plan

**Status:** proposed on 2026-11-11. Stage 0 done on 2026-11-12, its `reguli/` entries of `CLAUDE.md` with stage 2. Stage 1 implemented on 2026-11-12, and the release `dexonline-2026-11-11` published. CI's first build from it gave the MariaDB build of the dump of 2026-11-12, byte for byte (`web/grafia.dat: sha256 13a6afe8550fbb50e7995fd08060a28370ad09610bd681ca13c40784d2f5019e`). Stage 1's gate passed on 2026-11-12: the test pull request #2 (*coajă* `co-a-jă`) listed *coajă* and *coaja* in its comment, and its revert no change. Stage 2 implemented on 2026-11-12: a temporary test found the generated constants equal to the old ones before these went, and on 2026-11-13 each list of words was declared once (`grafia_reguli::LISTS`), the regex `A_KEPT` became the list `A_ROOTS`, and the names in -oe left `ae_oe`, which holds them from `nume` (the first point of stage 7); the local `grafia.dat` differs from the one of the old constants only in the rules hash and the checksum; the gate waits for CI's diff on its pull request, which also carries Cezar's corrections of *înfieri* (`în-fi-eri`), *răspieri* (`răs-pieri`) and *vieri* (`vi-eri`), which change no spelling. Each stage ends with a gate, and the next one starts only once it passes. The plan is deleted when its last stage is done, like `rust-port-plan.md`. `DECISIONS.md` prevails over it.

## Goal

A linguist who doesn't program adds and adjusts rules by editing YAML files in `reguli/`, on GitHub or locally:
- every error is reported against the rule: the file, the line and the column, with a message in Romanian; in CI, on that line of the pull request;
- on every pull request, CI rebuilds `grafia.dat` and says, in Romanian, which words change and how the rules compare with dexonline. Nobody needs a MariaDB for that;
- a rule can cite dexonline as its evidence (DEX-MDA-DOOM words only), and the build checks the citations against the data;
- for any word, a command tells which rule, by file and line, gave each of its cuts.

## Scope

These move to files:
- the data of the rules: the hand lists and the four exception tables;
- the fallback rules themselves, as an ordered list of steps (see *Inventory*).

These mechanisms stay in Rust:
- the precedence of the explicit syllabifications and of dexonline;
- the matching of patterns and contexts;
- the derivation of a form from its lemma;
- the priority of records;
- the -eẹ/aẹ/oẹ rule;
- the pairing of obsolete forms;
- reverse conversion.

A mechanism moves to a file only when a later decision asks for it.

The web app and the apps don't change: the files are compiled into Rust data, and no YAML parser ships.

## dexonline's syllabifications (checked on 2026-11-11)

What a rule can cite, and what the pipeline can read, was checked on the dump and the site of 2026-11-11:
- **dexonline's hyphenation field (`Lexeme.hyphenations`) is DOOM's syllabification.** The check used a random sample of 94 DOOM 3 headwords with ie/ia/ea/oa:
  - the 44 that DOOM 3 syllabifies (`(desp. la-u-re-at)`) all have the field, and it matches. Sometimes the field completes DOOM's fragment to the whole word: DOOM gives `a-gre-a-`, the field `a-gre-a-bil`;
  - the other 50 have no field;
  - of 46 words checked by hand, the 37 that DOOM syllabifies agree as well, and the other 9 have no field.

  The field holds 20,542 lexemes, 8,557 of them among the 63,642 with a candidate digraph. A citation of a dexonline syllabification is therefore a citation of DOOM.
- **DOOM also syllabifies some inflected forms** (*pietrei* `pie-trei`, *vieții* `vie-`, *reală* `re-a-`, *fenicieni* `-ci-eni`). 24 of the 94 entries do. The 11 forms tested all agree with today's derivation from the lemma.
  - The public dump gives only the first 20 characters of DOOM 2 and DOOM 3 definitions. It does the same for DEX '09, MDA2 and NODEX. The pipeline therefore can't read these syllabifications; only the site shows them.
  - The plan does not read the site. `explain` links to the word's page instead.
- **DEX '98 and DEX '96 are the only DEX-MDA-DOOM sources the dump gives in full.** Their pronunciation notes (`[Pr.: la-u-re-at]`) cover 4,464 headwords with ie/ia/ea/oa. Compared with today's cuts, pair by pair:
  - **3,697 pairs agree.**
  - **20 disagree with dexonline's field.** These are older pronunciations: *piemont* `pi-e-`, *toaletă* `to-a-`, *fiestă* `fi-es-`, *farniente*. There DOOM stays first.
  - **66 disagree with the rules alone:**
    - the -ean adjectives: *mediteranean*, *marmorean*, *hiperborean*, *trahean*;
    - compounds whose prefix isn't listed or whose rest isn't a known word: *turboagregat*, *aminoacidurie*, *hipoacuzic*, *monoax*, *paleoantropologic*, *neabordabil*;
    - *idea*, *ideare*, *grea*, *greare*, *pean*, *meat*;
    - the *piezo-* words.

  They are evidence to weigh, not yet a source: stage 5 reports them, and stage 8 asks Cezar.

## Inventory

| Today (`crates/core/src/rules/`) | File (`reguli/`) | Key | Stage |
|---|---|---|---|
| `SYLLABIFICATIONS` | `silabisiri.yaml` | (a map: word → syllabifications) | 2 |
| `OBSOLETE`, `IARE_KEPT` | `invechite.yaml` | `lemme`, `iare_pastrate` | 2 |
| `NAMES`, `AE_OE`, `EEA_UNCHANGED`, `EE_NOT_MASCULINE` | `ee.yaml` | `nume`, `ae_oe`, `eea_neschimbate`, `fara_masculin` | 2 |
| `NO_CONVERT` | `neconvertite.yaml` | `nume_latine` | 2 |
| `A_ROOTS` (before stage 2, the regex `A_KEPT`: `r[ou]mân`) | `a-i.yaml` | `radacini_cu_a` | 2 |
| `IESC_ENDINGS` | `necunoscute.yaml` | `iesc` | 2 |
| `IA_DIPHTHONG` | `diftong-ia.yaml` | (a list of entries) | 3 |
| `EA_HIATUS` | `hiat-ea.yaml` | (a list of entries) | 3 |
| `IE_HIATUS` | `hiat-ie.yaml` | (a list of entries) | 3 |
| `OA_HIATUS` | `hiat-oa.yaml` | (a list of entries) | 3 |
| `rule_cut`, with `IENT`, `PREFIXES`, `IERE_ENDINGS`, `IER_ENDINGS`, `IERE_DIPHTHONG` | `pasi.yaml` | (the steps, in order) | 4 |
| `LABIALS`, and the palatals written into `rule_cut` | `clase.yaml` | `L`, `P` | 4 |

The vowels stay in Rust, since the derivation and the spelling use them too. The endings written into `ee.rs` and `build.rs` are part of the -eẹ/aẹ/oẹ mechanism and stay as well.

## Decisions to log (stage 0)

1. **The rules live in YAML files in `reguli/`, which a linguist edits:** the hand lists, the exception tables and the order of the fallback rules. Why: a linguist who doesn't program can add and adjust rules, and every rule sits next to its examples and its evidence.
2. **The keys are ASCII, with no diacritics:**
   - the patterns and contexts: `tipar`, `post`, `non_post`, `ante`, `non_ante`;
   - the evidence: `exemple`, `contraexemple`, `reper_dex`;
   - the steps: `pas`, `rezultat`, `tabel`, `prefixe`, `rest_cunoscut`, `non_prefixe`, `doar`;
   - the obsolete lemmas: `fara_forme`, and the other fields of `invechite.yaml`.

   So are the fixed values: `hiat`, `diftong`, `verb`. Words are values, written with their diacritics. Why: a key is typed often and must be typed exactly; a word is data.
3. **The files are compiled into Rust data when the core is built.** `RULES_HASH` covers what they produce, not their comments, examples or `reper_dex`. Why: the web app and the apps ship no YAML parser, and a change that can't change a spelling changes nothing in `grafia.dat`.
4. **Errors give the file, the line and the column, in Romanian**, and in CI also as annotations on that line of the pull request.
5. **`reper_dex`: an entry may cite the dexonline words that back it**, DEX-MDA-DOOM lemmas whose dexonline syllabification (DOOM's, see above) cuts the pair as the entry does. The build checks them.
6. **The export of dexonline's records is published as a release asset, and CI builds `grafia.dat` from it.** `dexonline.lock` pins the export, and `grafia.dat` is no longer committed. Why: a linguist's pull request then needs nobody with a MariaDB, and the deployed file is always built with the rules it ships with. This reverses "CI can't build it" (2026-10-10). The rules hash stays, as a check on local builds.
7. **On every pull request, CI reports in Romanian the words whose spelling changes**, and from stage 5 how the rules compare with dexonline. Why: Cezar and the linguist review words, not code.

## Layout

```
reguli/                      the rule files; each starts with a Romanian comment on what it holds
crates/reguli/               package grafia-reguli: reads reguli/, checks it, writes the Rust code;
                             its binary runs the loading checks (cargo run -p grafia-reguli)
crates/core/codegen.rs       the core's build script (Cargo.toml: build = "codegen.rs", since
                             src/build.rs is the data file builder): writes $OUT_DIR/reguli.rs
                             (the lists, the tables and the steps: hashed) and
                             $OUT_DIR/reguli_meta.rs (positions, exemple, contraexemple,
                             reper_dex: not hashed, unused by the web app)
crates/core/tests/reguli.rs  the rules checks: every exemple and contraexemple
crates/pipeline              export (also date.txt; from stage 5 dex98.tsv); build and check,
                             with the data checks; report; explain
xtask                        cargo xtask data: fetches the pinned export into data/, builds
                             web/grafia.dat; cargo xtask pin DIR: packs an export and writes
                             dexonline.lock; cargo xtask diff BASE: the snapshot diff, in
                             Romanian; cargo xtask reguli: the loading checks, then the rules checks
dexonline.lock               the pinned export: its release asset and SHA-256
```

## Format

### General

- YAML 1.2, UTF-8. Keys are lowercase ASCII with `_`. An unknown key, a duplicate key or a value of the wrong type is an error, so a typo is never silently ignored. Under YAML 1.2, `n`, `y`, `no` and `on` are words, not booleans (the context `n` of `ghia`), and every field is typed as text anyway.
- Words are written in lower case, in the standard orthography: NFC, ș/ț with a comma below, â inside words.
- In a pattern or a context, a name in capitals is a class of letters:
  - `C` (a consonant) and `V` (a vowel) are given;
  - the others are defined in `clase.yaml`.
- Order is kept where it matters:
  - the lemmas of `invechite.yaml`: the first is the default;
  - the readings of `silabisiri.yaml`: the first is the default;
  - the entries of a table: the first that applies is the one reported;
  - the steps of `pasi.yaml`: the first that applies decides.
- `#` starts a comment. Comments explain; they never change a spelling.

### Lists

```yaml
# silabisiri.yaml: cuvânt: silabisire, sau [silabisiri], prima fiind cea implicită
neant: ne-ant
vier: [vier, vi-er]        # vierul; păzitor de vie (rar)
coastă: coas-tă            # co- nu e prefix aici

# invechite.yaml
lemme:
  - lema: a-li-ne-a        # silabisită: dă grafia ei proprie (alineēză)
    actuala: alinia
    tulpina: aline
  - lema: ra-chi-a-rea-să
    actuala: rachiereasă
    tulpina: rachiare
    fara_forme: true       # dexonline o dă fără forme (implicit: false)
iare_pastrate: [fiare, mobiliare]              # lemme în -iare care nu sunt infinitive lungi

# ee.yaml
nume: [andreea, medeea, leea, zoe, cloe]       # nume feminine de persoane
ae_oe: [nae, nicolae, noe, aloe]               # și numele în -oe din nume (zoe, cloe)
eea_neschimbate: [aceea, oareceea]
fara_masculin: [coree]
```

The other lists are of the same shape: `nume_latine: [traian]`, `radacini_cu_a: [român, rumân]`, `iesc: [ești, escu, ...]`.

### Patterns and contexts

Exception tables and steps share these keys:

| Key | Meaning |
|---|---|
| `tipar` | one or more patterns: letters, with `^` for the word start and `$` for the word end |
| `post` | just before the pattern is one of these |
| `non_post` | just before the pattern is none of these |
| `ante` | just after the pattern is one of these |
| `non_ante` | just after the pattern is none of these |

A pattern applies to a pair of vowels when it holds both of them.

What a context item can be:
- **Letters and classes.** An item of `post` ends right before the pattern, and an item of `ante` starts right after it.
- **Anchors.** A leading `^` ties a `post` item to the word start, and a trailing `$` ties an `ante` item to the word end:
  - `^` alone: the pattern starts the word;
  - `^gor`: the word starts with *gor* right before the pattern;
  - `^C`: a single consonant comes before the pattern, from the word start;
  - `$` alone: the pattern ends the word;
  - `e$`: an e follows the pattern, then the word ends.

No other notation exists: no regex.

### Exception tables

A table holds the exceptions of one step of `pasi.yaml`, which names it in its `tabel`. Each entry:

| Key | Required | Meaning |
|---|---|---|
| `tipar`, `post`, `non_post`, `ante`, `non_ante` | `tipar` | see above |
| `exemple` | yes | words the entry decides |
| `contraexemple` | with `non_post` or `non_ante` | words that hold a pattern, but which the contexts exclude |
| `reper_dex` | no | dexonline words that back the entry (see *Checks*) |

```yaml
# diftong-ia.yaml: tulpini native în care ia e diftong (o silabă)
- tipar: [ghia]
  non_post: [n, litur, ^gor]
  non_ante: [t, n, l, ce]
  exemple: [maghiar, ghiară, ghiarele]
  contraexemple: [unghiar, liturghiar, gorghias, privileghiat, aghian, oghial, eleghiacesc]

# hiat-ea.yaml: neologisme în care ea e hiat (două silabe)
- tipar: [eat]
  post: [gr, aur, lor, lin, ucl, un, ct, id, upl]
  exemple: [agreat, laureat, bacaloreat, alineat, nucleat, cuneat, bracteat, ideat, supleat]
  contraexemple: [beat, cleat, pleată, soreată]
  reper_dex: [laureat]     # DOOM 3: la-u-re-at

- tipar: [react, reacț, steat, meandr, aleat, beatif]
  exemple: [reactor, reacție, steatoză, meandru, aleator, beatific]

# hiat-ie.yaml
- tipar: [bier]
  non_post: [^, z, s]
  exemple: [bărbierit, corăbierie]
  contraexemple: [zbiera]
```

The examples given today in `crates/core/tests/tables.rs` (the `SYLLABIFY` lines of these four lists) are enough to write `exemple` and `contraexemple` for every entry.

### Steps

`pasi.yaml` holds the fallback rules: for two adjacent vowels that neither an explicit syllabification nor dexonline separates, the first step that applies decides. Each step:

| Key | Required | Meaning |
|---|---|---|
| `pas` | yes | the step's name, used by the trace and the messages |
| `tipar`, `post`, `non_post`, `ante`, `non_ante` | no | see above; a step with no `tipar` and no `prefixe` applies to every pair |
| `prefixe` | no | the word starts with one of these, and the pair is the prefix's last vowel and the next letter |
| `rest_cunoscut` | with `prefixe` | the rest of the word is a known word (see the spec's fallback table) of at least this many letters |
| `non_prefixe` | no | not when the word is also one of these followed by a known word |
| `doar` | no | `verb`: only in a verb infinitive (dexonline model V or VT) |
| `tabel` | no | a table: the step applies only where one of its entries also applies; otherwise the next step is tried |
| `rezultat` | yes | `hiat` (two syllables) or `diftong` (one syllable; for vowels that make no digraph, simply no cut) |
| `exemple` | unless `tabel` | words the step decides |
| `contraexemple` | with `non_post`, `non_ante`, `non_prefixe`, `rest_cunoscut` or `doar` | words that hold a pattern or a prefix, but which a condition excludes |
| `reper_dex` | no | as in a table |

The file, as today's `rule_cut` reads in it:

```yaml
# pasi.yaml: regulile de rezervă, în ordine; primul pas care se aplică decide.
# Clasele: C consoană, V vocală; L labială, P palatală (clase.yaml).

- pas: e-e
  tipar: [ee]
  rezultat: hiat
  exemple: [idee]

- pas: prefix și un cuvânt cunoscut
  prefixe: [re, pre, ne, tele, co, semi, anti, multi, auto, micro, macro, foto, pseudo, bio,
            neo, radio, aero, agro, electro, hidro, termo, geo, proto, socio, psiho, stereo]
  rest_cunoscut: 4
  non_prefixe: [prea]      # prea + un cuvânt cunoscut e o silabă: preacurat
  rezultat: hiat
  exemple: [reabilitat, neadevărat, autoanaliză, coactiv]
  contraexemple: [neam, neagră, autoare, preacurat]

- pas: -ient
  tipar: [ient, ienț]
  ante: [a, ă, e, i, $]
  rezultat: hiat
  exemple: [client, ambient, experiență, clientelă]

- pas: -abil
  tipar: [iabil, eabil]
  post: [^, C]
  rezultat: hiat
  exemple: [fiabil, agreabil]
  contraexemple: [debreiabil]

- pas: bi-en
  tipar: [^bien]
  rezultat: hiat
  exemple: [bienal]

- pas: ea în neologisme
  tipar: [ea]
  tabel: hiat-ea.yaml
  rezultat: hiat

- pas: oa în neologisme
  tipar: [oa]
  tabel: hiat-oa.yaml
  rezultat: hiat

- pas: -ii final la verbe
  tipar: [ii$]
  post: [C]
  doar: verb
  rezultat: hiat
  exemple: [sfii, prii]
  contraexemple: [copii]

- pas: ie, ia la început sau după vocală
  tipar: [ie, ia]
  post: [^, V]
  rezultat: diftong
  exemple: [iarbă, femeie, tăia, tăiere, înmuiere]

- pas: ie, ia final după consoană
  tipar: [ie$, ia$]
  post: [C]
  rezultat: hiat
  exemple: [familie, vie, istoria]

- pas: -iare care nu e infinitiv lung
  tipar: [iare$]
  post: [L, P]
  rezultat: diftong
  exemple: [fiare]

- pas: tulpini native cu ia
  tipar: [ia]
  post: [C]
  tabel: diftong-ia.yaml
  rezultat: diftong

- pas: ia după consoană
  tipar: [ia]
  post: [C]
  rezultat: hiat
  exemple: [apropiat, pian, social, specializat, glaciar, variare]

- pas: -iere nativ
  tipar: [^mier, ^înmier, ^fier, ^pier]
  ante: [e$, ea$, i$, ii$, ile$, ilor$]
  rezultat: diftong
  exemple: [miere, mierea, înmiere, fiere, pieri]

- pas: -iere, infinitive lungi
  tipar: [ier]
  post: [L, P]
  ante: [e$, ea$, i$, ii$, ile$, ilor$]
  rezultat: hiat
  exemple: [copiere, apreciere, veghiere, învierea, aprecierilor]

- pas: -ier, -ieră, -iereasă
  tipar: [ier]
  post: [L, P]
  ante: [$, ă$, easă$, easa$, ese$, esei$, esele$, eselor$]
  non_post: [^C, ^CC, ^CCC]  # nu în cuvintele de o silabă
  rezultat: hiat
  exemple: [plumier, banchier, bărbier, legumieră, rachiereasă, rachieresele]
  contraexemple: [fier, zbier]

- pas: -iesc
  tipar: [iesc$]
  post: [L, P]
  rezultat: hiat
  exemple: [zapciesc, surugiesc]

- pas: ie în neologisme
  tipar: [ie]
  post: [L, P]
  tabel: hiat-ie.yaml
  rezultat: hiat

- pas: ie după labială sau palatală
  tipar: [ie]
  post: [L, P]
  rezultat: diftong
  exemple: [fier, piept, pieptene]

- pas: ie după altă consoană
  tipar: [ie]
  post: [C]
  rezultat: hiat
  exemple: [prieten, cartier]

- pas: restul
  rezultat: diftong
  exemple: [seară, soare]
```

```yaml
# clase.yaml: clase de litere pentru tipare și contexte; C și V sunt date
L: [p, b, f, v, m]         # labiale
P: [c, g, ch, gh]          # palatale (înainte de i)
```

The step that names a table says where the table applies. The tables therefore need no comment on their scope, and the scope can't drift from the code.

Two steps read differently from today's code. The gate of stage 4 shows whether any word depends on either:
- **`-ier, -ieră, -iereasă`:** today a one-syllable word (*fier*) is decided there as a diphthong. Here it goes on to the later steps.
- **`prea` + a known word:** today it is decided in the prefix step as one syllable. Here it goes on to the later steps.

### Checks and messages

| Level | Run by | Checks |
|---|---|---|
| loading | the build script (it stops the build), `cargo run -p grafia-reguli` | see below |
| rules | `cargo test`, `cargo xtask reguli` | see below |
| data | `grafia-pipeline build` and `check`, on the export (in CI) or on MariaDB | see below |

The loading checks:
- the YAML syntax, the keys and their types;
- the characters:
  - errors: ş/ţ with a cedilla, ã, a decomposed letter, a capital outside a class name, a letter outside the Romanian alphabet;
  - a warning: î inside a word, since *neînțeles* is right;
- the classes: each is defined, `C` and `V` are not redefined, and each class holds single letters or digraphs;
- duplicates;
- each list's shape:
  - the letters of a syllabification are its word's, and its readings differ;
  - `tulpina` starts `lema`;
  - the endings of the words: `nume` in -eea/-ee/-oe, `ae_oe` in -ae/-oe, `eea_neschimbate` in -eea, `fara_masculin` in -ee, `nume_latine` in a vowel + -ian, `iare_pastrate` in -iare;
  - `radacini_cu_a` hold â;
  - no name in -oe of `nume` is written again in `ae_oe`, which holds them (from stage 7, done with stage 2);
- each pattern holds a pair of vowels;
- each table is named by exactly one step, and the step's patterns hold the pairs of the table's patterns;
- the last step has no condition, so every pair is decided.

The rules checks:
- each `exemple` word: the rules alone decide the pair by this entry or step, its way. The rules run with no dexonline and the tests' known words;
- each `contraexemple` word: the entry or step doesn't apply, and a later one decides the pair the other way;
- each pattern, each prefix and each item of `post` and `ante` has an `exemple` word;
- each item of `non_post` and `non_ante` excludes a `contraexemple` word;
- no word is in both `exemple` and `contraexemple`.

The data checks:
- **`reper_dex`:** each word is a DEX-MDA-DOOM lemma whose dexonline syllabification covers the pair the entry matches and cuts it as the entry does. From stage 6, the word's DEX '98/'96 note counts where the field doesn't cover the pair. A disagreement is an error;
- **the words of the lists are in dexonline:**
  - a warning, but an error for `actuala`;
  - *Cloe* is missing on purpose;
- **the words of the lists are DEX-MDA-DOOM:** a warning, names aside.

A message names the place and says what to do:

```
reguli/diftong-ia.yaml:5:30: eroare: „ghiare” nu e decis de acest rând, ci de pasul „-iare care nu e infinitiv lung” (pasi.yaml:66); alege un cuvânt pe care îl decide doar rândul acesta.
reguli/hiat-ie.yaml:9:9: eroare: „tăietor”: pasul „ie, ia la început sau după vocală” (pasi.yaml:54) îl decide înaintea acestui tabel, căci ie urmează după o vocală.
reguli/ee.yaml:3:30: eroare: „zoş” are ş cu sedilă; scrie ș, cu virgulă.
reguli/pasi.yaml:26:14: eroare: clasa „LAB” nu e definită; clasele sunt C, V și cele din clase.yaml (L, P).
reguli/hiat-ie.yaml:31:14: eroare: reper_dex „piemont”: dexonline (DOOM) îl silabisește pie-mont, deci ie e diftong acolo, iar tabelul îl face hiat. DEX '98 dă pi-e-mont, dar DOOM are întâietate.
```

With `GITHUB_ACTIONS` set, each message is also printed as `::error file=…,line=…,col=…::…` (or `::warning`), which GitHub shows on that line of the pull request's changed files.

## Stages

### Stage 0: decisions and docs

- Log the decisions above in `DECISIONS.md` (*Platforms and stack*).
- `CLAUDE.md`:
  - the layout: `reguli/`, `crates/reguli/`, `dexonline.lock`;
  - this plan among the docs;
  - the convention that rules go in `reguli/`;
  - from stage 3, the convention that an example of a table entry or a step is one of its `exemple` or `contraexemple` words, not a line of `SYLLABIFY`.
- Choose the YAML parser. It must follow the YAML 1.2 core schema, give the line and column of every value, reject duplicate keys, be maintained and be pure Rust. It is a dependency of `grafia-reguli` only; `serde_yaml` is archived. Chosen: `marked-yaml` 0.8, with `LoaderOptions` `error_on_duplicate_keys(true)`, `prevent_coercion(true)` and `toplevel_sequence()` for the tables (see `DECISIONS.md`).

**Gate:** Cezar approves this spec.

### Stage 1: the published export, and `grafia.dat` built by CI

- `grafia-pipeline export` also writes `date.txt`, the date of the dexonline data, and `build --text` reads it, so a file built from the export is the one built from MariaDB, byte for byte.
- The export (`records.tsv`, `known.txt`, `date.txt`) is published as one archive on a GitHub release, `dexonline-YYYY-MM-DD`, with dexonline's attribution (GPL) in its notes. `dexonline.lock` names the archive and its SHA-256. `cargo xtask pin DIR` packs the archive, the same bytes for the same export, and writes the lock.
- `cargo xtask data` fetches the pinned export into `data/`, unless it is already there with that checksum, and builds `web/grafia.dat` from it. `web/grafia.dat` leaves git (`.gitignore`), after the gate's comparison.
- `cargo xtask diff BASE` does what the `snapshot-diff` skill does by hand:
  - it builds the snapshots of BASE, in a worktree, and of the working tree, on the pinned export, or on both exports when `dexonline.lock` changed;
  - it writes `data/snapshot/diff.md`, in Romanian, with the skill's groups;
  - the skill then calls it.
- CI:
  - `cargo run -p grafia-reguli` (from stage 2) and `cargo xtask data` run before the tests;
  - the deployed `grafia.dat` is the one CI builds;
  - on a pull request, `cargo xtask diff` runs against the base branch. Its `diff.md` goes into the job summary and into one comment on the pull request, replaced on each push. A pull request from a fork gets the summary only, since its token can't write.
- A dexonline update:
  - someone with the MariaDB runs `load` and `export`, then publishes the release;
  - a pull request changes `dexonline.lock`, and its comment shows what the update changed.
- Docs:
  - `README.md`: MariaDB only for dexonline updates, and `cargo xtask data`;
  - `CLAUDE.md`: the commands. "Rebuild and commit it after…" goes, and the update procedure replaces it;
  - the spec's *Data*: where the file comes from;
  - `README.ro.md` follows.

**Gate:**
- CI's `grafia.dat` is byte for byte the one built from MariaDB on the same dump;
- a test pull request that changes one rule shows its words in the comment, and its revert shows none.

### Stage 2: the loader, and the lists

- `grafia-reguli`: reading with positions, the loading checks, the messages and the code generation. The binary prints the messages, and the annotations in CI.
- `codegen.rs` writes the constants with the same names as today, which `lists.rs` includes: `SYLLABIFICATIONS`, `OBSOLETE`, and one `&[&str]` per list of words. Each list is declared once, in `grafia_reguli::LISTS`: its file, key, constant and the shape of its words. The regex `A_KEPT` becomes the list `A_ROOTS`, which `a_to_i` reads.
- `RULES_HASH` also hashes `$OUT_DIR/reguli.rs`, beside what stays of `lists.rs` and `words.rs`.
- The files: `silabisiri`, `invechite`, `ee`, `neconvertite`, `a-i`, `necunoscute`.
- A table of broken files with the message expected for each (`crates/reguli/tests/`), one line per case, like the other test tables.
- Docs:
  - the spec gets a section *Rule files* (the general format and the lists), and names the files where it named `rules::X`;
  - `README.ro.md` follows;
  - the `rule-change` and `test-gauntlet` skills point to `reguli/`.

**Gate:**
- a temporary test compares each generated constant with today's, then goes away with the old constants;
- `cargo test --workspace` and the WASM tests pass;
- CI's diff shows no change: the data file differs only in the rules hash and the checksum.

### Stage 3: the exception tables, and the trace

- **The trace.** `rule_cut` tells what decided each pair: a step, by its name in the code for now, or a table entry, by file and line.
  - `syllabify` keeps its signature;
  - a traced variant gives each pair's source: `silabisiri.yaml` and its line, dexonline, a partial syllabification, a step or an entry.
- **The files.** The four tables move to files, with `exemple` and `contraexemple` taken from `SYLLABIFY`, whose lines for these lists then go. Until stage 4, each file's header comment says where its step consults it.
- **Each entry is matched on its own.** Today each list is one regex, whose matches don't overlap, so a match of one alternative can hide another. The gate shows whether any word depends on that.
- **The rules checks** go in `crates/core/tests/reguli.rs`, and run through `cargo xtask reguli` too.
- Docs:
  - the spec's *Rule files* gets the tables, and its fallback table names the files;
  - `README.ro.md` follows;
  - the `test-gauntlet` skill: the rules checks replace the manual check that each alternative has a word it alone decides.

**Gate:**
- CI's diff shows no change. A change is reviewed as a rule change;
- every `exemple` and `contraexemple` passes.

### Stage 4: the order of the steps

- `pasi.yaml` and `clase.yaml`, as in *Format*:
  - `rule_cut` becomes their interpreter;
  - `IENT`, `PREFIXES`, `IERE_ENDINGS`, `IER_ENDINGS`, `IERE_DIPHTHONG` and `LABIALS` leave the code;
  - the tables lose their scope comments.
- The trace gives the step's line.
- The steps' `exemple` and `contraexemple` replace the `SYLLABIFY` lines of the fallback rules. `SYLLABIFY` keeps the dexonline, partial and explicit cases.
- Docs:
  - the spec's fallback table follows the order of `pasi.yaml` and names its steps;
  - `README.ro.md` follows.

**Gate:**
- CI's diff shows no change. The two steps that read differently (see *Steps*) are checked first;
- every `exemple` and `contraexemple` passes.

### Stage 5: the report and the explanation

- **`grafia-pipeline report`** runs on the export, so in CI too. It writes `raport.md`, in Romanian, with, for each step and each table entry, by file and line:
  - the words of the database it decides, how many are DEX-MDA-DOOM, and the steps and entries that decide none;
  - the lemmas it decides by rules alone that dexonline syllabifies, compared with dexonline pair by pair, the disagreements of DEX-MDA-DOOM words first;
  - the DEX-MDA-DOOM words that dexonline syllabifies as it does: candidates for `reper_dex`.
- **The export gains `dex98.tsv`:** each headword's DEX '98/'96 pronunciation note. The report also compares the rules with these notes, as for dexonline, and a new export is published.
- **`grafia-pipeline explain WORD…`** prints, for each record of the word:
  - its lemma, the lemma's dexonline hyphenation and DEX '98 note, and a link to its dexonline page, which shows DOOM's own syllabifications;
  - the source of each cut (a file and line, dexonline, or a step) and the derivation row;
  - its spelling, and which spelling is the default.
- On a pull request, the comment also lists the lines of the report that changed.

**Gate:** both run on the current export, and Cezar reviews the first report. Nothing in the conversion changes.

### Stage 6: `reper_dex` and the data checks

- The data checks, in `grafia-pipeline build` and in `grafia-pipeline check`, which writes no data file. Both run in CI.
- The first `reper_dex`, chosen by Cezar among the report's candidates.
- Docs: `reper_dex` and the data checks in the spec's *Rule files* and in `README.ro.md`.

**Gate:** the checks pass on the current export, and a table of records covers each error. `reper_dex` is not hashed, so adding one changes nothing in `grafia.dat`.

### Stage 7: each fact written once

- The names in -oe are written only in `nume`, and `AE_OE` is generated with them: done with stage 2, on 2026-11-13 (`plus` in `grafia_reguli::LISTS`).
- `fara_forme` is computed from the records, if they tell it reliably; otherwise it stays.

**Gate:** CI's diff shows no change.

### Stage 8: rule changes the files make easy

Each one is a decision for Cezar, carried through the `rule-change` skill, not a refactor. The first candidates:
- **`forme: toate` in `neconvertite.yaml`.** Every dexonline form of a Latin name would then keep its spelling (*Traiana*, *Traianului*). On 2026-10-14 `DECISIONS.md` left those forms to "join the list later".
- **The 66 pairs where DEX '98 disagrees with the rules alone** (see *dexonline's syllabifications*). There are two ways to settle them:
  - **as a source:** the notes become a syllabification source after dexonline's own and before the rules;
  - **case by case:** new table entries and prefixes, with the notes as their `reper_dex`. This covers the -ean adjectives, and *turbo-*, *amino-*, *hipo-*, *mono-*, *paleo-*.

## The linguist's workflow (after stage 6)

1. Edit a file of `reguli/`, on GitHub or locally, and open a pull request.
2. CI shows the loading and rules errors on the lines in question. It then rebuilds `grafia.dat`, and comments with the words whose spelling changed and with what changed in the report.
3. Cezar and the linguist review the words. Once the pull request is merged, the web app deploys with the new data file.
4. A wrong spelling found later: `grafia-pipeline explain` names the rule that gave it.

Only a dexonline update needs someone with the MariaDB (see stage 1).

## Risks

- **The export becomes the input of every CI build.** A wrong export breaks every pull request. It is pinned by checksum and changed only in a pull request of its own, whose diff shows its effect.
- **The step interpreter replaces the code of `rule_cut`** (stage 4). The order and the contexts must reproduce it exactly. The steps' examples and the diff on every form of the database are the check.
- **Matching each entry on its own** (stage 3) may decide a word that a hidden alternative missed. The gate shows it.
- **The YAML parser** must give positions and follow YAML 1.2. A parser that lacks either fails the goal of error messages by line.
- **Docs to keep in step:** the spec, `README.ro.md`, `CLAUDE.md` and three skills, in stages 1 to 4 and 6.

## Open questions

1. **DEX '98 as a source** (stage 8): rank it after dexonline's field, or use its notes only as evidence for new entries?
2. **DOOM's syllabifications of inflected forms** are on the site only. Should the pipeline ever read them, if dexonline agrees to give them? Today they all confirm the derivation, so nothing asks for it yet.
