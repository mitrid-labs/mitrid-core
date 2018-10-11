use mitrid_core::models::BlockNode as BaseBlockNode;

use fixtures::crypto::Digest;

#[allow(dead_code)]
pub type BlockNode = BaseBlockNode<Digest>;