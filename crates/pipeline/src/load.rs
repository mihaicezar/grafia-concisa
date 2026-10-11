//! Loads dexonline's public database dump into a local MariaDB or MySQL server.
//!
//! Only the tables `build` reads are imported (see TABLES). Each load replaces
//! them, so loading a newer dump is how dexonline updates come in.
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::{Context, Result, ensure};
use clap::Args;
use flate2::read::MultiGzDecoder;
use ureq::tls::{RootCerts, TlsConfig, TlsProvider};

use crate::Server;

const DUMP_URL: &str = "https://dexonline.ro/static/download/dex-database.sql.gz";
/// Everything `build` reads: Lexeme, InflectedForm, Inflection, and the rest for the DEX-MDA-DOOM flag.
const TABLES: [&str; 7] = [
    "Lexeme",
    "InflectedForm",
    "Inflection",
    "EntryLexeme",
    "EntryDefinition",
    "Definition",
    "Source",
];
const TABLE_START: &[u8] = b"-- Table structure for table `";
const COUNTS: &str =
    "SELECT 'Lexeme', COUNT(*) FROM Lexeme UNION ALL SELECT 'InflectedForm', COUNT(*) FROM InflectedForm";

#[derive(Args)]
pub struct LoadArgs {
    /// Download the latest dump first
    #[arg(long)]
    download: bool,
    #[arg(long, default_value = "data/dex-database.sql.gz")]
    dump: PathBuf,
    /// The MariaDB or MySQL command-line client
    #[arg(long, default_value = "mariadb")]
    client: String,
    #[command(flatten)]
    server: Server,
}

pub fn load(args: &LoadArgs) -> Result<()> {
    if args.download {
        download(DUMP_URL, &args.dump)?;
    }
    let db = &args.server.database;
    let client = || {
        let s = &args.server;
        let mut command = Command::new(&args.client);
        command.args([
            &format!("--host={}", s.host),
            &format!("--port={}", s.port),
            &format!("--user={}", s.user),
            "--default-character-set=utf8mb4",
            "--max-allowed-packet=1G",
        ]);
        if !s.password.is_empty() {
            command.arg(format!("--password={}", s.password));
        }
        command
    };
    let create = format!("CREATE DATABASE IF NOT EXISTS `{db}` CHARACTER SET utf8mb4 COLLATE utf8mb4_romanian_ci");
    let status = client()
        .args(["-e", &create])
        .status()
        .with_context(|| format!("running {}", args.client))?;
    ensure!(status.success(), "{} failed: {status}", args.client);

    let mut child = client().arg(db).stdin(Stdio::piped()).spawn()?;
    let stdin = child.stdin.take().expect("piped stdin");
    let copied = File::open(&args.dump)
        .map_err(anyhow::Error::from)
        .and_then(|dump| copy_tables(dump, stdin));
    let status = child.wait()?;
    // a bad dump also makes the client fail, on the SQL cut short
    copied.with_context(|| format!("loading {} ({}: {status})", args.dump.display(), args.client))?;
    ensure!(status.success(), "{} failed: {status}", args.client);

    let counts = client()
        .args(["--batch", "--skip-column-names", db, "-e", COUNTS])
        .output()?;
    ensure!(counts.status.success(), "{} failed: {}", args.client, counts.status);
    print!("{}", String::from_utf8_lossy(&counts.stdout));
    Ok(())
}

/// Downloads `url` to `dump`.
fn download(url: &str, dump: &Path) -> Result<()> {
    let tls = TlsConfig::builder()
        .provider(TlsProvider::NativeTls)
        .root_certs(RootCerts::PlatformVerifier)
        .build();
    let agent = ureq::Agent::config_builder().tls_config(tls).build().new_agent();
    let response = agent.get(url).call().with_context(|| format!("downloading {url}"))?;
    let header = |name| {
        let value = response.headers().get(name).and_then(|v| v.to_str().ok());
        value.unwrap_or("?").to_string()
    };
    println!(
        "{url}: Last-Modified {}, {} bytes",
        header("Last-Modified"),
        header("Content-Length")
    );

    if let Some(dir) = dump.parent() {
        fs::create_dir_all(dir)?;
    }
    let part = dump.with_file_name(format!("{}.part", dump.file_name().context("no file name")?.display()));
    let mut file = BufWriter::new(File::create(&part)?);
    io::copy(&mut response.into_body().into_reader(), &mut file)?;
    file.into_inner()?.sync_all()?;
    fs::rename(&part, dump)?;
    Ok(())
}

/// Copies the gzipped `dump` to `out`: its header (the session settings) and the sections of TABLES.
fn copy_tables(dump: impl Read, out: impl Write) -> Result<()> {
    let mut dump = BufReader::new(MultiGzDecoder::new(dump));
    let mut out = BufWriter::new(out);
    let (mut line, mut keep) = (Vec::new(), true);
    while dump.read_until(b'\n', &mut line)? > 0 {
        if let Some(rest) = line.strip_prefix(TABLE_START) {
            let table = rest.split(|&b| b == b'`').next().unwrap_or_default();
            keep = TABLES.iter().any(|t| t.as_bytes() == table);
        }
        if keep {
            out.write_all(&line)?;
        }
        line.clear();
    }
    out.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use flate2::Compression;
    use flate2::write::GzEncoder;

    use super::*;

    #[test]
    fn copies_the_tables_read() {
        let dump = "SET NAMES utf8mb4;\n\
            -- Table structure for table `Lexeme`\nCREATE TABLE `Lexeme`;\n\
            -- Table structure for table `Meaning`\nCREATE TABLE `Meaning`;\n\
            -- Table structure for table `Source`\nCREATE TABLE `Source`;\n";
        let mut gz = GzEncoder::new(Vec::new(), Compression::fast());
        gz.write_all(dump.as_bytes()).unwrap();
        let mut out = Vec::new();
        copy_tables(&gz.finish().unwrap()[..], &mut out).unwrap();
        // the header is kept, and Meaning, which the build doesn't read, is skipped
        let expected = "SET NAMES utf8mb4;\n\
            -- Table structure for table `Lexeme`\nCREATE TABLE `Lexeme`;\n\
            -- Table structure for table `Source`\nCREATE TABLE `Source`;\n";
        assert_eq!(String::from_utf8(out).unwrap(), expected);
    }
}
