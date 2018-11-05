use mitrid_core::model::BlockNode as BaseBlockNode;

use fixture::crypto::Digest;

pub type BlockNode = BaseBlockNode<Digest>;