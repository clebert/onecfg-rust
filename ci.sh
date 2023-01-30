#!/usr/bin/env bash

set -Eeuxo pipefail

cargo build --release
cargo fmt --all -- --check
cargo clippy --all-targets --all-features
cargo test
