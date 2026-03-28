use crate::pb::turkiumd_message::Payload as TurkiumdMessagePayload;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum TurkiumdMessagePayloadType {
    Addresses = 0,
    Block,
    Transaction,
    BlockLocator,
    RequestAddresses,
    RequestRelayBlocks,
    RequestTransactions,
    IbdBlock,
    InvRelayBlock,
    InvTransactions,
    Ping,
    Pong,
    Verack,
    Version,
    TransactionNotFound,
    Reject,
    PruningPointUtxoSetChunk,
    RequestIbdBlocks,
    UnexpectedPruningPoint,
    IbdBlockLocator,
    IbdBlockLocatorHighestHash,
    RequestNextPruningPointUtxoSetChunk,
    DonePruningPointUtxoSetChunks,
    IbdBlockLocatorHighestHashNotFound,
    BlockWithTrustedData,
    DoneBlocksWithTrustedData,
    RequestPruningPointAndItsAnticone,
    BlockHeaders,
    RequestNextHeaders,
    DoneHeaders,
    RequestPruningPointUtxoSet,
    RequestHeaders,
    RequestBlockLocator,
    PruningPoints,
    RequestPruningPointProof,
    PruningPointProof,
    Ready,
    BlockWithTrustedDataV4,
    TrustedData,
    RequestIbdChainBlockLocator,
    IbdChainBlockLocator,
    RequestAntipast,
    RequestNextPruningPointAndItsAnticoneBlocks,
    BlockBody,
    RequestBlockBodies,
}

