//! The core's build script (`src/build.rs` is the data file's builder, not this): it compiles
//! the rule files of `reguli/` into `$OUT_DIR/reguli.rs`, which `src/rules/lists.rs` includes
//! and `RULES_HASH` hashes. A rule file with an error stops the build, with its messages
//! (`cargo run -p grafia-reguli` prints them too, and in CI on the lines of the pull request).
#![allow(clippy::disallowed_methods)] // a build script writes a file; the core lib may not
use std::path::Path;
use std::{env, fs, process};

fn main() {
    let dir = grafia_reguli::default_dir();
    println!("cargo::rerun-if-changed={}", dir.display());
    let (rules, messages) = grafia_reguli::load(&dir);
    let Some(rules) = rules else {
        for m in &messages {
            eprintln!("{m}");
        }
        process::exit(1);
    };
    for m in &messages {
        println!("cargo::warning={m}");
    }
    let out = Path::new(&env::var_os("OUT_DIR").expect("cargo sets OUT_DIR")).join("reguli.rs");
    fs::write(out, grafia_reguli::rust(&rules)).expect("write reguli.rs");
}
