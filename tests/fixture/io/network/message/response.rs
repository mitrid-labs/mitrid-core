use mitrid_core::io::network::Response as BasicResponse;

use fixture::base::Payload;
use fixture::crypto::Digest;

pub type Response = BasicResponse<(), Digest, Payload>;