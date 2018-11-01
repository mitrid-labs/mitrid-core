use mitrid_core::base::Result;
use mitrid_core::base::Sizable;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::base::Datable;

use fixtures::io::Session;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash, Serialize, Deserialize)]
pub struct DumpAll {
    pub sessions_count: u64,
    pub sessions: Vec<(u64, Session)>,
    pub items_count: u64,
    pub items: Vec<(Vec<u8>, Vec<u8>)>,
}

impl DumpAll {
    pub fn new(sessions: &Vec<Session>, items: &Vec<(Vec<u8>, Vec<u8>)>) -> Result<DumpAll> {
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
        self.sessions.check()?;
        self.items.check()?;
        self.items_count.check()?;

        for (id, session) in self.sessions.iter() {
            if session.id != *id {
                return Err(String::from("invalid id"));
            }
        }

        if self.sessions_count != self.sessions.len() as u64 {
            return Err(String::from("invalid length"));
        }

        if self.items_count != self.items.len() as u64 {
            return Err(String::from("invalid length"));
        }

        Ok(())
    }
}

impl Serializable for DumpAll {}

impl Datable for DumpAll {}