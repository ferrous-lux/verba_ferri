#!/usr/bin/env bash
set -euo pipefail

source ~/.cargo/env

echo "=== Building WASM ==="
wasm-pack build --release --target web

echo "=== Running cargo build (trigger build.rs) ==="
cargo build

echo "=== Assembling www/ ==="
rm -rf www
mkdir -p www

python3 build/assemble.py

echo "=== Done ==="
