//! XLayer gas price configuration

use alloy_primitives::U256;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// XLayer gas price types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GasPriceType {
    /// Default gas price from config
    Default,
    /// Calculate gas price based on L1 gas price
    Follower,
    /// Fixed gas price in USDT
    Fixed,
}

impl Default for GasPriceType {
    fn default() -> Self {
        Self::Default
    }
}

impl std::fmt::Display for GasPriceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Default => write!(f, "default"),
            Self::Follower => write!(f, "follower"),
            Self::Fixed => write!(f, "fixed"),
        }
    }
}

impl std::str::FromStr for GasPriceType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "default" => Ok(Self::Default),
            "follower" => Ok(Self::Follower),
            "fixed" => Ok(Self::Fixed),
            _ => Err(format!("Unknown gas price type: {}", s)),
        }
    }
}

/// XLayer gas price configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XLayerGasPriceConfig {
    /// Gas price calculation type
    pub price_type: GasPriceType,

    /// Gas price update period
    pub update_period: Duration,

    /// Gas price calculation factor (for follower mode - TODO)
    pub factor: f64,

    /// L1 coin ID for price tracking (for follower mode - TODO)
    pub l1_coin_id: Option<i32>,

    /// L2 coin ID for price tracking (for follower/fixed modes - TODO)
    pub l2_coin_id: Option<i32>,

    /// Default L1 coin price fallback (for follower mode - TODO)
    pub default_l1_coin_price: f64,

    /// Default L2 coin price fallback (for follower/fixed modes - TODO)
    pub default_l2_coin_price: f64,

    /// Fixed gas price in USDT (for fixed mode - TODO)
    pub gas_price_usdt: f64,

    /// Congestion threshold for dynamic gas price adjustment
    pub congestion_threshold: i32,

    /// Default gas price for XLayer (in wei)
    pub default: U256,
}

impl Default for XLayerGasPriceConfig {
    fn default() -> Self {
        Self {
            price_type: GasPriceType::Default,
            update_period: Duration::from_secs(10),
            factor: 1.0,
            l1_coin_id: None,
            l2_coin_id: None,
            default_l1_coin_price: 0.0,
            default_l2_coin_price: 0.0,
            gas_price_usdt: 0.0,
            congestion_threshold: 0,
            default: U256::from(crate::DEFAULT_XLAYER_PRICE),
        }
    }
}

