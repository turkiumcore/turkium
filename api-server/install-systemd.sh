#!/bin/bash

set -e

echo "🚀 Turkium API Server - Systemd Installation Script"
echo "===================================================="
echo ""

# Check if running as root
if [ "$EUID" -ne 0 ]; then 
    echo "❌ This script must be run as root (use sudo)"
    exit 1
fi

# Check if binary exists
if [ ! -f "target/release/turkium-api-server" ]; then
    echo "❌ Binary not found. Please run: cargo build --release"
    exit 1
fi

echo "📋 Installation Steps:"
echo "1. Creating turkium user..."
echo "2. Creating directories..."
echo "3. Copying binary..."
echo "4. Setting permissions..."
echo "5. Creating systemd service..."
echo "6. Enabling service..."
echo ""

# Step 1: Create turkium user if it doesn't exist
if ! id "turkium" &>/dev/null; then
    echo "👤 Creating turkium user..."
    useradd -r -s /bin/bash -d /home/turkium -m turkium
    echo "✅ User created"
else
    echo "✅ User turkium already exists"
fi

# Step 2: Create directories
echo "📁 Creating directories..."
mkdir -p /opt/turkium-api-server
mkdir -p /var/log/turkium-api-server
mkdir -p /etc/turkium-api-server

# Step 3: Copy binary
echo "📦 Copying binary..."
cp target/release/turkium-api-server /opt/turkium-api-server/
chmod +x /opt/turkium-api-server/turkium-api-server

# Step 4: Copy configuration
echo "⚙️  Copying configuration..."
if [ ! -f "/etc/turkium-api-server/.env" ]; then
    if [ -f ".env.example" ]; then
        cp .env.example /etc/turkium-api-server/.env
    else
        # Create default .env if .env.example doesn't exist
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
    fi
    echo "📝 Configuration file created at /etc/turkium-api-server/.env"
    echo "⚠️  Please edit it with your settings:"
    echo "   sudo nano /etc/turkium-api-server/.env"
else
    echo "✅ Configuration already exists"
fi

# Step 5: Set permissions
echo "🔐 Setting permissions..."
chown -R turkium:turkium /opt/turkium-api-server
chown -R turkium:turkium /var/log/turkium-api-server
chown -R turkium:turkium /etc/turkium-api-server
chmod 755 /opt/turkium-api-server
chmod 755 /var/log/turkium-api-server
chmod 755 /etc/turkium-api-server
chmod 600 /etc/turkium-api-server/.env

# Step 6: Create systemd service file
echo "🔧 Creating systemd service..."
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

# Step 7: Reload systemd daemon
echo "🔄 Reloading systemd daemon..."
systemctl daemon-reload

# Step 8: Enable service
echo "✅ Enabling service..."
systemctl enable turkium-api

echo ""
echo "╔════════════════════════════════════════════════════════════╗"
echo "║   ✅ Installation Complete!                               ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo "📝 Next Steps:"
echo ""
echo "1. Edit configuration:"
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
echo "5. Stop the service:"
echo "   sudo systemctl stop turkium-api"
echo ""
echo "6. Restart the service:"
echo "   sudo systemctl restart turkium-api"
echo ""
echo "📍 Service Details:"
echo "   Binary: /opt/turkium-api-server/turkium-api-server"
echo "   Config: /etc/turkium-api-server/.env"
echo "   Logs: journalctl -u turkium-api -f"
echo "   Status: sudo systemctl status turkium-api"
echo ""
echo "🔗 API Server will be available at:"
echo "   http://localhost:3001"
echo ""
echo "✅ Health check:"
echo "   curl http://localhost:3001/health"
echo ""
