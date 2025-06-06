[1.2.2] - 2025-06-06
Changed
Updated global function for extraction to return blockchain parameter.
[1.2.1] - 2025-06-06
Chanaged
Updated all `extract_*_address` functions to return a tuple `(Option<String>, Blockchain)` instead of just `Option<String>`.

[1.2.0] - 2025-04-15
Changed  
Added extracting multiple solana address trought new `extract_all_solana_addresses` function.

[1.1.0] - 2025-04-10
Changed  
Added so that Solana address can be either 43 or 44 characters length.

[1.0.1] - 2025-04-10
Changed  
Updated README.md.

[1.0.0] - 2025-04-06
Changed  
Optimized address validation for Solana, Ethereum-based, and Tron addresses with precompiled regex patterns, improving performance while maintaining the same logic.