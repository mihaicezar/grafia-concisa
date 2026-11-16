//! `cargo xtask diff BASE`: what the working tree changes in the core's output on every
//! form of the database, against the commit BASE. It writes the snapshots of both
//! (`crates/core/examples/snapshot.rs`) into `data/snapshot/before` and `after`, BASE's
//! in a worktree and on BASE's export, then lists what changed, in Romanian, in
//! `data/snapshot/diff.md` (the snapshot-diff skill; CI comments it on pull requests).
use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::fmt::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

use anyhow::{Context, Result, bail, ensure};

use crate::export::{self, LOCK, Lock};

/// A file of the snapshot other than `forms.tsv`: its leading `key` columns name a row.
struct Section {
    file: &'static str,
    key: usize,
    title: &'static str,
    /// what a row holds, as `key: value`
    legend: &'static str,
}

const SECTIONS: [Section; 9] = [
    Section {
        file: "unknown.tsv",
        key: 2,
        title: "Cuvintele necunoscute, convertite doar de reguli",
        legend: "forma și comutatorul -eẹ/aẹ/oẹ (0 sau 1): grafiile",
    },
    Section {
        file: "revert.tsv",
        key: 2,
        title: "Conversia inversă",
        legend: "grafia și comutatorul -eẹ/aẹ/oẹ (0 sau 1): textul standard",
    },
    Section {
        file: "ee_revert.tsv",
        key: 1,
        title: "Inversarea regulii -eẹ/aẹ/oẹ",
        legend: "grafia cu -eẹ/aẹ/oẹ: forma standard | celelalte forme, la egalitate",
    },
    Section {
        file: "stats.tsv",
        key: 1,
        title: "Statisticile construcției",
        legend: "",
    },
    Section {
        file: "references.tsv",
        key: 3,
        title: "Silabisirile lemelor",
        legend: "lema, silabisirea din dexonline și V la verbe: silabisirile, cu sursa lor",
    },
    Section {
        file: "convert.tsv",
        key: 2,
        title: "Conversia textelor, cu comutatoarele",
        legend: "forma și comutatoarele -eẹ/aẹ/oẹ și î (0 sau 1): textul „formă Formă FORMĂ” convertit",
    },
    Section {
        file: "eu_lemmas.txt",
        key: 1,
        title: "Lemele în -eu",
        legend: "",
    },
    Section {
        file: "ea_lemmas.txt",
        key: 1,
        title: "Lemele în -ea",
        legend: "",
    },
    Section {
        file: "iesc_lemmas.tsv",
        key: 1,
        title: "Lemele în -iesc",
        legend: "lema: silabisirea",
    },
];

/// The columns of `forms.tsv` after the form, as (column, title), in the report's order.
const FORM_COLUMNS: [(usize, &str); 4] = [
    (0, "Grafia implicită"),
    (2, "Alternativele"),
    (1, "Grafia cu -eẹ/aẹ/oẹ"),
    (3, "Forma actuală a formelor învechite"),
];

const FORMS: (&str, &str) = ("formă", "forme");
const ROWS: (&str, &str) = ("rând", "rânduri");

pub fn diff(root: &Path, base: &str, max: usize) -> Result<()> {
    let commit = git(root, ["rev-parse", "--short", &format!("{base}^{{commit}}")])
        .with_context(|| format!("{base} is not a commit"))?;
    let lock = Lock::read(root)?;
    // BASE's export: the one its lock pins, or the working tree's if it has no lock
    let base_lock = match git(root, ["show", &format!("{commit}:{LOCK}")]) {
        Ok(text) => Lock::parse(&text).with_context(|| format!("{base}:{LOCK}"))?,
        Err(_) => lock.clone(),
    };
    let after_export = export::fetch(root, &lock)?;
    let before_export = match base_lock == lock {
        true => after_export.clone(),
        false => export::fetch(root, &base_lock)?,
    };

    let snapshot = root.join("data").join("snapshot");
    let (before, after) = (snapshot.join("before"), snapshot.join("after"));
    for dir in [&before, &after] {
        if dir.exists() {
            fs::remove_dir_all(dir).with_context(|| format!("removing {}", dir.display()))?;
        }
    }
    let worktree = root.join("target").join("diff-base");
    let _ = git(
        root,
        [
            OsStr::new("worktree"),
            "remove".as_ref(),
            "--force".as_ref(),
            worktree.as_os_str(),
        ],
    );
    git(root, ["worktree", "prune"])?;
    git(
        root,
        [
            OsStr::new("worktree"),
            "add".as_ref(),
            "--detach".as_ref(),
            worktree.as_os_str(),
            commit.as_ref(),
        ],
    )?;
    let written = write_snapshot(root, &worktree, &before_export, &before);
    git(
        root,
        [
            OsStr::new("worktree"),
            "remove".as_ref(),
            "--force".as_ref(),
            worktree.as_os_str(),
        ],
    )?;
    written.with_context(|| format!("the snapshot of {base}"))?;
    write_snapshot(root, root, &after_export, &after).context("the snapshot of the working tree")?;

    let (body, counts) = report(&before, &after, max)?;
    let exports = match base_lock == lock {
        true => format!("pe exportul dexonline `{}`", lock.name()?),
        false => format!("pe exporturile dexonline `{}` → `{}`", base_lock.name()?, lock.name()?),
    };
    let mut text = format!(
        "## Ce se schimbă\n\nGrafiile tuturor formelor din baza de date, față de `{base}` (`{commit}`), {exports}.\n\n"
    );
    if counts.is_empty() {
        text += "**Nicio schimbare**: fiecare formă are aceleași grafii, și nimic altceva nu diferă.\n";
    }
    text += &body;
    let path = snapshot.join("diff.md");
    fs::write(&path, text).with_context(|| format!("writing {}", path.display()))?;

    for (title, count) in &counts {
        println!("{count:>8}  {title}");
    }
    if counts.is_empty() {
        println!("no change");
    }
    println!("{}", path.display());
    Ok(())
}

