//! # Session
//!
//! `session` is the module providing the type that represent an I/O session.

use base::Result;
use base::Checkable;
use base::Serializable;
use base::Sizable;
use base::Datable;
use utils::Timestamp;
use io::Permission;

/// A type representing an I/O session.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Session<P>
    where   P: Datable
{
    /// Id of the session.
    pub id: u64,
    /// Permission guaranteed by the session.
    pub permission: Permission,
    /// Expiration time of the session.
    pub expire_at: Timestamp,
    /// Payload of the session.
    pub payload: P,
}

impl<P> Session<P>
    where   P: Datable
{
    /// Creates a new `Session` from its components.
    pub fn new(id: u64, permission: &Permission, expire_at: &Timestamp, payload: &P) -> Result<Session<P>> {
        permission.check()?;
        payload.check()?;
        
        let session = Session {
            id: id,
            permission: permission.to_owned(),
            expire_at: expire_at.to_owned(),
            payload: payload.to_owned(),
        };

        Ok(session)
    }

    /// Returns if the `Session` has already expired.
    pub fn is_expired(&self) -> Result<bool> {
        let now = Timestamp::now()?;
        Ok(self.expire_at <= now)
    }
}

impl<P> Checkable for Session<P>
    where   P: Datable
{
    fn check(&self) -> Result<()> {
        self.permission.check()?;
        self.payload.check()
    }
}         

impl<P> Sizable for Session<P>
    where   P: Datable
{
    fn size(&self) -> u64 {
        self.id.size() +
            self.permission.size() +
            self.expire_at.size() +
            self.payload.size()
    }
}       

impl<P> Serializable for Session<P>
    where   P: Datable + Serializable
{}

impl<P> Datable for Session<P>
    where   P: Datable
{}  