# XLayer Legacy RPC Implementation Status

## Legacy RPC API Implementation

| # | RPC Method | Category | Implementation Status | Notes |
|---|------------|----------|-----------------------|-------|
| 1 | `eth_getBlockByNumber` | Block Query | ✅ Implemented | Routes to legacy for blocks < cutoff |
| 2 | `eth_getBlockByHash` | Block Query | ✅ Implemented | Hash-based fallback to legacy |
| 3 | `eth_getBlockTransactionCountByNumber` | Block Query | ✅ Implemented | Routes based on block number |
| 4 | `eth_getBlockTransactionCountByHash` | Block Query | ✅ Implemented | Hash-based fallback |
| 5 | `eth_getBlockReceipts` | Block Query | ✅ Implemented | Routes based on block ID |
| 6 | `eth_getTransactionByHash` | Transaction | ✅ Implemented | Try local first, fallback to legacy |
| 7 | `eth_getTransactionReceipt` | Transaction | ✅ Implemented | Try local first, fallback to legacy |
| 8 | `eth_getTransactionByBlockHashAndIndex` | Transaction | ✅ Implemented | Hash-based fallback |
| 9 | `eth_getTransactionByBlockNumberAndIndex` | Transaction | ✅ Implemented | Routes based on block number |
| 10 | `eth_sendRawTransaction` | Transaction | ✅ Implemented | Forwards to legacy RPC |
| 11 | `eth_getBalance` | State Query | ✅ Implemented | Routes based on block ID |
| 12 | `eth_getCode` | State Query | ✅ Implemented | Routes based on block ID |
| 13 | `eth_getStorageAt` | State Query | ✅ Implemented | Routes based on block ID |
| 14 | `eth_getTransactionCount` | State Query | ✅ Implemented | Routes based on block ID |
| 15 | `eth_getProof` | State Query | ✅ Implemented | Routes based on block ID |
| 16 | `eth_call` | Execution | ✅ Implemented | Routes based on block ID (no state override) |
| 17 | `eth_estimateGas` | Execution | ✅ Implemented | Routes based on block ID (no state override) |
| 18 | `eth_createAccessList` | Execution | ✅ Implemented | Routes based on block ID (no state override) |
| 19 | `eth_getLogs` | Logs | ✅ Implemented | **Supports cross-boundary merge** |
| 20 | `eth_getUncleCountByBlockNumber` | Uncle | ✅ Implemented | Routes based on block number |
| 21 | `eth_getUncleCountByBlockHash` | Uncle | ✅ Implemented | Hash-based fallback |
| 22 | `eth_getUncleByBlockNumberAndIndex` | Uncle | ✅ Implemented | Routes based on block number |
| 23 | `eth_getUncleByBlockHashAndIndex` | Uncle | ✅ Implemented | Hash-based fallback |
| 24 | `eth_gasPrice` | Gas | ✅ Implemented | Forwards to legacy RPC |
| 25 | `eth_maxPriorityFeePerGas` | Gas | ✅ Implemented | Forwards to legacy RPC |
| 26 | `eth_feeHistory` | Gas | ✅ Implemented | Forwards to legacy RPC |
| 27 | `eth_blobBaseFee` | Gas | ✅ Implemented | Forwards to legacy RPC |
| 28 | `eth_newFilter` | Filter | ❌ Not Implemented | Local only, no legacy routing |
| 29 | `eth_newBlockFilter` | Filter | ❌ Not Implemented | Local only, no legacy routing |
| 30 | `eth_newPendingTransactionFilter` | Filter | ❌ Not Implemented | Local only, no legacy routing |
| 31 | `eth_getFilterLogs` | Filter | ❌ Not Implemented | Local only, no legacy routing |
| 32 | `eth_getFilterChanges` | Filter | ❌ Not Implemented | Local only, no legacy routing |
| 33 | `eth_uninstallFilter` | Filter | ❌ Not Implemented | Local only, no legacy routing |
