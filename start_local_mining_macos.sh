#!/bin/bash

# Turkium Local Mining Setup for macOS
# Connects to remote turkium node at 188.132.197.20
# Uses the same blockchain (no separate genesis block)

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Remote server configuration
REMOTE_SERVER="188.132.197.20"
REMOTE_P2P_PORT="5206"
REMOTE_GRPC_PORT="5200"

# Local configuration (handle both cases: script in root or in Turkium folder)
if [ -f "$SCRIPT_DIR/target/release/Turkiumd" ]; then
    LOCAL_DAEMON_BIN="$SCRIPT_DIR/target/release/turkiumd"
    LOCAL_MINER_BIN="$SCRIPT_DIR/target/release/turkium-miner"
else
    # Script is in Turkium subfolder
    LOCAL_DAEMON_BIN="$SCRIPT_DIR/../target/release/turkiumd"
    LOCAL_MINER_BIN="$SCRIPT_DIR/../target/release/turkium-miner"
fi
LOCAL_GRPC_PORT="5200"
LOCAL_P2P_PORT="5207"

MINING_ADDRESS="turkium:qzq34kp6kxqrput2n60sf9qdaealtpwlrd4nhnets479kd3nwd3t6xgv9xd45"

# Check if binaries exist
if [ ! -f "$LOCAL_DAEMON_BIN" ]; then
    echo "❌ Error: Daemon binary not found at $LOCAL_DAEMON_BIN"
    echo "Please build the project first: cargo build --release"
    exit 1
fi

if [ ! -f "$LOCAL_MINER_BIN" ]; then
    echo "❌ Error: Miner binary not found at $LOCAL_MINER_BIN"
    echo "Please build the project first: cargo build --release"
    exit 1
fi

echo "🚀 Starting turkium Local Mining (macOS)"
echo "=========================================="
echo ""
echo "Configuration:"
echo "   Remote Server: $REMOTE_SERVER:$REMOTE_P2P_PORT"
echo "   Local GRPC: 127.0.0.1:$LOCAL_GRPC_PORT"
echo "   Local P2P: 127.0.0.1:$LOCAL_P2P_PORT"
echo "   Mining Address: $MINING_ADDRESS"
echo ""

# Kill any existing processes
echo "🛑 Stopping any existing turkium processes..."
pkill -f "turkiumd" || true
pkill -f "turkium-miner" || true
sleep 2

echo ""
echo "✅ Starting turkium Daemon (connecting to remote node)..."
echo ""

# Start daemon and connect to remote server
# --rpclisten=127.0.0.1:5200: Local GRPC server
# --listen=127.0.0.1:5207: Local P2P port (different from remote)
# --addpeer=188.132.197.20:5206: Connect to remote turkium node
# --enable-unsynced-mining: Allow mining while syncing
"$LOCAL_DAEMON_BIN" \
    --rpclisten=127.0.0.1:$LOCAL_GRPC_PORT \
    --listen=127.0.0.1:$LOCAL_P2P_PORT \
    --addpeer=$REMOTE_SERVER:$REMOTE_P2P_PORT \
    --enable-unsynced-mining &

DAEMON_PID=$!
echo "✅ Daemon started (PID: $DAEMON_PID)"

sleep 5

# Wait for GRPC server to be ready
echo "⏳ Waiting for GRPC server to be ready..."
for i in {1..30}; do
    if nc -z 127.0.0.1 $LOCAL_GRPC_PORT 2>/dev/null; then
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
echo "⏳ Waiting for daemon to connect to remote node..."
echo "   This may take 10-30 seconds..."
echo ""

# Wait for peer connection
PEER_FOUND=false
for i in {1..60}; do
    # Check if connected to peer (look for connection logs)
    if pgrep -f "turkiumd" > /dev/null; then
        # Simple check: if daemon is still running, it's trying to connect
        if [ $((i % 10)) -eq 0 ]; then
            echo "⏳ Connecting to peer... ($i seconds)"
        fi
    fi
    sleep 1
done

echo ""
echo "✅ Starting turkium CPU Miner (6 threads)..."
echo "   Mining Address: $MINING_ADDRESS"
echo ""

# Start miner
# --mining-address: Where to send mined blocks
# --mine-when-not-synced: Allow mining while syncing blockchain
"$LOCAL_MINER_BIN" \
    --mining-address "$MINING_ADDRESS" \
    --mine-when-not-synced &

MINER_PID=$!

echo ""
echo "=========================================="
echo "✅ Turkium Local Mining Started!"
echo "=========================================="
echo ""
echo "📊 Process Information:"
echo "   Daemon PID: $DAEMON_PID"
echo "   Miner PID: $MINER_PID"
echo ""
echo "🔗 Connection Status:"
echo "   Remote Node: $REMOTE_SERVER:$REMOTE_P2P_PORT"
echo "   Local GRPC: 127.0.0.1:$LOCAL_GRPC_PORT"
echo "   Local P2P: 127.0.0.1:$LOCAL_P2P_PORT"
echo ""
echo "⏳ Mining Status:"
echo "   - Daemon is connecting to remote node"
echo "   - Miner is mining to: $MINING_ADDRESS"
echo "   - Mining threads: 6"
echo ""
echo "🔍 What to look for:"
echo "   1. Miner will show 'GetTemplate' messages"
echo "   2. Look for 'Found a block' messages"
echo "   3. Hashrate should be displayed every 10 seconds"
echo "   4. Daemon should sync with remote blockchain"
echo ""
echo "📝 To monitor daemon:"
echo "   tail -f ~/turkium/turkium-mainnet/logs/turkium.log"
echo ""
echo "💡 To stop mining:"
echo "   kill $DAEMON_PID $MINER_PID"
echo ""
echo "📌 To check peer connection:"
echo "   lsof -i :$LOCAL_P2P_PORT"
echo "   netstat -an | grep $LOCAL_P2P_PORT"
echo ""

# Keep script running
wait
