use crate::protowire::{self};
use crate::{from, try_from};
use Turkium_rpc_core::RpcError;

// ----------------------------------------------------------------------------
// rpc_core to protowire
// ----------------------------------------------------------------------------

from!(item: &Turkium_rpc_core::RpcDataVerbosityLevel, protowire::RpcDataVerbosityLevel, {
    match item {
        Turkium_rpc_core::RpcDataVerbosityLevel::None => protowire::RpcDataVerbosityLevel::None,
        Turkium_rpc_core::RpcDataVerbosityLevel::Low => protowire::RpcDataVerbosityLevel::Low,
        Turkium_rpc_core::RpcDataVerbosityLevel::High => protowire::RpcDataVerbosityLevel::High,
        Turkium_rpc_core::RpcDataVerbosityLevel::Full => protowire::RpcDataVerbosityLevel::Full,
    }
});

// ----------------------------------------------------------------------------
// protowire to rpc_core
// ----------------------------------------------------------------------------

try_from!(item: &protowire::RpcDataVerbosityLevel, Turkium_rpc_core::RpcDataVerbosityLevel,  {
    match item {
        protowire::RpcDataVerbosityLevel::None => Turkium_rpc_core::RpcDataVerbosityLevel::None,
        protowire::RpcDataVerbosityLevel::Low => Turkium_rpc_core::RpcDataVerbosityLevel::Low,
        protowire::RpcDataVerbosityLevel::High => Turkium_rpc_core::RpcDataVerbosityLevel::High,
        protowire::RpcDataVerbosityLevel::Full => Turkium_rpc_core::RpcDataVerbosityLevel::Full
    }
});
