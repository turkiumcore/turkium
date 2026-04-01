use secp256k1::{Secp256k1, SecretKey};
use rand::Rng;

fn main() {
    let secp = Secp256k1::new();
    let mut rng = rand::thread_rng();
    
    // Generate a random secret key
    let secret_key = SecretKey::new(&mut rng);
    let public_key = secret_key.public_key(&secp);
    
    // Get x-only public key (32 bytes)
    let (xonly, _) = public_key.x_only_public_key();
    let payload = xonly.serialize();
    
    println!("Secret Key: {}", secret_key.display_secret());
    println!("Public Key: {}", public_key);
    println!("X-Only Public Key: {}", hex::encode(&payload));
    
    // For now, just print the payload in hex
    // The actual address encoding would require the address library
    println!("\nPayload (hex): {}", hex::encode(&payload));
}
