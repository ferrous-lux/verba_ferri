#!/usr/bin/env bash
set -euo pipefail

source ~/.cargo/env

echo "=== Building WASM ==="
wasm-pack build --release --target web

echo "=== Running cargo build (trigger build.rs) ==="
cargo build

echo "=== Assembling www/ ==="
rm -rf www
mkdir -p www/static www/pkg

cp index.html game.html www/
cp static/style.css static/sw.js static/manifest.json static/icon.svg static/word-list.html www/static/
cp pkg/verba_ferri.js pkg/verba_ferri_bg.wasm pkg/verba_ferri.d.ts pkg/verba_ferri_bg.wasm.d.ts www/pkg/

echo "=== Done ==="
echo "www/ is ready to deploy."
