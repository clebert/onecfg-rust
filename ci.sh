#!/usr/bin/env bash

set -Eeuxo pipefail

cargo check --all-targets --profile=test
cargo test
cargo fmt --all -- --check
cargo clippy --all-targets --all-features
