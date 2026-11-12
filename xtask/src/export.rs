//! The pinned export of dexonline, which `grafia.dat` is built from: `dexonline.lock`
//! names an archive of the files `grafia-pipeline export` writes, published on a release
//! of this repository, and its SHA-256 (README.md, *The data file*).
use std::env;
use std::fs::{self, File};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, bail, ensure};
use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use sha2::{Digest, Sha256};
use ureq::tls::{RootCerts, TlsConfig, TlsProvider};

pub const LOCK: &str = "dexonline.lock";
/// Where the archives are published: `RELEASES/<tag>/<tag>.tar.gz`.
const RELEASES: &str = "https://github.com/mihaicezar/grafia-concisa/releases/download";
/// The files of an export, in the archive's order.
const FILES: [&str; 3] = ["date.txt", "known.txt", "records.tsv"];

/// The export that `dexonline.lock` pins.
#[derive(Clone, Debug, PartialEq)]
pub struct Lock {
    /// The archive's URL, ending in `dexonline-<date>.tar.gz`.
    pub url: String,
    /// Its SHA-256, in lowercase hexadecimal.
    pub sha256: String,
}

impl Lock {
    /// Reads `key = value` lines, `url` and `sha256`; empty lines and # comments are skipped.
    pub fn parse(text: &str) -> Result<Lock> {
        let (mut url, mut sha256) = (None, None);
        for (i, line) in text.lines().enumerate().map(|(i, line)| (i + 1, line.trim())) {
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let Some((key, value)) = line.split_once('=') else {
                bail!("{LOCK}:{i}: not a `key = value` line");
            };
            let value = Some(value.trim().to_string());
            match key.trim() {
                "url" => url = value,
                "sha256" => sha256 = value,
                key => bail!("{LOCK}:{i}: unknown key `{key}`"),
            }
        }
        let lock = Lock {
            url: url.with_context(|| format!("{LOCK}: no url"))?,
            sha256: sha256.with_context(|| format!("{LOCK}: no sha256"))?,
        };
        let hex = |s: &str| s.len() == 64 && s.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b));
        ensure!(
            hex(&lock.sha256),
            "{LOCK}: the sha256 is not 64 lowercase hexadecimal digits"
        );
        lock.name()?;
        Ok(lock)
    }

    /// The working tree's lock.
    pub fn read(root: &Path) -> Result<Lock> {
        let path = root.join(LOCK);
        Lock::parse(&fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?)
    }

    /// The archive's name, `dexonline-<date>`, without `.tar.gz`.
    pub fn name(&self) -> Result<&str> {
        let file = self.url.rsplit('/').next().unwrap_or_default();
        let name = file.strip_suffix(".tar.gz").unwrap_or_default();
        ensure!(
            name.starts_with("dexonline-") && !name.contains(['\\', ':']),
            "{LOCK}: the url must end in dexonline-<date>.tar.gz: {}",
            self.url
        );
        Ok(name)
    }

    fn text(&self) -> String {
        format!(
            "# The export of dexonline that grafia.dat is built from (README.md, *The data file*):\n\
             # an asset of a release of this repository, and its SHA-256. `cargo xtask pin` writes this file.\n\
             url = {}\nsha256 = {}\n",
            self.url, self.sha256
        )
    }
}

/// `cargo xtask data`: builds `web/grafia.dat` from the pinned export.
pub fn data(root: &Path) -> Result<()> {
    let dir = fetch(root, &Lock::read(root)?)?;
    let path = |file: &str| dir.join(file).into_os_string();
    let out = root.join("web").join("grafia.dat");
    let cargo = env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let status = Command::new(cargo)
        .current_dir(root)
        .args(["run", "--release", "-p", "grafia-pipeline", "--", "build"])
        .arg("--text")
        .arg(path("records.tsv"))
        .arg("--known")
        .arg(path("known.txt"))
        .arg("--date")
        .arg(path("date.txt"))
        .arg("--out")
        .arg(&out)
        .status()
        .context("running cargo")?;
    ensure!(status.success(), "building web/grafia.dat failed");
    // the same export and rules give the same file anywhere: this compares it with CI's
    println!("web/grafia.dat: sha256 {}", sha256(&mut File::open(&out)?)?);
    Ok(())
}

