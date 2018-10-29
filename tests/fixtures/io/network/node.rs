use mitrid_core::io::network::Node as BasicNode;

use fixtures::base::Payload;
use fixtures::io::Address;

pub type Node = BasicNode<Address, Payload>;