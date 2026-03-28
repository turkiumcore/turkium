use secp256k1::{Secp256k1, SecretKey};
use Turkium_addresses::{Address, Prefix, Version};
use bip39::Mnemonic;
use pbkdf2::pbkdf2_hmac;
use sha2::Sha512;
use rand::Rng;

fn main() {
    let secp = Secp256k1::new();
    let mut rng = rand::thread_rng();

    // Generate 5 random mainnet addresses for mining
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║     Turkium Mainnet Mining Addresses with BIP39 Mnemonic      ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    for i in 1..=5 {
        // Generate random entropy (16 bytes = 128 bits for 12-word mnemonic)
        let entropy: [u8; 16] = rng.gen();
        
        // Generate BIP39 mnemonic from entropy
        let mnemonic = Mnemonic::from_entropy(&entropy)
            .expect("Failed to generate mnemonic from entropy");
        let mnemonic_str = mnemonic.to_string();
        
        // Convert mnemonic to seed (128 hex characters / 64 bytes)
        let seed = mnemonic_to_seed(&mnemonic_str, "");
        
        // Derive private key from seed (first 32 bytes)
        let secret_key = SecretKey::from_slice(&seed[0..32])
            .expect("Failed to create secret key from seed");
        let public_key = secret_key.public_key(&secp);

        // Get x-only public key (32 bytes)
        let (xonly, _) = public_key.x_only_public_key();
        let payload: Vec<u8> = xonly.serialize().to_vec();

        // Create mainnet address
        let address = Address::new(Prefix::Mainnet, Version::PubKey, &payload);
        let address_str: String = address.into();

        // Get private key as hex
        let private_key_hex = bytes_to_hex(&secret_key.secret_bytes());

        println!("┌─ Address #{} ─────────────────────────────────────────────────┐", i);
        println!("│");
        println!("│  Mining Address:");
        println!("│  {}", address_str);
        println!("│");
        println!("│  BIP39 Mnemonic (12 words):");
        println!("│  {}", mnemonic_str);
        println!("│");
        println!("│  Private Key (HEX - for reference only):");
        println!("│  {}", private_key_hex);
        println!("│");
        println!("│  ⚠️  IMPORTANT: Save the MNEMONIC securely!");
        println!("│  ⚠️  Use MNEMONIC to import into Turkium wallet (Android/iOS)");
        println!("│  ⚠️  Anyone with this mnemonic can access the funds!");
        println!("│");
        println!("└────────────────────────────────────────────────────────────────┘\n");
    }

    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║                        USAGE EXAMPLES                         ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    println!("1. Mining with the address:");
    println!("   ./turkium-miner -a turk:YOUR_ADDRESS_HERE\n");

    println!("2. Import into Turkium Wallet (Android/iOS):");
    println!("   - Open Turkium wallet app");
    println!("   - Tap 'Import Wallet'");
    println!("   - Select 'Mnemonic Phrase'");
    println!("   - Paste the 12 words");
    println!("   - Wallet recovers all addresses\n");

    println!("3. Backup the mnemonic:");
    println!("   - Write down all 12 words in order");
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

/// Convert BIP39 mnemonic to seed using PBKDF2
/// This matches the standard BIP39 seed derivation
fn mnemonic_to_seed(mnemonic: &str, passphrase: &str) -> Vec<u8> {
    let password = mnemonic.as_bytes();
    let salt = format!("mnemonic{}", passphrase);
    let salt_bytes = salt.as_bytes();
    
    let mut seed = vec![0u8; 64];
    pbkdf2_hmac::<Sha512>(password, salt_bytes, 2048, &mut seed);
    seed
}

/// Convert bytes to hex string
fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()
}
