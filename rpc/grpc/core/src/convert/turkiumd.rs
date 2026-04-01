use crate::protowire::{TurkiumdRequest, TurkiumdResponse, turkiumd_request};

impl From<turkiumd_request::Payload> for TurkiumdRequest {
    fn from(item: turkiumd_request::Payload) -> Self {
        TurkiumdRequest { id: 0, payload: Some(item) }
    }
}

impl AsRef<TurkiumdRequest> for TurkiumdRequest {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsRef<TurkiumdResponse> for TurkiumdResponse {
    fn as_ref(&self) -> &Self {
        self
    }
}

pub mod turkiumd_request_convert {
    use crate::protowire::*;
    use turkium_rpc_core::{RpcError, RpcResult};

    impl_into_turkiumd_request!(Shutdown);
    impl_into_turkiumd_request!(SubmitBlock);
    impl_into_turkiumd_request!(GetBlockTemplate);
    impl_into_turkiumd_request!(GetBlock);
    impl_into_turkiumd_request!(GetInfo);

    impl_into_turkiumd_request!(GetCurrentNetwork);
    impl_into_turkiumd_request!(GetPeerAddresses);
    impl_into_turkiumd_request!(GetSink);
    impl_into_turkiumd_request!(GetMempoolEntry);
    impl_into_turkiumd_request!(GetMempoolEntries);
    impl_into_turkiumd_request!(GetConnectedPeerInfo);
    impl_into_turkiumd_request!(AddPeer);
    impl_into_turkiumd_request!(SubmitTransaction);
    impl_into_turkiumd_request!(SubmitTransactionReplacement);
    impl_into_turkiumd_request!(GetSubnetwork);
    impl_into_turkiumd_request!(GetVirtualChainFromBlock);
    impl_into_turkiumd_request!(GetBlocks);
    impl_into_turkiumd_request!(GetBlockCount);
    impl_into_turkiumd_request!(GetBlockDagInfo);
    impl_into_turkiumd_request!(ResolveFinalityConflict);
    impl_into_turkiumd_request!(GetHeaders);
    impl_into_turkiumd_request!(GetUtxosByAddresses);
    impl_into_turkiumd_request!(GetBalanceByAddress);
    impl_into_turkiumd_request!(GetBalancesByAddresses);
    impl_into_turkiumd_request!(GetSinkBlueScore);
    impl_into_turkiumd_request!(Ban);
    impl_into_turkiumd_request!(Unban);
    impl_into_turkiumd_request!(EstimateNetworkHashesPerSecond);
    impl_into_turkiumd_request!(GetMempoolEntriesByAddresses);
    impl_into_turkiumd_request!(GetCoinSupply);
    impl_into_turkiumd_request!(Ping);
    impl_into_turkiumd_request!(GetMetrics);
    impl_into_turkiumd_request!(GetConnections);
    impl_into_turkiumd_request!(GetSystemInfo);
    impl_into_turkiumd_request!(GetServerInfo);
    impl_into_turkiumd_request!(GetSyncStatus);
    impl_into_turkiumd_request!(GetDaaScoreTimestampEstimate);
    impl_into_turkiumd_request!(GetFeeEstimate);
    impl_into_turkiumd_request!(GetFeeEstimateExperimental);
    impl_into_turkiumd_request!(GetCurrentBlockColor);
    impl_into_turkiumd_request!(GetUtxoReturnAddress);
    impl_into_turkiumd_request!(GetVirtualChainFromBlockV2);

    impl_into_turkiumd_request!(NotifyBlockAdded);
    impl_into_turkiumd_request!(NotifyNewBlockTemplate);
    impl_into_turkiumd_request!(NotifyUtxosChanged);
    impl_into_turkiumd_request!(NotifyPruningPointUtxoSetOverride);
    impl_into_turkiumd_request!(NotifyFinalityConflict);
    impl_into_turkiumd_request!(NotifyVirtualDaaScoreChanged);
    impl_into_turkiumd_request!(NotifyVirtualChainChanged);
    impl_into_turkiumd_request!(NotifySinkBlueScoreChanged);

