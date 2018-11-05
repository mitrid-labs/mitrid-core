use mitrid_core::base::Result;
use mitrid_core::base::Sizable;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::base::Datable;

use fixture::io::store::eval::{DumpSessions, DumpItems, DumpAll};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Serialize, Deserialize)]
pub enum StoreEvalResult {
    Size(u64),
    DumpSessions(DumpSessions),
    DumpItems(DumpItems),
    DumpAll(DumpAll),
}

impl StoreEvalResult {
    pub fn new_size(size: u64) -> StoreEvalResult {
        StoreEvalResult::Size(size)
    }

    pub fn new_dump_sessions(dump: &DumpSessions) -> Result<StoreEvalResult> {
        dump.check()?;

        let res = StoreEvalResult::DumpSessions(dump.to_owned());

        Ok(res)
    }

    pub fn new_dump_items(dump: &DumpItems) -> Result<StoreEvalResult> {
        dump.check()?;

        let res = StoreEvalResult::DumpItems(dump.to_owned());

        Ok(res)
    }

    pub fn new_dump_all(dump: &DumpAll) -> Result<StoreEvalResult> {
        dump.check()?;

        let res = StoreEvalResult::DumpAll(dump.to_owned());

        Ok(res)
    }
}

impl Default for StoreEvalResult {
    fn default() -> StoreEvalResult {
        StoreEvalResult::DumpAll(DumpAll::default())
    }
}

impl Sizable for StoreEvalResult {
    fn size(&self) -> u64 {
        match self {
            &StoreEvalResult::Size(size) => size.size(),
            &StoreEvalResult::DumpSessions(ref result) => result.size(),
            &StoreEvalResult::DumpItems(ref result) => result.size(),
            &StoreEvalResult::DumpAll(ref result) => result.size(),
        }
    }
}

impl Checkable for StoreEvalResult {
    fn check(&self) -> Result<()> {
        match self {
            &StoreEvalResult::Size(result) => result.check(),
            &StoreEvalResult::DumpSessions(ref result) => result.check(),
            &StoreEvalResult::DumpItems(ref result) => result.check(),
            &StoreEvalResult::DumpAll(ref result) => result.check(),
        }
    }
}

impl Serializable for StoreEvalResult {}

impl Datable for StoreEvalResult {}