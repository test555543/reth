//! XLayer gas price oracle implementation
//!
//! This crate provides gas price calculation strategies for XLayer:
//! - Default: Uses a fixed default gas price from configuration
//! - Follower: Calculates gas price based on L1 gas price and coin prices
//! - Fixed: Uses a fixed USDT price converted to native token

#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

/// Gas price cache implementation
pub mod cache;

/// Gas price configuration
pub mod config;

/// Default gas price strategy
pub mod default;

/// Fixed gas price strategy (USDT-based) - TODO: Not yet implemented
pub mod fixed;

/// Follower gas price strategy (L1-based) - TODO: Not yet implemented
pub mod follower;

/// Gas price scheduler
pub mod scheduler;

/// Gas price suggester interface
pub mod suggester;

/// Utility functions
pub mod utils;

// Re-exports
pub use cache::GasPriceCache;
pub use config::XLayerGasPriceConfig;
pub use scheduler::XLayerScheduler;
pub use suggester::{L2GasPricer, NewL2GasPriceSuggester};

/// Default XLayer gas price (1 GWei)
pub const DEFAULT_XLAYER_PRICE: u64 = 1_000_000_000; // 1 GWei

/// Maximum cache size for raw gas prices
pub const MAX_CACHE_SIZE: usize = 30;

/// Minimum gas price window size for recent calculations
pub const MIN_GP_WINDOW_SIZE: usize = 27;

/// Minimum USDT price threshold
pub const MIN_USDT_PRICE: f64 = 1e-18;

/// Wei to Eth conversion factor
pub const WEI_TO_ETH: f64 = 1e18;

