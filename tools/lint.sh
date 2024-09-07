#!/bin/sh

cargo fmt --check --manifest-path cbof/Cargo.toml

# TODO: Keep targets updated
cargo check --target x86_64-unknown-none --manifest-path cbof/Cargo.toml
# cargo clippy --target x86_64-unknown-none --manifest-path cbof/Cargo.toml

grep -rn '#\[allow\(.*\)\]' ./cbof/src/
