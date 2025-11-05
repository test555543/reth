//! Fixed gas price strategy
//!
//! TODO: Not yet implemented
//!
//! This strategy will use a fixed USDT price and convert it to the native token
//! based on current L2 coin price.
//!
//! Formula: L2_GP = GasPriceUSDT / L2_CoinPrice × 10^18

use alloy_primitives::U256;
use parking_lot::RwLock;
use std::sync::Arc;

use crate::{cache::GasPriceCache, config::XLayerGasPriceConfig, suggester::L2GasPricer};

/// Fixed gas price suggester (placeholder)
#[derive(Debug)]
pub struct FixedGasPricer {
    config: RwLock<XLayerGasPriceConfig>,
    last_raw_gp: RwLock<U256>,
    gas_cache: Arc<GasPriceCache>,
}

impl FixedGasPricer {
    /// Creates a new fixed gas price suggester
    pub fn new(config: XLayerGasPriceConfig) -> Self {
        let default_price = config.default;
        tracing::warn!("FixedGasPricer is not yet implemented, using default price");

        Self {
            config: RwLock::new(config),
            last_raw_gp: RwLock::new(default_price),
            gas_cache: Arc::new(GasPriceCache::new()),
        }
    }
}

impl L2GasPricer for FixedGasPricer {
    fn update_gas_price_avg(&self, _l1_gas_price: U256) {
        // TODO: Implement fixed logic
        // - Get L2 coin price from external source
        // - Convert fixed USDT price to native token
        // - Apply min/max bounds
        tracing::warn!("FixedGasPricer.update_gas_price_avg not implemented");
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