    macro_rules! impl_into_turkiumd_request {
        ($name:tt) => {
            paste::paste! {
                impl_into_turkiumd_request_ex!(turkium_rpc_core::[<$name Request>],[<$name RequestMessage>],[<$name Request>]);
            }
        };
    }

    use impl_into_turkiumd_request;

    macro_rules! impl_into_turkiumd_request_ex {
        // ($($core_struct:ident)::+, $($protowire_struct:ident)::+, $($variant:ident)::+) => {
        ($core_struct:path, $protowire_struct:ident, $variant:ident) => {
            // ----------------------------------------------------------------------------
            // rpc_core to protowire
            // ----------------------------------------------------------------------------

            impl From<&$core_struct> for turkiumd_request::Payload {
                fn from(item: &$core_struct) -> Self {
                    Self::$variant(item.into())
                }
            }

            impl From<&$core_struct> for TurkiumdRequest {
                fn from(item: &$core_struct) -> Self {
                    Self { id: 0, payload: Some(item.into()) }
                }
            }

            impl From<$core_struct> for turkiumd_request::Payload {
                fn from(item: $core_struct) -> Self {
                    Self::$variant((&item).into())
                }
            }

            impl From<$core_struct> for TurkiumdRequest {
                fn from(item: $core_struct) -> Self {
                    Self { id: 0, payload: Some((&item).into()) }
                }
            }

            // ----------------------------------------------------------------------------
            // protowire to rpc_core
            // ----------------------------------------------------------------------------

            impl TryFrom<&turkiumd_request::Payload> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &turkiumd_request::Payload) -> RpcResult<Self> {
                    if let turkiumd_request::Payload::$variant(request) = item {
                        request.try_into()
                    } else {
                        Err(RpcError::MissingRpcFieldError("Payload".to_string(), stringify!($variant).to_string()))
                    }
                }
            }

            impl TryFrom<&TurkiumdRequest> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &TurkiumdRequest) -> RpcResult<Self> {
                    item.payload
                        .as_ref()
                        .ok_or(RpcError::MissingRpcFieldError("TurkiumRequest".to_string(), "Payload".to_string()))?
                        .try_into()
                }
            }

            impl From<$protowire_struct> for TurkiumdRequest {
                fn from(item: $protowire_struct) -> Self {
                    Self { id: 0, payload: Some(turkiumd_request::Payload::$variant(item)) }
                }
            }

            impl From<$protowire_struct> for turkiumd_request::Payload {
                fn from(item: $protowire_struct) -> Self {
                    turkiumd_request::Payload::$variant(item)
                }
            }
        };
    }
    use impl_into_turkiumd_request_ex;
}

pub mod turkiumd_response_convert {
    use crate::protowire::*;
    use turkium_rpc_core::{RpcError, RpcResult};

    impl_into_turkiumd_response!(Shutdown);
    impl_into_turkiumd_response!(SubmitBlock);
    impl_into_turkiumd_response!(GetBlockTemplate);
    impl_into_turkiumd_response!(GetBlock);
    impl_into_turkiumd_response!(GetInfo);
    impl_into_turkiumd_response!(GetCurrentNetwork);

