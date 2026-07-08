//! rust-installer — a bespoke, dependency-free installer written in pure Rust.
//!
//! Design goal: implement everything (argument parsing, recursive payload
//! staging, a manifest, verification, uninstall, and rollback-on-failure)
//! using ONLY the Rust standard library — no crates.
//!
//! Commands:
//!   install <SRC> <DEST>   copy payload SRC into DEST, recording a manifest
//!   verify  <DEST>         check every manifested file is present
//!   uninstall <DEST>       remove everything the manifest installed
//!   --version

use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const MANIFEST: &str = ".install-manifest";

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let result = match args.get(1).map(String::as_str) {
        Some("install") => cmd_install(&args),
        Some("verify") => cmd_verify(&args),
        Some("uninstall") => cmd_uninstall(&args),
        Some("--version") | Some("-V") => {
            println!("rust-installer {VERSION}");
            return ExitCode::SUCCESS;
        }
        _ => {
            print_usage();
            return ExitCode::SUCCESS;
        }
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn print_usage() {
    println!(
        "rust-installer {VERSION}\n\n\
         USAGE:\n\
         \x20 rust-installer install <SRC> <DEST>   Copy payload SRC into DEST, write a manifest\n\
         \x20 rust-installer verify <DEST>          Check every manifested file is present\n\
         \x20 rust-installer uninstall <DEST>       Remove everything the manifest installed\n\
         \x20 rust-installer --version"
    );
}

fn arg(args: &[String], i: usize, what: &str) -> io::Result<PathBuf> {
    args.get(i).map(PathBuf::from).ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, format!("missing argument: {what}"))
    })
}

fn cmd_install(args: &[String]) -> io::Result<()> {
    let src = arg(args, 2, "<SRC>")?;
    let dest = arg(args, 3, "<DEST>")?;
    if !src.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("source payload not found (need a directory): {}", src.display()),
        ));
    }
    fs::create_dir_all(&dest)?;

    // Track every file we create so we can undo a partial install.
    let mut installed: Vec<PathBuf> = Vec::new();
    match copy_dir(&src, &src, &dest, &mut installed) {
        Ok(()) => {
            let mut mf = fs::File::create(dest.join(MANIFEST))?;
            writeln!(mf, "# rust-installer {VERSION} manifest")?;
            for rel in &installed {
                writeln!(mf, "{}", rel.display())?;
            }
            println!("\nInstalled {} file(s) into {}", installed.len(), dest.display());
            Ok(())
        }
        Err(e) => {
            eprintln!("install failed ({e}); rolling back {} file(s)...", installed.len());
            rollback(&dest, &installed);
            Err(e)
        }
    }
}

/// Recursively copy the contents of `dir` (rooted at `root`) into `dest`,
/// recording each copied file's path relative to `root`.
fn copy_dir(root: &Path, dir: &Path, dest: &Path, installed: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let rel = path
            .strip_prefix(root)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "path outside payload root"))?
            .to_path_buf();
        if path.is_dir() {
            fs::create_dir_all(dest.join(&rel))?;
            copy_dir(root, &path, dest, installed)?;
        } else if path.is_file() {
            let target = dest.join(&rel);
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(&path, &target)?;
            println!("  + {}", rel.display());
            installed.push(rel);
        }
    }
    Ok(())
}

/// Undo a (partial) install: remove copied files, then prune empty dirs.
fn rollback(dest: &Path, installed: &[PathBuf]) {
    for rel in installed.iter().rev() {
        let _ = fs::remove_file(dest.join(rel));
    }
    prune_empty_dirs(dest);
}

fn prune_empty_dirs(dir: &Path) {
    if let Ok(entries) = fs::read_dir(dir) {
        for e in entries.flatten() {
            let p = e.path();
            if p.is_dir() {
                prune_empty_dirs(&p);
                let _ = fs::remove_dir(&p); // only succeeds if now empty
            }
        }
    }
}

fn read_manifest(dest: &Path) -> io::Result<Vec<PathBuf>> {
    let f = fs::File::open(dest.join(MANIFEST)).map_err(|_| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("no manifest in {} — was it installed here?", dest.display()),
        )
    })?;
    let mut out = Vec::new();
    for line in io::BufReader::new(f).lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        out.push(PathBuf::from(line));
    }
    Ok(out)
}

fn cmd_verify(args: &[String]) -> io::Result<()> {
    let dest = arg(args, 2, "<DEST>")?;
    let files = read_manifest(&dest)?;
    let mut missing = 0usize;
    for rel in &files {
        let ok = dest.join(rel).is_file();
        println!("  {} {}", if ok { "ok     " } else { "MISSING" }, rel.display());
        if !ok {
            missing += 1;
        }
    }
    if missing == 0 {
        println!("verify: all {} file(s) present", files.len());
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, format!("{missing} file(s) missing")))
    }
}

fn cmd_uninstall(args: &[String]) -> io::Result<()> {
    let dest = arg(args, 2, "<DEST>")?;
    let files = read_manifest(&dest)?;
    for rel in &files {
        match fs::remove_file(dest.join(rel)) {
            Ok(()) => println!("  - {}", rel.display()),
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(e) => return Err(e),
        }
    }
    let _ = fs::remove_file(dest.join(MANIFEST));
    prune_empty_dirs(&dest);
    let _ = fs::remove_dir(&dest); // remove DEST itself if now empty
    println!("uninstalled {} file(s) from {}", files.len(), dest.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp(tag: &str) -> PathBuf {
        env::temp_dir().join(format!("ri_{}_{}", tag, std::process::id()))
    }

    #[test]
    fn install_verify_uninstall_roundtrip() {
        let base = tmp("rt");
        let src = base.join("payload");
        let dest = base.join("out");
        fs::create_dir_all(src.join("sub")).unwrap();
        fs::write(src.join("a.txt"), b"hello").unwrap();
        fs::write(src.join("sub").join("b.txt"), b"world").unwrap();

        let mut installed = Vec::new();
        copy_dir(&src, &src, &dest, &mut installed).unwrap();
        assert_eq!(installed.len(), 2);
        assert!(dest.join("a.txt").is_file());
        assert!(dest.join("sub").join("b.txt").is_file());

        // write + read manifest
        let mut mf = fs::File::create(dest.join(MANIFEST)).unwrap();
        for rel in &installed {
            writeln!(mf, "{}", rel.display()).unwrap();
        }
        drop(mf);
        assert_eq!(read_manifest(&dest).unwrap().len(), 2);

        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn rollback_removes_copied_files() {
        let base = tmp("rb");
        let dest = base.join("out");
        fs::create_dir_all(&dest).unwrap();
        fs::write(dest.join("x.txt"), b"1").unwrap();
        rollback(&dest, &[PathBuf::from("x.txt")]);
        assert!(!dest.join("x.txt").exists());
        let _ = fs::remove_dir_all(&base);
    }
}
