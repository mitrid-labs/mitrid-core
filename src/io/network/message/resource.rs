//! # Resource
//!
//! `method` is the module providing the type describing the network resources.

use std::fmt;

use base::Result;
use base::Checkable;
use base::Sizable;
use base::Datable;
use base::Serializable;

/// Type that represent a network resource.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum Resource {
    /// No resource.
    None,
    /// Session resource.
    Session,
    /// Node resource.
    Node,
    /// Coin resource.
    Coin,
    /// Input resource.
    Input,
    /// Output resource.
    Output,
    /// Transaction resource.
    Transaction,
    /// Blocknode resource.
    BlockNode,
    /// Block resource.
    Block,
    /// Blockgraph resource.
    BlockGraph,
    /// Custom resource.
    Custom,
}

impl Resource {
    /// Parses a `Resource` from a `&str`.
    pub fn parse(s: &str) -> Result<Resource> {
        match s {
            "none" => Ok(Resource::None),
            "session" => Ok(Resource::Session),
            "node" => Ok(Resource::Node),
            "coin" => Ok(Resource::Coin),
            "input" => Ok(Resource::Input),
            "output" => Ok(Resource::Output),
            "transaction" => Ok(Resource::Transaction),
            "blocknode" => Ok(Resource::BlockNode),
            "block" => Ok(Resource::Block),
            "blockgraph" => Ok(Resource::BlockGraph),
            "custom" => Ok(Resource::Custom),
            _ => Err("unknown method".into())
        }
    }
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Resource::None => write!(f, "none"),
            Resource::Session => write!(f, "session"),
            Resource::Node => write!(f, "node"),
            Resource::Coin => write!(f, "coin"),
            Resource::Input => write!(f, "input"),
            Resource::Output => write!(f, "output"),
            Resource::Transaction => write!(f, "transaction"),
            Resource::BlockNode => write!(f, "blocknode"),
            Resource::Block => write!(f, "block"),
            Resource::BlockGraph => write!(f, "blockgraph"),
            Resource::Custom => write!(f, "custom"),
        }
    }
}

impl Default for Resource {
    fn default() -> Resource {
        Resource::None
    }
}

impl Sizable for Resource {
    fn size(&self) -> u64 {
        0u8.size()
    }
}

impl Checkable for Resource {}

impl Serializable for Resource {}

impl Datable for Resource {}