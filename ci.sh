#!/usr/bin/env bash

set -Eeuxo pipefail

cargo fmt --all -- --check
cargo clippy --all-targets --all-features
cargo build --release
cargo test
