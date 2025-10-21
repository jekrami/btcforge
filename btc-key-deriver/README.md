# BTC Key Deriver

A high-performance command-line tool written in Rust for deriving Bitcoin keys and addresses from BIP-39 seed phrases. Optimized for batch processing with flexible output options.

**Version:** 1.0.0

## Overview

BTC Key Deriver is designed to efficiently generate Bitcoin addresses and keys from BIP-39 mnemonic phrases. It supports multiple derivation paths and address types, making it suitable for wallet management, address generation, and balance checking workflows.

### Key Features

- ✅ **Fast Batch Processing**: Process multiple seed phrases efficiently
- ✅ **Multiple Derivation Paths**: Support for standard BIP-44, BIP-49, BIP-84 paths
- ✅ **Multiple Address Types**: P2PKH (Legacy), P2WPKH-nested-in-P2SH, and P2WPKH (Bech32)
- ✅ **10 Addresses per Path**: Generates indices 0-9 for each derivation path
- ✅ **Flexible Output**: Optional full details or addresses-only mode
- ✅ **Mandatory Address Export**: Always generates addresses-only file for balance checking
- ✅ **Secure Key Handling**: Uses `zeroize` crate to securely wipe private keys from memory
- ✅ **CSV Format**: Easy integration with databases and balance checking tools

## Derivation Paths

The tool generates addresses for the following derivation paths:

### Standard Paths (10 addresses each)

1. **`m/0'/0'/0'` to `m/0'/0'/9'`** (P2PKH - Legacy)
   - Legacy Bitcoin addresses starting with `1`
   - Example: `1GyNWR7LPXdLSHeN4nE4b9P3gNEcjZkmzd`

2. **`m/44'/0'/0'/0/0'` to `m/44'/0'/0'/0/9'`** (P2PKH - BIP-44)
   - Standard BIP-44 derivation for legacy addresses
   - Example: `1Jo3qrSUxWYYJdhDawJ58QU7wtyVtqAK5A`

3. **`m/49'/0'/0'/0/0'` to `m/49'/0'/0'/0/9'`** (P2WPKH-nested-in-P2SH - BIP-49)
   - SegWit addresses wrapped in P2SH (backward compatible)
   - Addresses starting with `3`
   - Example: `33ML21FE9QSqh9wizdQbZsHfE41vwkRT78`

4. **`m/84'/0'/0'/0/0'` to `m/84'/0'/0'/0/9'`** (P2WPKH - BIP-84)
   - Native SegWit (Bech32) addresses
   - Addresses starting with `bc1q`
   - Example: `bc1qnc9umhdc04u0u5qfg0qu3aj75wvfps4z4sj7g6`

### Additional Paths (10 addresses each)

5. **`m/0/0'` to `m/0/9'`** (P2WPKH-nested-in-P2SH)
   - Alternative derivation for nested SegWit
   - Addresses starting with `3`

6. **`m/0/0'` to `m/0/9'`** (P2WPKH)
   - Alternative derivation for native SegWit
   - Addresses starting with `bc1q`

**Total: 60 addresses per seed phrase**

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.56 or later)

### Building from Source

```bash
# Clone the repository
git clone <repository-url>
cd btc-key-deriver

# Build in release mode
cargo build --release

# Binary location: target/release/btc-key-deriver.exe (Windows) or target/release/btc-key-deriver (Linux/macOS)
```

## Usage

### Command-Line Options

```
USAGE:
    btc-key-deriver [OPTIONS] --input <INPUT>

OPTIONS:
    -i, --input <INPUT>              Input file with BIP-39 seed phrases (one per line) [REQUIRED]
    -a, --addresses <ADDRESSES>      Output file for addresses only [default: addressonly.txt]
    -o, --output <OUTPUT>            Output file for full details (keys, addresses, paths) [OPTIONAL]
    -f, --full                       Generate full output file with all details [OPTIONAL FLAG]
    -h, --help                       Print help information
    -V, --version                    Print version
```

