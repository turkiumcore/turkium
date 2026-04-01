#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

SERVICE_NAME="turkium-api"
BINARY_PATH="/opt/turkium-api-server/turkium-api-server"
CONFIG_PATH="/etc/turkium-api-server/.env"
LOG_PATH="/var/log/turkium-api-server"

# Function to print colored output
print_status() {
    echo -e "${GREEN}✅${NC} $1"
}

print_error() {
    echo -e "${RED}❌${NC} $1"
}

print_info() {
    echo -e "${BLUE}ℹ️${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠️${NC} $1"
}

# Function to check if running as root
check_root() {
    if [ "$EUID" -ne 0 ]; then 
        print_error "This command must be run as root (use sudo)"
        exit 1
    fi
}

# Function to start service
start_service() {
    check_root
    print_info "Starting $SERVICE_NAME..."
    systemctl start $SERVICE_NAME
    sleep 2
    if systemctl is-active --quiet $SERVICE_NAME; then
        print_status "Service started successfully"
        systemctl status $SERVICE_NAME
    else
        print_error "Failed to start service"
        journalctl -u $SERVICE_NAME -n 20
        exit 1
    fi
}

# Function to stop service
stop_service() {
    check_root
    print_info "Stopping $SERVICE_NAME..."
    systemctl stop $SERVICE_NAME
    sleep 1
    print_status "Service stopped"
}

# Function to restart service
restart_service() {
    check_root
    print_info "Restarting $SERVICE_NAME..."
    systemctl restart $SERVICE_NAME
    sleep 2
    if systemctl is-active --quiet $SERVICE_NAME; then
        print_status "Service restarted successfully"
        systemctl status $SERVICE_NAME
    else
        print_error "Failed to restart service"
        journalctl -u $SERVICE_NAME -n 20
        exit 1
    fi
}

# Function to show status
show_status() {
    print_info "Service Status:"
    systemctl status $SERVICE_NAME
    echo ""
    print_info "Service Details:"
    echo "  Binary: $BINARY_PATH"
    echo "  Config: $CONFIG_PATH"
    echo "  Logs: $LOG_PATH"
    echo ""
    print_info "Recent Logs:"
    journalctl -u $SERVICE_NAME -n 10 --no-pager
}

# Function to show logs
show_logs() {
    print_info "Showing logs (Ctrl+C to exit)..."
    journalctl -u $SERVICE_NAME -f
}

# Function to show recent logs
show_recent_logs() {
    local lines=${1:-50}
    print_info "Last $lines log lines:"
    journalctl -u $SERVICE_NAME -n $lines --no-pager
}

# Function to enable service
enable_service() {
    check_root
    print_info "Enabling $SERVICE_NAME..."
    systemctl enable $SERVICE_NAME
    print_status "Service enabled (will start on boot)"
}

# Function to disable service
disable_service() {
    check_root
    print_info "Disabling $SERVICE_NAME..."
    systemctl disable $SERVICE_NAME
    print_status "Service disabled (won't start on boot)"
}

# Function to edit configuration
edit_config() {
    check_root
    if [ ! -f "$CONFIG_PATH" ]; then
        print_error "Configuration file not found: $CONFIG_PATH"
        exit 1
    fi
    print_info "Editing configuration..."
    nano "$CONFIG_PATH"
    print_info "Restarting service to apply changes..."
    restart_service
}

# Function to check health
check_health() {
    print_info "Checking API server health..."
    if curl -s http://localhost:3001/health > /dev/null 2>&1; then
        response=$(curl -s http://localhost:3001/health)
        print_status "API Server is healthy"
        echo "$response" | jq . 2>/dev/null || echo "$response"
    else
        print_error "API Server is not responding"
        print_info "Service status:"
        systemctl status $SERVICE_NAME || true
    fi
}

# Function to show help
show_help() {
    cat << EOF
${BLUE}Turkium API Server - Service Management${NC}

Usage: $0 <command>

Commands:
  start              Start the service
  stop               Stop the service
  restart            Restart the service
  status             Show service status
  logs               Show live logs (Ctrl+C to exit)
  logs <N>           Show last N log lines
  enable             Enable service (start on boot)
  disable            Disable service (don't start on boot)
  config             Edit configuration file
  health             Check API server health
  help               Show this help message

Examples:
  sudo $0 start
  sudo $0 restart
  sudo $0 logs
  sudo $0 logs 100
  sudo $0 status
  sudo $0 config
  $0 health

Service Details:
  Name: $SERVICE_NAME
  Binary: $BINARY_PATH
  Config: $CONFIG_PATH
  Logs: journalctl -u $SERVICE_NAME -f

EOF
}

# Main script logic
case "${1:-help}" in
    start)
        start_service
        ;;
    stop)
        stop_service
        ;;
    restart)
        restart_service
        ;;
    status)
        show_status
        ;;
    logs)
        if [ -z "$2" ]; then
            show_logs
        else
            show_recent_logs "$2"
        fi
        ;;
    enable)
        enable_service
        ;;
    disable)
        disable_service
        ;;
    config)
        edit_config
        ;;
    health)
        check_health
        ;;
    help|--help|-h)
        show_help
        ;;
    *)
        print_error "Unknown command: $1"
        echo ""
        show_help
        exit 1
        ;;
esac
