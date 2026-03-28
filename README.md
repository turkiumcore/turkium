# Turkium Core [TURK, Ŧ]

<div align="center">
  <img src="assets/turkium.png" alt="Turkium" width="256"/>
  <br/><br/>
  <strong>A High-Performance Cryptocurrency Built on Rust</strong>
</div>

---

## 🚀 About Turkium

Turkium is a revolutionary cryptocurrency that emerged from the vision of creating a more inclusive and accessible digital economy. Built on a proven foundation while introducing unique innovations, Turkium combines the reliability of time-tested blockchain technology with modern enhancements.

From developers to miners, from enthusiasts to everyday users, the Turkium community spans across continents and cultures. We welcome contributors who share our vision of a fair, open, and decentralized financial future.

**Turkium isn't just another cryptocurrency—it's a promise of what blockchain can achieve when guided by principle, powered by community, and driven by innovation.**

---

## 📋 Quick Start

### Prerequisites

- **Rust 1.70+** (Install from [rustup.rs](https://rustup.rs/))
- **macOS/Linux** (Windows support coming soon)
- **4GB RAM minimum** (8GB+ recommended for mining)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/turkium/turkium.git
cd Turkium

# Build release binaries
cargo build --release

# Binaries will be in target/release/
```

### Running a Node

```bash
# Start daemon on devnet (for testing)
./target/release/turkiumd --devnet

# Start daemon on mainnet
./target/release/turkiumd --mainnet

# View available options
./target/release/turkiumd --help
```

### Mining

```bash
# Generate a mining address
./target/release/create_devnet_address

# Start mining on devnet (4 threads)
./start_mainnet_mining.sh
```

---

## 🏗️ Project Structure

```
Turkium/
├── consensus/              # Consensus engine and DAG implementation
│   ├── core/              # Core consensus types and configuration
│   ├── client/            # Consensus client interface
│   ├── notify/            # Consensus notifications
│   └── pow/               # Proof-of-Work implementation
├── cpuminer/              # CPU mining implementation
├── crypto/                # Cryptographic primitives
│   ├── addresses/         # Address generation and validation
│   ├── hashes/            # Hash functions (SHA256, RIPEMD160)
│   ├── muhash/            # MuHash for UTXO commitments
│   └── merkle/            # Merkle tree implementation
├── rpc/                   # RPC interfaces
│   ├── service/           # RPC service implementation
│   ├── grpc/              # gRPC server and client
│   └── wrpc/              # WebSocket RPC
├── wallet/                # Wallet implementation
│   ├── core/              # Wallet core logic
│   ├── bip32/             # BIP32 hierarchical deterministic wallets
│   └── keys/              # Key management
├── indexes/               # Blockchain indexing
│   ├── core/              # Index core
│   ├── processor/         # Index processor
│   └── utxoindex/         # UTXO index
├── turkiumd/              # Daemon entry point
├── turksim/               # Simulator for testing
└── Cargo.toml             # Workspace configuration
```

---

## 🔧 Network Configuration

Turkium supports multiple network types for different use cases:

| Network | P2P Port | RPC Port | Use Case |
|---------|----------|----------|----------|
| **Mainnet** | 5200 | 5200 | Production network |
| **Testnet** | 5201 | 5201 | Public testing |
| **Devnet** | 5202 | 5202 | Local development |
| **Simnet** | 5203 | 5203 | Simulation/testing |

### Starting Different Networks

```bash
# Mainnet (requires peers)
./target/release/turkiumd --mainnet --rpclisten=127.0.0.1:5200

# Testnet (public test network)
./target/release/turkiumd --testnet --rpclisten=127.0.0.1:5201

# Devnet (local development, no peers required)
./target/release/turkiumd --devnet --rpclisten=127.0.0.1:5202

# Simnet (simulation mode)
./target/release/turkiumd --simnet --rpclisten=127.0.0.1:5203
```

---

## ⛏️ Mining

### CPU Mining

Turkium includes a built-in CPU miner for testing and small-scale mining:

```bash
# Generate a devnet mining address
./target/release/create_devnet_address

# Start mining with 4 threads
./target/release/turkium-miner \
  --mining-address "turkdev:qr4c6yyjxq90fkmkuzrx3dzf8hm09t55fes79axstd96mh0cyl2n2jstmtckw" \
  --turkiumd-address 127.0.0.1 \
  --port 5202 \
  --threads 4
```

### Automated Mining Setup

Use the provided mining script for easy setup:

```bash
# Edit the script to configure your mining address and network
nano start_mainnet_mining.sh

# Run the script
./start_mainnet_mining.sh
```

---

## 📡 RPC Interface

Turkium provides both gRPC and WebSocket RPC interfaces for interacting with the network.

### Common RPC Commands

```bash
# Get block template for mining
turkium-cli getblocktemplate --address "turkdev:..."

# Submit a block
turkium-cli submitblock <block_hex>

# Get network info
turkium-cli getnetworkinfo

# Get wallet balance
turkium-cli getbalance

# Send transaction
turkium-cli sendtoaddress <address> <amount>
```

### RPC Documentation

The JSON-RPC API is self-documenting:

```bash
# List all available commands
./target/release/turkium-cli help

# Get help for a specific command
./target/release/turkium-cli help <command>
```

---

## 🧪 Testing

### Running Tests

```bash
# Run all tests
cargo test --release

# Run tests for a specific package
cargo test -p consensus --release

# Run with logging
RUST_LOG=debug cargo test --release -- --nocapture
```

### Property-Based Testing

Turkium uses property-based testing for correctness verification:

```bash
# Run property-based tests
cargo test --release --features "bench"
```

---

## 🔐 Security

### Address Formats

Turkium uses Bech32-encoded addresses with network-specific prefixes:

- **Mainnet**: `turk:...` (PubKey), `turk:p...` (ScriptHash)
- **Testnet**: `turktest:...` (PubKey), `turktest:p...` (ScriptHash)
- **Devnet**: `turkdev:...` (PubKey), `turkdev:p...` (ScriptHash)
- **Simnet**: `turksim:...` (PubKey), `turksim:p...` (ScriptHash)

### Key Management

- Private keys are never stored in plaintext
- Use BIP32 hierarchical deterministic wallets for key derivation
- Support for hardware wallet integration

---

## 📚 Documentation

- [Installation Guide](INSTALL.md)
- [Getting Started](doc/getting-started.md)
- [Fee Recommendation](doc/fee-recommendation.md)
- [FAQ](doc/FAQ.md)
- [Contributing Guide](CONTRIBUTING.md)

---

## 🤝 Contributing

We welcome contributions from the community! Please see the [contribution guide](CONTRIBUTING.md) for details on how to participate.

### Reporting Issues

Found a bug? Please report it using the [issue system](https://github.com/turkium/turkium/issues/new?assignees=&labels=bug&template=bug_report.md&title=%5Bbug%5D+).

### Development Branches

- **master**: Unstable, latest development code
- **maintenance**: Stable, previous release maintenance (format: `<version>-maint`)
- **development**: Unstable, upcoming release code (format: `<version>-dev`)

**Submit pull requests against `master`**

---

## 📊 Development Status

Turkium Core is actively developed and maintained. Check out:

- [GitHub Projects](https://github.com/turkium/turkium/projects) - Planned and in-progress work
- [GitHub Discussions](https://github.com/turkium/turkium/discussions) - Feature discussions and proposals

### Version Strategy

Version numbers follow `major.minor.patch` semantics.

---

## ❓ FAQ

**Q: Can I mine on mainnet?**
A: Yes, but mainnet requires peer connectivity. For testing, use devnet which doesn't require peers.

**Q: What are the system requirements?**
A: Minimum 4GB RAM, 2GB disk space. 8GB+ RAM recommended for mining.

**Q: How do I report a security vulnerability?**
A: Please email security@turkium.org with details. Do not open public issues for security vulnerabilities.

**Q: Can I run multiple nodes on the same machine?**
A: Yes, use different ports for each node (e.g., 5202 for devnet, 5203 for simnet).

For more FAQs, see [doc/FAQ.md](doc/FAQ.md)

---

## 📜 License

Turkium Core is released under the terms of the MIT license. See [COPYING](COPYING) for more information.

---

## 🌍 Community

Join our community and be part of the Turkium revolution:

- **GitHub**: [github.com/turkium/turkium](https://github.com/turkium/turkium)
- **Discord**: [discord.gg/turkium](https://discord.gg/turkium)
- **Twitter**: [@TurkiumCrypto](https://twitter.com/TurkiumCrypto)
- **Website**: [turkium.org](https://turkium.org)

---

**Together, we're building the future of digital finance. Join us! 🚀**
