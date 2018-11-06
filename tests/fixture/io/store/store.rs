use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use mitrid_core::base::Result;
use mitrid_core::base::Sizable;
use mitrid_core::base::Checkable;
use mitrid_core::util::{Timestamp, TimestampDiff};
use mitrid_core::io::Permission;
use mitrid_core::io::Store as BasicStore;

use fixture::io::Session;

pub const SESSION_DURATION: u64 = 3600;

#[derive(Debug, Default)]
pub struct Store {
    pub(crate) sessions: Arc<Mutex<HashMap<u64, Session>>>,
    pub(crate) items: Arc<Mutex<HashMap<Vec<u8>, Vec<u8>>>>,
}

impl Store {
    pub fn new() -> Store {
        Store::default()
    }

    pub fn clear(&mut self) {
        let sessions = &mut *self.sessions.lock().unwrap();
        let items = &mut *self.items.lock().unwrap();

        sessions.clear();
        items.clear();
    }
}

impl Sizable for Store {
    fn size(&self) -> u64 {
        let sessions = &*self.sessions.lock().unwrap();
        let items = &*self.items.lock().unwrap();

        let mut size = 0;

        for (id, session) in sessions.iter() {
            size += id.size();
            size += session.size();
        }

        for (key, value) in items.iter() {
            size += key.size();
            size += value.size();
        }

        size
    }
}

impl Checkable for Store {
    fn check(&self) -> Result<()> {
        let sessions = &*self.sessions.lock().unwrap();

        for (id, session) in sessions.iter() {
            session.check()?;

            if session.id != *id {
                return Err(String::from("invalid id"));
            }
        }

        Ok(())
    }
}

impl BasicStore<()> for Store {
    fn session(&mut self, permission: &Permission) -> Result<Session> {
        permission.check()?;

        let sessions = &mut *self.sessions.lock().unwrap();

        let id = (sessions.len() + 1) as u64;

        let mut expires_at = Timestamp::now()?;
        let duration = TimestampDiff::from_secs(SESSION_DURATION);
        expires_at += duration;

        let session = Session::new(id, permission, &expires_at, &())?;
        sessions.insert(id, session.clone());

        Ok(session)
    }
    
    fn count(&mut self,
            session: &Session,
            from: Option<Vec<u8>>,
            to: Option<Vec<u8>>)
        -> Result<u64>
    {
        session.check()?;

        if session.is_expired()? {
            return Err(String::from("expired session"));
        }

        if session.permission > Permission::Read {
            return Err(String::from("invalid permission")).into();
        }

        from.check()?;
        to.check()?;

        if let Some(ref from) = from {
            if let Some(ref to) = to {
                if from >= to {
                    return Err(String::from("invalid range"));
                } 
            }
        }

        let sessions = &*self.sessions.lock().unwrap();

        if !sessions.contains_key(&session.id) {
            return Err(String::from("session not found"));
        }

        let items = &*self.items.lock().unwrap();

        if from.is_none() && to.is_none() {
            return Ok(items.len() as u64);
        }

        let mut count = 0;

        for key in items.keys() {
            if let Some(ref from) = from {
                if key < from {
                    continue;
                }
            }

            if let Some(ref to) = to {
                if key >= to {
                    break;
                }
            }
            
            count += 1;
        }

        Ok(count)
    }
    
    fn list(&mut self,
            session: &Session,
            from: Option<Vec<u8>>,
            to: Option<Vec<u8>>,
            count: Option<u64>)
        -> Result<Vec<Vec<u8>>>
    {
        session.check()?;

        if session.is_expired()? {
            return Err(String::from("expired session"));
        }

        if session.permission > Permission::Read {
            return Err(String::from("invalid permission")).into();
        }

        from.check()?;
        to.check()?;
        count.check()?;

        if let Some(ref from) = from {
            if let Some(ref to) = to {
                if from >= to {
                    return Err(String::from("invalid range"));
                } 
            }
        }

        if let Some(count) = count {
            if count == 0 {
                return Err(String::from("invalid count"));
            }
        }

        let sessions = &*self.sessions.lock().unwrap();

        if !sessions.contains_key(&session.id) {
            return Err(String::from("session not found"));
        }

        let items = &*self.items.lock().unwrap();

        if from.is_none() && to.is_none() {
            let mut values = Vec::new();

            for value in items.values() {
                values.push(value.to_owned());
            }

            return Ok(values);
        }

        let mut list = Vec::new();
        let mut cnt: i64 = if let Some(count) = count {
            count.to_owned() as i64
        } else {
            -1
        };

        for (key, value) in items.iter() {
            if cnt == 0 {
                break;
            }

            if let Some(ref from) = from {
                if key < from {
                    continue;
                }
            }

            if let Some(ref to) = to {
                if key >= to {
                    break;
                }
            }

            list.push(value.to_owned());
            cnt -= 1;
        }

        Ok(list)
    }
    
