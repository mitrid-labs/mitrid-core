use mitrid_core::app::Request as BasicRequest;

use fixture::base::Payload;
use fixture::app::Address;

pub type Request = BasicRequest<Address, Payload>;