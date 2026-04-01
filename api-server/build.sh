#!/bin/bash

set -e

echo "🚀 Turkium API Server - Build Script"
echo "===================================="
echo ""

# Check Rust installation
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed. Please install Rust from https://rustup.rs/"
    exit 1
fi

echo "✅ Rust version: $(rustc --version)"
echo ""

# Clean previous builds
echo "🧹 Cleaning previous builds..."
cargo clean

# Build release binary
echo "🔨 Building release binary..."
cargo build --release

# Check if build was successful
if [ -f "target/release/turkium-api-server" ]; then
    echo ""
    echo "✅ Build successful!"
    echo ""
    echo "📍 Binary location: $(pwd)/target/release/turkium-api-server"
    echo "📊 Binary size: $(du -h target/release/turkium-api-server | cut -f1)"
    echo ""
    echo "🚀 To run the server:"
    echo "   ./target/release/turkium-api-server"
    echo ""
    echo "📝 Or with custom configuration:"
    echo "   RUST_LOG=debug ./target/release/turkium-api-server"
    echo ""
else
    echo "❌ Build failed!"
    exit 1
fi
