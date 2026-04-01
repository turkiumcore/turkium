use crate::pb::turkiumd_message::Payload as turkiumdMessagePayload;

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

impl From<&turkiumdMessagePayload> for TurkiumdMessagePayloadType {
    fn from(payload: &turkiumdMessagePayload) -> Self {
        match payload {
            turkiumdMessagePayload::Addresses(_) => TurkiumdMessagePayloadType::Addresses,
            turkiumdMessagePayload::Block(_) => TurkiumdMessagePayloadType::Block,
            turkiumdMessagePayload::Transaction(_) => TurkiumdMessagePayloadType::Transaction,
            turkiumdMessagePayload::BlockLocator(_) => TurkiumdMessagePayloadType::BlockLocator,
            turkiumdMessagePayload::RequestAddresses(_) => TurkiumdMessagePayloadType::RequestAddresses,
            turkiumdMessagePayload::RequestRelayBlocks(_) => TurkiumdMessagePayloadType::RequestRelayBlocks,
            turkiumdMessagePayload::RequestTransactions(_) => TurkiumdMessagePayloadType::RequestTransactions,
            turkiumdMessagePayload::IbdBlock(_) => TurkiumdMessagePayloadType::IbdBlock,
            turkiumdMessagePayload::InvRelayBlock(_) => TurkiumdMessagePayloadType::InvRelayBlock,
            turkiumdMessagePayload::InvTransactions(_) => TurkiumdMessagePayloadType::InvTransactions,
            turkiumdMessagePayload::Ping(_) => TurkiumdMessagePayloadType::Ping,
            turkiumdMessagePayload::Pong(_) => TurkiumdMessagePayloadType::Pong,
            turkiumdMessagePayload::Verack(_) => TurkiumdMessagePayloadType::Verack,
            turkiumdMessagePayload::Version(_) => TurkiumdMessagePayloadType::Version,
            turkiumdMessagePayload::TransactionNotFound(_) => TurkiumdMessagePayloadType::TransactionNotFound,
            turkiumdMessagePayload::Reject(_) => TurkiumdMessagePayloadType::Reject,
            turkiumdMessagePayload::PruningPointUtxoSetChunk(_) => TurkiumdMessagePayloadType::PruningPointUtxoSetChunk,
            turkiumdMessagePayload::RequestIbdBlocks(_) => TurkiumdMessagePayloadType::RequestIbdBlocks,
            turkiumdMessagePayload::UnexpectedPruningPoint(_) => TurkiumdMessagePayloadType::UnexpectedPruningPoint,
            turkiumdMessagePayload::IbdBlockLocator(_) => TurkiumdMessagePayloadType::IbdBlockLocator,
            turkiumdMessagePayload::IbdBlockLocatorHighestHash(_) => TurkiumdMessagePayloadType::IbdBlockLocatorHighestHash,
            turkiumdMessagePayload::RequestNextPruningPointUtxoSetChunk(_) => {
                TurkiumdMessagePayloadType::RequestNextPruningPointUtxoSetChunk
            }
            turkiumdMessagePayload::DonePruningPointUtxoSetChunks(_) => TurkiumdMessagePayloadType::DonePruningPointUtxoSetChunks,
            turkiumdMessagePayload::IbdBlockLocatorHighestHashNotFound(_) => {
                TurkiumdMessagePayloadType::IbdBlockLocatorHighestHashNotFound
            }
            turkiumdMessagePayload::BlockWithTrustedData(_) => TurkiumdMessagePayloadType::BlockWithTrustedData,
            turkiumdMessagePayload::DoneBlocksWithTrustedData(_) => TurkiumdMessagePayloadType::DoneBlocksWithTrustedData,
            turkiumdMessagePayload::RequestPruningPointAndItsAnticone(_) => {
                TurkiumdMessagePayloadType::RequestPruningPointAndItsAnticone
            }
            turkiumdMessagePayload::BlockHeaders(_) => TurkiumdMessagePayloadType::BlockHeaders,
            turkiumdMessagePayload::RequestNextHeaders(_) => TurkiumdMessagePayloadType::RequestNextHeaders,
            turkiumdMessagePayload::DoneHeaders(_) => TurkiumdMessagePayloadType::DoneHeaders,
            turkiumdMessagePayload::RequestPruningPointUtxoSet(_) => TurkiumdMessagePayloadType::RequestPruningPointUtxoSet,
            turkiumdMessagePayload::RequestHeaders(_) => TurkiumdMessagePayloadType::RequestHeaders,
            turkiumdMessagePayload::RequestBlockLocator(_) => TurkiumdMessagePayloadType::RequestBlockLocator,
            turkiumdMessagePayload::PruningPoints(_) => TurkiumdMessagePayloadType::PruningPoints,
            turkiumdMessagePayload::RequestPruningPointProof(_) => TurkiumdMessagePayloadType::RequestPruningPointProof,
            turkiumdMessagePayload::PruningPointProof(_) => TurkiumdMessagePayloadType::PruningPointProof,
            turkiumdMessagePayload::Ready(_) => TurkiumdMessagePayloadType::Ready,
            turkiumdMessagePayload::BlockWithTrustedDataV4(_) => TurkiumdMessagePayloadType::BlockWithTrustedDataV4,
            turkiumdMessagePayload::TrustedData(_) => TurkiumdMessagePayloadType::TrustedData,
            turkiumdMessagePayload::RequestIbdChainBlockLocator(_) => TurkiumdMessagePayloadType::RequestIbdChainBlockLocator,
            turkiumdMessagePayload::IbdChainBlockLocator(_) => TurkiumdMessagePayloadType::IbdChainBlockLocator,
            turkiumdMessagePayload::RequestAntipast(_) => TurkiumdMessagePayloadType::RequestAntipast,
            turkiumdMessagePayload::RequestNextPruningPointAndItsAnticoneBlocks(_) => {
                TurkiumdMessagePayloadType::RequestNextPruningPointAndItsAnticoneBlocks
            }
            turkiumdMessagePayload::BlockBody(_) => TurkiumdMessagePayloadType::BlockBody,
            turkiumdMessagePayload::RequestBlockBodies(_) => TurkiumdMessagePayloadType::RequestBlockBodies,
        }
    }
}
