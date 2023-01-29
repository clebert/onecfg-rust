#!/usr/bin/env bash

set -Eeuxo pipefail

cargo run --bin onecfg -- onecfg.json
cargo fmt --all -- --check
cargo clippy --all-targets --all-features
cargo test
