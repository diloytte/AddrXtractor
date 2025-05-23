use crate::regexes::ETHEREUM_ADDRESS_REGEX;

/// Extracts the first Ethereum-based address from a given text.
///
/// This function looks for Ethereum addresses that start with `0x` followed by exactly 40 hexadecimal characters.
/// If an address is found, it returns the first occurrence as a `String`. Otherwise, it returns `None`.
///
/// # Arguments
///
/// * `text` - A string slice containing the input text to search.
///
/// # Returns
///
/// * `Some(String)` containing the first Ethereum address found.
/// * `None` if no valid address is found.
///
/// # Examples
///
/// ```
/// use token_address_extractor::extract_ethereum_based_address;
/// 
/// let text = "Random text with an address: 0xAb5801a7D398351b8bE11C439e05C5b3259aec9B";
/// let result = extract_ethereum_based_address(text);
/// assert_eq!(result, Some("0xAb5801a7D398351b8bE11C439e05C5b3259aec9B".to_string()));
/// ```
pub fn extract_ethereum_based_address(text: &str) -> Option<String> {
    let pattern = &ETHEREUM_ADDRESS_REGEX;
    pattern.find(text).map(|m| m.as_str().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_ethereum_address() {
        let input = "
            Some random text
            0xAb5801a7D398351b8bE11C439e05C5b3259aec9B
            more random text
        ";

        let expected_address = Some("0xAb5801a7D398351b8bE11C439e05C5b3259aec9B".to_string());

        let result = extract_ethereum_based_address(input);
        assert_eq!(result, expected_address);
    }

    #[test]
    fn test_no_addresses() {
        let input = "This is a test with no valid Ethereum addresses!";
        let expected_address = None;

        let result = extract_ethereum_based_address(input);
        assert_eq!(result, expected_address);
    }

    #[test]
    fn test_link() {
        let input = "https://dexscreener.com/bsc/0x2c46eb9820f048eeba1ce7b1dcfd302916284dad";
        let expected_address = Some("0x2c46eb9820f048eeba1ce7b1dcfd302916284dad".to_string());

        let result = extract_ethereum_based_address(input);
        assert_eq!(expected_address, result);
    }

    #[test]
    fn test_multiple_addresses() {
        let input = "
            0xAb5801a7D398351b8bE11C439e05C5b3259aec9B
            0x4E9ce36E442e55EcD9025B9a6E0D88485d628A67
        ";

        // The function should return only the first valid Ethereum address found.
        let expected_address = Some("0xAb5801a7D398351b8bE11C439e05C5b3259aec9B".to_string());

        let result = extract_ethereum_based_address(input);
        assert_eq!(result, expected_address);
    }
}
