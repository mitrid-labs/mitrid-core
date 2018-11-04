use mitrid_core::io::network::Request as BasicRequest;

use fixtures::base::Payload;
use fixtures::crypto::Digest;
use fixtures::io::Address;

pub type Request = BasicRequest<(), Address, Payload, Digest, Payload>;