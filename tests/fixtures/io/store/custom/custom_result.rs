use mitrid_core::base::Result;
use mitrid_core::base::Sizable;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::base::Datable;

use fixtures::io::store::custom::{DumpSessions, DumpItems, DumpAll};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Serialize, Deserialize)]
pub enum CustomResult {
    Size(u64),
    DumpSessions(DumpSessions),
    DumpItems(DumpItems),
    DumpAll(DumpAll),
}

impl CustomResult {
    pub fn new_size(size: u64) -> CustomResult {
        CustomResult::Size(size)
    }

    pub fn new_dump_sessions(dump: &DumpSessions) -> Result<CustomResult> {
        dump.check()?;

        let res = CustomResult::DumpSessions(dump.to_owned());

        Ok(res)
    }

    pub fn new_dump_items(dump: &DumpItems) -> Result<CustomResult> {
        dump.check()?;

        let res = CustomResult::DumpItems(dump.to_owned());

        Ok(res)
    }

    pub fn new_dump_all(dump: &DumpAll) -> Result<CustomResult> {
        dump.check()?;

        let res = CustomResult::DumpAll(dump.to_owned());

        Ok(res)
    }
}

impl Default for CustomResult {
    fn default() -> CustomResult {
        CustomResult::DumpAll(DumpAll::default())
    }
}

impl Sizable for CustomResult {
    fn size(&self) -> u64 {
        match self {
            &CustomResult::Size(size) => size.size(),
            &CustomResult::DumpSessions(ref result) => result.size(),
            &CustomResult::DumpItems(ref result) => result.size(),
            &CustomResult::DumpAll(ref result) => result.size(),
        }
    }
}

impl Checkable for CustomResult {
    fn check(&self) -> Result<()> {
        match self {
            &CustomResult::Size(result) => result.check(),
            &CustomResult::DumpSessions(ref result) => result.check(),
            &CustomResult::DumpItems(ref result) => result.check(),
            &CustomResult::DumpAll(ref result) => result.check(),
        }
    }
}

impl Serializable for CustomResult {}

impl Datable for CustomResult {}