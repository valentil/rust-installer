#!/usr/bin/env bash
# Build + demo the installer: stage the sample payload into ./demo-install, then verify.
cd "$(dirname "$0")"
set -e
echo "== cargo build =="; cargo build --quiet
echo; echo "== install payload -> demo-install =="; cargo run --quiet -- install ./payload ./demo-install
echo; echo "== verify demo-install =="; cargo run --quiet -- verify ./demo-install
echo; echo "(run 'cargo run -- uninstall ./demo-install' to remove it, or 'cargo test' for the suite)"
