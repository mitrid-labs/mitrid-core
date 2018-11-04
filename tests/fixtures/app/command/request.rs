use mitrid_core::app::Request as BasicRequest;

use fixtures::base::Payload;
use fixtures::app::Address;

pub type Request = BasicRequest<Address, Payload>;