# BTC Key Deriver

This is a command-line tool written in Rust that derives Bitcoin keys and addresses from 12-word BIP-39 seed phrases.

## Features

*   Reads 12-word BIP-39 seed phrases from a text file (one per line).
*   Derives keys and addresses for the following derivation paths:
    *   **P2PKH (Legacy):** `m/44'/0'/0'/0/0`
    *   **P2SH (Nested SegWit):** `m/49'/0'/0'/0/0`
    *   **P2WPKH (Bech32):** `m/84'/0'/0'/0/0`
    *   **P2WSH (Native SegWit Script-Hash):** `m/87'/0'/0'/0/0`
    *   **P2TR (Taproot, Bech32m):** `m/86'/0'/0'/0/0`
*   Outputs a CSV file containing the derived keys and addresses.
*   Uses the `zeroize` crate to securely wipe private keys from memory after use.

## Usage

### Prerequisites

*   [Rust](https://www.rust-lang.org/tools/install)

### Building the Project

1.  Clone the repository:
    ```bash
    git clone <repository-url>
    ```
2.  Navigate to the project directory:
    ```bash
    cd btc-key-deriver
    ```
3.  Build the project in release mode:
    ```bash
    cargo build --release
    ```

### Running the Program

1.  Create a file named `seeds.txt` (or any other name) in the project directory and add your 12-word BIP-39 seed phrases, one per line. For example:

    ```
    motor venture dilemma quote subject magnet keep large dry gossip bean paper
    ```

2.  Run the program with the following command, specifying the input and output files:

    ```bash
    cargo run --release -- --input seeds.txt --output btc_keys.csv
    ```

    The derived keys and addresses will be saved in the `btc_keys.csv` file.

## Output Format

The output CSV file will have the following columns:

*   `seed_index`: The 1-based index of the seed phrase in the input file.
*   `seed`: The 12-word BIP-39 seed phrase.
*   `derivation_path`: The derivation path used to derive the keys.
*   `address_index`: The address index (always 0 in this version).
*   `address`: The derived Bitcoin address.
*   `public_key`: The compressed public key in hex format.
*   `private_key`: The private key in hex format.
*   `private_key_wif`: The private key in Wallet Import Format (WIF).
*   `script_semantics`: The script semantics (P2PKH, P2SH, P2WPKH, P2WSH, or P2TR).