impl From<&TurkiumdMessagePayload> for TurkiumdMessagePayloadType {
    fn from(payload: &TurkiumdMessagePayload) -> Self {
        match payload {
            TurkiumdMessagePayload::Addresses(_) => TurkiumdMessagePayloadType::Addresses,
            TurkiumdMessagePayload::Block(_) => TurkiumdMessagePayloadType::Block,
            TurkiumdMessagePayload::Transaction(_) => TurkiumdMessagePayloadType::Transaction,
            TurkiumdMessagePayload::BlockLocator(_) => TurkiumdMessagePayloadType::BlockLocator,
            TurkiumdMessagePayload::RequestAddresses(_) => TurkiumdMessagePayloadType::RequestAddresses,
            TurkiumdMessagePayload::RequestRelayBlocks(_) => TurkiumdMessagePayloadType::RequestRelayBlocks,
            TurkiumdMessagePayload::RequestTransactions(_) => TurkiumdMessagePayloadType::RequestTransactions,
            TurkiumdMessagePayload::IbdBlock(_) => TurkiumdMessagePayloadType::IbdBlock,
            TurkiumdMessagePayload::InvRelayBlock(_) => TurkiumdMessagePayloadType::InvRelayBlock,
            TurkiumdMessagePayload::InvTransactions(_) => TurkiumdMessagePayloadType::InvTransactions,
            TurkiumdMessagePayload::Ping(_) => TurkiumdMessagePayloadType::Ping,
            TurkiumdMessagePayload::Pong(_) => TurkiumdMessagePayloadType::Pong,
            TurkiumdMessagePayload::Verack(_) => TurkiumdMessagePayloadType::Verack,
            TurkiumdMessagePayload::Version(_) => TurkiumdMessagePayloadType::Version,
            TurkiumdMessagePayload::TransactionNotFound(_) => TurkiumdMessagePayloadType::TransactionNotFound,
            TurkiumdMessagePayload::Reject(_) => TurkiumdMessagePayloadType::Reject,
            TurkiumdMessagePayload::PruningPointUtxoSetChunk(_) => TurkiumdMessagePayloadType::PruningPointUtxoSetChunk,
            TurkiumdMessagePayload::RequestIbdBlocks(_) => TurkiumdMessagePayloadType::RequestIbdBlocks,
            TurkiumdMessagePayload::UnexpectedPruningPoint(_) => TurkiumdMessagePayloadType::UnexpectedPruningPoint,
            TurkiumdMessagePayload::IbdBlockLocator(_) => TurkiumdMessagePayloadType::IbdBlockLocator,
            TurkiumdMessagePayload::IbdBlockLocatorHighestHash(_) => TurkiumdMessagePayloadType::IbdBlockLocatorHighestHash,
            TurkiumdMessagePayload::RequestNextPruningPointUtxoSetChunk(_) => {
                TurkiumdMessagePayloadType::RequestNextPruningPointUtxoSetChunk
            }
            TurkiumdMessagePayload::DonePruningPointUtxoSetChunks(_) => TurkiumdMessagePayloadType::DonePruningPointUtxoSetChunks,
            TurkiumdMessagePayload::IbdBlockLocatorHighestHashNotFound(_) => {
                TurkiumdMessagePayloadType::IbdBlockLocatorHighestHashNotFound
            }
            TurkiumdMessagePayload::BlockWithTrustedData(_) => TurkiumdMessagePayloadType::BlockWithTrustedData,
            TurkiumdMessagePayload::DoneBlocksWithTrustedData(_) => TurkiumdMessagePayloadType::DoneBlocksWithTrustedData,
            TurkiumdMessagePayload::RequestPruningPointAndItsAnticone(_) => {
                TurkiumdMessagePayloadType::RequestPruningPointAndItsAnticone
            }
            TurkiumdMessagePayload::BlockHeaders(_) => TurkiumdMessagePayloadType::BlockHeaders,
            TurkiumdMessagePayload::RequestNextHeaders(_) => TurkiumdMessagePayloadType::RequestNextHeaders,
            TurkiumdMessagePayload::DoneHeaders(_) => TurkiumdMessagePayloadType::DoneHeaders,
            TurkiumdMessagePayload::RequestPruningPointUtxoSet(_) => TurkiumdMessagePayloadType::RequestPruningPointUtxoSet,
            TurkiumdMessagePayload::RequestHeaders(_) => TurkiumdMessagePayloadType::RequestHeaders,
            TurkiumdMessagePayload::RequestBlockLocator(_) => TurkiumdMessagePayloadType::RequestBlockLocator,
            TurkiumdMessagePayload::PruningPoints(_) => TurkiumdMessagePayloadType::PruningPoints,
            TurkiumdMessagePayload::RequestPruningPointProof(_) => TurkiumdMessagePayloadType::RequestPruningPointProof,
            TurkiumdMessagePayload::PruningPointProof(_) => TurkiumdMessagePayloadType::PruningPointProof,
            TurkiumdMessagePayload::Ready(_) => TurkiumdMessagePayloadType::Ready,
            TurkiumdMessagePayload::BlockWithTrustedDataV4(_) => TurkiumdMessagePayloadType::BlockWithTrustedDataV4,
            TurkiumdMessagePayload::TrustedData(_) => TurkiumdMessagePayloadType::TrustedData,
            TurkiumdMessagePayload::RequestIbdChainBlockLocator(_) => TurkiumdMessagePayloadType::RequestIbdChainBlockLocator,
            TurkiumdMessagePayload::IbdChainBlockLocator(_) => TurkiumdMessagePayloadType::IbdChainBlockLocator,
            TurkiumdMessagePayload::RequestAntipast(_) => TurkiumdMessagePayloadType::RequestAntipast,
            TurkiumdMessagePayload::RequestNextPruningPointAndItsAnticoneBlocks(_) => {
                TurkiumdMessagePayloadType::RequestNextPruningPointAndItsAnticoneBlocks
            }
            TurkiumdMessagePayload::BlockBody(_) => TurkiumdMessagePayloadType::BlockBody,
            TurkiumdMessagePayload::RequestBlockBodies(_) => TurkiumdMessagePayloadType::RequestBlockBodies,
        }
    }
}
