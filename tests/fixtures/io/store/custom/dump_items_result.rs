use mitrid_core::base::Result;
use mitrid_core::base::Sizable;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::base::Datable;

use fixtures::io::store::{StoreKey, StoreValue};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash, Serialize, Deserialize)]
pub struct DumpItems {
    pub count: u64,
    pub items: Vec<(StoreKey, StoreValue)>,
}

impl DumpItems {
    pub fn new(items: &Vec<(StoreKey, StoreValue)>) -> DumpItems {
        DumpItems {
            count: items.len() as u64,
            items: items.to_owned(),
        }
    }
}

impl Sizable for DumpItems {
    fn size(&self) -> u64 {
        self.count.size() + self.items.size()
    }
}

impl Checkable for DumpItems {
    fn check(&self) -> Result<()> {
        self.count.check()?;
        self.items.check()?;

        if self.count != self.items.len() as u64 {
            return Err(String::from("invalid length"));
        }

        Ok(())
    }
}

impl Serializable for DumpItems {}

impl Datable for DumpItems {}