    impl_into_turkiumd_response!(GetPeerAddresses);
    impl_into_turkiumd_response!(GetSink);
    impl_into_turkiumd_response!(GetMempoolEntry);
    impl_into_turkiumd_response!(GetMempoolEntries);
    impl_into_turkiumd_response!(GetConnectedPeerInfo);
    impl_into_turkiumd_response!(AddPeer);
    impl_into_turkiumd_response!(SubmitTransaction);
    impl_into_turkiumd_response!(SubmitTransactionReplacement);
    impl_into_turkiumd_response!(GetSubnetwork);
    impl_into_turkiumd_response!(GetVirtualChainFromBlock);
    impl_into_turkiumd_response!(GetBlocks);
    impl_into_turkiumd_response!(GetBlockCount);
    impl_into_turkiumd_response!(GetBlockDagInfo);
    impl_into_turkiumd_response!(ResolveFinalityConflict);
    impl_into_turkiumd_response!(GetHeaders);
    impl_into_turkiumd_response!(GetUtxosByAddresses);
    impl_into_turkiumd_response!(GetBalanceByAddress);
    impl_into_turkiumd_response!(GetBalancesByAddresses);
    impl_into_turkiumd_response!(GetSinkBlueScore);
    impl_into_turkiumd_response!(Ban);
    impl_into_turkiumd_response!(Unban);
    impl_into_turkiumd_response!(EstimateNetworkHashesPerSecond);
    impl_into_turkiumd_response!(GetMempoolEntriesByAddresses);
    impl_into_turkiumd_response!(GetCoinSupply);
    impl_into_turkiumd_response!(Ping);
    impl_into_turkiumd_response!(GetMetrics);
    impl_into_turkiumd_response!(GetConnections);
    impl_into_turkiumd_response!(GetSystemInfo);
    impl_into_turkiumd_response!(GetServerInfo);
    impl_into_turkiumd_response!(GetSyncStatus);
    impl_into_turkiumd_response!(GetDaaScoreTimestampEstimate);
    impl_into_turkiumd_response!(GetFeeEstimate);
    impl_into_turkiumd_response!(GetFeeEstimateExperimental);
    impl_into_turkiumd_response!(GetCurrentBlockColor);
    impl_into_turkiumd_response!(GetUtxoReturnAddress);
    impl_into_turkiumd_response!(GetVirtualChainFromBlockV2);

    impl_into_turkiumd_notify_response!(NotifyBlockAdded);
    impl_into_turkiumd_notify_response!(NotifyNewBlockTemplate);
    impl_into_turkiumd_notify_response!(NotifyUtxosChanged);
    impl_into_turkiumd_notify_response!(NotifyPruningPointUtxoSetOverride);
    impl_into_turkiumd_notify_response!(NotifyFinalityConflict);
    impl_into_turkiumd_notify_response!(NotifyVirtualDaaScoreChanged);
    impl_into_turkiumd_notify_response!(NotifyVirtualChainChanged);
    impl_into_turkiumd_notify_response!(NotifySinkBlueScoreChanged);

    impl_into_turkiumd_notify_response!(NotifyUtxosChanged, StopNotifyingUtxosChanged);
    impl_into_turkiumd_notify_response!(NotifyPruningPointUtxoSetOverride, StopNotifyingPruningPointUtxoSetOverride);

    macro_rules! impl_into_turkiumd_response {
        ($name:tt) => {
            paste::paste! {
                impl_into_turkiumd_response_ex!(turkium_rpc_core::[<$name Response>],[<$name ResponseMessage>],[<$name Response>]);
            }
        };
        ($core_name:tt, $protowire_name:tt) => {
            paste::paste! {
                impl_into_turkiumd_response_base!(turkium_rpc_core::[<$core_name Response>],[<$protowire_name ResponseMessage>],[<$protowire_name Response>]);
            }
        };
    }
    use impl_into_turkiumd_response;

    macro_rules! impl_into_turkiumd_response_base {
        ($core_struct:path, $protowire_struct:ident, $variant:ident) => {
            // ----------------------------------------------------------------------------
            // rpc_core to protowire
            // ----------------------------------------------------------------------------

            impl From<RpcResult<$core_struct>> for $protowire_struct {
                fn from(item: RpcResult<$core_struct>) -> Self {
                    item.as_ref().map_err(|x| (*x).clone()).into()
                }
            }

            impl From<RpcError> for $protowire_struct {
                fn from(item: RpcError) -> Self {
                    let x: RpcResult<&$core_struct> = Err(item);
                    x.into()
                }
            }

            impl From<$protowire_struct> for turkiumd_response::Payload {
                fn from(item: $protowire_struct) -> Self {
                    turkiumd_response::Payload::$variant(item)
                }
            }

            impl From<$protowire_struct> for TurkiumdResponse {
                fn from(item: $protowire_struct) -> Self {
                    Self { id: 0, payload: Some(turkiumd_response::Payload::$variant(item)) }
                }
            }
        };
    }
    use impl_into_turkiumd_response_base;

