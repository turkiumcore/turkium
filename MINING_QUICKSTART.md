# Turkium Pool Mining - Quick Start

## TL;DR

```bash
# Build everything
cd Turkium
cargo build --release

# Terminal 1: Start pool server
./target/release/stratum-pool

# Terminal 2: Start mining
./target/release/turkium-miner -a turk:your_address_here
```

Done! Miner connects to pool automatically.

## One-Liner Commands

### Mainnet Mining
```bash
./target/release/turkium-miner -a turk:your_address_here
```

### Testnet Mining
```bash
./target/release/turkium-miner -a turktest:your_address_here
```

### Devnet Mining
```bash
./target/release/turkium-miner -a turkdev:your_address_here
```

### Custom Pool
```bash
./target/release/turkium-miner -a turk:your_address_here -s 192.168.1.100:3333
```

### Multiple Threads
```bash
./target/release/turkium-miner -a turk:your_address_here -t 8
```

### Debug Mode
```bash
./target/release/turkium-miner -a turk:your_address_here --debug
```

## What Changed?

✅ **Miner is now pool-only** - No direct blockchain connections
✅ **Auto-detection** - Finds pool automatically based on address prefix
✅ **Simplified** - Just `-a` for address, `-s` for custom pool
✅ **Network safe** - Prevents overload from direct connections

## Architecture

```
Blockchain (turkiumd)
        ↓
   Stratum Pool (stratum-pool)
        ↓
   Miners (turkium-miner)
```


## Troubleshooting

### "Connection refused"
- Is pool running? `./target/release/stratum-pool`
- Is port 3333 open? Check firewall

### "Invalid mining address"
- Use correct format: `turk:...`, `turktest:...`, or `turkdev:...`

### "No shares accepted"
- Check pool logs
- Verify miner address is correct
- Ensure pool is connected to blockchain


## Next Steps

1. Build: `cargo build --release`
2. Start pool: `./target/release/stratum-pool`
3. Start miner: `./target/release/turkium-miner -a turk:...`
4. Monitor: Check logs for shares and blocks

That's it! 🚀
