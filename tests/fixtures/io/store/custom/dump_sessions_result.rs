use mitrid_core::base::Result;
use mitrid_core::base::Sizable;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::base::Datable;

use fixtures::io::Session;

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

        if self.count != self.sessions.len() as u64 {
            return Err(String::from("invalid length"));
        }

        Ok(())
    }
}

impl Serializable for DumpSessions {}

impl Datable for DumpSessions {}