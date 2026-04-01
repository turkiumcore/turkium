#!/bin/bash

# Turkium API Server - Build and Install Script for Ubuntu
# This script builds the Rust API server and installs it as a systemd service

set -e

echo "🚀 Turkium API Server - Build and Install Script"
echo "=================================================="
echo ""

# Check if running as root
if [ "$EUID" -ne 0 ]; then 
    echo "❌ This script must be run as root (use sudo)"
    exit 1
fi

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed. Please install Rust first:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo "📦 Rust version:"
rustc --version
cargo --version
echo ""

# Step 1: Build release binary
echo "🔨 Building release binary..."
cargo build --release

# Check if build was successful
if [ ! -f "target/release/turkium-api-server" ]; then
    echo "❌ Build failed!"
    exit 1
fi

BINARY_SIZE=$(du -h target/release/turkium-api-server | cut -f1)
echo "✅ Build successful! Binary size: $BINARY_SIZE"
echo ""

# Step 2: Run systemd installation
echo "🔧 Installing systemd service..."
bash install-systemd.sh

echo ""
echo "╔════════════════════════════════════════════════════════════╗"
echo "║   ✅ Build and Installation Complete!                     ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo "🚀 Quick Start:"
echo ""
echo "1. Edit configuration (if needed):"
echo "   sudo nano /etc/turkium-api-server/.env"
echo ""
echo "2. Start the service:"
echo "   sudo systemctl start turkium-api"
echo ""
echo "3. Check status:"
echo "   sudo systemctl status turkium-api"
echo ""
echo "4. View logs:"
echo "   sudo journalctl -u turkium-api -f"
echo ""
echo "5. Test API:"
echo "   curl http://localhost:3001/health"
echo ""
