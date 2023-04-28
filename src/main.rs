use clap::{App, Arg};
use ethers::prelude::*;
use rand::Rng;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use sha3::{Digest, Keccak256};
use std::str::FromStr;

fn generate_ethereum_address() -> (String, SecretKey) {
    let secp = Secp256k1::new();
    let mut rng = rand::thread_rng();
    let secret_key =
        SecretKey::from_slice(&rand::random::<[u8; 32]>()).expect("Secret key creation failed");
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    let serialized_public_key = public_key.serialize_uncompressed();
    let ethereum_address: Address = {
        let mut keccak = Keccak256::new();
        keccak.update(&serialized_public_key[1..]);
        let result = keccak.finalize();
        Address::from_slice(&result[12..])
    };

    (ethereum_address.to_string(), secret_key)
}

fn main() {
    let matches = App::new("Ethereum Vanity Address Generator")
        .version("1.0")
        .author("0xKoda")
        .about("Generates Ethereum vanity addresses")
        .arg(
            Arg::with_name("pattern")
                .short('p')
                .long("pattern")
                .value_name("PATTERN")
                .help("Pattern to search for in the Ethereum address")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let pattern = matches.value_of("pattern").unwrap();

    loop {
        let (address, secret_key) = generate_ethereum_address();
        if address.contains(pattern) {
            println!("Found vanity address: {}", address);
            println!(
                "Corresponding private key (hex): {}",
                hex::encode(&secret_key[..])
            );
            break;
        }
    }
}
