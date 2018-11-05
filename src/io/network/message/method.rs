//! # Method
//!
//! `method` is the module providing the type describing the network actions' methods.

use std::fmt;

use base::Result;
use base::Checkable;
use base::Sizable;
use base::Datable;
use base::Serializable;
use io::Permission;

/// Type that represent a network action method.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum Method {
    /// Ping a node.
    Ping,
    /// Retrieve a session.
    Session,
    /// Count a resource.
    Count,
    /// List a resource.
    List,
    /// Lookup a resource.
    Lookup,
    /// Get a resource.
    Get,
    /// Create a resource.
    Create,
    /// Update a resource.
    Update,
    /// Upsert a resource.
    Upsert,
    /// Delete a resource.
    Delete,
    /// Eval operation.
    Eval,
    /// Mutable eval operation.
    EvalMut,
}

impl Method {
    /// Parses a `Method` from a `&str`.
    pub fn parse(s: &str) -> Result<Method> {
        match s {
            "ping" => Ok(Method::Ping),
            "session" => Ok(Method::Session),
            "count" => Ok(Method::Count),
            "list" => Ok(Method::List),
            "lookup" => Ok(Method::Lookup),
            "get" => Ok(Method::Get),
            "create" => Ok(Method::Create),
            "update" => Ok(Method::Update),
            "upsert" => Ok(Method::Upsert),
            "delete" => Ok(Method::Delete),
            "eval" => Ok(Method::Eval),
            "evalmut" => Ok(Method::EvalMut),
            _ => Err("unknown method".into())
        }
    }

    /// Checks a `Permission` against the `Method`.
    pub fn check_permission(&self, permission: &Permission) -> Result<()> {
        if self == &Method::Session {
            return Ok(());
        }

        if permission >= &Permission::Write {
            if self < &Method::Create || self == &Method::Eval {
                return Err(String::from("invalid permission"));
            }
        } else if permission > &Permission::None && permission < &Permission::Write {
            if self == &Method::Ping || (self >= &Method::Create && self != &Method::Eval) {
                return Err(String::from("invalid permission"));
            }
        } else if permission == &Permission::None {
            if self != &Method::Ping {
                return Err(String::from("invalid permission"));
            }
        }

        Ok(())
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Method::Ping => write!(f, "ping"),
            Method::Session => write!(f, "session"),
            Method::Count => write!(f, "count"),
            Method::List => write!(f, "list"),
            Method::Lookup => write!(f, "lookup"),
            Method::Get => write!(f, "get"),
            Method::Create => write!(f, "create"),
            Method::Update => write!(f, "update"),
            Method::Upsert => write!(f, "upsert"),
            Method::Delete => write!(f, "delete"),
            Method::Eval => write!(f, "eval"),
            Method::EvalMut => write!(f, "evalmut"),
        }
    }
}

impl Default for Method {
    fn default() -> Method {
        Method::Ping
    }
}

impl Sizable for Method {
    fn size(&self) -> u64 {
        0u8.size()
    }
}

impl Checkable for Method {}

impl Serializable for Method {}

impl Datable for Method {}