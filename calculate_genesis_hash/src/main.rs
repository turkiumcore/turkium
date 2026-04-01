use turkium_consensus_core::block::Block;
use turkium_consensus_core::config::genesis::{DEVNET_GENESIS, GENESIS, SIMNET_GENESIS, TESTNET11_GENESIS, TESTNET_GENESIS};

fn main() {
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║           TURKIUM GENESIS BLOCK HASH CALCULATOR               ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    let genesis_blocks = vec![
        ("MAINNET", &GENESIS),
        ("TESTNET", &TESTNET_GENESIS),
        ("TESTNET11", &TESTNET11_GENESIS),
        ("SIMNET", &SIMNET_GENESIS),
        ("DEVNET", &DEVNET_GENESIS),
    ];

    for (name, genesis) in genesis_blocks {
        let block: Block = genesis.into();
        let calculated_hash = block.hash();
        let expected_hash = genesis.hash;

        println!("Network: {}", name);
        println!("  Expected Hash:   {}", expected_hash);
        println!("  Calculated Hash: {}", calculated_hash);

        if calculated_hash == expected_hash {
            println!("  Status: ✅ MATCH");
        } else {
            println!("  Status: ❌ MISMATCH");
            println!("  Expected bytes:   {:#04x?}", expected_hash.as_bytes());
            println!("  Calculated bytes: {:#04x?}", calculated_hash.as_bytes());
        }
        println!();
    }
}
