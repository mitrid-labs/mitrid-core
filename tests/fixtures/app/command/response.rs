use mitrid_core::app::Response as BasicResponse;

use fixtures::base::Payload;
use fixtures::app::Address;

pub type Response = BasicResponse<Address, Payload>;