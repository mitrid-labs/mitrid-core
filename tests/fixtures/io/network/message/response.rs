use mitrid_core::io::network::Response as BasicResponse;

use fixtures::base::Payload;
use fixtures::crypto::Digest;
use fixtures::io::Address;

pub type Response = BasicResponse<(), Address, Payload, Digest, Payload>;