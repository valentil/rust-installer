# Rust Installer

[![Made with FeatureBoard](https://img.shields.io/badge/Made_with-FeatureBoard-00c8ff?style=flat-square)](https://featureboard.ai) [![License: MIT](https://img.shields.io/badge/License-MIT-3fb950?style=flat-square)](./LICENSE)

A **bespoke, dependency-free software installer written in pure Rust** — standard library only, no crates. The goal is to implement everything an installer needs (argument parsing, filesystem staging, a manifest, progress, and eventually rollback) from scratch, as a study in doing it without pulling in the ecosystem.

> One of the example projects built with [FeatureBoard](https://featureboard.ai).

## Status: early scaffold

Honest note: the original project folder was an empty stub. This is a **clean, compiling starting point** — a working CLI skeleton with a unit test — not a finished installer. It's included in the showcase as a work-in-progress that demonstrates the intended std-only architecture.

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
