#!/bin/bash

set -e

echo "🔧 Turkium API Server - Ubuntu Environment Fix"
echo "=============================================="
echo ""

# Check if running as root
if [ "$EUID" -ne 0 ]; then 
    echo "❌ This script must be run as root (use sudo)"
    exit 1
fi

echo "📋 Steps:"
echo "1. Creating config directory..."
echo "2. Creating .env file..."
echo "3. Restarting service..."
echo ""

# Step 1: Create config directory
echo "📁 Creating config directory..."
mkdir -p /etc/turkium-api-server
echo "✅ Directory created"

# Step 2: Create .env file with correct port 5200
echo "📝 Creating .env file with port 5200..."
cat > /etc/turkium-api-server/.env << 'ENVEOF'
SERVER_HOST=0.0.0.0
SERVER_PORT=3001
ENVIRONMENT=production
TURKIUM_NODE_HOST=localhost
TURKIUM_NODE_PORT=5200
TURKIUM_NODE_TIMEOUT_SECS=10
CACHE_TTL_SECS=30
CACHE_MAX_CAPACITY=10000
CORS_ALLOWED_ORIGINS=*
RATE_LIMIT_REQUESTS=1000
RATE_LIMIT_WINDOW_SECS=60
RUST_LOG=info,turkium_api_server=debug
ENVEOF

# Set permissions
chmod 600 /etc/turkium-api-server/.env
chown turkium:turkium /etc/turkium-api-server/.env 2>/dev/null || true

echo "✅ .env file created with port 5200"

# Step 3: Create systemd service file
echo "🔧 Creating systemd service file..."
cat > /etc/systemd/system/turkium-api.service << 'EOF'
[Unit]
Description=Turkium API Server
Documentation=https://github.com/turkium/turkium
After=network.target
Wants=network-online.target

[Service]
Type=simple
User=turkium
Group=turkium
WorkingDirectory=/opt/turkium-api-server

# Load environment variables from config file
EnvironmentFile=/etc/turkium-api-server/.env

# Set default environment variables if not specified in .env
Environment="RUST_LOG=info,turkium_api_server=debug"
Environment="SERVER_HOST=0.0.0.0"
Environment="SERVER_PORT=3001"
Environment="TURKIUM_NODE_HOST=localhost"
Environment="TURKIUM_NODE_PORT=5200"

# Run the server
ExecStart=/opt/turkium-api-server/turkium-api-server

# Restart policy
Restart=on-failure
RestartSec=10
StartLimitInterval=60
StartLimitBurst=5

# Security settings
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=yes
ReadWritePaths=/var/log/turkium-api-server

# Resource limits
LimitNOFILE=65536
LimitNPROC=65536

# Logging
StandardOutput=journal
StandardError=journal
SyslogIdentifier=turkium-api

[Install]
WantedBy=multi-user.target
EOF

echo "✅ Service file created"

# Step 4: Reload systemd and restart service
echo "🔄 Reloading systemd daemon..."
systemctl daemon-reload
echo "✅ Daemon reloaded"

echo "🚀 Restarting service..."
systemctl restart turkium-api
sleep 3
echo "✅ Service restarted"

# Step 5: Verify
echo ""
echo "🔍 Verifying configuration..."
echo ""

if systemctl is-active --quiet turkium-api; then
    echo "✅ Service is running"
else
    echo "❌ Service failed to start"
    systemctl status turkium-api
    exit 1
fi

echo ""
echo "📋 Recent logs:"
journalctl -u turkium-api -n 10 --no-pager

echo ""
echo "╔════════════════════════════════════════════════════════════╗"
echo "║   ✅ Environment Fix Complete!                            ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo "📍 Configuration:"
echo "   Config file: /etc/turkium-api-server/.env"
echo "   Service file: /etc/systemd/system/turkium-api.service"
echo "   Port: 5200 (configured)"
echo ""
echo "🔗 API Server:"
echo "   URL: http://localhost:3001"
echo "   Health: curl http://localhost:3001/health"
echo ""
echo "📊 View logs:"
echo "   sudo journalctl -u turkium-api -f"
echo ""
