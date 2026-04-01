use crate::protowire::{self};
use crate::{from, try_from};
use turkium_rpc_core::RpcError;

// ----------------------------------------------------------------------------
// rpc_core to protowire
// ----------------------------------------------------------------------------

from!(item: &turkium_rpc_core::RpcDataVerbosityLevel, protowire::RpcDataVerbosityLevel, {
    match item {
        turkium_rpc_core::RpcDataVerbosityLevel::None => protowire::RpcDataVerbosityLevel::None,
        turkium_rpc_core::RpcDataVerbosityLevel::Low => protowire::RpcDataVerbosityLevel::Low,
        turkium_rpc_core::RpcDataVerbosityLevel::High => protowire::RpcDataVerbosityLevel::High,
        turkium_rpc_core::RpcDataVerbosityLevel::Full => protowire::RpcDataVerbosityLevel::Full,
    }
});

// ----------------------------------------------------------------------------
// protowire to rpc_core
// ----------------------------------------------------------------------------

try_from!(item: &protowire::RpcDataVerbosityLevel, turkium_rpc_core::RpcDataVerbosityLevel,  {
    match item {
        protowire::RpcDataVerbosityLevel::None => turkium_rpc_core::RpcDataVerbosityLevel::None,
        protowire::RpcDataVerbosityLevel::Low => turkium_rpc_core::RpcDataVerbosityLevel::Low,
        protowire::RpcDataVerbosityLevel::High => turkium_rpc_core::RpcDataVerbosityLevel::High,
        protowire::RpcDataVerbosityLevel::Full => turkium_rpc_core::RpcDataVerbosityLevel::Full
    }
});
