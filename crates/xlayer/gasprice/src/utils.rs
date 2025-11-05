//! Utility functions for gas price calculations

use alloy_primitives::U256;

use crate::WEI_TO_ETH;

/// Converts U256 to f64 for calculations
///
/// Note: This may lose precision for very large values
pub fn u256_to_f64(value: U256) -> f64 {
    // Convert to string and parse to avoid overflow
    let s = value.to_string();
    s.parse::<f64>().unwrap_or(0.0)
}

/// Converts f64 to U256
///
/// Handles fractional values by truncating
pub fn f64_to_u256(value: f64) -> U256 {
    if value <= 0.0 {
        return U256::ZERO;
    }

    // Convert to integer part
    let int_value = value.floor() as u128;
    U256::from(int_value)
}

/// Converts OKB (or other native token) to Wei
///
/// Multiplies by 10^18 to convert from token units to Wei
pub fn okb_to_wei(okb: f64) -> U256 {
    let wei_value = okb * WEI_TO_ETH;
    f64_to_u256(wei_value)
}

/// Truncates gas price to 3 significant digits
///
/// This helps stabilize gas prices by reducing noise in the lower digits.
/// For example: 123456789 -> 123000000
pub fn truncate_gas_price(price: U256) -> U256 {
    let price_str = price.to_string();
    let len = price_str.len();

    if len <= 3 {
        return price;
    }

    // Keep first 3 digits, zero out the rest
    let significant = &price_str[..3];
    let zeros = "0".repeat(len - 3);
    let truncated_str = format!("{}{}", significant, zeros);

    U256::from_str_radix(&truncated_str, 10).unwrap_or(price)
}

/// Calculates the average of two gas prices
pub fn avg_price(low: U256, high: U256) -> U256 {
    (low + high) / U256::from(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u256_to_f64() {
        let value = U256::from(1_000_000_000u64);
        let result = u256_to_f64(value);
        assert_eq!(result, 1_000_000_000.0);
    }

    #[test]
    fn test_f64_to_u256() {
        let value = 1_000_000_000.5;
        let result = f64_to_u256(value);
        assert_eq!(result, U256::from(1_000_000_000u64));
    }

    #[test]
    fn test_okb_to_wei() {
        let okb = 1.0;
        let wei = okb_to_wei(okb);
        assert_eq!(wei, U256::from(1_000_000_000_000_000_000u64));
    }

    #[test]
    fn test_truncate_gas_price() {
        let price = U256::from(123_456_789u64);
        let truncated = truncate_gas_price(price);
        assert_eq!(truncated, U256::from(123_000_000u64));
    }

    #[test]
    fn test_truncate_small_price() {
        let price = U256::from(123u64);
        let truncated = truncate_gas_price(price);
        assert_eq!(truncated, U256::from(123u64));
    }

    #[test]
    fn test_avg_price() {
        let low = U256::from(100u64);
        let high = U256::from(200u64);
        let avg = avg_price(low, high);
        assert_eq!(avg, U256::from(150u64));
    }
}

