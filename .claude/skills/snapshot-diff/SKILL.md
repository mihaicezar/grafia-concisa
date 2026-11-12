---
name: snapshot-diff
description: Use after a rule change or a dexonline update to list every word whose output changed, across all forms in the database, for Cezar to review. Also when asked "what did this change affect?".
---

# Snapshot diff

The test tables check chosen words. This checks every form in the database: it writes the output of the core for all forms, before and after, and lists what changed. Cezar reviews the words, not the code.

## Run it

```
cargo xtask diff <base commit>
```

`<base commit>` is `main` for a branch, or the commit before the change. No MariaDB is needed:
- the records come from the export of dexonline that `dexonline.lock` pins; `cargo xtask diff` fetches it into `data/` (README.md, *The data file*);
- it writes the snapshot of the base commit, in a worktree, and of the working tree, uncommitted changes included, into `data/snapshot/before` and `data/snapshot/after`, with `crates/core/examples/snapshot.rs` (its header documents the files);
- when `dexonline.lock` changed (a dexonline update), each side uses its own export;
- the report, in Romanian, is `data/snapshot/diff.md`.

A `forms.tsv` row is `form, concise, concise_ee, alternatives, current`. `diff.md` sorts the changed rows into:
1. default spelling changed (column 2);
2. alternatives added or removed (column 4);
3. -eẹ/aẹ/oẹ spelling changed (column 3);
4. current form of an obsolete form changed (column 5);
5. forms added or removed (a dexonline update, or a change to which forms are stored).

Then come the changed rows of the other files: `unknown.tsv` (rules alone), `revert.tsv`, `ee_revert.tsv`, `stats.tsv`, `references.tsv`, `convert.tsv` and the lemma lists.

`data/` is ignored by git; nothing here is committed. On a pull request, CI runs the same command against the base branch and comments `diff.md`.

## Report

In the reply give the counts per group and up to 30 words per group as `form: old → new`. For a rule change, separate the words the decision predicts (its examples, or its pattern) from the others. The others are side effects: list them for Cezar, in Romanian, and don't commit until he has seen them. No change at all after a rule change means the rule touched no form in the database: say so.
