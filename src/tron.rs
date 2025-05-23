use crate::regexes::TRON_ADDRESS_REGEX;

/// Extracts the first Tron address from a given text.
///
/// This function searches for Tron addresses, which start with 'T' followed by 33 alphanumeric characters.
/// If an address is found, it returns the first occurrence as a `String`. Otherwise, it returns `None`.
///
/// # Arguments
///
/// * `text` - A string slice containing the input text to search.
///
/// # Returns
///
/// * `Some(String)` containing the first Tron address found.
/// * `None` if no valid address is found.
///
/// # Examples
///
/// ```
/// use token_address_extractor::extract_tron_address;
/// 
/// let text = "Some text with a Tron address: TLa2f6VPqDgRE67v1736s7bJ8Ray5wYjU7";
/// let result = extract_tron_address(text);
/// assert_eq!(result, Some("TLa2f6VPqDgRE67v1736s7bJ8Ray5wYjU7".to_string()));
/// ```
///
pub fn extract_tron_address(text: &str) -> Option<String> {
    let pattern = &TRON_ADDRESS_REGEX;
    pattern.find(text).map(|m| m.as_str().to_string())
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
        assert_eq!(result, expected_address);
    }

    #[test]
    fn test_no_addresses() {
        let input = "This is a test with no valid Tron addresses!";
        let expected_address = None;

        let result = extract_tron_address(input);
        assert_eq!(result, expected_address);
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
        assert_eq!(result, expected_address);
    }

    #[test]

    fn test_link() {
        let input = "https://dexscreener.com/tron/TDECSYjDTLFVRtNHeydKkKJq6kXU9hHDex";
        let expected_address = Some("TDECSYjDTLFVRtNHeydKkKJq6kXU9hHDex".to_string());

        let result = extract_tron_address(input);
        assert_eq!(expected_address, result);
    }
}
