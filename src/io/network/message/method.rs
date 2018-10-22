//! # Method
//!
//! `method` is the module providing the type describing the network actions' methods.

use std::fmt;

use base::Result;
use base::Checkable;
use base::Sizable;
use base::Datable;
use base::Serializable;

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
    /// Upgrade a resource.
    Upgrade,
    /// Delete a resource.
    Delete,
    /// Custom action.
    Custom,
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
            "upgrade" => Ok(Method::Upgrade),
            "delete" => Ok(Method::Delete),
            "custom" => Ok(Method::Custom),
            _ => Err("unknown method".into())
        }
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
            Method::Upgrade => write!(f, "upgrade"),
            Method::Delete => write!(f, "delete"),
            Method::Custom => write!(f, "custom"),
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