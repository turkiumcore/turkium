# turkium DNS Seeder

DNS seeder for turkium blockchain peer discovery. This application crawls the turkium P2P network and provides DNS A records for peer discovery.

## Features

- **Peer Crawling**: Automatically discovers and tracks active peers in the Turkium network
- **DNS Server**: Responds to DNS queries with peer IP addresses
- **Health Checking**: Monitors peer health and removes expired/unhealthy peers
- **Bootstrap Support**: Can bootstrap from a known peer to start discovery
- **Configurable**: Supports mainnet, testnet, and devnet networks

## Building

```bash
cd Turkium/dns-seeder
cargo build --release
```

## Running

### Mainnet Seeder

```bash
./target/release/turkium-dns-seeder \
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
```

### Testnet Seeder

```bash
./target/release/turkium-dns-seeder \
  --hostname testnet-seeder-1.turkium.org \
  --network testnet \
  --bootstrap-peer 188.132.197.20:5207 \
  --dns-listen 0.0.0.0 \
  --dns-port 53 \
  --min-peers 30 \
  --max-peers 300 \
  --log-level info
```

### Devnet Seeder

```bash
./target/release/turkium-dns-seeder \
  --hostname devnet-seeder-1.turkium.org \
  --network devnet \
  --bootstrap-peer 188.132.197.20:5208 \
  --dns-listen 0.0.0.0 \
  --dns-port 53 \
  --min-peers 10 \
  --max-peers 100 \
  --log-level debug
```

## Configuration

### Command Line Arguments

- `--hostname`: DNS seeder hostname (e.g., mainnet-seeder-1.turkium.org)
- `--peer-port`: turkium P2P port to crawl (default: 5206 for mainnet)
- `--dns-port`: DNS server listen port (default: 53)
- `--dns-listen`: DNS server listen address (default: 0.0.0.0)
- `--bootstrap-peer`: Initial peer to bootstrap from (e.g., 188.132.197.20:5206)
- `--network`: Network type - mainnet, testnet, or devnet (default: mainnet)
- `--min-peers`: Minimum number of peers to maintain (default: 50)
- `--max-peers`: Maximum number of peers to track (default: 500)
- `--crawl-interval`: Crawl interval in seconds (default: 300)
- `--peer-timeout`: Peer timeout in seconds (default: 600)
- `--log-level`: Log level - trace, debug, info, warn, error (default: info)

## DNS Setup

After running the seeder, configure DNS records:

```
mainnet-seeder-1.turkium.org  A  <seeder-ip>
mainnet-seeder-2.turkium.org  A  <seeder-ip>
mainnet-seeder-3.turkium.org  A  <seeder-ip>
mainnet-seeder-4.turkium.org  A  <seeder-ip>
```

## How It Works

1. **Bootstrap**: Connects to initial peer (188.132.197.20) to get peer list
2. **Crawl**: Periodically crawls peers to discover new peers and verify health
3. **Track**: Maintains a list of healthy peers with last-seen timestamps
4. **Serve**: Responds to DNS queries with random healthy peer IPs
5. **Cleanup**: Removes expired peers that haven't been seen recently

## Peer Discovery Flow

```
New Peer Starts
    ↓
Queries DNS Seeder (mainnet-seeder-1.turkium.org)
    ↓
DNS Seeder Returns List of Peer IPs
    ↓
New Peer Connects to Discovered Peers
    ↓
New Peer Joins Network
```

## Systemd Service

Create `/etc/systemd/system/turkium-dns-seeder.service`:

```ini
[Unit]
Description=turkium DNS Seeder
After=network.target

[Service]
Type=simple
User=turkium
WorkingDirectory=/opt/turkium-dns-seeder
ExecStart=/opt/turkium-dns-seeder/target/release/turkium-dns-seeder \
  --hostname mainnet-seeder-1.turkium.org \
  --network mainnet \
  --bootstrap-peer 188.132.197.20:5206 \
  --dns-listen 0.0.0.0 \
  --dns-port 53 \
  --log-level info
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

Enable and start:

```bash
sudo systemctl enable turkium-dns-seeder
sudo systemctl start turkium-dns-seeder
sudo systemctl status turkium-dns-seeder
```

## Monitoring

View logs:

```bash
sudo journalctl -u turkium-dns-seeder -f
```

Test DNS resolution:

```bash
nslookup mainnet-seeder-1.turkium.org
dig mainnet-seeder-1.turkium.org
```

## Performance Tuning

- **min-peers**: Lower for faster startup, higher for stability
- **max-peers**: Higher for better peer discovery, lower for memory efficiency
- **crawl-interval**: Lower for faster peer discovery, higher for less network load
- **peer-timeout**: Lower to remove stale peers faster, higher to be more forgiving

## Troubleshooting

### No peers discovered

- Check bootstrap peer is reachable: `nc -zv 188.132.197.20 5206`
- Check logs: `journalctl -u turkium-dns-seeder -f`
- Verify network connectivity

### DNS not responding

- Check port 53 is open: `sudo netstat -tlnp | grep 53`
- Check firewall rules
- Test with: `dig @localhost mainnet-seeder-1.turkium.org`

### High memory usage

- Reduce `--max-peers`
- Increase `--peer-timeout` to remove stale peers less frequently

## License

Same as turkium blockchain
