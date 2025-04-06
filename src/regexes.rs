use once_cell::sync::Lazy;
use regex::Regex;

pub static SOLANA_ADDRESS_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b[a-zA-Z0-9]{44}\b").unwrap()
});

pub static TRON_ADDRESS_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\bT[a-zA-Z0-9]{33}\b").unwrap()
});

pub static ETHEREUM_ADDRESS_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b0x[a-fA-F0-9]{40}\b").unwrap()
});