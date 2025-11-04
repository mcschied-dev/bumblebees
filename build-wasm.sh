#!/bin/bash
# Build BumbleBees for WASM/Web deployment

set -e

echo "🐝 Building BumbleBees for WASM..."

# Check if wasm32 target is installed
if ! rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then
    echo "📦 Installing wasm32-unknown-unknown target..."
    rustup target add wasm32-unknown-unknown
fi

# Build the WASM binary
echo "🔨 Compiling to WASM..."
cargo build --target wasm32-unknown-unknown --lib --release

# Check if wasm-bindgen is installed
if ! command -v wasm-bindgen &> /dev/null; then
    echo "📦 Installing wasm-bindgen-cli..."
    cargo install wasm-bindgen-cli --version 0.2.105
fi

# Generate JavaScript bindings
echo "🔗 Generating JavaScript bindings..."
wasm-bindgen --target web --out-dir pkg target/wasm32-unknown-unknown/release/ten.wasm

# Get the size of the WASM file
WASM_SIZE=$(du -h pkg/ten_bg.wasm | cut -f1)

echo "✅ WASM build complete!"
echo "📦 Output directory: pkg/"
echo "📊 WASM size: $WASM_SIZE"
echo ""
echo "🚀 To test locally, run:"
echo "   python3 -m http.server 8000"
echo "   Then open: http://localhost:8000"
