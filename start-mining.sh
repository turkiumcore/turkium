#!/bin/bash

# Turkium Mainnet Mining Setup Script
# This script starts the Turkium daemon and CPU miner on mainnet
# Mainnet is the production network for Turkium cryptocurrency

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Support both local development and Ubuntu production deployment
# Priority: /opt/turkium (Ubuntu production) > local target/release (macOS development)
if [ -f "/opt/turkium/turkiumd" ]; then
    DAEMON_BIN="/opt/turkium/turkiumd"
    MINER_BIN="/opt/turkium/turkium-miner"
    DEPLOYMENT_MODE="ubuntu-production"
else
    DAEMON_BIN="$SCRIPT_DIR/target/release/turkiumd"
    MINER_BIN="$SCRIPT_DIR/target/release/turkium-miner"
    DEPLOYMENT_MODE="local-development"
fi

MINING_ADDRESS="turkium:qzq34kp6kxqrput2n60sf9qdaealtpwlrd4nhnets479kd3nwd3t6xgv9xd45"

# Check if binaries exist
if [ ! -f "$DAEMON_BIN" ]; then
    echo "❌ Error: Daemon binary not found at $DAEMON_BIN"
    echo "Please build the project first: cargo build --release"
    exit 1
fi

if [ ! -f "$MINER_BIN" ]; then
    echo "❌ Error: Miner binary not found at $MINER_BIN"
    echo "Please build the project first: cargo build --release"
    exit 1
fi

echo "🚀 Starting turkium Mainnet Mining Setup (Production)"
echo "=========================================="
echo ""
echo "Deployment Mode: $DEPLOYMENT_MODE"
echo "Network: Mainnet (production network)"
echo "Mining Address: $MINING_ADDRESS"
echo "Daemon Binary: $DAEMON_BIN"
echo "Miner Binary: $MINER_BIN"
echo ""
echo "⚠️  Production Configuration:"
echo "   - GRPC Server: 0.0.0.0:5200 (all interfaces)"
echo "   - P2P Server: 0.0.0.0:5206 (all interfaces)"
echo "   - Peer connectivity: ENABLED"
echo "   - Mining threads: 6"
echo ""

# Check if services are already running
if [ "$DEPLOYMENT_MODE" = "ubuntu-production" ]; then
    echo "🔍 Checking service status..."
    
    TURKIUMD_RUNNING=false
    STRATUM_RUNNING=false
    MINER_RUNNING=false
    
    if systemctl is-active --quiet turkiumd.service; then
        echo "✅ turkiumd.service is already running"
        TURKIUMD_RUNNING=true
    fi
    
    if systemctl is-active --quiet stratum-pool.service; then
        echo "✅ stratum-pool.service is already running"
        STRATUM_RUNNING=true
    else
        echo "🔄 Starting stratum-pool.service..."
        sudo systemctl start stratum-pool.service
        sleep 2
        if systemctl is-active --quiet stratum-pool.service; then
            echo "✅ stratum-pool.service started successfully"
            STRATUM_RUNNING=true
        else
            echo "⚠️  Warning: stratum-pool.service failed to start"
        fi
    fi
    
    if pgrep -f "turkium-miner" > /dev/null; then
        echo "✅ turkium-miner is already running"
        MINER_RUNNING=true
    fi
    
    if [ "$TURKIUMD_RUNNING" = true ] && [ "$STRATUM_RUNNING" = true ] && [ "$MINER_RUNNING" = true ]; then
        echo ""
        echo "=========================================="
        echo "✅ All services are already running!"
        echo "=========================================="
        echo ""
        echo "📊 Service Status:"
        echo "   Daemon: Running"
        echo "   Pool: Running"
        echo "   Miner: Running"
        echo ""
        echo "📌 To monitor:"
        echo "   sudo journalctl -u turkiumd.service -f"
        echo "   sudo journalctl -u stratum-pool.service -f"
        echo ""
        exit 0
    fi
else
    # Local development mode: kill existing processes
    echo "🛑 Stopping any existing turkium processes..."
    pkill -f "turkiumd" || true
    pkill -f "turkium-miner" || true
    sleep 2

    # Clean up old blockchain data for fresh start
    echo "🧹 Cleaning up old blockchain data..."
    rm -rf ~/turkium/turkium-mainnet/
    sleep 1
fi

echo ""
echo "✅ Starting turkium Daemon (Mainnet - Production)..."
echo "   GRPC Server: 0.0.0.0:5200"
echo "   P2P Server: 0.0.0.0:5206"
echo "   Network: Mainnet (production network)"
echo "   Mode: $DEPLOYMENT_MODE"
echo ""

# Start daemon in background for mainnet production
# --rpclisten=0.0.0.0:5200: Allow external peer connections on all interfaces
# --listen=0.0.0.0:5206: P2P server listens on all interfaces for peer connectivity
# --enable-unsynced-mining: Allow mining even if node reports not synced (required for initial startup)
if [ "$DEPLOYMENT_MODE" = "ubuntu-production" ]; then
    # Ubuntu production: use systemd-managed daemon
    echo "⚠️  Note: Daemon is managed by systemd (turkiumd.service)"
    echo "   Use: sudo systemctl status turkiumd.service"
    echo "   Use: sudo journalctl -u turkiumd.service -f"
    
    # Wait for daemon to be running
    for i in {1..10}; do
        DAEMON_PID=$(pgrep -f "turkiumd" || echo "0")
        if [ "$DAEMON_PID" != "0" ]; then
            echo "✅ Daemon is running (PID: $DAEMON_PID)"
            break
        fi
        if [ $i -eq 10 ]; then
            echo "❌ Error: turkiumd is not running. Start it with:"
            echo "   sudo systemctl start turkiumd.service"
            exit 1
        fi
        sleep 1
    done
