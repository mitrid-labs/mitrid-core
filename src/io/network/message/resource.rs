//! # Resource
//!
//! `resource` is the module providing the type representing a network resource.

use std::fmt;

use base::Result;
use base::Sizable;
use base::Checkable;
use base::Serializable;
use base::Datable;
use io::Method;

/// Type representing the data of a network message.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum Resource {
    /// No data.
    None,
    /// Session.
    Session,
    /// Node data.
    Node,
    /// Nodes data
    Nodes,
    /// Coin data.
    Coin,
    /// Coins data.
    Coins,
    /// Input data.
    Input,
    /// Inputs data.
    Inputs,
    /// Output data.
    Output,
    /// Outputs data.
    Outputs,
    /// Transaction data.
    Transaction,
    /// Transactions data.
    Transactions,
    /// Blocknode data.
    BlockNode,
    /// Blocknodes data.
    BlockNodes,
    /// Block data.
    Block,
    /// Blocks data.
    Blocks,
    /// Blockgraph data.
    BlockGraph,
    /// Blockgraphs data.
    BlockGraphs,
    /// Custom data.
    Custom,
    /// Error data.
    Error,
}

impl Resource {
    /// Parses a `Resource` from a `&str`.
    pub fn parse(s: &str) -> Result<Resource> {
        match s {
            "none" => Ok(Resource::None),
            "session" => Ok(Resource::Session),
            "node" => Ok(Resource::Node),
            "nodes" => Ok(Resource::Nodes),
            "coin" => Ok(Resource::Coin),
            "coins" => Ok(Resource::Coins),
            "input" => Ok(Resource::Input),
            "inputs" => Ok(Resource::Inputs),
            "output" => Ok(Resource::Output),
            "outputs" => Ok(Resource::Outputs),
            "transaction" => Ok(Resource::Transaction),
            "transactions" => Ok(Resource::Transactions),
            "blocknode" => Ok(Resource::BlockNode),
            "blocknodes" => Ok(Resource::BlockNodes),
            "block" => Ok(Resource::Block),
            "blocks" => Ok(Resource::Block),
            "blockgraph" => Ok(Resource::BlockGraph),
            "blockgraphs" => Ok(Resource::BlockGraphs),
            "custom" => Ok(Resource::Custom),
            "error" => Ok(Resource::Error),
            _ => Err("unknown resource".into())
        }
    }

    /// Checks a `Method` against the `Resource`.
    pub fn check_method(&self, method: &Method) -> Result<()> {
        match *method as u8 {
            0 => {
                if self != &Resource::None {
                    return Err(String::from("invalid method"));
                }
            },
            1 => {
                match self {
                    &Resource::Session => {},
                    _ => {
                        return Err(String::from("invalid method"));
                    },   
                }
            },
            2...9 => {
                match self {
                    &Resource::None | &Resource::Session | &Resource::Custom => {
                        return Err(String::from("invalid method"));
                    },
                    _ => {},
                }
            },
            10 => {
                match self {
                    &Resource::Custom => {},
                    _ => {
                        return Err(String::from("invalid method"));
                    },
                }
            },
            _ => {
                return Err(String::from("invalid method"));
            }
        }

        Ok(())
    }
}

impl Default for Resource {
    fn default() -> Self {
        Resource::None
    }
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Resource::None => write!(f, "none"),
            Resource::Session => write!(f, "session"),
            Resource::Node => write!(f, "node"),
            Resource::Nodes => write!(f, "nodes"),
            Resource::Coin => write!(f, "coin"),
            Resource::Coins => write!(f, "coins"),
            Resource::Input => write!(f, "input"),
            Resource::Inputs => write!(f, "inputs"),
            Resource::Output => write!(f, "output"),
            Resource::Outputs => write!(f, "outputs"),
            Resource::Transaction => write!(f, "transaction"),
            Resource::Transactions => write!(f, "transactions"),
            Resource::BlockNode => write!(f, "blocknode"),
            Resource::BlockNodes => write!(f, "blocknodes"),
            Resource::Block => write!(f, "block"),
            Resource::Blocks => write!(f, "blocks"),
            Resource::BlockGraph => write!(f, "blockgraph"),
            Resource::BlockGraphs => write!(f, "blockgraphs"),
            Resource::Custom => write!(f, "custom"),
            Resource::Error => write!(f, "error"),
        }
    }
}

impl Sizable for Resource {
    fn size(&self) -> u64 {
        1
    }
}

impl Checkable for Resource {}

impl Serializable for Resource {}

impl Datable for Resource {}