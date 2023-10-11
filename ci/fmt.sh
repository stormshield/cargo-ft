#!/usr/bin/env sh

set -ex

cargo fmt --all --check
cargo sort --workspace --check --check-format
