#!/bin/bash

# Turkium Systemd Services Installation Script
# This script sets up turkium blockchain and pool to auto-start on Ubuntu

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
TURKIUM_USER="turkium"
TURKIUM_HOME="/opt/turkium"
TURKIUM_DATA="${TURKIUM_HOME}/data"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo -e "${GREEN}=== Turkium Systemd Services Installation ===${NC}"
echo ""

# Check if running as root
if [[ $EUID -ne 0 ]]; then
   echo -e "${RED}Error: This script must be run as root${NC}"
   echo "Usage: sudo ./install-services.sh"
   exit 1
fi

# Check if systemd is available
if ! command -v systemctl &> /dev/null; then
    echo -e "${RED}Error: systemd is not installed${NC}"
    exit 1
fi

echo -e "${YELLOW}Step 1: Creating Turkium user and directories...${NC}"

# Create user if it doesn't exist
if ! id "$TURKIUM_USER" &>/dev/null; then
    useradd -r -s /bin/bash -d "$TURKIUM_HOME" "$TURKIUM_USER"
    echo -e "${GREEN}✓ Created user: $TURKIUM_USER${NC}"
else
    echo -e "${GREEN}✓ User already exists: $TURKIUM_USER${NC}"
fi

# Create directories
mkdir -p "$TURKIUM_DATA"
chown -R "$TURKIUM_USER:$TURKIUM_USER" "$TURKIUM_HOME"
chmod 755 "$TURKIUM_HOME"
chmod 700 "$TURKIUM_DATA"
echo -e "${GREEN}✓ Created directories${NC}"

echo ""
echo -e "${YELLOW}Step 2: Verifying binaries in $TURKIUM_HOME...${NC}"

# List what we have
echo "Files in $TURKIUM_HOME:"
ls -lh "$TURKIUM_HOME" | grep -E "turkiumd|stratum-pool|turkium-miner" || echo "No binaries found yet"

# Check if binaries exist
BINARIES_OK=true

if [ ! -f "$TURKIUM_HOME/turkiumd" ]; then
    echo -e "${RED}✗ Missing: $TURKIUM_HOME/turkiumd${NC}"
    BINARIES_OK=false
else
    echo -e "${GREEN}✓ Found: $TURKIUM_HOME/turkiumd${NC}"
fi

if [ ! -f "$TURKIUM_HOME/stratum-pool" ] && [ ! -f "$TURKIUM_HOME/turkium-stratum-pool" ]; then
    echo -e "${RED}✗ Missing: $TURKIUM_HOME/stratum-pool or $TURKIUM_HOME/turkium-stratum-pool${NC}"
    BINARIES_OK=false
else
    if [ -f "$TURKIUM_HOME/turkium-stratum-pool" ]; then
        echo -e "${GREEN}✓ Found: $TURKIUM_HOME/turkium-stratum-pool${NC}"
        # Rename to standard name
        mv "$TURKIUM_HOME/turkium-stratum-pool" "$TURKIUM_HOME/stratum-pool"
        echo -e "${GREEN}✓ Renamed to: $TURKIUM_HOME/stratum-pool${NC}"
    else
        echo -e "${GREEN}✓ Found: $TURKIUM_HOME/stratum-pool${NC}"
    fi
fi

if [ ! -f "$TURKIUM_HOME/turkium-miner" ]; then
    echo -e "${YELLOW}⚠ Optional: $TURKIUM_HOME/turkium-miner (not required for daemon/pool)${NC}"
else
    echo -e "${GREEN}✓ Found: $TURKIUM_HOME/turkium-miner${NC}"
fi

if [ "$BINARIES_OK" = false ]; then
    echo ""
    echo -e "${RED}Error: Required binaries not found!${NC}"
    echo ""
    echo "Copy binaries from source:"
    echo "  sudo cp /home/Turkium/target/release/turkiumd /opt/turkium/turkiumd"
    echo "  sudo cp /home/Turkium/target/release/turkium-stratum-pool /opt/turkium/stratum-pool"
    echo "  sudo cp /home/Turkium/target/release/turkium-miner /opt/turkium/turkium-miner"
    echo ""
    echo "Then set permissions:"
    echo "  sudo chown turkium:turkium /opt/turkium/turkiumd /opt/turkium/stratum-pool /opt/turkium/turkium-miner"
    echo "  sudo chmod 755 /opt/turkium/turkiumd /opt/turkium/stratum-pool /opt/turkium/turkium-miner"
    echo ""
    echo "Then run this script again."
    exit 1
