use mitrid_core::base::Result;
use mitrid_core::base::Sizable;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::base::Datable;

use fixtures::io::store::custom::DumpParams;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Serialize, Deserialize)]
#[allow(unused_attributes)]
pub enum CustomParams {
    #[repr(u8)]
    Size,
    Dump(DumpParams)
}

impl Default for CustomParams {
    fn default() -> CustomParams {
        CustomParams::Dump(DumpParams::default())
    }
}

impl Sizable for CustomParams {
    fn size(&self) -> u64 {
        match self {
            &CustomParams::Size => 0u8.size(),
            &CustomParams::Dump(ref params) => params.size(),
        }
    }
}

impl Checkable for CustomParams {
    fn check(&self) -> Result<()> {
        match self {
            &CustomParams::Size => Ok(()),
            &CustomParams::Dump(ref params) => params.check()
        }
    }
}

impl Serializable for CustomParams {}

impl Datable for CustomParams {}