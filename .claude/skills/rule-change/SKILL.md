---
name: rule-change
description: Use when Cezar settles or reopens a linguistic decision (a spelling, a syllabification, an exception list, a switch rule). Carries it through DECISIONS.md, the test tables, the rule files of reguli/ or the Rust core, the spec, README.ro.md and one commit.
---

# Rule change

A rule change is one commit that touches, in this order: the decision, the tests, the rule files or the code, the spec and its Romanian rendering. The tests come before the code, so the new lines fail first.

## 0. Is the decision settled?

Go on only if Cezar stated the decision, with at least one example word. If a word, a cut or an exception is not settled by Cezar, `DECISIONS.md`, the spec or dexonline, ask him in Romanian and stop. Never guess a linguistic case.

## 1. DECISIONS.md

Add the entry under the section that matches the spec section it changes:
- the date in bold, the principle, **Why:**, an example, and the spec section;
- no mechanics (suffix lists, regexes, tables): those live only in the spec;
- reopening a decision keeps the old entry and adds the new date (see the 2026-10-06 entries).

## 2. Test lines first (red)

Add one line per example word of the decision, and one per counter-example ("not X"), in `crates/core/tests/tables.rs`:

| The rule is about | Table |
|---|---|
| the fallback syllabifier, an exception list or a regex (`IA_DIPHTHONG`, `EA_HIATUS`, `IE_HIATUS`, `OA_HIATUS`, `PREFIXES`, `IERE_DIPHTHONG`, ...) | `SYLLABIFY` |
| a syllabified word to its concise spelling | `WORDS` |
| a form's cuts derived from its lemma | `DERIVED` |
| the -ee rule | `EE_RULE`, `EE_WORDS` |
| a whole text through the database (case, unknown words, switches) | `TEXTS` |
| reverse conversion | `REVERTS`, `REVERT_HOMOGRAPHS` |
| homographs and the default spelling | `HOMOGRAPH_RECORDS` + `HOMOGRAPHS` |
| unknown words with the -ee switch | `UNKNOWN_EE` |

Each new regex alternative or list entry gets its own positive line, and a negative line when it has an exception (a lookaround, a "not X"). Mutation testing can't see regexes or list entries, so these lines are the only guard.

Run `cargo test --workspace`. At least one new line must fail. If all pass, the lines don't test the change: find a word whose output the decision changes, or ask Cezar for one.

## 3. Rule files or code (green)

- A word of a hand list changes in its rule file of `reguli/`, never in Rust: the explicit syllabifications (`silabisiri.yaml`), the obsolete lemmas and the -iare lemmas kept (`invechite.yaml`), the lists of the -eẹ/aẹ/oẹ rule (`ee.yaml`), the Latin names (`neconvertite.yaml`), the roots that keep â (`a-i.yaml`) and the -iesc endings (`necunoscute.yaml`). The format is in the spec's *Rule files*; comments are in Romanian. Run `cargo run -p grafia-reguli`: it reports what is wrong, by line.
- Anything else changes `crates/core` only. A new list of words is one line of `LISTS` in `crates/reguli/src/lib.rs` (its file, key, constant and the shape of its words), plus the core code that reads its constant; a new check of the rule files also changes `crates/reguli`, with a line of `BROKEN` in its `tests/broken.rs` per new message.
- Keep the constants' names: the spec cites them as `rules::X`, and the lists of `reguli/` by file and key (`ee.yaml: nume`).
- If the rule needs a new record flag, add it to the query in `crates/pipeline/src/records.rs`, and document it in `crates/core/src/build.rs` (module documentation), in `build --help` (`crates/pipeline/src/main.rs`) and in the spec's *Data* section.
- Run `cargo test --workspace`. It passes.

## 4. Spec and README.ro.md

- `concise-writing-spec.md`: the complete current rule, no history. Its examples must agree with the test lines.
- `README.ro.md`: the same change in Romanian, nothing beyond the spec.
- If the spec and DECISIONS.md disagree, fix the spec.

## 5. Gates

- Run the `test-gauntlet` skill on the change.
- Run the `snapshot-diff` skill and show Cezar the words whose output changed. A change the decision does not predict is a question for Cezar, in Romanian, before the commit. If the pinned dexonline export can't be fetched, say so in the final message.
- Rebuild the web app's data file, since it holds the rules' output: `cargo xtask data` (README.md, *Rule changes*; no MariaDB). Then run `cargo test -p grafia-wasm --target wasm32-unknown-unknown`. The data file is not committed: CI builds it from the same export on every push.

## 6. Commit

One commit with DECISIONS.md, the tests, the rule files or the code, the spec and README.ro.md:

```
feat(rules) : <summary>

- <change>
- <change>

Rezumat:
- <în română>
```

Type `feat` for a new rule, `fix` for a correction. No line wrapping.
