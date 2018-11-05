use mitrid_core::io::network::Response as BasicResponse;

use fixture::base::Payload;
use fixture::crypto::Digest;
use fixture::io::Address;

pub type Response = BasicResponse<(), Address, Payload, Digest, Payload>;