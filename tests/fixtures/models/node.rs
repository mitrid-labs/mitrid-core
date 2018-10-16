use mitrid_core::models::Node as BasicNode;

use fixtures::models::Address;
use fixtures::models::Payload;

pub type Node = BasicNode<Address, Payload>;