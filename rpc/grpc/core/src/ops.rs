use crate::protowire::{turkiumd_request::Payload as RequestPayload, turkiumd_response::Payload as ResponsePayload, *};
use turkium_rpc_core::RpcError;
use workflow_core::enums::Describe;

macro_rules! payload_type_enum {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
    $($(#[$variant_meta:meta])* $variant_name:ident $(= $zero:literal)?,)*
    }) => {
        paste::paste! {
            $(#[$meta])*
            $vis enum $name {
                $($(#[$variant_meta])* $variant_name $(= $zero)?),*
            }

            impl std::convert::From<&RequestPayload> for $name {
                fn from(value: &RequestPayload) -> Self {
                    match value {
                        $(RequestPayload::[<$variant_name Request>](_) => $name::$variant_name),*
                    }
                }
            }

            impl TryFrom<&ResponsePayload> for $name {
                type Error = ();

                fn try_from(value: &ResponsePayload) -> Result<Self, Self::Error> {
                    match value {
                        $(ResponsePayload::[<$variant_name Response>](_) => Ok($name::$variant_name)),*,
                        _ => Err(())
                    }
                }
            }

        }
    }
}

payload_type_enum! {
#[repr(u8)]
#[derive(Describe, Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum TurkiumdPayloadOps {
    SubmitBlock = 0,
    GetBlockTemplate,
    GetCurrentNetwork,
    GetBlock,
    GetBlocks,
    GetInfo,
    Shutdown,
    GetPeerAddresses,
    GetSink,
    GetMempoolEntry,
    GetMempoolEntries,
    GetConnectedPeerInfo,
    AddPeer,
    SubmitTransaction,
    SubmitTransactionReplacement,
    GetSubnetwork,
    GetVirtualChainFromBlock,
    GetBlockCount,
    GetBlockDagInfo,
    ResolveFinalityConflict,
    GetHeaders,
    GetUtxosByAddresses,
    GetBalanceByAddress,
    GetBalancesByAddresses,
    GetSinkBlueScore,
    Ban,
    Unban,
    EstimateNetworkHashesPerSecond,
    GetMempoolEntriesByAddresses,
    GetCoinSupply,
    Ping,
    GetMetrics,
    GetConnections,
    GetSystemInfo,
    GetServerInfo,
    GetSyncStatus,
    GetDaaScoreTimestampEstimate,
    GetFeeEstimate,
    GetFeeEstimateExperimental,
    GetCurrentBlockColor,
    GetUtxoReturnAddress,
    GetVirtualChainFromBlockV2,

    // Subscription commands for starting/stopping notifications
    NotifyBlockAdded,
    NotifyNewBlockTemplate,
    NotifyFinalityConflict,
    NotifyUtxosChanged,
    NotifySinkBlueScoreChanged,
    NotifyPruningPointUtxoSetOverride,
    NotifyVirtualDaaScoreChanged,
    NotifyVirtualChainChanged,

    // Legacy stop subscription commands
    StopNotifyingUtxosChanged,
    StopNotifyingPruningPointUtxoSetOverride,

    // Please note:
    // Notification payloads existing in ResponsePayload are not considered valid ops.
    // The conversion from a notification ResponsePayload into TurkiumdPayloadOps fails.
}
}

// Manual implementation of to_error_response to handle all variants including StopNotifying
impl TurkiumdPayloadOps {
    pub fn to_error_response(&self, error: RpcError) -> ResponsePayload {
        match self {
            TurkiumdPayloadOps::SubmitBlock => {
                let result = Err::<&turkium_rpc_core::SubmitBlockResponse, RpcError>(error);
                let response: SubmitBlockResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::GetBlockTemplate => {
                let result = Err::<&turkium_rpc_core::GetBlockTemplateResponse, RpcError>(error);
                let response: GetBlockTemplateResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::GetCurrentNetwork => {
                let result = Err::<&turkium_rpc_core::GetCurrentNetworkResponse, RpcError>(error);
                let response: GetCurrentNetworkResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::GetBlock => {
                let result = Err::<&turkium_rpc_core::GetBlockResponse, RpcError>(error);
                let response: GetBlockResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::GetBlocks => {
                let result = Err::<&turkium_rpc_core::GetBlocksResponse, RpcError>(error);
                let response: GetBlocksResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::GetInfo => {
                let result = Err::<&turkium_rpc_core::GetInfoResponse, RpcError>(error);
                let response: GetInfoResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::Shutdown => {
                let result = Err::<&turkium_rpc_core::ShutdownResponse, RpcError>(error);
                let response: ShutdownResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::GetPeerAddresses => {
                let result = Err::<&turkium_rpc_core::GetPeerAddressesResponse, RpcError>(error);
                let response: GetPeerAddressesResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::GetSink => {
                let result = Err::<&turkium_rpc_core::GetSinkResponse, RpcError>(error);
                let response: GetSinkResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::GetMempoolEntry => {
                let result = Err::<&turkium_rpc_core::GetMempoolEntryResponse, RpcError>(error);
                let response: GetMempoolEntryResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::GetMempoolEntries => {
                let result = Err::<&turkium_rpc_core::GetMempoolEntriesResponse, RpcError>(error);
                let response: GetMempoolEntriesResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::GetConnectedPeerInfo => {
                let result = Err::<&turkium_rpc_core::GetConnectedPeerInfoResponse, RpcError>(error);
                let response: GetConnectedPeerInfoResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::AddPeer => {
                let result = Err::<&turkium_rpc_core::AddPeerResponse, RpcError>(error);
                let response: AddPeerResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::SubmitTransaction => {
                let result = Err::<&turkium_rpc_core::SubmitTransactionResponse, RpcError>(error);
                let response: SubmitTransactionResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::SubmitTransactionReplacement => {
                let result = Err::<&turkium_rpc_core::SubmitTransactionReplacementResponse, RpcError>(error);
                let response: SubmitTransactionReplacementResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::GetSubnetwork => {
                let result = Err::<&turkium_rpc_core::GetSubnetworkResponse, RpcError>(error);
                let response: GetSubnetworkResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::GetVirtualChainFromBlock => {
                let result = Err::<&turkium_rpc_core::GetVirtualChainFromBlockResponse, RpcError>(error);
                let response: GetVirtualChainFromBlockResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::GetBlockCount => {
                let result = Err::<&turkium_rpc_core::GetBlockCountResponse, RpcError>(error);
                let response: GetBlockCountResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::GetBlockDagInfo => {
                let result = Err::<&turkium_rpc_core::GetBlockDagInfoResponse, RpcError>(error);
                let response: GetBlockDagInfoResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::ResolveFinalityConflict => {
                let result = Err::<&turkium_rpc_core::ResolveFinalityConflictResponse, RpcError>(error);
                let response: ResolveFinalityConflictResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::GetHeaders => {
                let result = Err::<&turkium_rpc_core::GetHeadersResponse, RpcError>(error);
                let response: GetHeadersResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::GetUtxosByAddresses => {
                let result = Err::<&turkium_rpc_core::GetUtxosByAddressesResponse, RpcError>(error);
                let response: GetUtxosByAddressesResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::GetBalanceByAddress => {
                let result = Err::<&turkium_rpc_core::GetBalanceByAddressResponse, RpcError>(error);
                let response: GetBalanceByAddressResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::GetBalancesByAddresses => {
                let result = Err::<&turkium_rpc_core::GetBalancesByAddressesResponse, RpcError>(error);
                let response: GetBalancesByAddressesResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::GetSinkBlueScore => {
                let result = Err::<&turkium_rpc_core::GetSinkBlueScoreResponse, RpcError>(error);
                let response: GetSinkBlueScoreResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::Ban => {
                let result = Err::<&turkium_rpc_core::BanResponse, RpcError>(error);
                let response: BanResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::Unban => {
                let result = Err::<&turkium_rpc_core::UnbanResponse, RpcError>(error);
                let response: UnbanResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::EstimateNetworkHashesPerSecond => {
                let result = Err::<&turkium_rpc_core::EstimateNetworkHashesPerSecondResponse, RpcError>(error);
                let response: EstimateNetworkHashesPerSecondResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::GetMempoolEntriesByAddresses => {
                let result = Err::<&turkium_rpc_core::GetMempoolEntriesByAddressesResponse, RpcError>(error);
                let response: GetMempoolEntriesByAddressesResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::GetCoinSupply => {
                let result = Err::<&turkium_rpc_core::GetCoinSupplyResponse, RpcError>(error);
                let response: GetCoinSupplyResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::Ping => {
                let result = Err::<&turkium_rpc_core::PingResponse, RpcError>(error);
                let response: PingResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::GetMetrics => {
                let result = Err::<&turkium_rpc_core::GetMetricsResponse, RpcError>(error);
                let response: GetMetricsResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::GetConnections => {
                let result = Err::<&turkium_rpc_core::GetConnectionsResponse, RpcError>(error);
                let response: GetConnectionsResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::GetSystemInfo => {
                let result = Err::<&turkium_rpc_core::GetSystemInfoResponse, RpcError>(error);
                let response: GetSystemInfoResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::GetServerInfo => {
                let result = Err::<&turkium_rpc_core::GetServerInfoResponse, RpcError>(error);
                let response: GetServerInfoResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::GetSyncStatus => {
                let result = Err::<&turkium_rpc_core::GetSyncStatusResponse, RpcError>(error);
                let response: GetSyncStatusResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::GetDaaScoreTimestampEstimate => {
                let result = Err::<&turkium_rpc_core::GetDaaScoreTimestampEstimateResponse, RpcError>(error);
                let response: GetDaaScoreTimestampEstimateResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::GetFeeEstimate => {
                let result = Err::<&turkium_rpc_core::GetFeeEstimateResponse, RpcError>(error);
                let response: GetFeeEstimateResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::GetFeeEstimateExperimental => {
                let result = Err::<&turkium_rpc_core::GetFeeEstimateExperimentalResponse, RpcError>(error);
                let response: GetFeeEstimateExperimentalResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::GetCurrentBlockColor => {
                let result = Err::<&turkium_rpc_core::GetCurrentBlockColorResponse, RpcError>(error);
                let response: GetCurrentBlockColorResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::GetUtxoReturnAddress => {
                let result = Err::<&turkium_rpc_core::GetUtxoReturnAddressResponse, RpcError>(error);
                let response: GetUtxoReturnAddressResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::GetVirtualChainFromBlockV2 => {
                let result = Err::<&turkium_rpc_core::GetVirtualChainFromBlockV2Response, RpcError>(error);
                let response: GetVirtualChainFromBlockV2ResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::NotifyBlockAdded => {
                let result = Err::<&turkium_rpc_core::NotifyBlockAddedResponse, RpcError>(error);
                let response: NotifyBlockAddedResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::NotifyNewBlockTemplate => {
                let result = Err::<&turkium_rpc_core::NotifyNewBlockTemplateResponse, RpcError>(error);
                let response: NotifyNewBlockTemplateResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::NotifyFinalityConflict => {
                let result = Err::<&turkium_rpc_core::NotifyFinalityConflictResponse, RpcError>(error);
                let response: NotifyFinalityConflictResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::NotifyUtxosChanged => {
                let result = Err::<&turkium_rpc_core::NotifyUtxosChangedResponse, RpcError>(error);
                let response: NotifyUtxosChangedResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::NotifySinkBlueScoreChanged => {
                let result = Err::<&turkium_rpc_core::NotifySinkBlueScoreChangedResponse, RpcError>(error);
                let response: NotifySinkBlueScoreChangedResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::NotifyPruningPointUtxoSetOverride => {
                let result = Err::<&turkium_rpc_core::NotifyPruningPointUtxoSetOverrideResponse, RpcError>(error);
                let response: NotifyPruningPointUtxoSetOverrideResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::NotifyVirtualDaaScoreChanged => {
                let result = Err::<&turkium_rpc_core::NotifyVirtualDaaScoreChangedResponse, RpcError>(error);
                let response: NotifyVirtualDaaScoreChangedResponseMessage = result.into();
                response.into()
            }
            TurkiumdPayloadOps::NotifyVirtualChainChanged => {
                let result = Err::<&turkium_rpc_core::NotifyVirtualChainChangedResponse, RpcError>(error);
                let response: NotifyVirtualChainChangedResponseMessage = result.into();
                response.into()
            }
            // Special handling for StopNotifying commands which don't have rpc_core Response types
            TurkiumdPayloadOps::StopNotifyingUtxosChanged => {
                let response: StopNotifyingUtxosChangedResponseMessage = error.into();
                response.into()
            }
            TurkiumdPayloadOps::StopNotifyingPruningPointUtxoSetOverride => {
                let response: StopNotifyingPruningPointUtxoSetOverrideResponseMessage = error.into();
                response.into()
            }
        }
    }
}
