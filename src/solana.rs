use crate::{regexes::SOLANA_ADDRESS_REGEX, Blockchain};

/// Extracts the first Solana address from a given text and identifies the blockchain.
///
/// This function searches the input text for a valid Solana address, which is typically
/// a 44-character-long base58 string (alphanumeric, case-sensitive).
/// If a valid address is found, it returns the first match along with the
/// `Blockchain::Solana` enum variant.
///
/// # Arguments
///
/// * `text` - A string slice containing the input text to search.
///
/// # Returns
///
/// * `(Some(String), Blockchain::Solana)` if a valid Solana address is found.
/// * `(None, Blockchain::Solana)` if no valid address is found.
///
/// # Examples
///
/// ```
/// use token_address_extractor::{extract_solana_address, Blockchain};
///
/// let text = "A sample text with a Solana address: frhb8l7y9qq41qzxyltc2nw8an1rjfllxrf2x9rwllmo";
/// let (address, blockchain) = extract_solana_address(text);
/// assert_eq!(address, Some("frhb8l7y9qq41qzxyltc2nw8an1rjfllxrf2x9rwllmo".to_string()));
/// assert_eq!(blockchain, Blockchain::Solana);
/// ```

pub fn extract_solana_address(text: &str) -> (Option<String>,Blockchain) {
    let pattern = &SOLANA_ADDRESS_REGEX;
    (pattern.find(text).map(|m| m.as_str().to_string()),Blockchain::Solana)
}


pub fn extract_all_solana_addresses(text: &str) -> Vec<String> {
    let pattern = &SOLANA_ADDRESS_REGEX;
    pattern.find_iter(text).map(|m| m.as_str().to_string()).collect()
}

///
/// 
/// best entry on $DB
// Such a bullish chart
// https://axiom.trade/meme/4mLVKoaTB8C2KotdrmKtU8ryx95QZaztNRScjBRr7PjE
// 43SXvpf4c41t2uErsw7aL6w5qhnie6BXSSPqiTcTpump
// ovo mora zbog ovog razloga, nadji jednostavno sve.


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
        assert_eq!(result.0, expected_address);
    }

    #[test]
    fn test_no_addresses() {
        let input = "
            This is a test with no valid Solana addresses!
        ";

        let expected_address = None;

        let result = extract_solana_address(input);
        assert_eq!(result.0, expected_address);
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
        assert_eq!(result.0, expected_address);
    }

    #[test]
    fn test_link() {
        let input = "https://dexscreener.com/solana/6UeL7hzjCzKBqKap8vtay6SfCaCkQAUpidTWayrwpump";
        let expected_address = Some("6UeL7hzjCzKBqKap8vtay6SfCaCkQAUpidTWayrwpump".to_string());

        let result = extract_solana_address(input);
        assert_eq!(expected_address, result.0);
    }

    #[test]
    fn test_link_photon() {
        let input = "https://photon-sol.tinyastro.io/en/lp/FtpuprhMrBqhEGTTTiFZDHRnpwiAU2ryAN8VJ7G1Dhyy?handle=13750337e9c16b15406821";
        let expected_address = Some("FtpuprhMrBqhEGTTTiFZDHRnpwiAU2ryAN8VJ7G1Dhyy".to_string());

        let result = extract_solana_address(input);
        assert_eq!(expected_address, result.0);
    }


    #[test]
    fn test_43() {
        let input = "test test test CA:J73GTrhWEofgZqBjLLaxA9uN63urYJoaUcqjxMppump $HDR test test test";
        let expected_address = Some("J73GTrhWEofgZqBjLLaxA9uN63urYJoaUcqjxMppump".to_string());

        let result = extract_solana_address(input);
        assert_eq!(expected_address, result.0);
    }


    #[test]
    fn test_dex() {
        let input = "https://dexscreener.com/solana/dcqnsnwcblgeyw6vgbpnlzrr8pbbjovergra8qapguhw";
        let expected_address = Some("dcqnsnwcblgeyw6vgbpnlzrr8pbbjovergra8qapguhw".to_string());

        let result = extract_solana_address(input);
        assert_eq!(expected_address, result.0);
    }

    #[test]
    fn test_extract_all_solana_addresses_none() {
        let input = "There are no valid Solana addresses here!";
        let result = extract_all_solana_addresses(input);
        assert!(result.is_empty());
    }

    #[test]
    fn test_extract_all_solana_addresses_single() {
        let input = "Here is one: frhb8l7y9qq41qzxyltc2nw8an1rjfllxrf2x9rwllmo";
        let expected = vec!["frhb8l7y9qq41qzxyltc2nw8an1rjfllxrf2x9rwllmo".to_string()];
        let result = extract_all_solana_addresses(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_extract_all_solana_addresses_multiple() {
        let input = "
            frhb8l7y9qq41qzxyltc2nw8an1rjfllxrf2x9rwllmo
            some random text
            4nLgH9D5bPQoGeXkP9eXhCTRYD5U5YBKnPXaM1D9U6uj
            another line
            dcqnsnwcblgeyw6vgbpnlzrr8pbbjovergra8qapguhw
        ";
        let expected = vec![
            "frhb8l7y9qq41qzxyltc2nw8an1rjfllxrf2x9rwllmo".to_string(),
            "4nLgH9D5bPQoGeXkP9eXhCTRYD5U5YBKnPXaM1D9U6uj".to_string(),
            "dcqnsnwcblgeyw6vgbpnlzrr8pbbjovergra8qapguhw".to_string(),
        ];
        let result = extract_all_solana_addresses(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_extract_all_solana_addresses_with_links() {
        let input = "
            https://dexscreener.com/solana/6UeL7hzjCzKBqKap8vtay6SfCaCkQAUpidTWayrwpump
            https://photon-sol.tinyastro.io/en/lp/FtpuprhMrBqhEGTTTiFZDHRnpwiAU2ryAN8VJ7G1Dhyy
        ";
        let expected = vec![
            "6UeL7hzjCzKBqKap8vtay6SfCaCkQAUpidTWayrwpump".to_string(),
            "FtpuprhMrBqhEGTTTiFZDHRnpwiAU2ryAN8VJ7G1Dhyy".to_string(),
        ];
        let result = extract_all_solana_addresses(input);
        assert_eq!(result, expected);
    }

}
