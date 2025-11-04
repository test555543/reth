//! XLayer-specific extensions for EthApi

use super::EthApi;
use reth_rpc_convert::RpcConvert;
use reth_rpc_eth_api::RpcNodeCore;
use reth_rpc_eth_types::LegacyRpcClient;
use std::sync::Arc;

/// XLayer: Implement LegacyRpc trait for EthApi to enable legacy RPC routing
impl<N, Rpc> reth_rpc_eth_api::helpers::LegacyRpc for EthApi<N, Rpc>
where
    N: RpcNodeCore,
    Rpc: RpcConvert,
{
    fn legacy_rpc_client(&self) -> Option<&Arc<LegacyRpcClient>> {
        self.inner.legacy_rpc_client()
    }
}