else
    # Local development: start daemon manually
    "$DAEMON_BIN" --rpclisten=0.0.0.0:5200 --listen=0.0.0.0:5206 --enable-unsynced-mining &
    DAEMON_PID=$!
fi
sleep 5

# Wait for GRPC server to be ready
echo "⏳ Waiting for GRPC server to be ready..."
for i in {1..30}; do
    if nc -z 127.0.0.1 5200 2>/dev/null; then
        echo "✅ GRPC server is ready"
        break
    fi
    if [ $i -eq 30 ]; then
        echo "❌ GRPC server failed to start"
        kill $DAEMON_PID 2>/dev/null || true
        exit 1
    fi
    sleep 1
done

echo ""
echo "⏳ Waiting for daemon to sync blockchain..."
echo "   This may take several minutes on first startup..."
echo ""

# Wait for daemon to be synced before starting miner
# Check sync status via GRPC or log monitoring
SYNC_TIMEOUT=600  # 10 minutes timeout
SYNC_CHECK_INTERVAL=10
ELAPSED=0

while [ $ELAPSED -lt $SYNC_TIMEOUT ]; do
    # Try to get sync status from daemon logs
    SYNC_STATUS=$(sudo journalctl -u turkiumd.service -n 50 --no-pager 2>/dev/null | grep -i "synced\|sync complete\|blockchain synced" | tail -1)
    
    if [ ! -z "$SYNC_STATUS" ]; then
        echo "✅ Daemon sync status: $SYNC_STATUS"
        break
    fi
    
    # Alternative: check if daemon is responding to GRPC calls
    if nc -z 127.0.0.1 5200 2>/dev/null; then
        # Daemon is responding, assume it's syncing
        echo "⏳ Daemon syncing... ($ELAPSED seconds elapsed)"
    fi
    
    sleep $SYNC_CHECK_INTERVAL
    ELAPSED=$((ELAPSED + SYNC_CHECK_INTERVAL))
done

if [ $ELAPSED -ge $SYNC_TIMEOUT ]; then
    echo "⚠️  Warning: Daemon sync timeout after 10 minutes"
    echo "   Proceeding with mining anyway (--mine-when-not-synced enabled)"
fi

echo ""
echo "✅ Starting turkium CPU Miner (6 threads)..."
echo "   Mining Address: $MINING_ADDRESS"
echo ""

# Start miner in background for mainnet production
# Mining address: turk: prefix for mainnet
# --mine-when-not-synced: Allow mining even if daemon reports not synced
"$MINER_BIN" \
    --mining-address "$MINING_ADDRESS" \
    --mine-when-not-synced &
MINER_PID=$!

echo ""
echo "=========================================="
echo "✅ Turkium Mainnet Mining Started! (Production)"
echo "=========================================="
echo ""
echo "📊 Process Information:"
echo "   Daemon PID: $DAEMON_PID"
echo "   Miner PID: $MINER_PID"
echo "   Deployment Mode: $DEPLOYMENT_MODE"
echo ""
if [ "$DEPLOYMENT_MODE" = "ubuntu-production" ]; then
    echo "📝 Log Files:"
    echo "   Daemon: sudo journalctl -u turkiumd.service -f"
    echo "   Pool: sudo journalctl -u stratum-pool.service -f"
else
    echo "📝 Log Files:"
    echo "   Daemon: ~/.turkium/turkium-mainnet/logs/turkium.log"
fi
echo ""
echo "⏳ Mining Status:"
echo "   - Daemon is accepting peer connections on 0.0.0.0:5206"
echo "   - Miner is mining to: $MINING_ADDRESS"
echo "   - Mining threads: 6"
echo ""
echo "🔍 What to look for:"
echo "   1. Miner will show 'GetTemplate' messages"
echo "   2. Look for 'Found a block' messages"
echo "   3. Hashrate should be displayed every 10 seconds"
echo "   4. Peers can connect to your node on port 5206"
echo ""
echo "💡 To stop mining:"
if [ "$DEPLOYMENT_MODE" = "ubuntu-production" ]; then
    echo "   sudo systemctl stop stratum-pool.service"
    echo "   (Daemon continues running via systemd)"
else
    echo "   kill $DAEMON_PID $MINER_PID"
fi
echo ""
echo "📌 To monitor:"
if [ "$DEPLOYMENT_MODE" = "ubuntu-production" ]; then
    echo "   sudo journalctl -u turkiumd.service -f"
    echo "   sudo journalctl -u stratum-pool.service -f"
else
    echo "   tail -f ~/.turkium/turkium-mainnet/logs/turkium.log"
fi
echo ""

# Wait for processes (only if local development mode)
if [ "$DEPLOYMENT_MODE" = "local-development" ]; then
    wait
else
    # Ubuntu production: miner runs in background, script exits
    echo ""
    echo "✅ Miner is running in background"
    echo "   Monitor with: tail -f /opt/turkium/data/miner.log"
fi
