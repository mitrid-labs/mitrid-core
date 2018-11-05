use mitrid_core::base::Result;
use mitrid_core::base::Sizable;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::base::Datable;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum StoreEvalMutResult {
    Cleared
}

impl Default for StoreEvalMutResult {
    fn default() -> StoreEvalMutResult {
        StoreEvalMutResult::Cleared
    }
}

impl Sizable for StoreEvalMutResult {
    fn size(&self) -> u64 {
        match self {
            &StoreEvalMutResult::Cleared => 0u8.size()
        }
    }
}

impl Checkable for StoreEvalMutResult {
    fn check(&self) -> Result<()> {
        match self {
            &StoreEvalMutResult::Cleared => Ok(())
        }
    }
}

impl Serializable for StoreEvalMutResult {}

impl Datable for StoreEvalMutResult {}