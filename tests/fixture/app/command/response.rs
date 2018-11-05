use mitrid_core::app::Response as BasicResponse;

use fixture::base::Payload;
use fixture::app::Address;

pub type Response = BasicResponse<Address, Payload>;