use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;
use bip39::Mnemonic;
use bitcoin::network::constants::Network;
use bitcoin::address::Address;
use bitcoin::bip32::{DerivationPath, ExtendedPrivKey};
use bitcoin::key::Secp256k1;
use bitcoin::PrivateKey;
use hex;

/// A simple program to derive Bitcoin keys and addresses from BIP-39 seed phrases.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Input file containing 12-word BIP-39 seed phrases, one per line.
    #[clap(short, long, value_parser)]
    input: PathBuf,

    /// Output file for addresses only (mandatory). Default: addressonly.txt
    #[clap(short, long, value_parser, default_value = "addressonly.txt")]
    addresses: PathBuf,

    /// Output file for the derived keys and addresses in CSV format (optional).
    #[clap(short, long, value_parser)]
    output: Option<PathBuf>,

    /// Generate full output file with all details (keys, addresses, paths).
    #[clap(short = 'f', long)]
    full: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command-line arguments.
    let args = Args::parse();
    // Create a new Secp256k1 context.
    let secp = Secp256k1::new();

    // Open the input file.
    let input_file = File::open(&args.input)?;
    let reader = BufReader::new(input_file);

    // Create output file for writing
    let mut output_content = String::new();

    // Define the base derivation paths to be used.
    // Format: (base_path, suffix_pattern, script_semantics, num_addresses)
    // suffix_pattern: "hardened" means append /i', "normal_hardened" means append /0/i'
    let base_paths: Vec<(&str, &str, &str, u32)> = vec![
        ("m/0'/0'", "hardened", "P2PKH", 10),
        ("m/44'/0'/0'", "normal_hardened", "P2PKH", 10),
        ("m/49'/0'/0'", "normal_hardened", "P2WPKH nested in P2SH", 10),
        ("m/84'/0'/0'", "normal_hardened", "P2WPKH", 10),
    ];

    // Additional paths with different script semantics
    // Format: (paths, script_semantics)
    // Generate paths from m/0/0' to m/0/9' for each script semantics
    let mut p2wpkh_nested_paths: Vec<&str> = vec![];
    let mut p2wpkh_paths: Vec<&str> = vec![];
    for i in 0..10 {
        let path = format!("m/0/{}'", i);
        let leaked_path: &str = Box::leak(path.into_boxed_str());
        p2wpkh_nested_paths.push(leaked_path);
        p2wpkh_paths.push(leaked_path);
    }
    let additional_paths_grouped: Vec<(Vec<&str>, &str)> = vec![
        (p2wpkh_nested_paths, "P2WPKH nested in P2SH"),
        (p2wpkh_paths, "P2WPKH"),
    ];

    // Collect all addresses for the addresses-only output file
    let mut addresses_content = String::new();
    addresses_content.push_str("address\n");

    // Iterate over each line in the input file.
    for line in reader.lines() {
        let seed_phrase = line?;
        // Skip empty lines.
        if seed_phrase.trim().is_empty() {
            continue;
        }

        // Parse the mnemonic from the seed phrase.
        let mnemonic = Mnemonic::from_str(&seed_phrase)?;
        // Generate the seed from the mnemonic.
        let seed = mnemonic.to_seed("");

        // Create a new master key from the seed.
        let master_key = ExtendedPrivKey::new_master(Network::Bitcoin, &seed)?;

        // Output BIP39 Mnemonic and Seed
        output_content.push_str(&format!("BIP39 Mnemonic: {}\n\n", seed_phrase));
        output_content.push_str(&format!("BIP39 Seed:{}\n\n", hex::encode(&seed)));
        output_content.push_str("Coin: BTC\n\n");
        output_content.push_str("Derivation Path and outputs\n\n");
        output_content.push_str("path,address,public key,private key\n\n");

        // Iterate over each base derivation path.
        for (base_path_str, suffix_pattern, script_semantics, num_addresses) in &base_paths {
            // Derive addresses for each base path.
            for i in 0..*num_addresses {
                // Build the path based on the suffix pattern
                let path_str = match *suffix_pattern {
                    "hardened" => format!("{}/{}'", base_path_str, i),
                    "normal_hardened" => format!("{}/0/{}'", base_path_str, i),
                    _ => panic!("Unknown suffix pattern"),
                };
                // Parse the derivation path.
                let path = DerivationPath::from_str(&path_str)?;
                // Derive the child key from the master key and path.
                let child_key = master_key.derive_priv(&secp, &path)?;

                // Create a new `PrivateKey` from the derived key.
                let private_key = PrivateKey::new(child_key.private_key, Network::Bitcoin);
                // Derive the public key from the private key.
                let public_key = private_key.public_key(&secp);

                // Generate the address based on the script semantics.
                let address = match *script_semantics {
                    "P2PKH" => Address::p2pkh(&public_key, Network::Bitcoin),
                    "P2WPKH nested in P2SH" => Address::p2shwpkh(&public_key, Network::Bitcoin)?,
                    "P2WPKH" => Address::p2wpkh(&public_key, Network::Bitcoin)?,
                    _ => panic!("Unknown script semantics"),
                };

                // Write the derived key and address to the output.
                output_content.push_str(&format!(
                    "{},{},{},{}\n",
                    path_str,
                    address.to_string(),
                    public_key.to_string(),
                    private_key.to_wif(),
                ));

                // Collect address for addresses-only output
                addresses_content.push_str(&format!("{}\n", address.to_string()));
            }
            output_content.push_str("\n");
        }

        // Iterate over additional paths with different script semantics
        for (paths, script_semantics) in &additional_paths_grouped {
            output_content.push_str(&format!("Script Semantics: {}\n", script_semantics));
            // Derive addresses for each path in the group.
            for path_str_ref in paths {
                // For additional paths, use the path as-is (no index appending)
                let path_str = path_str_ref.to_string();
                // Parse the derivation path.
                let path = DerivationPath::from_str(&path_str)?;
                // Derive the child key from the master key and path.
                let child_key = master_key.derive_priv(&secp, &path)?;

                // Create a new `PrivateKey` from the derived key.
                let private_key = PrivateKey::new(child_key.private_key, Network::Bitcoin);
                // Derive the public key from the private key.
                let public_key = private_key.public_key(&secp);

                // Generate the address based on the script semantics.
                let address = match *script_semantics {
                    "P2PKH" => Address::p2pkh(&public_key, Network::Bitcoin),
                    "P2WPKH nested in P2SH" => Address::p2shwpkh(&public_key, Network::Bitcoin)?,
                    "P2WPKH" => Address::p2wpkh(&public_key, Network::Bitcoin)?,
                    _ => panic!("Unknown script semantics"),
                };

                // Write the derived key and address to the output.
                output_content.push_str(&format!(
                    "{},{},{},{}\n",
                    path_str,
                    address.to_string(),
                    public_key.to_string(),
                    private_key.to_wif(),
                ));

                // Collect address for addresses-only output
                addresses_content.push_str(&format!("{}\n", address.to_string()));
            }
            output_content.push_str("\n");
        }
    }

    // Write addresses-only to file (mandatory)
    std::fs::write(&args.addresses, &addresses_content)?;
    println!("Successfully wrote addresses to {}", args.addresses.display());

    // Write full output to file if requested
    if args.full || args.output.is_some() {
        let output_path = args.output.as_ref().unwrap_or(&args.addresses);
        std::fs::write(output_path, &output_content)?;
        println!("Successfully wrote derived keys to {}", output_path.display());
    }

    Ok(())
}
