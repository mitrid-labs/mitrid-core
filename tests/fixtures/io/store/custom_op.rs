use std::fmt;

use mitrid_core::base::Result;
use mitrid_core::base::Sizable;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::base::Datable;

use fixtures::io::Session;
use fixtures::io::store::{StoreKey, StoreValue};

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

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Serialize, Deserialize)]
pub enum CustomParams {
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
            &CustomParams::Dump(ref params) => params.size()
        }
    }
}

impl Checkable for CustomParams {
    fn check(&self) -> Result<()> {
        match self {
            &CustomParams::Dump(ref params) => params.check()
        }
    }
}

impl Serializable for CustomParams {}

impl Datable for CustomParams {}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash, Serialize, Deserialize)]
pub struct DumpSessions {
    pub count: u64,
    pub sessions: Vec<(u64, Session)>,
}

impl Sizable for DumpSessions {
    fn size(&self) -> u64 {
        self.count.size() + self.sessions.size()
    }
}

impl Checkable for DumpSessions {
    fn check(&self) -> Result<()> {
        self.count.check()?;
        self.sessions.check()
    }
}

impl Serializable for DumpSessions {}

impl Datable for DumpSessions {}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash, Serialize, Deserialize)]
pub struct DumpItems {
    pub count: u64,
    pub items: Vec<(StoreKey, StoreValue)>,
}

impl Sizable for DumpItems {
    fn size(&self) -> u64 {
        self.count.size() + self.items.size()
    }
}

impl Checkable for DumpItems {
    fn check(&self) -> Result<()> {
        self.count.check()?;
        self.items.check()
    }
}

impl Serializable for DumpItems {}

impl Datable for DumpItems {}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash, Serialize, Deserialize)]
pub struct DumpAll {
    pub sessions_count: u64,
    pub sessions: Vec<(u64, Session)>,
    pub items_count: u64,
    pub items: Vec<(StoreKey, StoreValue)>,
}

impl Sizable for DumpAll {
    fn size(&self) -> u64 {
        self.sessions_count.size() +
        self.items.size() +
        self.items_count.size() +
        self.sessions.size()
    }
}

impl Checkable for DumpAll {
    fn check(&self) -> Result<()> {
        self.sessions_count.check()?;
        self.items.check()?;
        self.items_count.check()?;
        self.sessions.check()
    }
}

impl Serializable for DumpAll {}

impl Datable for DumpAll {}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Serialize, Deserialize)]
pub enum CustomResult {
    DumpSessions(DumpSessions),
    DumpItems(DumpItems),
    DumpAll(DumpAll),
}

impl Default for CustomResult {
    fn default() -> CustomResult {
        CustomResult::DumpAll(DumpAll::default())
    }
}

impl Sizable for CustomResult {
    fn size(&self) -> u64 {
        match self {
            &CustomResult::DumpSessions(ref result) => result.size(),
            &CustomResult::DumpItems(ref result) => result.size(),
            &CustomResult::DumpAll(ref result) => result.size(),
        }
    }
}

impl Checkable for CustomResult {
    fn check(&self) -> Result<()> {
        match self {
            &CustomResult::DumpSessions(ref result) => result.check(),
            &CustomResult::DumpItems(ref result) => result.check(),
            &CustomResult::DumpAll(ref result) => result.check(),
        }
    }
}

impl Serializable for CustomResult {}

impl Datable for CustomResult {}