use std::fmt;

use mitrid_core::base::Result;
use mitrid_core::base::Sizable;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::base::Datable;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum DumpParams {
    Sessions,
    Items,
    All,
}

impl DumpParams {
    #[allow(dead_code)]
    pub fn parse(s: &str) -> Result<DumpParams> {
        match s {
            "sessions" => Ok(DumpParams::Sessions),
            "items" => Ok(DumpParams::Items),
            "all" => Ok(DumpParams::All),
            _ => Err("unknown params".into())
        }
    }
}

impl fmt::Display for DumpParams {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DumpParams::Sessions => write!(f, "sessions"),
            DumpParams::Items => write!(f, "items"),
            DumpParams::All => write!(f, "all"),
        }
    }
}

impl Default for DumpParams {
    fn default() -> DumpParams {
        DumpParams::All
    }
}

impl Sizable for DumpParams {
    fn size(&self) -> u64 {
        0u8.size()
    }
}

impl Checkable for DumpParams {}

impl Serializable for DumpParams {}

impl Datable for DumpParams {}