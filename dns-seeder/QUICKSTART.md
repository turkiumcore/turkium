# turkium DNS Seeder - Quick Start Guide

## 5 Minute Setup

### 1. Build

```bash
cd Turkium/dns-seeder
cargo build --release
```

### 2. Run Locally (Testing)

```bash
./target/release/turkium-dns-seeder \
  --hostname localhost \
  --bootstrap-peer 188.132.197.20:5206 \
  --dns-listen 127.0.0.1 \
  --dns-port 5353 \
  --log-level debug
```

Test DNS:
```bash
nslookup -port=5353 localhost 127.0.0.1
```

### 3. Deploy to Server

On your Linux server (188.132.197.20):

```bash
# Copy files
scp -r Turkium/dns-seeder user@188.132.197.20:/tmp/

# SSH into server
ssh user@188.132.197.20

# Run deployment script
cd /tmp/dns-seeder
sudo bash deploy.sh
```

### 4. Configure

Edit the systemd service:

```bash
sudo nano /etc/systemd/system/turkium-dns-seeder.service
```

Update:
- `--hostname mainnet-seeder-1.turkium.org` → Your hostname
- `--bootstrap-peer 188.132.197.20:5206` → Your server IP:port
- `--network mainnet` → Your network (mainnet/testnet/devnet)

### 5. Start

```bash
sudo systemctl daemon-reload
sudo systemctl start turkium-dns-seeder
sudo systemctl status turkium-dns-seeder
```

### 6. Verify

```bash
# Check logs
sudo journalctl -u turkium-dns-seeder -f

# Test DNS
nslookup mainnet-seeder-1.turkium.org
dig mainnet-seeder-1.turkium.org

# Check peer count
# (Look for "Peer stats" in logs)
```

## Configuration Examples

### Mainnet (Production)

```bash
./target/release/turkium-dns-seeder \
  --hostname mainnet-seeder-1.turkium.org \
  --network mainnet \
  --bootstrap-peer 188.132.197.20:5206 \
  --min-peers 100 \
  --max-peers 1000 \
  --crawl-interval 300
```

### Testnet

```bash
./target/release/turkium-dns-seeder \
  --hostname testnet-seeder-1.turkium.org \
  --network testnet \
  --bootstrap-peer 188.132.197.20:5207 \
  --min-peers 50 \
  --max-peers 500
```

### Devnet (Development)

```bash
./target/release/turkium-dns-seeder \
  --hostname devnet-seeder-1.turkium.org \
  --network devnet \
  --bootstrap-peer 188.132.197.20:5208 \
  --min-peers 10 \
  --max-peers 100 \
  --log-level debug
```

## DNS Records Setup

After seeder is running, add DNS A records:

```
mainnet-seeder-1.turkium.org  A  188.132.197.20
mainnet-seeder-2.turkium.org  A  188.132.197.20
mainnet-seeder-3.turkium.org  A  188.132.197.20
mainnet-seeder-4.turkium.org  A  188.132.197.20
```

## Troubleshooting

### Seeder won't start

```bash
# Check logs
sudo journalctl -u turkium-dns-seeder -n 50

# Check if port 53 is available
sudo netstat -tlnp | grep 53

# Try running manually
sudo -u turkium /opt/turkium-dns-seeder/turkium-dns-seeder --log-level debug
```

### No peers discovered

```bash
# Check bootstrap peer is reachable
nc -zv 188.132.197.20 5206

# Check logs for bootstrap errors
sudo journalctl -u turkium-dns-seeder -f | grep -i bootstrap
```

### DNS not responding

```bash
# Test locally
dig @localhost mainnet-seeder-1.turkium.org

# Check firewall
sudo ufw status
sudo ufw allow 53/udp
sudo ufw allow 53/tcp
```

## Monitoring

### View Real-time Logs

```bash
sudo journalctl -u turkium-dns-seeder -f
```

### Check Peer Count

```bash
sudo journalctl -u turkium-dns-seeder | grep "Peer stats"
```

### Monitor DNS Queries

```bash
sudo tcpdump -i any -n port 53
```

## Multiple Seeders

Run multiple seeders on different machines:

```bash
# Seeder 1 (188.132.197.20)
./turkium-dns-seeder \
  --hostname mainnet-seeder-1.turkium.org \
  --bootstrap-peer 188.132.197.20:5206

# Seeder 2 (192.168.1.100)
./turkium-dns-seeder \
  --hostname mainnet-seeder-2.turkium.org \
  --bootstrap-peer 188.132.197.20:5206

# Seeder 3 (10.0.0.50)
./turkium-dns-seeder \
  --hostname mainnet-seeder-3.turkium.org \
  --bootstrap-peer 188.132.197.20:5206
```

Then add DNS records for each:

```
mainnet-seeder-1.turkium.org  A  188.132.197.20
mainnet-seeder-2.turkium.org  A  192.168.1.100
mainnet-seeder-3.turkium.org  A  10.0.0.50
```

## Performance Tips

- **High peer discovery**: Increase `--crawl-interval` (lower = faster)
- **Low memory**: Decrease `--max-peers`
- **Stable network**: Increase `--peer-timeout`
- **Production**: Use `--log-level info` (not debug)

## Next Steps

1. ✅ Build and test locally
2. ✅ Deploy to production server
3. ✅ Configure DNS records
4. ✅ Monitor logs
5. ✅ Add to monitoring system (Prometheus, etc.)

See `README.md` for detailed documentation.
