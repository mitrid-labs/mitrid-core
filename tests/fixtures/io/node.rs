use mitrid_core::io::Node as BasicNode;

use fixtures::base::Payload;
use fixtures::io::Address;

pub type Node = BasicNode<Address, Payload>;