use mitrid_core::io::network::Node as BasicNode;

use fixture::base::Payload;
use fixture::io::Address;

pub type Node = BasicNode<Address, Payload>;