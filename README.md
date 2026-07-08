# Rust Installer

[![Made with FeatureBoard](https://img.shields.io/badge/Made_with-FeatureBoard-00c8ff?style=flat-square)](https://featureboard.ai) [![License: MIT](https://img.shields.io/badge/License-MIT-3fb950?style=flat-square)](./LICENSE)

A **bespoke, dependency-free software installer written in pure Rust** — standard library only, no crates. The goal is to implement everything an installer needs (argument parsing, filesystem staging, a manifest, progress, and eventually rollback) from scratch, as a study in doing it without pulling in the ecosystem.

> One of the example projects built with [FeatureBoard](https://featureboard.ai).

## Status: working

The original project folder was an empty stub; this is now a **functional, dependency-free installer** built entirely on `std`. It recursively stages a payload directory into a destination, records a manifest of everything it wrote, and can **verify** and **uninstall** from that manifest — with **rollback** if an install fails partway through. Ships with a sample `payload/` and a test suite.

### Commands

```bash
rust-installer install <SRC> <DEST>   # copy payload SRC into DEST, write a manifest
rust-installer verify   <DEST>        # check every manifested file is present
rust-installer uninstall <DEST>       # remove everything the manifest installed
rust-installer --version
```

### Try it (uses the bundled sample payload)

```bash
cargo run -- install ./payload ./demo-install   # stages README.txt, bin/, config/
cargo run -- verify ./demo-install              # all files present
cargo run -- uninstall ./demo-install           # cleanly removes them
cargo test                                       # install/verify/uninstall + rollback tests
```

`run.sh` / `run.bat` do the build → install → verify demo in one go.

## Build & run

```bash
cargo run -- install ./out     # stage payload into ./out (writes INSTALL_MANIFEST.txt)
cargo run -- --version
cargo test                     # runs the manifest test
```

## Design constraints

- **No dependencies.** `Cargo.toml` has an empty `[dependencies]` on purpose.
- Everything built on `std` (`std::fs`, `std::env`, `std::io`, `std::path`).

## License

MIT © Lewis Valentine

## The AI-native approach

Built the FeatureBoard way — see [How We Build](https://featureboard.ai/approach.html):

- **Planning & metadata as substrate** — scaffolded from an explicit std-only architecture brief.
- **Validation over review** — ships with a unit test proving the manifest is written.
