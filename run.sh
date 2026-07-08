#!/usr/bin/env bash
cd "$(dirname "$0")"
cargo run -- "${@:-install ./out}"
