//! Follower gas price strategy
//!
//! TODO: Not yet implemented
//!
//! This strategy will calculate L2 gas price based on:
//! - L1 gas price
//! - L1 coin price (e.g., ETH price in USDT)
//! - L2 coin price (e.g., OKB price in USDT)
//! - Configured factor
//!
//! Formula: L2_GP = L1_GP × Factor × (L1_CoinPrice / L2_CoinPrice)

use alloy_primitives::U256;
use parking_lot::RwLock;
use std::sync::Arc;

use crate::{cache::GasPriceCache, config::XLayerGasPriceConfig, suggester::L2GasPricer};

/// Follower gas price suggester (placeholder)
#[derive(Debug)]
pub struct FollowerGasPricer {
    config: RwLock<XLayerGasPriceConfig>,
    last_raw_gp: RwLock<U256>,
    gas_cache: Arc<GasPriceCache>,
}

impl FollowerGasPricer {
    /// Creates a new follower gas price suggester
    pub fn new(config: XLayerGasPriceConfig) -> Self {
        let default_price = config.default;
        tracing::warn!("FollowerGasPricer is not yet implemented, using default price");

        Self {
            config: RwLock::new(config),
            last_raw_gp: RwLock::new(default_price),
            gas_cache: Arc::new(GasPriceCache::new()),
        }
    }
}

impl L2GasPricer for FollowerGasPricer {
    fn update_gas_price_avg(&self, _l1_gas_price: U256) {
        // TODO: Implement follower logic
        // - Get L1 and L2 coin prices from external source
        // - Apply factor
        // - Calculate L2 gas price based on coin price ratio
        tracing::warn!("FollowerGasPricer.update_gas_price_avg not implemented");
    }

    fn update_config(&self, config: XLayerGasPriceConfig) {
        *self.config.write() = config;
    }

    fn get_last_raw_gp(&self) -> U256 {
        *self.last_raw_gp.read()
    }

    fn get_config(&self) -> XLayerGasPriceConfig {
        self.config.read().clone()
    }

    fn get_gas_cache(&self) -> Arc<GasPriceCache> {
        Arc::clone(&self.gas_cache)
    }
}
