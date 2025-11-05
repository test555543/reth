//! Default gas price strategy
//!
//! Uses a fixed default gas price from the configuration.
//! This is the simplest strategy and doesn't require external data sources.

use alloy_primitives::U256;
use parking_lot::RwLock;
use std::sync::Arc;

use crate::{cache::GasPriceCache, config::XLayerGasPriceConfig, suggester::L2GasPricer};

/// Default gas price suggester
///
/// Always returns the configured default gas price without any calculations.
#[derive(Debug)]
pub struct DefaultGasPricer {
    /// Configuration
    config: RwLock<XLayerGasPriceConfig>,
    /// Last calculated raw gas price
    last_raw_gp: RwLock<U256>,
    /// Gas price cache
    gas_cache: Arc<GasPriceCache>,
}

impl DefaultGasPricer {
    /// Creates a new default gas price suggester
    pub fn new(config: XLayerGasPriceConfig) -> Self {
        let default_price = config.default;
        Self {
            config: RwLock::new(config),
            last_raw_gp: RwLock::new(default_price),
            gas_cache: Arc::new(GasPriceCache::new()),
        }
    }
}

impl L2GasPricer for DefaultGasPricer {
    fn update_gas_price_avg(&self, _l1_gas_price: U256) {
        // For default strategy, always use the configured default price
        let default_price = self.config.read().default;
        *self.last_raw_gp.write() = default_price;
        tracing::debug!(
            price = %default_price,
            "Default gas price strategy: using configured default"
        );
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cache::GasPriceCacheTrait;

    #[test]
    fn test_default_pricer_returns_configured_price() {
        let config = XLayerGasPriceConfig {
            default: U256::from(1_000_000_000u64), // 1 GWei
            ..Default::default()
        };

        let pricer = DefaultGasPricer::new(config);

        // Update with any L1 price, should still return default
        pricer.update_gas_price_avg(U256::from(50_000_000_000u64));

        assert_eq!(pricer.get_last_raw_gp(), U256::from(1_000_000_000u64));
    }

    #[test]
    fn test_default_gas_price_cache_operations() {
        let cache = GasPriceCache::new();
        cache.set_latest(U256::from(200));
        assert_eq!(cache.get_latest(), U256::from(200));
        assert_eq!(cache.get_latest_raw_gp(), U256::from(200));
        assert_eq!(cache.get_min_raw_gp_recent(), U256::from(200));

        cache.set_latest_raw_gp(U256::from(300));
        assert_eq!(cache.get_latest(), U256::from(300));
    }
}

