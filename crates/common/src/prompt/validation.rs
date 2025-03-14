use std::{path::PathBuf, str::FromStr};
use url::Url;

pub fn validate_filename(val: &String) -> Result<(), String> {
    // Empty check (if needed, based on your requirements)
    if val.is_empty() {
        return Ok(());
    }

    // Basic path validation
    if PathBuf::from_str(val).is_err() {
        return Err("Invalid filename format".to_string());
    }

    // Length check
    if val.len() > 255 {
        return Err("Filename is too long (max 255 characters)".to_string());
    }

    // Invalid characters check
    let invalid_chars = ['/', '\\', '<', '>', ':', '"', '|', '?', '*'];
    if let Some(invalid_char) = val.chars().find(|c| invalid_chars.contains(c)) {
        return Err(format!(
            "Filename contains invalid character: {}",
            invalid_char
        ));
    }

    // Space and period checks
    if val.starts_with(' ') || val.ends_with(' ') {
        return Err("Filename cannot start or end with spaces".to_string());
    }

    // Extension check
    if !val.ends_with(".toml") {
        return Err("File must have a .toml extension".to_string());
    }

    Ok(())
}

pub fn validate_url(val: &String) -> Result<(), String> {
    // Empty check
    if val.is_empty() {
        return Ok(());
    }

    // Check if URL starts with valid scheme
    if !val.starts_with("http://")
        && !val.starts_with("https://")
        && !val.starts_with("ws://")
        && !val.starts_with("wss://")
    {
        return Err("URL must start with http://, https://, ws://, or wss://".to_string());
    }

    // Parse URL
    let url = Url::parse(val).map_err(|e| format!("Invalid URL format: {}", e))?;

    // Check for username/password in URL
    if url.username() != "" || url.password().is_some() {
        return Err("URL cannot contain authentication credentials".to_string());
    }

    // Validate host
    let host = url.host_str().ok_or("Missing host")?;

    // Check if host is not empty
    if host.is_empty() {
        return Err("Host cannot be empty".to_string());
    }

    // Check if host is localhost, IP address, or valid hostname
    if host != "localhost" && !is_valid_ip(host) && !is_valid_hostname(host) {
        return Err(
            "Invalid host: must be localhost, valid IP address, or valid hostname".to_string(),
        );
    }

    // Validate port if present
    if let Some(port) = url.port() {
        if port == 0 {
            return Err("Port cannot be 0".to_string());
        }
    }

    Ok(())
}

fn is_valid_ip(host: &str) -> bool {
    // Check IPv4
    if let Ok(segments) = host
        .split('.')
        .map(|s| s.parse::<u8>())
        .collect::<Result<Vec<_>, _>>()
    {
        return segments.len() == 4;
    }

    // Check IPv6 - simplified check
    host.split(':').count() == 8
}

fn is_valid_hostname(host: &str) -> bool {
    // Basic hostname validation rules
    let valid_chars = |c: char| c.is_ascii_alphanumeric() || c == '-' || c == '.';

    // Check length and character validity
    if host.len() > 253 || host.is_empty() {
        return false;
    }

    // Check each label
    for label in host.split('.') {
        if label.is_empty()
            || label.len() > 63
            || label.starts_with('-')
            || label.ends_with('-')
            || !label.chars().all(valid_chars)
        {
            return false;
        }
    }

    true
}

fn validate_hex_string(val: &str, expected_bytes: usize, field_name: &str) -> Result<(), String> {
    // Empty check
    if val.is_empty() {
        return Ok(());
    }

    // Check '0x' prefix
    if !val.starts_with("0x") {
        return Err(format!("{} must start with '0x'", field_name));
    }

    // Validate hex characters
    if !val[2..].chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(format!(
            "{} must contain only hexadecimal characters",
            field_name
        ));
    }

    // Check length (0x + expected_bytes * 2 hex chars)
    let expected_length = 2 + (expected_bytes * 2);
    if val.len() != expected_length {
        return Err(format!(
            "{} must be {} characters long (including '0x')",
            field_name, expected_length
        ));
    }

    Ok(())
}

pub fn validate_eth_address(val: &String) -> Result<(), String> {
    validate_hex_string(val, 20, "Ethereum address")
}

pub fn validate_private_key(val: &String) -> Result<(), String> {
    validate_hex_string(val, 32, "Private key")
}