/// Runs the snapshot example of the workspace at `tree` on `export`, into `out`. The
/// target directory is shared, so BASE's build reuses the dependencies.
fn write_snapshot(root: &Path, tree: &Path, export: &Path, out: &Path) -> Result<()> {
    let cargo = env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let target = env::var_os("CARGO_TARGET_DIR").map_or_else(|| root.join("target"), PathBuf::from);
    let status = Command::new(cargo)
        .current_dir(root)
        .env("CARGO_TARGET_DIR", target)
        .args(["run", "--release", "--manifest-path"])
        .arg(tree.join("Cargo.toml"))
        .args(["-p", "grafia-concisa", "--example", "snapshot", "--"])
        .args([export, out])
        .status()
        .context("running cargo")?;
    ensure!(status.success(), "the snapshot example failed");
    Ok(())
}

/// Runs git in `root`; returns its output, trimmed.
fn git<I: IntoIterator<Item = S>, S: AsRef<OsStr>>(root: &Path, args: I) -> Result<String> {
    let output = Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .context("running git")?;
    if !output.status.success() {
        bail!("git: {}", String::from_utf8_lossy(&output.stderr).trim());
    }
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

/// The report's sections, and each changed one's title and number of rows.
fn report(before: &Path, after: &Path, max: usize) -> Result<(String, Vec<(String, usize)>)> {
    let (mut out, mut counts) = (String::new(), Vec::new());
    let mut section = |title: &str, legend: &str, unit, rows: Vec<String>| {
        if !rows.is_empty() {
            render(&mut out, title, legend, unit, &rows, max);
            counts.push((title.to_string(), rows.len()));
        }
    };

    let (old, new) = (rows(&before.join("forms.tsv"), 1)?, rows(&after.join("forms.tsv"), 1)?);
    let column = |row: &[String], i: usize| row.get(i).cloned().unwrap_or_default();
    for (i, title) in FORM_COLUMNS {
        let changed = old.iter().filter_map(|(form, row)| {
            let (a, b) = (column(row, i), column(new.get(form)?, i));
            (a != b).then(|| format!("- {}: {} → {}", code(form), code(&a), code(&b)))
        });
        section(title, "", FORMS, changed.collect());
    }
    let added = new.iter().filter(|(form, _)| !old.contains_key(*form));
    let added = added.map(|(form, row)| format!("- + {}: {}", code(form), code(&column(row, 0))));
    let removed = old.iter().filter(|(form, _)| !new.contains_key(*form));
    let removed = removed.map(|(form, row)| format!("- − {}: {}", code(form), code(&column(row, 0))));
    section("Forme adăugate sau scoase", "", FORMS, added.chain(removed).collect());

    for s in SECTIONS {
        let (old, new) = (rows(&before.join(s.file), s.key)?, rows(&after.join(s.file), s.key)?);
        let value = |row: &[String]| row.join(" | ");
        let line = |sign: &str, key: &str, row: &[String]| match row.is_empty() {
            true => format!("- {sign}{}", code(key)),
            false => format!("- {sign}{}: {}", code(key), code(&value(row))),
        };
        let mut lines = Vec::new();
        for (key, a) in &old {
            match new.get(key) {
                Some(b) if a != b => lines.push(format!("- {}: {} → {}", code(key), code(&value(a)), code(&value(b)))),
                Some(_) => {}
                None => lines.push(line("− ", key, a)),
            }
        }
        lines.extend(
            new.iter()
                .filter(|(key, _)| !old.contains_key(*key))
                .map(|(key, b)| line("+ ", key, b)),
        );
        section(&format!("{} (`{}`)", s.title, s.file), s.legend, ROWS, lines);
    }
    Ok((out, counts))
}

/// The rows of a snapshot file, by their first `key` columns (the non-empty ones, joined
/// by spaces); a missing file has none.
fn rows(path: &Path, key: usize) -> Result<BTreeMap<String, Vec<String>>> {
    if !path.exists() {
        return Ok(BTreeMap::new());
    }
    let text = fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let row = |line: &str| {
        let columns: Vec<String> = line.split('\t').map(String::from).collect();
        let (key, value) = columns.split_at(key.min(columns.len()));
        let key: Vec<&str> = key.iter().map(String::as_str).filter(|c| !c.is_empty()).collect();
        (key.join(" "), value.to_vec())
    };
    Ok(text.lines().filter(|line| !line.is_empty()).map(row).collect())
}

/// A section: the title and the number of rows, which open it when there are few, then the rows.
fn render(out: &mut String, title: &str, legend: &str, unit: (&str, &str), rows: &[String], max: usize) {
    let open = if rows.len() <= 10 { " open" } else { "" };
    let _ = writeln!(
        out,
        "<details{open}><summary><b>{title}</b>: {}</summary>\n",
        count(rows.len(), unit)
    );
    if !legend.is_empty() {
        let _ = writeln!(out, "{legend}\n");
    }
    for row in rows.iter().take(max) {
        let _ = writeln!(out, "{row}");
    }
    if rows.len() > max {
        let _ = writeln!(out, "- … și încă {}", rows.len() - max);
    }
    let _ = writeln!(out, "\n</details>\n");
}

/// A number of things, in Romanian: 1 formă, 3 forme, 20 de forme, 101 forme.
fn count(n: usize, (one, many): (&str, &str)) -> String {
    match n % 100 {
        _ if n == 1 => format!("1 {one}"),
        1..=19 => format!("{n} {many}"),
        _ => format!("{n} de {many}"),
    }
}

/// A value as inline code, or a dash if it is empty.
fn code(value: &str) -> String {
    match value.is_empty() {
        true => "—".into(),
        false => format!("`{value}`"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts() {
        let counts = [1, 2, 19, 20, 100, 101, 119, 120].map(|n| count(n, FORMS));
        assert_eq!(
            counts,
            [
                "1 formă",
                "2 forme",
                "19 forme",
                "20 de forme",
                "100 de forme",
                "101 forme",
                "119 forme",
                "120 de forme"
            ]
        );
    }

    #[test]
    fn reports() {
        let dir = env::temp_dir().join(format!("xtask-diff-{}", std::process::id()));
        let (before, after) = (dir.join("before"), dir.join("after"));
        for (path, text) in [
            (
                before.join("forms.tsv"),
                "ceai\tcēi\t\t\nfier\tfẹr\t\t\nvechi\tvechi\t\t\n",
            ),
            (
                after.join("forms.tsv"),
                "ceai\tcēi\t\t\nfier\tfier\t\tfẹr\nnou\tnou\t\t\n",
            ),
            (before.join("stats.tsv"), "stored\t3\n"),
            (after.join("stats.tsv"), "stored\t3\n"),
            (before.join("references.tsv"), "neant\t\t\tne-ant/manual\n"),
            (after.join("references.tsv"), "neant\t\t\tneant/rules\n"),
            (before.join("ea_lemmas.txt"), "mea\n"),
        ] {
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, text).unwrap();
        }
        let (text, counts) = report(&before, &after, 1).unwrap();
        fs::remove_dir_all(&dir).unwrap();
        let counts: Vec<(&str, usize)> = counts.iter().map(|(t, n)| (t.as_str(), *n)).collect();
        assert_eq!(
            counts,
            [
                ("Grafia implicită", 1),
                ("Alternativele", 1),
                ("Forme adăugate sau scoase", 2),
                ("Silabisirile lemelor (`references.tsv`)", 1),
                ("Lemele în -ea (`ea_lemmas.txt`)", 1)
            ]
        );
        // the key leaves out its empty columns (no hyphenation, not a verb)
        assert!(text.contains("- `neant`: `ne-ant/manual` → `neant/rules`\n"));
        assert!(text.contains(
            "<details open><summary><b>Grafia implicită</b>: 1 formă</summary>\n\n- `fier`: `fẹr` → `fier`\n"
        ));
        assert!(text.contains("- `fier`: — → `fẹr`\n"));
        // at most one row: the other is counted
        assert!(text.contains("- + `nou`: `nou`\n- … și încă 1\n"));
        assert!(text.contains("- − `mea`\n"));
    }
}
