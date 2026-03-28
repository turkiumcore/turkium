use crate::{
    pow::{self, HeaderHasher},
    proto::{
        turkiumd_message::Payload, GetBlockTemplateRequestMessage, GetInfoRequestMessage,
        NotifyBlockAddedRequestMessage, NotifyNewBlockTemplateRequestMessage, RpcBlock, SubmitBlockRequestMessage,
        TurkiumdMessage,
    },
    Hash,
};

impl TurkiumdMessage {
    #[must_use]
    #[inline(always)]
    pub fn get_info_request() -> Self {
        TurkiumdMessage { payload: Some(Payload::GetInfoRequest(GetInfoRequestMessage {})) }
    }
    #[must_use]
    #[inline(always)]
    pub fn notify_block_added() -> Self {
        TurkiumdMessage { payload: Some(Payload::NotifyBlockAddedRequest(NotifyBlockAddedRequestMessage {})) }
    }
    #[must_use]
    #[inline(always)]
    pub fn submit_block(block: RpcBlock) -> Self {
        TurkiumdMessage {
            payload: Some(Payload::SubmitBlockRequest(SubmitBlockRequestMessage {
                block: Some(block),
                allow_non_daa_blocks: false,
            })),
        }
    }
}

impl From<GetInfoRequestMessage> for TurkiumdMessage {
    #[inline(always)]
    fn from(a: GetInfoRequestMessage) -> Self {
        TurkiumdMessage { payload: Some(Payload::GetInfoRequest(a)) }
    }
}
impl From<NotifyBlockAddedRequestMessage> for TurkiumdMessage {
    #[inline(always)]
    fn from(a: NotifyBlockAddedRequestMessage) -> Self {
        TurkiumdMessage { payload: Some(Payload::NotifyBlockAddedRequest(a)) }
    }
}

impl From<GetBlockTemplateRequestMessage> for TurkiumdMessage {
    #[inline(always)]
    fn from(a: GetBlockTemplateRequestMessage) -> Self {
        TurkiumdMessage { payload: Some(Payload::GetBlockTemplateRequest(a)) }
    }
}

impl From<NotifyNewBlockTemplateRequestMessage> for TurkiumdMessage {
    fn from(a: NotifyNewBlockTemplateRequestMessage) -> Self {
        TurkiumdMessage { payload: Some(Payload::NotifyNewBlockTemplateRequest(a)) }
    }
}

impl RpcBlock {
    #[must_use]
    #[inline(always)]
    pub fn block_hash(&self) -> Option<Hash> {
        let mut hasher = HeaderHasher::new();
        pow::serialize_header(&mut hasher, self.header.as_ref()?, false);
        Some(hasher.finalize())
    }
}
