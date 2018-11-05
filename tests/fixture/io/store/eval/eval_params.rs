use mitrid_core::base::Result;
use mitrid_core::base::Sizable;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::base::Datable;

use fixture::io::store::eval::DumpParams;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Serialize, Deserialize)]
#[allow(unused_attributes)]
pub enum StoreEvalParams {
    #[repr(u8)]
    Size,
    Dump(DumpParams)
}

impl Default for StoreEvalParams {
    fn default() -> StoreEvalParams {
        StoreEvalParams::Dump(DumpParams::default())
    }
}

impl Sizable for StoreEvalParams {
    fn size(&self) -> u64 {
        match self {
            &StoreEvalParams::Size => 0u8.size(),
            &StoreEvalParams::Dump(ref params) => params.size(),
        }
    }
}

impl Checkable for StoreEvalParams {
    fn check(&self) -> Result<()> {
        match self {
            &StoreEvalParams::Size => Ok(()),
            &StoreEvalParams::Dump(ref params) => params.check()
        }
    }
}

impl Serializable for StoreEvalParams {}

impl Datable for StoreEvalParams {}