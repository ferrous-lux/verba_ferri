#!/usr/bin/env bash
set -euo pipefail

source ~/.cargo/env

echo "=== Building WASM ==="
wasm-pack build --release --target web

echo "=== Running cargo build (trigger build.rs) ==="
cargo build

OUTPUT="${1:-www}"

echo "=== Assembling ${OUTPUT}/ ==="
rm -rf "${OUTPUT}"
mkdir -p "${OUTPUT}"

python3 build/assemble.py --output "${OUTPUT}"

echo "=== Done ==="
