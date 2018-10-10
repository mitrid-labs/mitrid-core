use mitrid_core::base::{Sizable, VariableSize};
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::base::Datable;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Address(String);

impl Address {
    pub fn new(addr: &str) -> Address {
        Address(addr.into())
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