### Usage Examples

#### 1. **Batch Mode - Addresses Only (Recommended for Balance Checking)**

Generate only the addresses file (fastest, minimal output):

```bash
./btc-key-deriver -i seeds.txt
```

**Output:**
- `addressonly.txt` - 60 addresses (one per line)

**Use Case:** Checking balances against a database, importing to balance checking tools

---

#### 2. **Batch Mode - Custom Addresses Filename**

```bash
./btc-key-deriver -i seeds.txt -a my_addresses.txt
```

**Output:**
- `my_addresses.txt` - 60 addresses

---

#### 3. **With Full Output Details**

Generate both addresses and full details:

```bash
./btc-key-deriver -i seeds.txt -o output.txt
```

**Output:**
- `addressonly.txt` - 60 addresses
- `output.txt` - Full details (paths, addresses, public keys, private keys)

---

#### 4. **Full Control - Custom Filenames**

```bash
./btc-key-deriver -i seeds.txt -a batch_addresses.txt -o batch_output.txt --full
```

**Output:**
- `batch_addresses.txt` - 60 addresses
- `batch_output.txt` - Full details

---

#### 5. **Force Full Output Without Custom Output File**

```bash
./btc-key-deriver -i seeds.txt --full
```

**Output:**
- `addressonly.txt` - 60 addresses
- Full details written to `addressonly.txt`

---

### Input File Format

Create a text file with BIP-39 seed phrases, one per line:

```
motor venture dilemma quote subject magnet keep large dry gossip bean paper
abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about
```

### Output File Formats

#### Addresses-Only File (`addressonly.txt`)

Simple CSV format with header:

```
address
1GyNWR7LPXdLSHeN4nE4b9P3gNEcjZkmzd
1GPruf7qZWTKbAmUH351MAwNpMVqJHjfUT
1LDZC5cG5ZJrDny7shvkf4BRbcwGyqMmXj
...
bc1qnh34wzxmq9u80w5gtsrwuqq2my9yj8erm44ugq
```

**Use Cases:**
- Import into balance checking databases
- Bulk address monitoring
- Wallet address verification

---

#### Full Output File (`output.txt`)

Detailed format with BIP39 info and all derivation paths:

```
BIP39 Mnemonic: motor venture dilemma quote subject magnet keep large dry gossip bean paper

BIP39 Seed:24bd1b243ec776dd97bc7487ad65c8966ff6e0b8654a25602a41994746957c49c813ba183e6d1646584cf810fcb9898f44571e3ccfe9fb266e3a66597fbcd7c4

Coin: BTC

Derivation Path and outputs

path,address,public key,private key

m/0'/0'/0',1GyNWR7LPXdLSHeN4nE4b9P3gNEcjZkmzd,0294f267b6174c3694da97f7e554069a7ef475a699753d9c7b568cc35fb0184a4d,KyNSzr7jYueYWvsg4cKhwQEmrXCwYmkVAc4qpUX3NU6AqqyNSK7X
m/0'/0'/1',1GPruf7qZWTKbAmUH351MAwNpMVqJHjfUT,0386bed3c7eac5487da18d35f1712e70a1770efe1b0afede80c79ecadcd39e0cd1,L1kzjvx2T4XNxpxVSUvKkCujDNZ1ex5iiRo3uGVEo8wMavEP79pd
...

Script Semantics: P2WPKH nested in P2SH
m/0/0',3HWZMAtc7MyENWguyhWaLrLjXpWTMpfZLh,028d59eab375e2cbc7de3539c18590f7b1ce121702bfaa5e9e92e2b715549ed283,L3V5wXPbC7VmDyh53LUPmYa28yRPz3Vu9Qwmkm6wcU3n8x8aRtDd
...

Script Semantics: P2WPKH
m/0/0',bc1qe59ssevhzy9v76syff0508ml97xm0rstcfdw0y,028d59eab375e2cbc7de3539c18590f7b1ce121702bfaa5e9e92e2b715549ed283,L3V5wXPbC7VmDyh53LUPmYa28yRPz3Vu9Qwmkm6wcU3n8x8aRtDd
...
```