    fn lookup(&mut self,
              session: &Session,
              key: &[u8])
        -> Result<bool>
    {
        session.check()?;

        if session.is_expired()? {
            return Err(String::from("expired session"));
        }

        if session.permission > Permission::Read {
            return Err(String::from("invalid permission")).into();
        }

        let sessions = &*self.sessions.lock().unwrap();

        if !sessions.contains_key(&session.id) {
            return Err(String::from("session not found"));
        }

        let items = &*self.items.lock().unwrap();

        let found = items.contains_key(key);

        Ok(found)
    }
    
    fn get(&mut self,
           session: &Session,
           key: &[u8])
        -> Result<Vec<u8>>
    {
        session.check()?;

        if session.is_expired()? {
            return Err(String::from("expired session"));
        }

        if session.permission > Permission::Read {
            return Err(String::from("invalid permission")).into();
        }

        let sessions = &*self.sessions.lock().unwrap();

        if !sessions.contains_key(&session.id) {
            return Err(String::from("session not found"));
        }

        let items = &*self.items.lock().unwrap();

        if let Some(item) = items.get(key) {
            Ok(item.to_owned())
        } else {
            Err(String::from("not found"))
        }
    }
    
    fn create(&mut self,
              session: &Session,
              key: &[u8],
              value: &[u8])
        -> Result<()>
    {
        session.check()?;

        if session.is_expired()? {
            return Err(String::from("expired session"));
        }

        if session.permission < Permission::Write {
            return Err(String::from("invalid permission")).into();
        }

        let sessions = &*self.sessions.lock().unwrap();

        if !sessions.contains_key(&session.id) {
            return Err(String::from("session not found"));
        }

        let items = &mut *self.items.lock().unwrap();

        if items.contains_key(key) {
            return Err(String::from("already found"));
        }

        items.insert(key.to_owned(), value.to_owned());

        Ok(())
    }
    
    fn update(&mut self,
              session: &Session,
              key: &[u8],
              value: &[u8])
        -> Result<()>
    {
        session.check()?;

        if session.is_expired()? {
            return Err(String::from("expired session"));
        }

        if session.permission < Permission::Write {
            return Err(String::from("invalid permission")).into();
        }

        let sessions = &*self.sessions.lock().unwrap();

        if !sessions.contains_key(&session.id) {
            return Err(String::from("session not found"));
        }

        let items = &mut *self.items.lock().unwrap();

        if !items.contains_key(key) {
            return Err(String::from("not found found"));
        }

        items.insert(key.to_owned(), value.to_owned());

        Ok(())
    }
    
    fn upsert(&mut self,
              session: &Session,
              key: &[u8],
              value: &[u8])
        -> Result<()>
    {
        session.check()?;

        if session.is_expired()? {
            return Err(String::from("expired session"));
        }

        if session.permission < Permission::Write {
            return Err(String::from("invalid permission")).into();
        }

        let sessions = &*self.sessions.lock().unwrap();

        if !sessions.contains_key(&session.id) {
            return Err(String::from("session not found"));
        }

        let items = &mut *self.items.lock().unwrap();

        items.insert(key.to_owned(), value.to_owned());

        Ok(())
    }
    
    fn delete(&mut self,
              session: &Session,
              key: &[u8])
        -> Result<()>
    {
        session.check()?;

        if session.is_expired()? {
            return Err(String::from("expired session"));
        }

        if session.permission < Permission::Write {
            return Err(String::from("invalid permission")).into();
        }

        let sessions = &*self.sessions.lock().unwrap();

        if !sessions.contains_key(&session.id) {
            return Err(String::from("session not found"));
        }

        let items = &mut *self.items.lock().unwrap();

        if let Some(_) = items.remove(key) {
            Ok(())
        } else {
            Err(String::from("not found"))
        }
    }
}