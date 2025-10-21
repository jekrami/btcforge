use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use bip39::Mnemonic;
use bitcoin::network::constants::Network;
use bitcoin::address::Address;
use bitcoin::bip32::{DerivationPath, ExtendedPrivKey};
use bitcoin::key::Secp256k1;
use bitcoin::PrivateKey;
use hex;
use rayon::prelude::*;

/// A simple program to derive Bitcoin keys and addresses from BIP-39 seed phrases.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Input file containing 12-word BIP-39 seed phrases, one per line.
    #[clap(short, long, value_parser)]
    input: PathBuf,

    /// Output file for addresses only. Default: addressonly.txt
    #[clap(short, long, value_parser, default_value = "addressonly.txt")]
    output: PathBuf,

    /// Generate full output file with all details (keys, addresses, paths) in a specified location (optional).
    #[clap(short = 'f', long, value_parser)]
    full: Option<PathBuf>,
}

/// Result of processing a single seed phrase
struct SeedPhraseResult {
    addresses: Vec<String>,
    full_details: String,
}

/// Process a single seed phrase and derive all keys and addresses
fn process_seed_phrase(
    seed_phrase: String,
    base_paths: &[(String, &str, &str, u32)],
    additional_paths_grouped: &[(Vec<String>, &str)],
) -> Result<SeedPhraseResult, String> {
    let secp = Secp256k1::new();

    // Parse the mnemonic from the seed phrase.
    let mnemonic = Mnemonic::from_str(&seed_phrase)
        .map_err(|e| format!("Failed to parse mnemonic: {}", e))?;
    // Generate the seed from the mnemonic.
    let seed = mnemonic.to_seed("");

    // Create a new master key from the seed.
    let master_key = ExtendedPrivKey::new_master(Network::Bitcoin, &seed)
        .map_err(|e| format!("Failed to create master key: {}", e))?;

    let mut addresses = Vec::new();
    let mut full_details = String::new();

    // Output BIP39 Mnemonic and Seed
    full_details.push_str(&format!("BIP39 Mnemonic: {}\n\n", seed_phrase));
    full_details.push_str(&format!("BIP39 Seed:{}\n\n", hex::encode(&seed)));
    full_details.push_str("Coin: BTC\n\n");
    full_details.push_str("Derivation Path and outputs\n\n");
    full_details.push_str("path,address,public key,private key\n\n");

    // Iterate over each base derivation path.
    for (base_path_str, suffix_pattern, script_semantics, num_addresses) in base_paths {
        // Derive addresses for each base path.
        for i in 0..*num_addresses {
            // Build the path based on the suffix pattern
            let path_str = match *suffix_pattern {
                "hardened" => format!("{}/{}'", base_path_str, i),
                "normal_hardened" => format!("{}/0/{}'", base_path_str, i),
                _ => panic!("Unknown suffix pattern"),
            };
            // Parse the derivation path.
            let path = DerivationPath::from_str(&path_str)
                .map_err(|e| format!("Failed to parse path {}: {}", path_str, e))?;
            // Derive the child key from the master key and path.
            let child_key = master_key.derive_priv(&secp, &path)
                .map_err(|e| format!("Failed to derive key for path {}: {}", path_str, e))?;

            // Create a new `PrivateKey` from the derived key.
            let private_key = PrivateKey::new(child_key.private_key, Network::Bitcoin);
            // Derive the public key from the private key.
            let public_key = private_key.public_key(&secp);

            // Generate the address based on the script semantics.
            let address = match *script_semantics {
                "P2PKH" => Address::p2pkh(&public_key, Network::Bitcoin),
                "P2WPKH nested in P2SH" => Address::p2shwpkh(&public_key, Network::Bitcoin)
                    .map_err(|e| format!("Failed to create P2WPKH nested address: {}", e))?,
                "P2WPKH" => Address::p2wpkh(&public_key, Network::Bitcoin)
                    .map_err(|e| format!("Failed to create P2WPKH address: {}", e))?,
                _ => return Err("Unknown script semantics".to_string()),
            };

            let address_str = address.to_string();
            addresses.push(address_str.clone());

            // Write the derived key and address to the output.
            full_details.push_str(&format!(
                "{},{},{},{}\n",
                path_str,
                address_str,
                public_key.to_string(),
                private_key.to_wif(),
            ));
        }
        full_details.push_str("\n");
    }

    // Iterate over additional paths with different script semantics
    for (paths, script_semantics) in additional_paths_grouped {
        full_details.push_str(&format!("Script Semantics: {}\n", script_semantics));
        // Derive addresses for each path in the group.
        for path_str_ref in paths {
            // Parse the derivation path.
            let path = DerivationPath::from_str(path_str_ref)
                .map_err(|e| format!("Failed to parse path {}: {}", path_str_ref, e))?;
            // Derive the child key from the master key and path.
            let child_key = master_key.derive_priv(&secp, &path)
                .map_err(|e| format!("Failed to derive key for path {}: {}", path_str_ref, e))?;

            // Create a new `PrivateKey` from the derived key.
            let private_key = PrivateKey::new(child_key.private_key, Network::Bitcoin);
            // Derive the public key from the private key.
            let public_key = private_key.public_key(&secp);

            // Generate the address based on the script semantics.
            let address = match *script_semantics {
                "P2PKH" => Address::p2pkh(&public_key, Network::Bitcoin),
                "P2WPKH nested in P2SH" => Address::p2shwpkh(&public_key, Network::Bitcoin)
                    .map_err(|e| format!("Failed to create P2WPKH nested address: {}", e))?,
                "P2WPKH" => Address::p2wpkh(&public_key, Network::Bitcoin)
                    .map_err(|e| format!("Failed to create P2WPKH address: {}", e))?,
                _ => return Err("Unknown script semantics".to_string()),
            };

            let address_str = address.to_string();
            addresses.push(address_str.clone());

            // Write the derived key and address to the output.
            full_details.push_str(&format!(
                "{},{},{},{}\n",
                path_str_ref,
                address_str,
                public_key.to_string(),
                private_key.to_wif(),
            ));
        }
        full_details.push_str("\n");
    }

    Ok(SeedPhraseResult {
        addresses,
        full_details,
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command-line arguments.
    let args = Args::parse();

    // Open the input file.
    let input_file = File::open(&args.input)?;
    let reader = BufReader::new(input_file);

    // Define the base derivation paths to be used.
    // Format: (base_path, suffix_pattern, script_semantics, num_addresses)
    // suffix_pattern: "hardened" means append /i', "normal_hardened" means append /0/i'
    let base_paths: Vec<(String, &str, &str, u32)> = vec![
        ("m/0'/0'".to_string(), "hardened", "P2PKH", 10),
        ("m/44'/0'/0'".to_string(), "normal_hardened", "P2PKH", 10),
        ("m/49'/0'/0'".to_string(), "normal_hardened", "P2WPKH nested in P2SH", 10),
        ("m/84'/0'/0'".to_string(), "normal_hardened", "P2WPKH", 10),
    ];

    // Additional paths with different script semantics
    let mut p2wpkh_nested_paths: Vec<String> = vec![];
    let mut p2wpkh_paths: Vec<String> = vec![];
    for i in 0..10 {
        let path = format!("m/0/{}'", i);
        p2wpkh_nested_paths.push(path.clone());
        p2wpkh_paths.push(path);
    }
    let additional_paths_grouped: Vec<(Vec<String>, &str)> = vec![
        (p2wpkh_nested_paths, "P2WPKH nested in P2SH"),
        (p2wpkh_paths, "P2WPKH"),
    ];

    // Wrap in Arc for thread-safe sharing
    let base_paths = Arc::new(base_paths);
    let additional_paths_grouped = Arc::new(additional_paths_grouped);

    // Read all seed phrases from file
    let seed_phrases: Vec<String> = reader
        .lines()
        .filter_map(|line| {
            let seed_phrase = line.ok()?;
            if seed_phrase.trim().is_empty() {
                None
            } else {
                Some(seed_phrase)
            }
        })
        .collect();

    println!("Processing {} seed phrases in parallel...", seed_phrases.len());

    // Process all seed phrases in parallel
    let results: Vec<SeedPhraseResult> = seed_phrases
        .into_par_iter()
        .map(|seed_phrase| {
            process_seed_phrase(
                seed_phrase,
                &base_paths,
                &additional_paths_grouped,
            )
        })
        .collect::<Result<Vec<_>, String>>()
        .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)) as Box<dyn std::error::Error>)?;

    // Aggregate results
    let mut addresses_content = String::from("address\n");
    let mut output_content = String::new();

    for result in results {
        for address in result.addresses {
            addresses_content.push_str(&format!("{}\n", address));
        }
        output_content.push_str(&result.full_details);
    }

    // Write addresses-only to file (mandatory)
    std::fs::write(&args.output, &addresses_content)?;
    println!("Successfully wrote addresses to {}", args.output.display());

    // Write full output to file if requested
    if let Some(full_path) = &args.full {
        std::fs::write(full_path, &output_content)?;
        println!("Successfully wrote derived keys to {}", full_path.display());
    }

    Ok(())
}
