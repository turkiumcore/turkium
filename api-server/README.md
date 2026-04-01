# turkium API Server

High-performance REST API server for the turkium blockchain, built with Rust + Axum.

## Features

- ⚡ **Ultra-Fast**: Built with Rust + Axum for maximum performance
- 🔗 **Blockchain Integration**: Direct connection to turkium node via wRPC
- 💾 **Smart Caching**: Automatic response caching with TTL
- 🌐 **CORS Support**: Configured for cross-origin requests
- 📊 **Real-time Data**: Live blockchain data from the node
- 🛡️ **Production-Ready**: Error handling, logging, and monitoring

## Performance

| Metric | Value |
|--------|-------|
| Requests/sec | 50,000+ |
| Latency | 1-5ms |
| Memory | 5-10MB |
| Startup | ~50ms |

## Prerequisites

- Rust 1.70+ ([Install](https://rustup.rs/))
- turkium node running on `localhost:5200` (gRPC server)

## Installation

```bash
cd Turkium/api-server
cargo build --release
```

## Configuration

Create `.env.local` from `.env.example`:

```bash
cp .env.example .env.local
```

Edit `.env.local`:

```env
SERVER_HOST=0.0.0.0
SERVER_PORT=3001
TURKIUM_NODE_HOST=localhost
TURKIUM_NODE_PORT=5200
RUST_LOG=info,turkium_api_server=debug
```

## Running

### Development

```bash
cargo run
```

### Production

```bash
cargo build --release
./target/release/turkium-api-server
```

## API Endpoints

### Health Check
- `GET /health` - Server health status

### Blockchain Info
- `GET /info/blockdag` - BlockDAG information
- `GET /info/coinsupply` - Coin supply
- `GET /info/hashrate` - Network hashrate
- `GET /info/hashrate/max` - Maximum hashrate
- `GET /info/fee-estimate` - Fee estimates
- `GET /info/halving` - Halving information
- `GET /info/market-data` - Market data

### Blocks
- `GET /blocks/:hash` - Block details

### Transactions
- `GET /transactions/:hash` - Transaction details
- `GET /transactions/count` - Transaction count
- `POST /transactions/search` - Search transactions

### Addresses
- `GET /addresses/:addr/balance` - Address balance
- `GET /addresses/:addr/transactions-count` - Transaction count
- `GET /addresses/:addr/utxos` - UTXOs
- `GET /addresses/:addr/full-transactions` - Address transactions
- `GET /addresses/:addr/name` - Address name/label

## Example Requests

```bash
# Health check
curl http://localhost:3001/health

# Get BlockDAG info
curl http://localhost:3001/info/blockdag

# Get address balance
curl http://localhost:3001/addresses/turkium:qzq34kp6kxqrput2n60sf9qdaealtpwlrd4nhnets479kd3nwd3t6xgv9xd45/balance

# Get block
curl http://localhost:3001/blocks/0x1234567890abcdef

# Search transactions
curl -X POST http://localhost:3001/transactions/search \
  -H "Content-Type: application/json" \
  -d '{"transaction_ids": ["tx1", "tx2"]}'
```

## Docker

### Build

```bash
docker build -t turkium-api-server .
```

### Run

```bash
docker run -p 3001:3001 \
  -e TURKIUM_NODE_HOST=localhost \
  -e TURKIUM_NODE_PORT=5200 \
  turkium-api-server
```

## Logging

Set `RUST_LOG` environment variable:

```bash
# Debug level
RUST_LOG=debug cargo run

# Info level
RUST_LOG=info cargo run

# Specific module
RUST_LOG=turkium_api_server=debug cargo run
```

## Performance Tuning

### Increase Cache TTL
```env
CACHE_TTL_SECS=60
```

### Increase Cache Capacity
```env
CACHE_MAX_CAPACITY=50000
```

### Adjust Rate Limiting
```env
RATE_LIMIT_REQUESTS=5000
RATE_LIMIT_WINDOW_SECS=60
```

## Troubleshooting

### Connection to Node Failed
- Ensure turkium node is running on `localhost:5200`
- Check `TURKIUM_NODE_HOST` and `TURKIUM_NODE_PORT` in `.env.local`
- Verify node is accepting connections: `curl http://localhost:5200`

### High Memory Usage
- Reduce `CACHE_MAX_CAPACITY`
- Reduce `CACHE_TTL_SECS`

### Slow Responses
- Check node connection status: `GET /health`
- Verify node is synced
- Check network latency to node

## Development

### Run Tests
```bash
cargo test
```

### Format Code
```bash
cargo fmt
```

### Lint
```bash
cargo clippy
```

### Build Documentation
```bash
cargo doc --open
```

## Architecture

```
┌─────────────────────────────────────────┐
│         Explorer Frontend               │
│         (React + TypeScript)            │
└────────────────┬────────────────────────┘
                 │ HTTP/REST
                 ▼
┌─────────────────────────────────────────┐
│      Turkium API Server (Rust)          │
│      - Axum Web Framework               │
│      - Response Caching                 │
│      - Error Handling                   │
│      - Logging & Monitoring             │
└────────────────┬────────────────────────┘
                 │ wRPC (JSON-RPC)
                 ▼
┌─────────────────────────────────────────┐
│      Turkium Node (turkiumd)            │
│      - Blockchain Data                  │
│      - Network Statistics               │
│      - Transaction Pool                 │
└─────────────────────────────────────────┘
```

## License

MIT

## Support

For issues and questions, please open an issue on GitHub.