fi

echo ""
echo -e "${YELLOW}Step 3: Setting permissions on binaries...${NC}"

# Fix permissions on binaries
chown "$TURKIUM_USER:$TURKIUM_USER" "$TURKIUM_HOME/turkiumd" "$TURKIUM_HOME/stratum-pool"
chmod 755 "$TURKIUM_HOME/turkiumd" "$TURKIUM_HOME/stratum-pool"

if [ -f "$TURKIUM_HOME/turkium-miner" ]; then
    chown "$TURKIUM_USER:$TURKIUM_USER" "$TURKIUM_HOME/turkium-miner"
    chmod 755 "$TURKIUM_HOME/turkium-miner"
fi

echo -e "${GREEN}✓ Permissions set${NC}"

echo ""
echo -e "${YELLOW}Step 4: Installing systemd service files...${NC}"

# Copy service files
if [ -f "$SCRIPT_DIR/turkiumd.service" ]; then
    cp "$SCRIPT_DIR/turkiumd.service" /etc/systemd/system/
    chmod 644 /etc/systemd/system/turkiumd.service
    echo -e "${GREEN}✓ Installed turkiumd.service${NC}"
else
    echo -e "${RED}Error: turkiumd.service not found in $SCRIPT_DIR${NC}"
    exit 1
fi

if [ -f "$SCRIPT_DIR/stratum-pool.service" ]; then
    cp "$SCRIPT_DIR/stratum-pool.service" /etc/systemd/system/
    chmod 644 /etc/systemd/system/stratum-pool.service
    echo -e "${GREEN}✓ Installed stratum-pool.service${NC}"
else
    echo -e "${RED}Error: stratum-pool.service not found in $SCRIPT_DIR${NC}"
    exit 1
fi

echo ""
echo -e "${YELLOW}Step 5: Reloading systemd daemon...${NC}"

systemctl daemon-reload
echo -e "${GREEN}✓ Systemd daemon reloaded${NC}"

echo ""
echo -e "${YELLOW}Step 6: Enabling services...${NC}"

systemctl enable turkiumd.service
echo -e "${GREEN}✓ Enabled turkiumd.service${NC}"

systemctl enable stratum-pool.service
echo -e "${GREEN}✓ Enabled stratum-pool.service${NC}"

echo ""
echo -e "${YELLOW}Step 7: Starting services...${NC}"

systemctl start turkiumd.service
sleep 2
echo -e "${GREEN}✓ Started turkiumd.service${NC}"

systemctl start stratum-pool.service
sleep 2
echo -e "${GREEN}✓ Started stratum-pool.service${NC}"

echo ""
echo -e "${YELLOW}Step 8: Verifying services...${NC}"

# Check status
if systemctl is-active --quiet turkiumd.service; then
    echo -e "${GREEN}✓ turkiumd.service is running${NC}"
else
    echo -e "${RED}✗ turkiumd.service is not running${NC}"
    systemctl status turkiumd.service
fi

if systemctl is-active --quiet stratum-pool.service; then
    echo -e "${GREEN}✓ stratum-pool.service is running${NC}"
else
    echo -e "${RED}✗ stratum-pool.service is not running${NC}"
    systemctl status stratum-pool.service
fi

echo ""
echo -e "${GREEN}=== Installation Complete ===${NC}"
echo ""
echo "Services are now installed and running!"
echo ""
echo "Useful commands:"
echo "  Check status:     sudo systemctl status turkiumd.service stratum-pool.service"
echo "  View logs:        sudo journalctl -u turkiumd.service -f"
echo "  Stop services:    sudo systemctl stop turkiumd.service stratum-pool.service"
echo "  Start services:   sudo systemctl start turkiumd.service stratum-pool.service"
echo "  Restart services: sudo systemctl restart turkiumd.service stratum-pool.service"
echo ""
echo "Services will automatically start on system boot."
echo "Database will persist across restarts."
echo ""