    macro_rules! impl_into_turkiumd_response_ex {
        ($core_struct:path, $protowire_struct:ident, $variant:ident) => {
            // ----------------------------------------------------------------------------
            // rpc_core to protowire
            // ----------------------------------------------------------------------------

            impl From<RpcResult<&$core_struct>> for turkiumd_response::Payload {
                fn from(item: RpcResult<&$core_struct>) -> Self {
                    turkiumd_response::Payload::$variant(item.into())
                }
            }

            impl From<RpcResult<&$core_struct>> for TurkiumdResponse {
                fn from(item: RpcResult<&$core_struct>) -> Self {
                    Self { id: 0, payload: Some(item.into()) }
                }
            }

            impl From<RpcResult<$core_struct>> for turkiumd_response::Payload {
                fn from(item: RpcResult<$core_struct>) -> Self {
                    turkiumd_response::Payload::$variant(item.into())
                }
            }

            impl From<RpcResult<$core_struct>> for TurkiumdResponse {
                fn from(item: RpcResult<$core_struct>) -> Self {
                    Self { id: 0, payload: Some(item.into()) }
                }
            }

            impl_into_turkiumd_response_base!($core_struct, $protowire_struct, $variant);

            // ----------------------------------------------------------------------------
            // protowire to rpc_core
            // ----------------------------------------------------------------------------

            impl TryFrom<&turkiumd_response::Payload> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &turkiumd_response::Payload) -> RpcResult<Self> {
                    if let turkiumd_response::Payload::$variant(response) = item {
                        response.try_into()
                    } else {
                        Err(RpcError::MissingRpcFieldError("Payload".to_string(), stringify!($variant).to_string()))
                    }
                }
            }

            impl TryFrom<&TurkiumdResponse> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &TurkiumdResponse) -> RpcResult<Self> {
                    item.payload
                        .as_ref()
                        .ok_or(RpcError::MissingRpcFieldError("TurkiumResponse".to_string(), "Payload".to_string()))?
                        .try_into()
                }
            }
        };
    }
    use impl_into_turkiumd_response_ex;

    macro_rules! impl_into_turkiumd_notify_response {
        ($name:tt) => {
            impl_into_turkiumd_response!($name);

            paste::paste! {
                impl_into_turkiumd_notify_response_ex!(turkium_rpc_core::[<$name Response>],[<$name ResponseMessage>]);
            }
        };
        ($core_name:tt, $protowire_name:tt) => {
            impl_into_turkiumd_response!($core_name, $protowire_name);

            paste::paste! {
                impl_into_turkiumd_notify_response_ex!(turkium_rpc_core::[<$core_name Response>],[<$protowire_name ResponseMessage>]);
            }
        };
    }
    use impl_into_turkiumd_notify_response;

    macro_rules! impl_into_turkiumd_notify_response_ex {
        ($($core_struct:ident)::+, $protowire_struct:ident) => {
            // ----------------------------------------------------------------------------
            // rpc_core to protowire
            // ----------------------------------------------------------------------------

            impl<T> From<Result<(), T>> for $protowire_struct
            where
                T: Into<RpcError>,
            {
                fn from(item: Result<(), T>) -> Self {
                    item
                        .map(|_| $($core_struct)::+{})
                        .map_err(|err| err.into()).into()
                }
            }

        };
    }
    use impl_into_turkiumd_notify_response_ex;
}
