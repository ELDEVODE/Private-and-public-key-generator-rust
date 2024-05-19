extern crate bitcoin;
extern crate rand;
extern crate secp256k1;

use rand::rngs::OsRng;
use rand::RngCore;
use secp256k1::{PublicKey, Secp256k1, SecretKey};

fn main() {
    // Create a Secp256k1 context
    let secp = Secp256k1::new();

    // Generate a random private key
    let secret_key = generate_private_key();

    let public_key = PublicKey::from_secret_key(&secp, &secret_key);

    // Print the private and public keys
    println!("Private Key: {:?}", secret_key);
    println!("Public Key: {:?}", public_key);
}

fn generate_private_key() -> SecretKey {
    // Generate a random 32-byte array
    let mut rng = OsRng::default();
    let mut secret_key_bytes = [0u8; 32];
    rng.fill_bytes(&mut secret_key_bytes);

    SecretKey::from_slice(&secret_key_bytes).expect("32 bytes, within curve order")
}
