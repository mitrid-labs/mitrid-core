use mitrid_core::models::Node as BasicNode;

use fixtures::base::Payload;
use fixtures::models::Address;

pub type Node = BasicNode<Address, Payload>;