//! Project tasks, run from anywhere in the repository as `cargo xtask <task>`
//! (the alias is in `.cargo/config.toml`):
//!   - `web`: builds the web app into `web/dist/` (see README.md, *How the web app works*).
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::{env, fs};

use anyhow::{Context, Result, bail, ensure};
use flate2::Compression;
use flate2::write::GzEncoder;
use grafia_concisa::DataFile;
use wasm_bindgen_cli_support::Bindgen;

fn main() -> Result<()> {
    match env::args().nth(1).as_deref() {
        Some("web") => web(),
        _ => bail!("usage: cargo xtask web"),
    }
}

// The lines of web/sw.js that the build fills in.
const VERSION: &str = r#"const VERSION = "dev";"#;
const FILES: &str = "const FILES = [];";

/// Builds `web/dist/`: the core in WASM with its JavaScript, the files of `web/`,
/// the data file gzipped, and the service worker, which caches all of them under
/// a name that changes with any of them.
fn web() -> Result<()> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .context("no workspace root")?;
    let web = root.join("web");
    let dist = web.join("dist");

    // The data file first: CI can't rebuild it, so one that doesn't open (an old format) stops the build.
    let data = read(&web.join("grafia.dat"))?;
    let date = DataFile::new(&data[..])
        .context("web/grafia.dat: rebuild it with the pipeline (README.md, *Updating the data*)")?
        .date()
        .to_string();

    // cargo's JSON messages on stdout give the path of the .wasm, wherever the target directory is
    let cargo = env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let output = Command::new(cargo)
        .current_dir(root)
        .args(["build", "-p", "grafia-wasm", "--target", "wasm32-unknown-unknown"])
        .args(["--profile", "wasm", "--message-format=json-render-diagnostics"])
        .stderr(Stdio::inherit())
        .output()
        .context("running cargo")?;
    ensure!(output.status.success(), "building grafia-wasm failed");
    let wasm = wasm_file(&output.stdout)?;

    if dist.exists() {
        fs::remove_dir_all(&dist).with_context(|| format!("removing {}", dist.display()))?;
    }
    fs::create_dir_all(&dist)?;
    Bindgen::new()
        .input_path(&wasm)
        .web(true)?
        .typescript(false)
        .out_name("grafia")
        .omit_default_module_path(false) // init() fetches grafia_bg.wasm (the CLI's default, not the library's)
        .remove_name_section(true)
        .remove_producers_section(true)
        .generate(&dist)?;

    let skipped = [dist.clone(), web.join("grafia.dat"), web.join("sw.js")];
    copy_dir(&web, &dist, &skipped)?;
    let mut gz = GzEncoder::new(Vec::new(), Compression::best());
    gz.write_all(&data)?;
    fs::write(dist.join("grafia.dat.gz"), gz.finish()?)?;

    let files = files(&dist)?;
    let mut hash = crc32fast::Hasher::new();
    for file in &files {
        hash.update(file.as_bytes());
        hash.update(&read(&dist.join(file))?);
    }
    let version = format!("{:08x}", hash.finalize());
    let sw = fs::read_to_string(web.join("sw.js")).context("reading web/sw.js")?;
    ensure!(
        sw.contains(VERSION) && sw.contains(FILES),
        "web/sw.js must contain the lines `{VERSION}` and `{FILES}`"
    );
    let list: Vec<String> = std::iter::once("./")
        .chain(files.iter().map(String::as_str))
        .map(|f| format!("\"{f}\""))
        .collect();
    let sw = sw
        .replacen(VERSION, &format!(r#"const VERSION = "{version}";"#), 1)
        .replacen(FILES, &format!("const FILES = [{}];", list.join(", ")), 1);
    fs::write(dist.join("sw.js"), sw)?;

    for file in files.iter().chain([&"sw.js".to_string()]) {
        let size = fs::metadata(dist.join(file))?.len();
        println!("{size:>9}  {file}");
    }
    let date = if date.is_empty() { "no date" } else { &date };
    println!("{}: version {version}, dexonline data of {date}", dist.display());
    Ok(())
}

/// The path of `grafia_wasm.wasm`, from the JSON messages of `cargo build`.
fn wasm_file(messages: &[u8]) -> Result<PathBuf> {
    messages
        .split(|&b| b == b'\n')
        .filter_map(|line| serde_json::from_slice::<serde_json::Value>(line).ok())
        .filter(|message| message["reason"] == "compiler-artifact")
        .flat_map(|message| message["filenames"].as_array().cloned().unwrap_or_default())
        .filter_map(|file| file.as_str().map(PathBuf::from))
        .find(|file| file.file_name().is_some_and(|name| name == "grafia_wasm.wasm"))
        .context("cargo built no grafia_wasm.wasm")
}

fn read(path: &Path) -> Result<Vec<u8>> {
    fs::read(path).with_context(|| format!("reading {}", path.display()))
}

/// Copies the files under `from` to `to`, except the paths in `skipped`.
fn copy_dir(from: &Path, to: &Path, skipped: &[PathBuf]) -> Result<()> {
    for entry in fs::read_dir(from).with_context(|| format!("reading {}", from.display()))? {
        let path = entry?.path();
        if skipped.contains(&path) {
            continue;
        }
        let dest = to.join(path.file_name().context("no file name")?);
        if path.is_dir() {
            fs::create_dir_all(&dest)?;
            copy_dir(&path, &dest, skipped)?;
        } else {
            fs::copy(&path, &dest).with_context(|| format!("copying {}", path.display()))?;
        }
    }
    Ok(())
}

/// The files under `dir`, as sorted relative URLs.
fn files(dir: &Path) -> Result<Vec<String>> {
    let mut out = Vec::new();
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        let name = path.file_name().context("no file name")?.to_string_lossy().into_owned();
        if path.is_dir() {
            out.extend(files(&path)?.into_iter().map(|f| format!("{name}/{f}")));
        } else {
            out.push(name);
        }
    }
    out.sort();
    Ok(out)
}
