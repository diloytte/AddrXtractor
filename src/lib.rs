mod ethereum_based;
mod solana;
mod tron;

pub use ethereum_based::extract_ethereum_based_address;
pub use solana::extract_solana_address;
pub use tron::extract_tron_address;

/// A collection of functions that extract blockchain addresses.
///
/// Currently supports Ethereum-based, Solana, and Tron addresses.
/// Note: These functions only extract the "address" portion and do not fully extract
/// ERC20/SPL token contract addresses.
///
/// TODO: Improve extraction to handle ERC20/SPL token address and not just any addrss..
pub static FUNCTIONS: &[fn(&str) -> Option<String>] = &[
    extract_ethereum_based_address,
    extract_solana_address,
    extract_tron_address,
];

/// Extracts the first token address (Ethereum-based, Solana, or Tron) found in a given text.
///
/// This function iterates through different blockchain address extraction functions
/// and returns the first valid match found.
///
/// # Arguments
///
/// * `text` - A string slice containing the input text to search.
///
/// # Returns
///
/// * `Some(String)` containing the first valid blockchain address found.
/// * `None` if no valid address is found.
///
/// # Examples
///
/// ```
/// let text = "Some text with an Ethereum address: 0xAb5801a7D398351b8bE11C439e05C5b3259aec9B";
/// let result = extract_token_address_from_message_text(text);
/// assert_eq!(result, Some("0xAb5801a7D398351b8bE11C439e05C5b3259aec9B".to_string()));
/// ```
pub fn extract_token_address_from_message_text(text: &str) -> Option<String> {
    let mut final_token_address: Option<String> = None;
    let len = FUNCTIONS.len();

    for i in 0..len {
        let extractor_function = FUNCTIONS.get(i);
        let extracted_address_option = extractor_function.unwrap()(text);
        final_token_address = extracted_address_option;

        if final_token_address.is_some() {
            break;
        }
    }

    final_token_address
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_ethereum_address() {
        let ethereum_text =
            "Check this Ethereum address 0x5f8F8E1dbB5bF65E3aF5F5dF8F8F8F8F8F8F8F8F test test test";
        let expected_address = Some("0x5f8F8E1dbB5bF65E3aF5F5dF8F8F8F8F8F8F8F8F".to_string());

        let result = extract_token_address_from_message_text(ethereum_text);

        assert_eq!(result, expected_address);
    }

    #[test]
    fn test_extract_solana_address() {
        let solana_text =
            "Solana address: 3KZs8bDozYngwMY52VrggD2nzAeNGiDTzKpHWZ4gN1Fq test test test";
        let expected_address = Some("3KZs8bDozYngwMY52VrggD2nzAeNGiDTzKpHWZ4gN1Fq".to_string());

        let result = extract_token_address_from_message_text(solana_text);

        assert_eq!(result, expected_address);
    }

    #[test]
    fn test_extract_tron_address() {
        let tron_text = "Tron address: TQ7r4rYjTnso3j7KwhZZfnZZG2eDZ6fvB7 test test test";
        let expected_address = Some("TQ7r4rYjTnso3j7KwhZZfnZZG2eDZ6fvB7".to_string());

        let result = extract_token_address_from_message_text(tron_text);

        assert_eq!(result, expected_address);
    }

    #[test]
    fn test_extract_token_address_from_message_text_no_address() {
        let text = "No valid address here!";

        let result = extract_token_address_from_message_text(text);

        assert_eq!(result, None);
    }
}
