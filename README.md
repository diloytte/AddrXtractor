# Extract Blockchain Addresses

This Rust crate provides functions to extract blockchain addresses from a given text input. Currently, it supports:

- **Ethereum-based addresses** (e.g., Ethereum, Binance Smart Chain, etc.)
- **Solana addresses**
- **Tron addresses**

## Installation

To use this crate in your Rust project, add the following dependency to your `Cargo.toml` file:

```toml
[dependencies]
extract-blockchain-address = "1.2.0"
```

## Usage

Below is an example of how to use this crate to extract blockchain addresses from a given text:

```rust
use extract_blockchain_address::extract_token_address_from_message_text;

fn main() {
    let text = "This message contains an Ethereum address: 0xAb5801a7D398351b8bE11C439e05C5b3259aec9B";
    
    if let Some(address) = extract_token_address_from_message_text(text) {
        println!("Found address: {}", address);
    } else {
        println!("No valid blockchain address found.");
    }
}
```

## Functions

### `extract_token_address_from_message_text(text: &str) -> Option<String>`

This function scans the given text and extracts the **first** blockchain address found. It supports Ethereum-based, Solana, and Tron addresses.

#### Example
```rust
let text = "Some Solana address: frhb8l7y9qq41qzxyltc2nw8an1rjfllxrf2x9rwllmo";
let result = extract_token_address_from_message_text(text);
assert_eq!(result, Some("frhb8l7y9qq41qzxyltc2nw8an1rjfllxrf2x9rwllmo".to_string()));
```
 
## Notes

- This crate **only extracts addresses**, not ERC-20/SPL token contract addresses.
- If multiple addresses are present, it **returns the first one** found in the text.

## License

This project is licensed under the MIT License.