/// The export `lock` pins, unpacked into `data/<name>/`: the archive is downloaded into
/// `data/` unless it is there already with its checksum. Returns the directory.
pub fn fetch(root: &Path, lock: &Lock) -> Result<PathBuf> {
    let name = lock.name()?;
    let data = root.join("data");
    let archive = data.join(format!("{name}.tar.gz"));
    if !(archive.exists() && sha256(&mut File::open(&archive)?)? == lock.sha256) {
        download(lock, &archive)?;
    }
    let dir = data.join(name);
    unpack(&archive, &dir)?;
    Ok(dir)
}

/// Downloads the archive to `to`, checking its SHA-256.
fn download(lock: &Lock, to: &Path) -> Result<()> {
    let tls = TlsConfig::builder()
        .provider(TlsProvider::NativeTls)
        .root_certs(RootCerts::PlatformVerifier)
        .build();
    let agent = ureq::Agent::config_builder().tls_config(tls).build().new_agent();
    println!("downloading {}", lock.url);
    let response = agent
        .get(&lock.url)
        .call()
        .with_context(|| format!("downloading {}", lock.url))?;
    fs::create_dir_all(to.parent().context("no data directory")?)?;
    let part = to.with_extension("part");
    let mut file = BufWriter::new(File::create(&part).with_context(|| format!("writing {}", part.display()))?);
    io::copy(&mut response.into_body().into_reader(), &mut file)?;
    file.into_inner()?.sync_all()?;
    let sha256 = sha256(&mut File::open(&part)?)?;
    if sha256 != lock.sha256 {
        let _ = fs::remove_file(&part);
        bail!(
            "{}: its SHA-256 is {sha256}, not the {} of {LOCK}",
            lock.url,
            lock.sha256
        );
    }
    fs::rename(&part, to).with_context(|| format!("writing {}", to.display()))?;
    Ok(())
}

/// Unpacks the files of an export from `archive` into `dir`; any other entry is an error.
fn unpack(archive: &Path, dir: &Path) -> Result<()> {
    fs::create_dir_all(dir).with_context(|| format!("creating {}", dir.display()))?;
    let file = File::open(archive).with_context(|| format!("reading {}", archive.display()))?;
    let mut tar = tar::Archive::new(GzDecoder::new(BufReader::new(file)));
    let mut found = Vec::new();
    for entry in tar.entries()? {
        let mut entry = entry?;
        let path = entry.path()?.into_owned();
        let Some(name) = path.to_str().filter(|name| FILES.contains(name)) else {
            bail!("{}: unexpected entry {}", archive.display(), path.display());
        };
        let mut out = BufWriter::new(File::create(dir.join(name))?);
        io::copy(&mut entry, &mut out)?;
        out.flush()?;
        found.push(name.to_string());
    }
    found.sort();
    ensure!(found == FILES, "{}: holds {found:?}, not {FILES:?}", archive.display());
    Ok(())
}

