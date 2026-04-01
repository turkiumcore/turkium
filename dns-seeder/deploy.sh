#!/bin/bash

# turkium DNS Seeder Deployment Script
# This script builds and deploys the DNS seeder on a Linux server

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INSTALL_DIR="/opt/turkium-dns-seeder"
SERVICE_NAME="turkium-dns-seeder"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}🚀 turkium DNS Seeder Deployment${NC}"
echo "=========================================="

# Check if running as root
if [ "$EUID" -ne 0 ]; then
    echo -e "${RED}❌ This script must be run as root${NC}"
    exit 1
fi

# Check dependencies
echo -e "${YELLOW}📦 Checking dependencies...${NC}"
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Rust/Cargo not found. Install from https://rustup.rs/${NC}"
    exit 1
fi

if ! command -v systemctl &> /dev/null; then
    echo -e "${RED}❌ Systemd not found${NC}"
    exit 1
fi

# Build
echo -e "${YELLOW}🔨 Building DNS seeder...${NC}"
cd "$SCRIPT_DIR"
cargo build --release

# Create installation directory
echo -e "${YELLOW}📁 Setting up installation directory...${NC}"
mkdir -p "$INSTALL_DIR"
cp -r target/release/turkium-dns-seeder "$INSTALL_DIR/"
cp README.md "$INSTALL_DIR/"

# Create turkium user if not exists
if ! id "turkium" &>/dev/null; then
    echo -e "${YELLOW}👤 Creating turkium user...${NC}"
    useradd -r -s /bin/bash turkium
fi

# Set permissions
chown -R turkium:turkium "$INSTALL_DIR"
chmod +x "$INSTALL_DIR/turkium-dns-seeder"

# Create systemd service
echo -e "${YELLOW}⚙️  Creating systemd service...${NC}"
cat > /etc/systemd/system/${SERVICE_NAME}.service << 'EOF'
[Unit]
Description=turkium DNS Seeder
After=network.target

[Service]
Type=simple
User=turkium
WorkingDirectory=/opt/turkium-dns-seeder
ExecStart=/opt/turkium-dns-seeder/turkium-dns-seeder \
  --hostname mainnet-seeder-1.turkium.org \
  --network mainnet \
  --bootstrap-peer 188.132.197.20:5206 \
  --dns-listen 0.0.0.0 \
  --dns-port 53 \
  --min-peers 50 \
  --max-peers 500 \
  --crawl-interval 300 \
  --peer-timeout 600 \
  --log-level info
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
EOF

# Reload systemd
systemctl daemon-reload

# Enable service
echo -e "${YELLOW}🔧 Enabling service...${NC}"
systemctl enable ${SERVICE_NAME}

echo ""
echo -e "${GREEN}✅ Installation Complete!${NC}"
echo "=========================================="
echo ""
echo -e "${BLUE}📝 Next Steps:${NC}"
echo ""
echo "1. Edit the systemd service to configure your seeder:"
echo "   sudo nano /etc/systemd/system/${SERVICE_NAME}.service"
echo ""
echo "2. Update these values:"
echo "   - --hostname: Your DNS seeder hostname"
echo "   - --bootstrap-peer: Your main blockchain server"
echo "   - --network: mainnet, testnet, or devnet"
echo ""
echo "3. Reload and start the service:"
echo "   sudo systemctl daemon-reload"
echo "   sudo systemctl start ${SERVICE_NAME}"
echo ""
echo "4. Check status:"
echo "   sudo systemctl status ${SERVICE_NAME}"
echo ""
echo "5. View logs:"
echo "   sudo journalctl -u ${SERVICE_NAME} -f"
echo ""
echo "6. Test DNS resolution:"
echo "   nslookup mainnet-seeder-1.turkium.org"
echo ""
echo -e "${BLUE}📌 Configuration File:${NC}"
echo "   /etc/systemd/system/${SERVICE_NAME}.service"
echo ""
echo -e "${BLUE}📂 Installation Directory:${NC}"
echo "   $INSTALL_DIR"
echo ""
