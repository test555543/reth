use alloy_primitives::U256;
use clap::Args;
use std::time::Duration;

/// Parameters to configure XLayer specific features
#[derive(Debug, Clone, Args, PartialEq)]
#[command(next_help_heading = "XLayer")]
pub struct XLayerArgs {
    // TODO: EnableInnerTx configuration
    // Enable inner transaction tracking
    // #[arg(long = "xlayer.enable-inner-tx")]
    // pub enable_inner_tx: bool,

    // TODO: OkPay configuration
    // OkX Pay priority transaction configuration
    // #[command(flatten)]
    // pub okpay: OkPayArgs,

    // TODO: Apollo configuration
    // Apollo dynamic configuration service
    // #[command(flatten)]
    // pub apollo: ApolloArgs,

    // TODO: LegacyPp (Migration) configuration
    // Legacy Erigon RPC endpoint for pre-migration blocks
    // #[command(flatten)]
    // pub legacy_pp: MigrationArgs,

    // TODO: Monitor configuration
    // Transaction monitoring and tracing
    // #[command(flatten)]
    // pub monitor: MonitorArgs,

    /// XLayer gas price configuration
    #[command(flatten)]
    pub gas_price: XLayerGasPriceArgs,
}

/// XLayer gas price configuration
#[derive(Debug, Clone, Args, PartialEq)]
#[command(next_help_heading = "XLayer Gas Price")]
pub struct XLayerGasPriceArgs {
    /// Gas price calculation type: "default", "follower", or "fixed"
    #[arg(long = "xlayer.gasprice.type", default_value = "default")]
    pub price_type: String,

    /// Gas price update period
    #[arg(long = "xlayer.gasprice.update-period", value_parser = parse_duration)]
    pub update_period: Option<Duration>,

    /// Gas price calculation factor
    #[arg(long = "xlayer.gasprice.factor")]
    pub factor: Option<f64>,

    /// Kafka URL for gas price updates
    #[arg(long = "xlayer.gasprice.kafka-url")]
    pub kafka_url: Option<String>,

    /// Kafka topic for gas price updates
    #[arg(long = "xlayer.gasprice.topic")]
    pub topic: Option<String>,

    /// Kafka consumer group ID
    #[arg(long = "xlayer.gasprice.group-id")]
    pub group_id: Option<String>,

    /// Kafka username for authentication
    #[arg(long = "xlayer.gasprice.username")]
    pub username: Option<String>,

    /// Kafka password for authentication
    #[arg(long = "xlayer.gasprice.password")]
    pub password: Option<String>,

    /// Path to Kafka root CA certificate
    #[arg(long = "xlayer.gasprice.root-ca-path")]
    pub root_ca_path: Option<String>,

    /// L1 coin ID for price tracking
    #[arg(long = "xlayer.gasprice.l1-coin-id")]
    pub l1_coin_id: Option<i32>,

    /// L2 coin ID for price tracking
    #[arg(long = "xlayer.gasprice.l2-coin-id")]
    pub l2_coin_id: Option<i32>,

    /// Default L1 coin price (fallback value)
    #[arg(long = "xlayer.gasprice.default-l1-coin-price")]
    pub default_l1_coin_price: Option<f64>,

    /// Default L2 coin price (fallback value)
    #[arg(long = "xlayer.gasprice.default-l2-coin-price")]
    pub default_l2_coin_price: Option<f64>,

    /// Fixed gas price in USDT (for "fixed" type)
    #[arg(long = "xlayer.gasprice.gas-price-usdt")]
    pub gas_price_usdt: Option<f64>,

    /// Congestion threshold for dynamic gas price adjustment
    #[arg(long = "xlayer.gasprice.congestion-threshold")]
    pub congestion_threshold: Option<i32>,

    /// Default gas price for XLayer (in wei)
    #[arg(long = "xlayer.gasprice.default")]
    pub default: Option<U256>,
}

/// Helper function to parse duration from string
fn parse_duration(s: &str) -> Result<Duration, String> {
    // Parse duration string like "10s", "5m", "1h"
    let s = s.trim();
    if s.is_empty() {
        return Err("empty duration string".to_string());
    }

    let (num_str, unit) = if let Some(pos) = s.find(|c: char| !c.is_ascii_digit()) {
        (&s[..pos], &s[pos..])
    } else {
        return Err("duration must have a unit (s, m, h)".to_string());
    };

    let num: u64 = num_str.parse().map_err(|e| format!("invalid number: {}", e))?;

    match unit {
        "s" | "sec" | "second" | "seconds" => Ok(Duration::from_secs(num)),
        "m" | "min" | "minute" | "minutes" => Ok(Duration::from_secs(num * 60)),
        "h" | "hour" | "hours" => Ok(Duration::from_secs(num * 3600)),
        "ms" | "millisecond" | "milliseconds" => Ok(Duration::from_millis(num)),
        _ => Err(format!("unknown duration unit: {}", unit)),
    }
}

impl Default for XLayerArgs {
    fn default() -> Self {
        Self {
            gas_price: XLayerGasPriceArgs::default(),
        }
    }
}

