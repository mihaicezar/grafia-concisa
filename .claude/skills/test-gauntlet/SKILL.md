---
name: test-gauntlet
description: Use before merging a branch, after a change to crates/core, crates/reguli or reguli/, or when asked how strong the tests are. Runs fmt, clippy, the tests and mutation testing on the changed Rust code, then closes each gap with a test line or a question to Cezar.
---

# Test gauntlet

The tests are the review: a change to `crates/core`, `crates/reguli` or `reguli/` is accepted when every gate passes and every surviving mutant is either killed or explained.

## Gates (all must pass)

```
cargo fmt --all --check
cargo run -p grafia-reguli   # the loading checks of reguli/: no error, and no warning left unexplained
cargo xtask data   # web/grafia.dat, which the WASM tests read: README.md, *The data file*
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --locked
cargo clippy -p grafia-wasm --target wasm32-unknown-unknown --all-targets -- -D warnings
cargo test -p grafia-wasm --target wasm32-unknown-unknown --locked   # under Node: README.md, *Tests*
```

## Mutation testing

Install once: `cargo install --locked cargo-mutants`.

On a branch, mutate only what changed:

```
git diff main > target/mutants.diff
cargo mutants -p grafia-concisa --in-diff target/mutants.diff
cargo mutants -p grafia-reguli --in-diff target/mutants.diff   # when crates/reguli changed
```

When asked for a full run, or after a large refactor: `cargo mutants -p grafia-concisa` (slow: run it in the background), or `-p grafia-reguli` (fast). Results are in `mutants.out/`: `missed.txt` lists the survivors.

For each survivor:
1. Find what behaviour it changes, and a word that shows it. Look first in the examples of `concise-writing-spec.md` and `DECISIONS.md`: Cezar has approved those.
2. Add that word as one line in the right table of `crates/core/tests/tables.rs` (the `rule-change` skill maps rules to tables), or, for `grafia-reguli`, a broken file and its messages as one line of `BROKEN` in `crates/reguli/tests/broken.rs`. Run that mutant again (`cargo mutants -p <package> -F '<part of its name>'`) to see it caught.
3. If no settled example exists, ask Cezar for a word, in Romanian. Never guess the expected output.
4. If the mutant can't change any output (an equivalent mutant), add it to `exclude_re` in `.cargo/mutants.toml` with a comment saying why. Create the file on the first such case.

## The blind spot

cargo-mutants does not mutate regex strings, the entries of constant lists (`IA_DIPHTHONG`, `EA_HIATUS`, `IE_HIATUS`, `OA_HIATUS`, `PREFIXES`, `IERE_DIPHTHONG`, ...), or the rule files of `reguli/` (`silabisiri.yaml`, `invechite.yaml`, `ee.yaml`, `neconvertite.yaml`, `a-i.yaml`, `necunoscute.yaml`). For each alternative, entry or word the diff adds or changes, check that a test line depends on it: a word that the entry alone decides, plus a negative word for each exception. A word that another rule already decides doesn't count (*reabilitat* is a hiatus by the -abil rule even without the prefix `re`).

## Rules

- Never weaken, delete or skip a test to make a gate pass.
- Never change an expected value in a table without a decision from Cezar: the tables record his decisions.
- Report: the gates, the mutants caught / missed / excluded, and each test line added.
