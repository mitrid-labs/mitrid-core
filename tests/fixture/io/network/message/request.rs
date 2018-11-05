use mitrid_core::io::network::Request as BasicRequest;

use fixture::base::Payload;
use fixture::crypto::Digest;
use fixture::io::Address;

pub type Request = BasicRequest<(), Address, Payload, Digest, Payload>;