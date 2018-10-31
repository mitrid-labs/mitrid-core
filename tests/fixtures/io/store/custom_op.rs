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

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash, Serialize, Deserialize)]
pub struct DumpSessions {
    pub count: u64,
    pub sessions: Vec<(u64, Session)>,
}

impl DumpSessions {
    pub fn new(sessions: &Vec<Session>) -> Result<DumpSessions> {
        sessions.check()?;

        let mut values = Vec::new();
        for session in sessions {
            values.push((session.id, session.to_owned()));
        }

        let dump = DumpSessions {
            count: values.len() as u64,
            sessions: values,
        };

        Ok(dump)
    }
}

impl Sizable for DumpSessions {
    fn size(&self) -> u64 {
        self.count.size() + self.sessions.size()
    }
}

impl Checkable for DumpSessions {
    fn check(&self) -> Result<()> {
        self.count.check()?;
        self.sessions.check()?;

        for (id, session) in self.sessions.iter() {
            if session.id != *id {
                return Err(String::from("invalid id"));
            }
        }

        Ok(())
    }
}

impl Serializable for DumpSessions {}

impl Datable for DumpSessions {}

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

impl DumpAll {
    pub fn new(sessions: &Vec<Session>, items: &Vec<(StoreKey, StoreValue)>) -> Result<DumpAll> {
        sessions.check()?;

        let mut session_values = Vec::new();
        for session in sessions {
            session_values.push((session.id, session.to_owned()));
        }

        let dump = DumpAll {
            sessions_count: session_values.len() as u64,
            sessions: session_values.to_owned(),
            items_count: items.len() as u64,
            items: items.to_owned(),
        };

        Ok(dump)
    }
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
            &CustomResult::Size(size) => size.check(),
            &CustomResult::DumpSessions(ref result) => result.check(),
            &CustomResult::DumpItems(ref result) => result.check(),
            &CustomResult::DumpAll(ref result) => result.check(),
        }
    }
}

impl Serializable for CustomResult {}

impl Datable for CustomResult {}