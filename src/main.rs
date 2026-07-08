//! rust-installer — a bespoke, dependency-free installer written in pure Rust.
//!
//! Design goal: implement everything (argument parsing, filesystem staging,
//! progress reporting, rollback) using ONLY the Rust standard library — no crates.
//! This file is an intentionally minimal, honest starting scaffold.

use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("install") => {
            let dest = args.get(2).map(PathBuf::from).unwrap_or_else(|| PathBuf::from("./install"));
            match install(&dest) {
                Ok(()) => {
                    println!("Install complete -> {}", dest.display());
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("Install failed: {e}");
                    ExitCode::FAILURE
                }
            }
        }
        Some("--version") | Some("-V") => {
            println!("rust-installer {VERSION}");
            ExitCode::SUCCESS
        }
        _ => {
            print_usage();
            ExitCode::SUCCESS
        }
    }
}

fn print_usage() {
    println!(
        "rust-installer {VERSION}\n\nUSAGE:\n  rust-installer install [DEST]   Stage the payload into DEST (default ./install)\n  rust-installer --version        Print version\n"
    );
}

/// Create the destination directory tree. Real payload logic goes here.
fn install(dest: &Path) -> io::Result<()> {
    fs::create_dir_all(dest)?;
    let manifest = dest.join("INSTALL_MANIFEST.txt");
    let mut f = fs::File::create(&manifest)?;
    writeln!(f, "rust-installer {VERSION}")?;
    writeln!(f, "installed_to = {}", dest.display())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn install_creates_manifest() {
        let tmp = env::temp_dir().join(format!("ri_test_{}", std::process::id()));
        install(&tmp).expect("install should succeed");
        assert!(tmp.join("INSTALL_MANIFEST.txt").exists());
        let _ = fs::remove_dir_all(&tmp);
    }
}
