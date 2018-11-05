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
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum Resource {
    /// No data.
    None,
    /// Session.
    Session,
    /// Node data.
    Node,
    /// Coin data.
    Coin,
    /// Input data.
    Input,
    /// Output data.
    Output,
    /// Transaction data.
    Transaction,
    /// Blocknode data.
    BlockNode,
    /// Block data.
    Block,
    /// Blockgraph data.
    BlockGraph,
    /// Eval params.
    EvalParams,
    /// Eval result.
    EvalResult,
    /// Mutable eval params.
    EvalMutParams,
    /// Mutable eval result.
    EvalMutResult,
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
            "coin" => Ok(Resource::Coin),
            "input" => Ok(Resource::Input),
            "output" => Ok(Resource::Output),
            "transaction" => Ok(Resource::Transaction),
            "blocknode" => Ok(Resource::BlockNode),
            "block" => Ok(Resource::Block),
            "blockgraph" => Ok(Resource::BlockGraph),
            "evalparams" => Ok(Resource::EvalParams),
            "evalresult" => Ok(Resource::EvalResult),
            "evalmutparams" => Ok(Resource::EvalMutParams),
            "evalmutresult" => Ok(Resource::EvalMutResult),
            "error" => Ok(Resource::Error),
            _ => Err("unknown resource".into())
        }
    }

    /// Checks a `Method` against the `Resource`.
    pub fn check_method(&self, method: &Method) -> Result<()> {
        if self == &Resource::Error {
            return Ok(());
        }

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
                if self <= &Resource::Session || self >= &Resource::EvalParams {
                    return Err(String::from("invalid method"));
                }
            },
            10 => {
                match self {
                    &Resource::EvalParams | &Resource::EvalResult => {},
                    _ => {
                        return Err(String::from("invalid method"));
                    },
                }
            },
            11 => {
                match self {
                    &Resource::EvalMutParams | &Resource::EvalMutResult => {},
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
            Resource::Coin => write!(f, "coin"),
            Resource::Input => write!(f, "input"),
            Resource::Output => write!(f, "output"),
            Resource::Transaction => write!(f, "transaction"),
            Resource::BlockNode => write!(f, "blocknode"),
            Resource::Block => write!(f, "block"),
            Resource::BlockGraph => write!(f, "blockgraph"),
            Resource::EvalParams => write!(f, "evalparams"),
            Resource::EvalResult => write!(f, "evalresult"),
            Resource::EvalMutParams => write!(f, "evalmutparams"),
            Resource::EvalMutResult => write!(f, "evalmutresult"),
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