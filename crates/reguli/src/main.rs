//! `cargo run -p grafia-reguli [DIR]`: the loading checks of the rule files in DIR (by default
//! the workspace's `reguli/`). It prints each message, and, with `GITHUB_ACTIONS` set (in CI),
//! also as an annotation, which GitHub shows on that line of the pull request's files. It fails
//! if a message is an error.
use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

use grafia_reguli::Level;

/// "1 eroare", "2 erori", "20 de erori": Romanian puts "de" after numbers ending in 00 or 20 to 99.
fn count(n: usize, one: &str, many: &str) -> String {
    match n {
        1 => format!("1 {one}"),
        n if n % 100 == 0 && n > 0 || n % 100 >= 20 => format!("{n} de {many}"),
        n => format!("{n} {many}"),
    }
}

fn main() -> ExitCode {
    let dir = env::args_os()
        .nth(1)
        .map_or_else(grafia_reguli::default_dir, PathBuf::from);
    let (rules, messages) = grafia_reguli::load(&dir);
    let shown = grafia_reguli::shown(&dir);
    let github = env::var_os("GITHUB_ACTIONS").is_some();
    for m in &messages {
        println!("{m}");
        if github {
            println!("{}", m.annotation());
        }
    }
    let errors = messages.iter().filter(|m| m.level == Level::Error).count();
    let warnings = messages.len() - errors;
    let warnings = if warnings > 0 {
        format!(", {}", count(warnings, "avertisment", "avertismente"))
    } else {
        String::new()
    };
    match rules {
        Some(_) => {
            println!("{shown}/: fără erori{warnings}.");
            ExitCode::SUCCESS
        }
        None => {
            println!("{shown}/: {}{warnings}.", count(errors, "eroare", "erori"));
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::count;

    #[test]
    fn romanian_counts() {
        for (n, expected) in [
            (0, "0 erori"),
            (1, "1 eroare"),
            (2, "2 erori"),
            (19, "19 erori"),
            (20, "20 de erori"),
            (99, "99 de erori"),
            (100, "100 de erori"),
            (101, "101 erori"),
            (119, "119 erori"),
            (120, "120 de erori"),
            (1000, "1000 de erori"),
        ] {
            assert_eq!(count(n, "eroare", "erori"), expected);
        }
    }
}
