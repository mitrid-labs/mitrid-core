use mitrid_core::base::Result;
use mitrid_core::base::Sizable;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::base::Datable;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum StoreEvalMutParams {
    Clear
}

impl Default for StoreEvalMutParams {
    fn default() -> StoreEvalMutParams {
        StoreEvalMutParams::Clear
    }
}

impl Sizable for StoreEvalMutParams {
    fn size(&self) -> u64 {
        match self {
            &StoreEvalMutParams::Clear => 0u8.size()
        }
    }
}

impl Checkable for StoreEvalMutParams {
    fn check(&self) -> Result<()> {
        match self {
            &StoreEvalMutParams::Clear => Ok(())
        }
    }
}

impl Serializable for StoreEvalMutParams {}

impl Datable for StoreEvalMutParams {}