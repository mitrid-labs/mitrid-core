use mitrid_core::io::network::Request as BasicRequest;

use fixture::base::Payload;
use fixture::crypto::Digest;

pub type Request = BasicRequest<(), Digest, Payload>;