#!/bin/bash
# Build script for WASM target

set -e

echo "Installing wasm-pack..."
cargo install wasm-pack

echo "Building WASM target..."
wasm-pack build --target web --release --out-dir web/pkg

echo "WASM build complete!"
echo "Output directory: web/pkg"
echo ""
echo "To serve locally:"
echo "  cd web && python3 -m http.server"
