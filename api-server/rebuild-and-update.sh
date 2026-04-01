#!/bin/bash

set -e

echo "🔄 Turkium API Server - Rebuild and Update Script"
echo "=================================================="
echo ""

# Check if running as root
if [ "$EUID" -ne 0 ]; then 
    echo "❌ This script must be run as root (use sudo)"
    exit 1
fi

# Check if we're in the api-server directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Please run this script from the api-server directory"
    echo "   cd /home/Turkium/api-server"
    exit 1
fi

echo "📋 Update Steps:"
echo "1. Stopping service..."
echo "2. Cleaning build artifacts..."
echo "3. Recompiling binary..."
echo "4. Updating binary..."
echo "5. Restarting service..."
echo "6. Verifying configuration..."
echo ""

# Step 1: Stop the service
echo "🛑 Stopping turkium-api service..."
systemctl stop turkium-api || true
sleep 2
echo "✅ Service stopped"

# Step 2: Clean build artifacts
echo "🧹 Cleaning build artifacts..."
cargo clean
echo "✅ Build artifacts cleaned"

# Step 3: Recompile binary
echo "🔨 Recompiling binary (this may take a few minutes)..."
cargo build --release
echo "✅ Binary compiled successfully"

# Verify binary exists
if [ ! -f "target/release/turkium-api-server" ]; then
    echo "❌ Binary compilation failed"
    exit 1
fi

BINARY_SIZE=$(ls -lh target/release/turkium-api-server | awk '{print $5}')
echo "   Binary size: $BINARY_SIZE"

# Step 4: Update binary
echo "📦 Updating binary..."
cp target/release/turkium-api-server /opt/turkium-api-server/
chown turkium:turkium /opt/turkium-api-server/turkium-api-server
chmod 755 /opt/turkium-api-server/turkium-api-server
echo "✅ Binary updated"

# Step 5: Restart service
echo "🚀 Starting turkium-api service..."
systemctl start turkium-api
sleep 3
echo "✅ Service started"

# Step 6: Verify configuration
echo "🔍 Verifying configuration..."
echo ""

# Check service status
if systemctl is-active --quiet turkium-api; then
    echo "✅ Service is running"
else
    echo "❌ Service failed to start"
    systemctl status turkium-api
    exit 1
fi

# Check logs for correct port
echo ""
echo "📋 Recent logs:"
journalctl -u turkium-api -n 10 --no-pager

# Extract port from logs
PORT_LOG=$(journalctl -u turkium-api -n 20 --no-pager | grep "Node:" | tail -1)
if [[ $PORT_LOG == *"5200"* ]]; then
    echo ""
    echo "✅ Correct port configuration detected: 5200"
elif [[ $PORT_LOG == *"5202"* ]]; then
    echo ""
    echo "⚠️  WARNING: Old port 5202 still detected in logs"
    echo "   This may be a cached log entry. Check again in a few seconds."
else
    echo ""
    echo "⚠️  Could not verify port from logs"
fi

echo ""
echo "╔════════════════════════════════════════════════════════════╗"
echo "║   ✅ Update Complete!                                      ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo "📍 Service Status:"
echo "   Status: $(systemctl is-active turkium-api)"
echo "   Binary: /opt/turkium-api-server/turkium-api-server"
echo "   Config: /etc/turkium-api-server/.env"
echo ""
echo "🔗 API Server:"
echo "   URL: http://localhost:3001"
echo "   Health: curl http://localhost:3001/health"
echo ""
echo "📊 View logs:"
echo "   sudo journalctl -u turkium-api -f"
echo ""
