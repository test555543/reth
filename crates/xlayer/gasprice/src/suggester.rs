//! Gas price suggester interface and factory
//!
//! Provides the main interface for gas price calculation strategies
//! and a factory function to create the appropriate suggester based on configuration.

use alloy_primitives::U256;
use std::sync::Arc;

use crate::{
    cache::GasPriceCache,
    config::{GasPriceType, XLayerGasPriceConfig},
    default::DefaultGasPricer,
    fixed::FixedGasPricer,
    follower::FollowerGasPricer,
};

/// Interface for L2 gas price calculation strategies
pub trait L2GasPricer: Send + Sync {
    /// Updates the gas price average based on L1 gas price
    fn update_gas_price_avg(&self, l1_gas_price: U256);

    /// Updates the configuration
    fn update_config(&self, config: XLayerGasPriceConfig);

    /// Gets the last calculated raw gas price
    fn get_last_raw_gp(&self) -> U256;

    /// Gets the current configuration
    fn get_config(&self) -> XLayerGasPriceConfig;

    /// Gets the gas price cache
    fn get_gas_cache(&self) -> Arc<GasPriceCache>;
}

/// Creates a new L2 gas price suggester based on the configuration
///
/// # Arguments
///
/// * `config` - The XLayer gas price configuration
///
/// # Returns
///
/// Returns an Arc-wrapped implementation of `L2GasPricer` based on the price type:
/// - `Default`: Uses a fixed default gas price
/// - `Follower`: Calculates based on L1 gas price and coin prices
/// - `Fixed`: Uses a fixed USDT price converted to native token
pub fn new_l2_gas_price_suggester(config: XLayerGasPriceConfig) -> Arc<dyn L2GasPricer> {
    match config.price_type {
        GasPriceType::Default => {
            tracing::info!("Creating Default gas price suggester");
            Arc::new(DefaultGasPricer::new(config))
        }
        GasPriceType::Follower => {
            tracing::info!("Creating Follower gas price suggester");
            Arc::new(FollowerGasPricer::new(config))
        }
        GasPriceType::Fixed => {
            tracing::info!("Creating Fixed gas price suggester");
            Arc::new(FixedGasPricer::new(config))
        }
    }
}

/// Type alias for the suggester factory function
pub type NewL2GasPriceSuggester = fn(XLayerGasPriceConfig) -> Arc<dyn L2GasPricer>;

