use crate::{regexes::TRON_ADDRESS_REGEX, Blockchain};

/// Extracts the first Tron address from a given text and identifies the blockchain.
///
/// This function searches the input text for a valid Tron address, which must start with `'T'`
/// followed by exactly 33 alphanumeric base58 characters.
/// If a valid address is found, it returns the first match along with the `Blockchain::Tron` enum variant.
///
/// # Arguments
///
/// * `text` - A string slice containing the input text to search.
///
/// # Returns
///
/// * `(Some(String), Blockchain::Tron)` if a valid Tron address is found.
/// * `(None, Blockchain::Tron)` if no valid address is found.
///
/// # Examples
///
/// ```
/// use token_address_extractor::{extract_tron_address, Blockchain};
///
/// let text = "Some text with a Tron address: TLa2f6VPqDgRE67v1736s7bJ8Ray5wYjU7";
/// let (address, blockchain) = extract_tron_address(text);
/// assert_eq!(address, Some("TLa2f6VPqDgRE67v1736s7bJ8Ray5wYjU7".to_string()));
/// assert_eq!(blockchain, Blockchain::Tron);
/// ```

pub fn extract_tron_address(text: &str) -> (Option<String>,Blockchain) {
    let pattern = &TRON_ADDRESS_REGEX;
    (pattern.find(text).map(|m| m.as_str().to_string()),Blockchain::Tron)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_tron_address() {
        let input = "
            Some random text
            TLa2f6VPqDgRE67v1736s7bJ8Ray5wYjU7
            more random text
        ";

        let expected_address = Some("TLa2f6VPqDgRE67v1736s7bJ8Ray5wYjU7".to_string());

        let result = extract_tron_address(input);
        assert_eq!(result.0, expected_address);
    }

    #[test]
    fn test_no_addresses() {
        let input = "This is a test with no valid Tron addresses!";
        let expected_address = None;

        let result = extract_tron_address(input);
        assert_eq!(result.0, expected_address);
    }

    #[test]
    fn test_multiple_addresses() {
        let input = "
            TLa2f6VPqDgRE67v1736s7bJ8Ray5wYjU7
            TPL66VynYgJoq4AuirFx9pPJTtL2PdRsy4
        ";

        // The function should return only the first valid Tron address found.
        let expected_address = Some("TLa2f6VPqDgRE67v1736s7bJ8Ray5wYjU7".to_string());

        let result = extract_tron_address(input);
        assert_eq!(result.0, expected_address);
    }

    #[test]

    fn test_link() {
        let input = "https://dexscreener.com/tron/TDECSYjDTLFVRtNHeydKkKJq6kXU9hHDex";
        let expected_address = Some("TDECSYjDTLFVRtNHeydKkKJq6kXU9hHDex".to_string());

        let result = extract_tron_address(input);
        assert_eq!(expected_address, result.0);
    }
}
