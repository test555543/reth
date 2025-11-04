# XLayer Legacy RPC Implementation Status

## Legacy RPC API Implementation

Based on [op-geth migration RPC implementation](https://github.com/okx/op-geth/pull/16)

| # | RPC Method | Policy | Reth Status | Notes |
|---|------------|--------|-------------|-------|
| **Block Query** |
| 1 | `eth_getBlockByNumber` | FORWARD | ✅ Implemented | Routes to legacy if block < cutoff |
| 2 | `eth_getBlockByHash` | LOCAL | ✅ Implemented | Try local first, fallback to legacy |
| 3 | `eth_getHeaderByNumber` | FORWARD | ✅ Implemented | Routes to legacy if block < cutoff |
| 4 | `eth_getHeaderByHash` | LOCAL | ✅ Implemented | Try local first, fallback to legacy |
| 5 | `eth_getBlockReceipts` | FORWARD | ✅ Implemented | Routes based on block ID |
| 6 | `eth_getBlockTransactionCountByNumber` | FORWARD | ✅ Implemented | Routes to legacy if block < cutoff |
| 7 | `eth_getBlockTransactionCountByHash` | LOCAL | ✅ Implemented | Try local first, fallback to legacy |
| **Transaction Query** |
| 8 | `eth_getTransactionByHash` | LOCAL | ✅ Implemented | Try local first, fallback to legacy |
| 9 | `eth_getTransactionReceipt` | LOCAL | ✅ Implemented | Try local first, fallback to legacy |
| 10 | `eth_getTransactionByBlockHashAndIndex` | LOCAL | ✅ Implemented | Try local first, fallback to legacy |
| 11 | `eth_getTransactionByBlockNumberAndIndex` | FORWARD | ✅ Implemented | Routes to legacy if block < cutoff |
| 12 | `eth_getRawTransactionByHash` | LOCAL | ✅ Implemented | Try local first, fallback to legacy |
| 13 | `eth_getRawTransactionByBlockHashAndIndex` | LOCAL | ✅ Implemented | Try local first, fallback to legacy |
| 14 | `eth_getRawTransactionByBlockNumberAndIndex` | FORWARD | ✅ Implemented | Routes to legacy if block < cutoff |
| **State Query** |
| 15 | `eth_getBalance` | FORWARD | ✅ Implemented | Routes based on block ID |
| 16 | `eth_getCode` | FORWARD | ✅ Implemented | Routes based on block ID |
| 17 | `eth_getStorageAt` | FORWARD | ✅ Implemented | Routes based on block ID |
| 18 | `eth_getTransactionCount` | FORWARD | ✅ Implemented | Routes based on block ID |
| **Logs & Filter** |
| 19 | `eth_getLogs` | SPECIAL | ✅ Implemented | **Supports cross-boundary merge** |
| 20 | `eth_newFilter` | SPECIAL | ❌ Not Implemented | op-geth has, Reth missing |
| 21 | `eth_getFilterLogs` | SPECIAL | ❌ Not Implemented | op-geth has, Reth missing |
| 22 | `eth_getFilterChanges` | SPECIAL | ❌ Not Implemented | op-geth has, Reth missing |
| 23 | `eth_uninstallFilter` | SPECIAL | ❌ Not Implemented | op-geth has, Reth missing |
| **XLayer-Specific (Internal Transactions)** |
| 24 | `eth_getBlockInternalTransactions` | FORWARD | ❌ Not Implemented | TODO |
| 25 | `eth_getInternalTransactions` | LOCAL | ❌ Not Implemented | TODO |

## Policy Definitions

- **FORWARD**: Routes to legacy RPC if block number < migration/cutoff block
- **LOCAL**: Tries local node first, falls back to legacy if not found
- **SPECIAL**: Complex logic for handling ranges that may span the migration block
