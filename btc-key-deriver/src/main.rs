use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;
use csv::WriterBuilder;
use bip39::Mnemonic;
use bitcoin::network::constants::Network;
use bitcoin::address::Address;
use bitcoin::bip32::{DerivationPath, ExtendedPrivKey};
use bitcoin::key::Secp256k1;
use bitcoin::script::ScriptBuf;
use bitcoin::PrivateKey;
use zeroize::Zeroize;
use hex;

/// A simple program to derive Bitcoin keys and addresses from BIP-39 seed phrases.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Input file containing 12-word BIP-39 seed phrases, one per line.
    #[clap(short, long, value_parser)]
    input: PathBuf,

    /// Output file for the derived keys and addresses in CSV format.
    #[clap(short, long, value_parser)]
    output: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command-line arguments.
    let args = Args::parse();
    // Create a new Secp256k1 context.
    let secp = Secp256k1::new();

    // Open the input file.
    let input_file = File::open(&args.input)?;
    let reader = BufReader::new(input_file);

    // Create a new CSV writer.
    let mut writer = WriterBuilder::new().has_headers(true).from_path(&args.output)?;
    // Write the CSV header.
    writer.write_record(&[
        "seed_index",
        "seed",
        "derivation_path",
        "address_index",
        "address",
        "public_key",
        "private_key",
        "private_key_wif",
        "script_semantics",
    ])?;

    // Define the base derivation paths to be used.
    let base_paths = vec![
        ("m/44'/0'/0'/0", "P2PKH"),
        ("m/49'/0'/0'/0", "P2SH"),
        ("m/84'/0'/0'/0", "P2WPKH"),
        ("m/87'/0'/0'/0", "P2WSH"),
        ("m/86'/0'/0'/0", "P2TR"),
    ];

    // Iterate over each line in the input file.
    for (seed_index, line) in reader.lines().enumerate() {
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

        // Iterate over each base derivation path.
        for (base_path_str, script_semantics) in &base_paths {
            // Derive 10 addresses for each base path.
            for i in 0..10 {
                // Append the index to the derivation path.
                let path_str = format!("{}/{}", base_path_str, i);
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
                    "P2SH" => Address::p2shwpkh(&public_key, Network::Bitcoin)?,
                    "P2WPKH" => Address::p2wpkh(&public_key, Network::Bitcoin)?,
                    "P2WSH" => {
                        let script = ScriptBuf::new_p2pk(&public_key);
                        Address::p2wsh(&script, Network::Bitcoin)
                    }
                    "P2TR" => {
                        let (x_only_pub_key, _) = public_key.inner.x_only_public_key();
                        Address::p2tr(&secp, x_only_pub_key, None, Network::Bitcoin)
                    }
                    _ => panic!("Unknown script semantics"),
                };

                // Write the derived key and address to the CSV file.
                writer.write_record(&[
                    (seed_index + 1).to_string(),
                    seed_phrase.clone(),
                    path_str.to_string(),
                    i.to_string(),
                    address.to_string(),
                    public_key.to_string(),
                    hex::encode(&private_key.inner.secret_bytes()),
                    private_key.to_wif(),
                    script_semantics.to_string(),
                ])?;
            }
        }
    }

    // Flush the CSV writer.
    writer.flush()?;
    println!("Successfully wrote derived keys to {}", args.output.display());

    Ok(())
}