**Columns:**
- `path` - Derivation path (e.g., `m/0'/0'/0'`)
- `address` - Bitcoin address
- `public key` - Compressed public key (hex)
- `private key` - Private key in WIF format

---

## Address Types Explained

### P2PKH (Pay-to-Public-Key-Hash) - Legacy
- **Address Format:** Starts with `1`
- **Example:** `1GyNWR7LPXdLSHeN4nE4b9P3gNEcjZkmzd`
- **Pros:** Widely supported, oldest format
- **Cons:** Larger transaction size, higher fees

### P2WPKH-nested-in-P2SH (Nested SegWit)
- **Address Format:** Starts with `3`
- **Example:** `33ML21FE9QSqh9wizdQbZsHfE41vwkRT78`
- **Pros:** SegWit benefits with backward compatibility
- **Cons:** Slightly larger than native SegWit

### P2WPKH (Native SegWit - Bech32)
- **Address Format:** Starts with `bc1q`
- **Example:** `bc1qnc9umhdc04u0u5qfg0qu3aj75wvfps4z4sj7g6`
- **Pros:** Smallest transaction size, lowest fees, modern standard
- **Cons:** Not supported by all older wallets

---

## Batch Processing Workflow

### Example: Balance Checking Pipeline

```bash
# 1. Generate addresses from seed phrases
./btc-key-deriver -i seeds.txt -a addresses.txt

# 2. Import addresses.txt into your balance checking database
# 3. Query balances for all addresses
# 4. Identify addresses with funds

# For detailed information (if needed):
./btc-key-deriver -i seeds.txt -o details.txt
```

---

## Security Considerations

⚠️ **Important Security Notes:**

1. **Private Key Handling:**
   - Private keys are only displayed in the full output file
   - Use `--full` flag only when necessary
   - Store output files securely
   - Never share output files containing private keys

2. **Memory Security:**
   - The tool uses the `zeroize` crate to securely wipe private keys from memory
   - Private keys are overwritten with zeros after use

3. **File Security:**
   - Protect output files with appropriate file permissions
   - Consider encrypting files containing private keys
   - Delete files after use if not needed

4. **Seed Phrase Security:**
   - Keep seed phrases secure and offline
   - Never share seed phrases
   - Use hardware wallets for long-term storage

---

## Performance

- **Processing Speed:** ~100-200 seed phrases per second (depends on system)
- **Memory Usage:** Minimal (~10-20 MB)
- **Output Size:** ~2.3 KB per seed phrase (addresses-only), ~10.6 KB (full details)

---

## Dependencies

- `bip39` - BIP-39 mnemonic support
- `bitcoin` - Bitcoin address and key generation
- `clap` - Command-line argument parsing
- `hex` - Hexadecimal encoding/decoding
- `zeroize` - Secure memory wiping

---

## Troubleshooting

### Issue: "Invalid seed phrase"
**Solution:** Ensure the seed phrase is exactly 12 words separated by spaces

### Issue: "File not found"
**Solution:** Verify the input file path is correct and the file exists

### Issue: "Permission denied" on output file
**Solution:** Check file permissions and ensure the directory is writable

---

## Version History

### v1.0.0 (Current)
- ✅ Flexible output options (addresses-only or full details)
- ✅ Mandatory addresses-only file generation
- ✅ Support for 6 derivation paths with 10 addresses each (60 total)
- ✅ Optimized for batch processing
- ✅ Custom output filenames
- ✅ Comprehensive documentation

### v0.1.0
- Initial release with basic functionality

---

## License

[Add your license here]

---

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

---

## Support

For issues, questions, or suggestions, please open an issue on the repository.
