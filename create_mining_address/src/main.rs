use turkium_addresses::{Address, Prefix, Version};
use turkium_bip32::{Mnemonic, Language, ExtendedPrivateKey, SecretKey};
use std::str::FromStr;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    // Generate 5 random mainnet addresses for mining
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║     Turkium Mainnet Mining Addresses with BIP32 HD Wallet      ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    for i in 1..=5 {
        // Generate random entropy (32 bytes = 256 bits for 24-word mnemonic)
        let entropy: [u8; 32] = rng.gen();
        
        // Generate BIP39 mnemonic from entropy using Turkium's BIP32 library
        let mnemonic = Mnemonic::from_entropy(entropy.to_vec(), Language::English)
            .expect("Failed to generate mnemonic from entropy");
        let mnemonic_str = mnemonic.phrase().to_string();
        
        // Convert mnemonic to seed using BIP39 standard (PBKDF2)
        let seed = mnemonic.to_seed("");
        
        // Create extended private key from seed (BIP32 root key)
        let root_key: ExtendedPrivateKey<SecretKey> = ExtendedPrivateKey::new(&seed)
            .expect("Failed to create root key from seed");
        
        // Derive using Turkium BIP32 path: m/44'/111111'/0'/0/0
        // Coin type 111111 is Turkium/Kaspa standard (not Bitcoin's 0)
        // This matches the wallet's derivation path for the first receive address
        let derivation_path = turkium_bip32::DerivationPath::from_str("m/44'/111111'/0'/0/0")
            .expect("Failed to parse derivation path");
        
        let derived_key = root_key.derive_path(&derivation_path)
            .expect("Failed to derive key from path");
        
        // Get the extended public key from the derived private key
        // Type must be explicitly specified as ExtendedPublicKey<secp256k1::PublicKey>
        let extended_public_key: turkium_bip32::ExtendedPublicKey<secp256k1::PublicKey> = derived_key.public_key();
        
        // Get the secp256k1 public key from the extended public key
        let public_key = &extended_public_key.public_key;
        
        // Get private key bytes directly from the extended key
        let private_key_bytes = derived_key.to_bytes();

        // Get x-only public key (32 bytes) - matches wallet's address generation
        let (xonly, _) = public_key.x_only_public_key();
        let payload = xonly.serialize().to_vec();

        // Create mainnet address
        let address = Address::new(Prefix::Mainnet, Version::PubKey, &payload);
        let address_str: String = address.into();

        // Get private key as hex
        let private_key_hex = bytes_to_hex(&private_key_bytes);

        println!("┌─ Address #{} ─────────────────────────────────────────────────┐", i);
        println!("│");
        println!("│  Mining Address (BIP32 m/44'/111111'/0'/0/0):");
        println!("│  {}", address_str);
        println!("│");
        println!("│  BIP39 Mnemonic (24 words):");
        println!("│  {}", mnemonic_str);
        println!("│");
        println!("│  Private Key (HEX - for reference only):");
        println!("│  {}", private_key_hex);
        println!("│");
        println!("│  ⚠️  IMPORTANT: Save the MNEMONIC securely!");
        println!("│  ⚠️  Use MNEMONIC to import into turkium wallet (Android/iOS)");
        println!("│  ⚠️  This address matches wallet's first receive address!");
        println!("│  ⚠️  Anyone with this mnemonic can access the funds!");
        println!("│");
        println!("└────────────────────────────────────────────────────────────────┘\n");
    }

    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║                        USAGE EXAMPLES                         ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    println!("1. Mining with the address:");
    println!("   ./turkium-miner --mining-address YOUR_ADDRESS_HERE\n");

    println!("2. Import into Turkium Wallet (Android/iOS):");
    println!("   - Open turkium wallet app");
    println!("   - Tap 'Import Wallet'");
    println!("   - Select 'Mnemonic Phrase'");
    println!("   - Paste the 24 words");
    println!("   - Wallet recovers all addresses (including this mining address)\n");

    println!("3. Backup the mnemonic:");
    println!("   - Write down all 24 words in order");
    println!("   - Store in a secure location");
    println!("   - Never share or photograph\n");

    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║                      SECURITY WARNING                         ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    println!("⚠️  NEVER share your mnemonic with anyone!");
    println!("⚠️  NEVER paste your mnemonic in public places!");
    println!("⚠️  NEVER commit mnemonic to version control!");
    println!("⚠️  Store mnemonic in a secure location!");
    println!("⚠️  Use a hardware wallet for large amounts!");
    println!("⚠️  Write mnemonic on paper and store securely!\n");
}

/// Convert bytes to hex string
fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()
}