pub fn validate_u64(val: &String) -> Result<(), String> {
    // Empty check
    if val.is_empty() {
        return Ok(());
    }

    // Try to parse as u64
    match val.parse::<u64>() {
        Ok(_) => Ok(()),
        Err(_) => Err("Value must be a valid unsigned 64-bit integer".to_string()),
    }
}

pub fn validate_time_with_unit(val: &String) -> Result<(), String> {
    // Empty check
    if val.is_empty() {
        return Ok(());
    }

    // Check if ends with 's'
    if !val.ends_with('s') {
        return Err("Time value must end with 's' (seconds)".to_string());
    }

    // Remove 's' and try to parse the number
    let number_str = &val[..val.len() - 1];

    // Check if starts with a number
    if !number_str
        .chars()
        .next()
        .map_or(false, |c| c.is_ascii_digit())
    {
        return Err("Time value must start with a number".to_string());
    }

    // Try to parse as f64 first to validate number format
    let number = number_str
        .parse::<f64>()
        .map_err(|_| "Invalid number format for time value".to_string())?;

    // Check if number is positive
    if number <= 0.0 {
        return Err("Time value must be greater than 0".to_string());
    }

    // Check decimal places (if it has decimals)
    if number_str.contains('.') {
        let decimal_places = number_str
            .split('.')
            .nth(1)
            .map(|dec| dec.len())
            .unwrap_or(0);

        if decimal_places > 2 {
            return Err("Time value can have maximum 2 decimal places".to_string());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_filename() {
        // Valid cases
        assert!(validate_filename(&"".to_string()).is_ok());
        assert!(validate_filename(&"config.toml".to_string()).is_ok());
        assert!(validate_filename(&"my_config_123.toml".to_string()).is_ok());
        assert!(validate_filename(&"complex-name_with-symbols.toml".to_string()).is_ok());

        // Invalid cases - wrong extension
        assert!(validate_filename(&"config.txt".to_string()).is_err());
        assert!(validate_filename(&"config".to_string()).is_err());
        assert!(validate_filename(&"config.toml.txt".to_string()).is_err());

        // Invalid cases - spaces
        assert!(validate_filename(&" config.toml".to_string()).is_err());
        assert!(validate_filename(&"config.toml ".to_string()).is_err());
        assert!(validate_filename(&" config.toml ".to_string()).is_err());

        // Invalid cases - special characters
        assert!(validate_filename(&"config/.toml".to_string()).is_err());
        assert!(validate_filename(&"config\\.toml".to_string()).is_err());
        assert!(validate_filename(&"config<.toml".to_string()).is_err());
        assert!(validate_filename(&"config>.toml".to_string()).is_err());
        assert!(validate_filename(&"config:.toml".to_string()).is_err());
        assert!(validate_filename(&"config\".toml".to_string()).is_err());
        assert!(validate_filename(&"config|.toml".to_string()).is_err());
        assert!(validate_filename(&"config?.toml".to_string()).is_err());
        assert!(validate_filename(&"config*.toml".to_string()).is_err());

        // Invalid cases - length
        assert!(validate_filename(&format!("{}.toml", "a".repeat(252))).is_err());
    }

    #[test]
    fn test_validate_url() {
        // Valid URLs
        assert!(validate_url(&"http://localhost:8545".to_string()).is_ok());
        assert!(validate_url(&"https://example.com:8545".to_string()).is_ok());
        assert!(validate_url(&"wss://eth-sepolia.g.alchemy.com/v2/APIKEY".to_string()).is_ok());
        assert!(validate_url(&"http://anvil:8545".to_string()).is_ok());
        assert!(validate_url(&"http://127.0.0.1:8545".to_string()).is_ok());
        assert!(validate_url(&"".to_string()).is_ok());

        // Invalid URLs
        assert!(validate_url(&"invalid-url".to_string()).is_err());
        assert!(validate_url(&"ftp://example.com".to_string()).is_err());
        assert!(validate_url(&"http://:8545".to_string()).is_err());
        assert!(validate_url(&"http://invalid@host:8545".to_string()).is_err());
    }

    #[test]
    fn test_is_valid_hostname() {
        assert!(is_valid_hostname("example.com"));
        assert!(is_valid_hostname("sub-domain.example.com"));
        assert!(is_valid_hostname("anvil"));
        assert!(!is_valid_hostname("-invalid.com"));
        assert!(!is_valid_hostname("invalid-.com"));
        assert!(!is_valid_hostname("invalid..com"));
    }

    #[test]
    fn test_is_valid_ip() {
        assert!(is_valid_ip("127.0.0.1"));
        assert!(is_valid_ip("192.168.1.1"));
        assert!(!is_valid_ip("256.256.256.256"));
        assert!(!is_valid_ip("127.0.0"));
    }

    #[test]
    fn test_validate_eth_address() {
        // Valid addresses
        assert!(validate_eth_address(&"".to_string()).is_ok());
        assert!(
            validate_eth_address(&"0x742d35Cc6634C0532925a3b844Bc454e4438f44e".to_string()).is_ok()
        );
        assert!(
            validate_eth_address(&"0x0000000000000000000000000000000000000000".to_string()).is_ok()
        );

        // Invalid addresses
        assert!(
            validate_eth_address(&"742d35Cc6634C0532925a3b844Bc454e4438f44e".to_string()).is_err()
        ); // Missing 0x
        assert!(
            validate_eth_address(&"0x742d35Cc6634C0532925a3b844Bc454e4438f44".to_string()).is_err()
        ); // Too short
        assert!(
            validate_eth_address(&"0x742d35Cc6634C0532925a3b844Bc454e4438f44ef".to_string())
                .is_err()
        ); // Too long
        assert!(
            validate_eth_address(&"0x742d35Cc6634C0532925a3b844Bc454e4438f44g".to_string())
                .is_err()
        ); // Invalid hex
    }

    #[test]
    fn test_validate_private_key() {
        // Valid private keys
        assert!(validate_private_key(&"".to_string()).is_ok());
        assert!(validate_private_key(
            &"0x0000000000000000000000000000000000000000000000000000000000000001".to_string()
        )
        .is_ok());
        assert!(validate_private_key(
            &"0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141".to_string()
        )
        .is_ok());

        // Invalid private keys
        assert!(validate_private_key(
            &"0000000000000000000000000000000000000000000000000000000000000001".to_string()
        )
        .is_err()); // Missing 0x
        assert!(validate_private_key(
            &"0x00000000000000000000000000000000000000000000000000000000000001".to_string()
        )
        .is_err()); // Too short
        assert!(validate_private_key(
            &"0x000000000000000000000000000000000000000000000000000000000000001".to_string()
        )
        .is_err()); // Too long
        assert!(validate_private_key(
            &"0x000000000000000000000000000000000000000000000000000000000000000g".to_string()
        )
        .is_err()); // Invalid hex
    }

    #[test]
    fn test_validate_u64() {
        // Valid values
        assert!(validate_u64(&"".to_string()).is_ok());
        assert!(validate_u64(&"0".to_string()).is_ok());
        assert!(validate_u64(&"42".to_string()).is_ok());
        assert!(validate_u64(&"18446744073709551615".to_string()).is_ok()); // max u64

        // Invalid values
        assert!(validate_u64(&"18446744073709551616".to_string()).is_err()); // > max u64
        assert!(validate_u64(&"-1".to_string()).is_err()); // negative
        assert!(validate_u64(&"3.14".to_string()).is_err()); // float
        assert!(validate_u64(&"abc".to_string()).is_err()); // non-numeric
        assert!(validate_u64(&"123abc".to_string()).is_err()); // mixed
    }

    #[test]
    fn test_validate_time_with_unit() {
        // Valid cases
        assert!(validate_time_with_unit(&"".to_string()).is_ok());
        assert!(validate_time_with_unit(&"15s".to_string()).is_ok());
        assert!(validate_time_with_unit(&"1s".to_string()).is_ok());
        assert!(validate_time_with_unit(&"1.5s".to_string()).is_ok());
        assert!(validate_time_with_unit(&"1.05s".to_string()).is_ok());
        assert!(validate_time_with_unit(&"100.00s".to_string()).is_ok());

        // Invalid cases
        assert!(validate_time_with_unit(&"0s".to_string()).is_err()); // Zero
        assert!(validate_time_with_unit(&"-1s".to_string()).is_err()); // Negative
        assert!(validate_time_with_unit(&"15".to_string()).is_err()); // Missing unit
        assert!(validate_time_with_unit(&"1.234s".to_string()).is_err()); // Too many decimals
        assert!(validate_time_with_unit(&"1,5s".to_string()).is_err()); // Wrong decimal separator
        assert!(validate_time_with_unit(&"abc".to_string()).is_err()); // Invalid format
        assert!(validate_time_with_unit(&"15m".to_string()).is_err()); // Wrong unit
        assert!(validate_time_with_unit(&"s".to_string()).is_err()); // Only unit
        assert!(validate_time_with_unit(&".5s".to_string()).is_err()); // Missing leading zero
    }
}
