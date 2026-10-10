---
name: snapshot-diff
description: Use after a rule change or a dexonline update to list every word whose output changed, across all forms in the database, for Cezar to review. Also when asked "what did this change affect?".
---

# Snapshot diff

The test tables check chosen words. This checks every form in the database: it writes the output of the core for all forms, before and after, and lists what changed. Cezar reviews the words, not the code.

## Inputs

- The export of dexonline records: `DATA/records.tsv` and `DATA/known.txt`, written by
  `uv run --extra pipeline python pipeline/build_db.py --export DATA --host 127.0.0.1 --port 3307`
  (MariaDB must be running: see README.md, *This machine*). `DATA` is `data` by default.
- The writer: `crates/core/examples/snapshot.rs`. It writes `references.tsv`, the five tables, `stats.tsv`, `convert.tsv`, `unknown.tsv` and `revert.tsv` (documented in `pipeline/parity.py`).

If there is no export and no MariaDB, stop and say so.

## Before and after

**Rule change** (same data, two versions of the code). Build "before" from the base commit in a worktree, run from the repository root so the data path resolves:

```
git worktree add ../grafia-base <base commit>
cargo run --release --manifest-path ../grafia-base/Cargo.toml -p grafia-concisa --example snapshot -- data data/snapshot/before
cargo run --release -p grafia-concisa --example snapshot -- data data/snapshot/after
git worktree remove ../grafia-base
```

**dexonline update** (same code, two exports). Keep the old export: export the new dump to a new directory (`data/export-YYYY-MM-DD`), then run the example once on each into `data/snapshot/before` and `data/snapshot/after`.

`data/` is ignored by git; nothing here is committed.

## The diff

```
git diff --no-index --stat data/snapshot/before data/snapshot/after
git diff --no-index -U0 data/snapshot/before/forms.tsv data/snapshot/after/forms.tsv
```

A `forms.tsv` row is `form, concise, concise_ee, alternatives`. Sort the changed rows into:
1. default spelling changed (column 2);
2. alternatives added or removed (column 4);
3. -ee spelling changed (column 3);
4. forms added or removed (a dexonline update, or a change to which forms are stored).

Then count the changed lines of `unknown.tsv` (rules alone), `revert.tsv`, `ee_revert.tsv` and `stats.tsv`, with a few examples each.

## Report

Write the full lists to `data/snapshot/diff.md`. In the reply give the counts per group and up to 30 words per group as `form: old → new`. For a rule change, separate the words the decision predicts (its examples, or its pattern) from the others. The others are side effects: list them for Cezar, in Romanian, and don't commit until he has seen them. No change at all after a rule change means the rule touched no form in the database: say so.
