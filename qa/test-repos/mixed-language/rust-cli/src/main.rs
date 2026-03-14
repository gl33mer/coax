//! Rust CLI Utility
//! Command-line tools for DevShield
//!
//! WARNING: Contains intentional secrets for testing.

use std::env;

// GitHub Token (CRITICAL - should be detected)
const GITHUB_TOKEN: &str = "ghp_1234567890abcdefghij1234567890abcdefghij";

// DigitalOcean Token (HIGH - should be detected)
const DIGITALOCEAN_TOKEN: &str = "dop_v1_abcdefghijklmnopqrstuvwxyz1234567890abcdefghijklmn";

/// Main entry point
fn main() {
    println!("DevShield Rust CLI");
    println!("GitHub Token configured: {}", !GITHUB_TOKEN.is_empty());
    
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("Arguments: {:?}", &args[1..]);
    }
}

/// Get GitHub token
pub fn get_github_token() -> &'static str {
    GITHUB_TOKEN
}

/// Get DigitalOcean token
pub fn get_digitalocean_token() -> &'static str {
    DIGITALOCEAN_TOKEN
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokens_exist() {
        assert!(!get_github_token().is_empty());
        assert!(!get_digitalocean_token().is_empty());
    }
}