impl Default for XLayerGasPriceArgs {
    fn default() -> Self {
        Self {
            price_type: "default".to_string(),
            update_period: None,
            factor: None,
            kafka_url: None,
            topic: None,
            group_id: None,
            username: None,
            password: None,
            root_ca_path: None,
            l1_coin_id: None,
            l2_coin_id: None,
            default_l1_coin_price: None,
            default_l2_coin_price: None,
            gas_price_usdt: None,
            congestion_threshold: None,
            default: None,
        }
    }
}

// TODO: OkPay configuration structure
// #[derive(Debug, Clone, Args, PartialEq)]
// #[command(next_help_heading = "XLayer OkPay")]
// pub struct OkPayArgs {
//     /// Enable OkX Pay priority transactions
//     #[arg(long = "xlayer.okpay.priority-enable")]
//     pub priority_enable: bool,
//
//     /// OkX Pay sender accounts list (comma-separated addresses)
//     #[arg(long = "xlayer.okpay.sender-accounts", value_delimiter = ',')]
//     pub sender_accounts_list: Vec<Address>,
//
//     /// Maximum number of OkX Pay priority transactions per block
//     #[arg(long = "xlayer.okpay.block-priority-txs-limit")]
//     pub block_priority_txs_limit: u64,
// }

// TODO: Apollo configuration structure
// #[derive(Debug, Clone, Args, PartialEq)]
// #[command(next_help_heading = "XLayer Apollo")]
// pub struct ApolloArgs {
//     /// Enable Apollo dynamic configuration service
//     #[arg(long = "xlayer.apollo.enable")]
//     pub enable: bool,
//
//     /// Apollo application ID
//     #[arg(long = "xlayer.apollo.app-id")]
//     pub app_id: Option<String>,
//
//     /// Apollo server endpoint IP
//     #[arg(long = "xlayer.apollo.ip")]
//     pub ip: Option<String>,
//
//     /// Apollo cluster name
//     #[arg(long = "xlayer.apollo.cluster")]
//     pub cluster: Option<String>,
//
//     /// Apollo namespace name
//     #[arg(long = "xlayer.apollo.namespace-name")]
//     pub namespace_name: Option<String>,
// }

// TODO: Migration configuration structure
// #[derive(Debug, Clone, Args, PartialEq)]
// #[command(next_help_heading = "XLayer Migration")]
// pub struct MigrationArgs {
//     /// Block height threshold for migration routing
//     #[arg(long = "xlayer.migration.migration-block")]
//     pub migration_block: Option<u64>,
//
//     /// XLayer-Erigon RPC endpoint URL for pre-migration blocks
//     #[arg(long = "xlayer.migration.pp-rpc-url")]
//     pub pp_rpc_url: Option<String>,
//
//     /// Timeout for PP RPC calls
//     #[arg(long = "xlayer.migration.pp-rpc-timeout", value_parser = parse_duration)]
//     pub pp_rpc_timeout: Option<Duration>,
// }

// TODO: Monitor configuration structure
// #[derive(Debug, Clone, Args, PartialEq)]
// #[command(next_help_heading = "XLayer Monitor")]
// pub struct MonitorArgs {
//     /// Enable transaction trace logging
//     #[arg(long = "xlayer.monitor.enable-trace-log")]
//     pub enable_trace_log: bool,
//
//     /// Path to transaction trace log file
//     #[arg(long = "xlayer.monitor.trace-log-path", default_value = "/var/log/reth/trace.log")]
//     pub trace_log_path: String,
// }

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    /// A helper type to parse Args more easily
    #[derive(Parser)]
    struct CommandParser<T: Args> {
        #[command(flatten)]
        args: T,
    }

    #[test]
    fn test_parse_xlayer_args() {
        let args = CommandParser::<XLayerArgs>::parse_from(["reth"]).args;
        assert_eq!(args, XLayerArgs::default());
    }

    #[test]
    fn test_parse_xlayer_gas_price_args() {
        let args = CommandParser::<XLayerGasPriceArgs>::parse_from([
            "reth",
            "--xlayer.gasprice.type",
            "follower",
            "--xlayer.gasprice.factor",
            "1.5",
        ])
        .args;
        assert_eq!(args.price_type, "follower");
        assert_eq!(args.factor, Some(1.5));
    }

    #[test]
    fn test_parse_duration() {
        assert_eq!(parse_duration("10s").unwrap(), Duration::from_secs(10));
        assert_eq!(parse_duration("5m").unwrap(), Duration::from_secs(300));
        assert_eq!(parse_duration("1h").unwrap(), Duration::from_secs(3600));
        assert_eq!(parse_duration("500ms").unwrap(), Duration::from_millis(500));
        assert!(parse_duration("invalid").is_err());
    }

    #[test]
    fn xlayer_args_default_sanity_test() {
        let default_args = XLayerArgs::default();
        let args = CommandParser::<XLayerArgs>::parse_from(["reth"]).args;
        assert_eq!(args, default_args);
    }
}