/// `cargo xtask pin DIR`: packs the export in `DIR` into `data/dexonline-<date>.tar.gz`,
/// pins it in `dexonline.lock`, and writes the release's notes into `data/dexonline-<date>.md`.
/// The archive holds only the files' bytes, so the same export always gives the same archive.
pub fn pin(root: &Path, dir: &Path) -> Result<()> {
    let date = fs::read_to_string(dir.join("date.txt")).with_context(|| format!("reading {}", dir.display()))?;
    let date = date.trim();
    ensure!(!date.is_empty(), "{}: no date", dir.join("date.txt").display());
    let name = format!("dexonline-{date}");
    let lock = Lock {
        url: format!("{RELEASES}/{name}/{name}.tar.gz"),
        sha256: String::new(),
    };
    lock.name()?;

    let archive = root.join("data").join(format!("{name}.tar.gz"));
    fs::create_dir_all(archive.parent().context("no data directory")?)?;
    let mut gz = GzEncoder::new(BufWriter::new(File::create(&archive)?), Compression::best());
    {
        let mut tar = tar::Builder::new(&mut gz);
        for file in FILES {
            let bytes = fs::read(dir.join(file)).with_context(|| format!("reading {}", dir.join(file).display()))?;
            let mut header = tar::Header::new_gnu();
            header.set_size(bytes.len() as u64);
            header.set_mode(0o644);
            header.set_mtime(0);
            tar.append_data(&mut header, file, &bytes[..])?;
        }
        tar.finish()?;
    }
    gz.finish()?.into_inner()?.sync_all()?;

    let lock = Lock {
        sha256: sha256(&mut File::open(&archive)?)?,
        ..lock
    };
    fs::write(root.join(LOCK), lock.text()).with_context(|| format!("writing {LOCK}"))?;
    fs::write(root.join("data").join(format!("{name}.md")), notes(date))?;
    let size = fs::metadata(&archive)?.len();
    println!("{}: {size} bytes, sha256 {}", archive.display(), lock.sha256);
    println!("{LOCK}: url = {}", lock.url);
    println!("Next, publish it as the release {name} (README.md, *Updating dexonline*):");
    println!("gh release create {name} data/{name}.tar.gz --title \"dexonline {date}\" --notes-file data/{name}.md");
    Ok(())
}

/// The notes of the release of the export of `date`, with dexonline's attribution.
fn notes(date: &str) -> String {
    let year = date.split('-').next().unwrap_or_default();
    format!(
        "The records of [dexonline](https://dexonline.ro)'s database, data of {date}, as `grafia-pipeline export` \
         writes them: `records.tsv` (the inflected forms), `known.txt` (the words the prefix rule checks against) \
         and `date.txt` (the date of the data). `cargo xtask data` builds `grafia.dat` from them.\n\
         \n\
         The data is dexonline's (© 2004-{year} dexonline, https://dexonline.ro), under the GNU General Public \
         License, version 2 or later.\n"
    )
}

/// The SHA-256 of what `from` reads, in lowercase hexadecimal.
fn sha256(from: &mut impl Read) -> Result<String> {
    let mut hash = Sha256::new();
    io::copy(from, &mut hash)?;
    Ok(hash.finalize().iter().map(|b| format!("{b:02x}")).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SHA: &str = "13a6afe8550fbb50e7995fd08060a28370ad09610bd681ca13c40784d2f5019e";

    #[test]
    fn locks() {
        let url = format!("{RELEASES}/dexonline-2026-11-11/dexonline-2026-11-11.tar.gz");
        let lock = Lock {
            url,
            sha256: SHA.into(),
        };
        assert_eq!(Lock::parse(&lock.text()).unwrap(), lock);
        assert_eq!(lock.name().unwrap(), "dexonline-2026-11-11");
        for (text, error) in [
            ("url = x/dexonline-1.tar.gz", "dexonline.lock: no sha256"),
            ("url x", "dexonline.lock:1: not a `key = value` line"),
            ("\nsize = 3", "dexonline.lock:2: unknown key `size`"),
            (
                "url = x/dexonline-1.tar.gz\nsha256 = ABC",
                "dexonline.lock: the sha256 is not 64 lowercase hexadecimal digits",
            ),
        ] {
            assert_eq!(Lock::parse(text).unwrap_err().to_string(), error, "{text}");
        }
        let bad = Lock {
            url: "https://x/records.zip".into(),
            sha256: SHA.into(),
        };
        assert!(Lock::parse(&bad.text()).is_err());
    }

    #[test]
    fn notes() {
        let notes = super::notes("2026-11-11");
        assert!(notes.contains("data of 2026-11-11,"), "{notes}");
        assert!(notes.contains("© 2004-2026 dexonline"), "{notes}");
    }

    #[test]
    fn hashes() {
        let sha = sha256(&mut &b"abc"[..]).unwrap();
        assert_eq!(sha, "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
    }
}
