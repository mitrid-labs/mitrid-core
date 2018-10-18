use base::Result;
use base::Checkable;
use base::Sizable;
use base::Serializable;
use base::Datable;

use std::fmt;

bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct Permission: u8 {
        const None = 0;
        const Read = 1 << 0; 
        const Write = 1 << 1;
    }
}

impl Permission {
    pub fn parse(s: &str) -> Result<Permission> {
        match s {
            "none" => Ok(Permission::None),
            "read" => Ok(Permission::Read),
            "write" => Ok(Permission::Write),
            _ => Err("unknown permission".into())
        }
    }
}

impl Default for Permission {
    fn default() -> Self {
        Permission::None
    }
}

impl fmt::Display for Permission {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Permission::None => write!(f, "none"),
            &Permission::Read => write!(f, "read"),
            &Permission::Write => write!(f, "write"),
            &Permission { .. } => Err(fmt::Error), // unreachable
        }
    }
}

impl Checkable for Permission {}

impl Serializable for Permission {}

impl Sizable for Permission {
    fn size(&self) -> u64 {
        0u8.size()
    }
}

impl Datable for Permission {}