use std::net::SocketAddr;
use std::str::FromStr;

use mitrid_core::base::Result;
use mitrid_core::base::{Sizable, VariableSize};
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::base::Datable;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Address(pub String);

impl Address {
    pub fn new(addr: &str) -> Address {
        Address(addr.into())
    }

    pub fn to_string(&self) -> String {
        self.0.clone().into()
    }

    pub fn from_socket(socket: &SocketAddr) -> Address {
        Address(socket.to_string())
    }

    pub fn to_socket(&self) -> Result<SocketAddr> {
        match SocketAddr::from_str(&self.0) {
            Ok(socket) => Ok(socket),
            Err(e) => Err(format!("{:?}", e)),
        }
    }
}

impl Sizable for Address {
    fn size(&self) -> u64 {
        self.0.size()
    }
}

impl VariableSize for Address {}

impl Checkable for Address {}

impl Serializable for Address {}

impl Datable for Address {}