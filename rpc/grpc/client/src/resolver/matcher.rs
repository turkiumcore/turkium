use turkium_grpc_core::protowire::{TurkiumdRequest, TurkiumdResponse, turkiumd_request, turkiumd_response};

pub(crate) trait Matcher<T> {
    fn is_matching(&self, response: T) -> bool;
}

impl Matcher<&turkiumd_response::Payload> for turkiumd_request::Payload {
    fn is_matching(&self, response: &turkiumd_response::Payload) -> bool {
        use turkiumd_request::Payload;
        match self {
            // TODO: implement for each payload variant supporting request/response pairing
            Payload::GetBlockRequest(request) => {
                if let turkiumd_response::Payload::GetBlockResponse(response) = response {
                    if let Some(block) = response.block.as_ref() {
                        if let Some(verbose_data) = block.verbose_data.as_ref() {
                            return verbose_data.hash == request.hash;
                        }
                        return true;
                    } else if let Some(error) = response.error.as_ref() {
                        // the response error message should contain the requested hash
                        return error.message.contains(request.hash.as_str());
                    }
                }
                false
            }

            _ => true,
        }
    }
}

impl Matcher<&TurkiumdResponse> for TurkiumdRequest {
    fn is_matching(&self, response: &TurkiumdResponse) -> bool {
        if let Some(ref response) = response.payload
            && let Some(ref request) = self.payload
        {
            return request.is_matching(response);
        }
        false
    }
}
