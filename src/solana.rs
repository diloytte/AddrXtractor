use regex::Regex;

/// Extracts the first Solana address from a given text.
///
/// This function searches for Solana addresses, which are 44-character-long alphanumeric strings.
/// If an address is found, it returns the first occurrence as a `String`. Otherwise, it returns `None`.
///
/// # Arguments
///
/// * `text` - A string slice containing the input text to search.
///
/// # Returns
///
/// * `Some(String)` containing the first Solana address found.
/// * `None` if no valid address is found.
///
/// # Examples
///
/// ```
/// let text = "A sample text with a Solana address: frhb8l7y9qq41qzxyltc2nw8an1rjfllxrf2x9rwllmo";
/// let result = extract_solana_address(text);
/// assert_eq!(result, Some("frhb8l7y9qq41qzxyltc2nw8an1rjfllxrf2x9rwllmo".to_string()));
/// ```
pub fn extract_solana_address(text: &str) -> Option<String> {
    let pattern = Regex::new(r"\b[a-zA-Z0-9]{44}\b").unwrap();
    pattern.find(text).map(|m| m.as_str().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_solana_address() {
        let input = "
            DijafwofOIJIFJWOIIWFIWOFJFJWF
            fewfkwwepfwkweff
            dqwkdqdqe21r-3kkrr09kr290k90dsad
            frhb8l7y9qq41qzxyltc2nw8an1rjfllxrf2x9rwllmo
            dpqwdwqodqdqw
            wd
        ";

        let expected_address = Some("frhb8l7y9qq41qzxyltc2nw8an1rjfllxrf2x9rwllmo".to_string());

        let result = extract_solana_address(input);
        assert_eq!(result, expected_address);
    }

    #[test]
    fn test_no_addresses() {
        let input = "
            This is a test with no valid Solana addresses!
        ";

        let expected_address = None;

        let result = extract_solana_address(input);
        assert_eq!(result, expected_address);
    }

    #[test]
    fn test_multiple_addresses() {
        let input = "
            frhb8l7y9qq41qzxyltc2nw8an1rjfllxrf2x9rwllmo
            anotherinvalidstring
            4nLgH9D5bPQoGeXkP9eXhCTRYD5U5YBKnPXaM1D9U6uj
        ";

        // The function should return only the first valid address found.
        let expected_address = Some("frhb8l7y9qq41qzxyltc2nw8an1rjfllxrf2x9rwllmo".to_string());

        let result = extract_solana_address(input);
        assert_eq!(result, expected_address);
    }

    #[test]
    fn test_link() {
        let input = "https://dexscreener.com/solana/6UeL7hzjCzKBqKap8vtay6SfCaCkQAUpidTWayrwpump";
        let expected_address = Some("6UeL7hzjCzKBqKap8vtay6SfCaCkQAUpidTWayrwpump".to_string());

        let result = extract_solana_address(input);
        assert_eq!(expected_address, result);
    }
}
