# Turkium CPU Miner 

## Installation

### From Binaries
The [release page](https://github.com/turkium/cpuminer/releases) includes precompiled binaries for Linux, macOS and Windows.

# Usage
To start mining you need to run [turkiumd](https://github.com/turkium/turkium) and have an address to send the rewards to.
See the Rusty Turkium testnet docs for running a full node and generating addresses: https://github.com/turkium/turkium/blob/master/docs/

### Help:
```
A Turkium high performance CPU miner

Usage: turkium-miner [OPTIONS] --mining-address <MINING_ADDRESS>

Options:
  -a, --mining-address <MINING_ADDRESS>
          The Turkium address for the miner reward
  -s, --turkiumd-address <TURKIUMD_ADDRESS>
          The IP of the turkiumd instance [default: 127.0.0.1]
  -p, --port <PORT>
          Turkiumd port [default: Mainnet = 5200, Testnet = 5202]
  -d, --debug
          Enable debug logging level
      --testnet
          Use testnet instead of mainnet [default: false]
  -t, --threads <NUM_THREADS>
          Amount of miner threads to launch [default: number of logical cpus]
      --devfund <DEVFUND_ADDRESS>
          Mine a percentage of the blocks to the Turkium devfund [default: Off]
      --devfund-percent <DEVFUND_PERCENT>
          The percentage of blocks to send to the devfund [default: 1]
      --mine-when-not-synced
          Mine even when turkiumd says it is not synced, only useful when passing `--allow-submit-block-when-not-synced` to turkiumd  [default: false]
      --throttle <THROTTLE>
          Throttle (milliseconds) between each pow hash generation (used for development testing)
      --altlogs
          Output logs in alternative format (same as turkiumd)
  -h, --help
          Print help
  -V, --version
          Print version
```

### Running

`./turkium-miner --testnet --mining-address turk:XXXXX`

This will run the miner on all the available CPU cores. Requires a testnet Turkiumd on localhost.

### Docker

`docker run --rm turkiumnet/cpuminer --testnet -s 123.123.123.123 -a turk:XXXXX`

Supply a valid testnet node with an open GRPC port to the -s parameter.

### Docker Compose

Create docker-compose.yaml:
```yaml
services:

  turkium_miner_testnet_10:
    container_name: turkium_miner_testnet_10
    image: turkiumnet/cpuminer
    restart: unless-stopped
    cpus: 0.1 # Increase if necessary, remove to use all cores
    command: --testnet -s 123.123.123.123 -a turk:XXXXX

  turkium_miner_testnet_12:
    container_name: turkium_miner_testnet_12
    image: turkiumnet/cpuminer
    restart: unless-stopped
    cpus: 0.1 # Increase if necessary, remove to use all cores
    command: --testnet -s 321.321.321.321 -a turk:XXXXX
```

Run in same directory:
`docker compose up -d`
