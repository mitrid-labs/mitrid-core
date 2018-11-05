//! # Store
//!
//! `store` is the module providing the traits implemented by stores and by types that
//! can be stored in and retrieved from a store.

use std::mem;

use base::Result;
use base::Checkable;
use base::Datable;
use base::Serializable;
use io::Permission;
use io::Session;

/// Trait representing the operations implemented by a store.
pub trait Store<S, PC, RC>
    where   S: Datable + Serializable,
            PC: Datable + Serializable,
            RC: Datable + Serializable,
            Self: 'static + Send + Sync + Checkable
{
    /// Retrieves a new `Session` from the store.
    fn session(&mut self, permission: &Permission) -> Result<Session<S>>;
    
    /// Counts the store items starting from the `from` key until, not included, the `to` key.
    fn count(&self, session: &Session<S>, from: &Option<Vec<u8>>, to: &Option<Vec<u8>>) -> Result<u64>;
    
    /// Lists the store items starting from the `from` key until, not included, the `to` key.
    fn list(&self,
            session: &Session<S>,
            from: &Option<Vec<u8>>,
            to: &Option<Vec<u8>>,
            count: &Option<u64>)
        -> Result<Vec<Vec<u8>>>;
    
    /// Lookups an item from its key.
    fn lookup(&self, session: &Session<S>, key: &[u8]) -> Result<bool>;
    
    /// Retrieves an item from its key. The item should already exist in the store before the operation.
    fn get(&self, session: &Session<S>, key: &[u8]) -> Result<Vec<u8>>;
    
    /// Creates an item in the store. The item should not exist in the store before the operation.
    fn create(&mut self, session: &Session<S>, key: &[u8], value: &[u8]) -> Result<()>;
    
    /// Updates an item in the store. The item should already exist in the store before the operation.
    fn update(&mut self, session: &Session<S>, key: &[u8], value: &[u8]) -> Result<()>;
    
    /// Creates an item in the store if absent, update it if present.
    fn upsert(&mut self, session: &Session<S>, key: &[u8], value: &[u8]) -> Result<()>;
    
    /// Deletes an item from the store. The item should already exist in the store before the operation.
    fn delete(&mut self, session: &Session<S>, key: &[u8]) -> Result<()>;
    
    /// Custom operation in the store.
    fn custom(&mut self, session: &Session<S>, params: &PC) -> Result<RC>;
}

/// Trait implemented by types that can be stored and retrieved from a store.
pub trait Storable<St, S, K, V, PC, RC>
    where   St: Store<S, PC, RC>,
            S: Datable + Serializable,
            K: Ord + Datable + Serializable,
            V: Datable + Serializable,
            PC: Datable + Serializable,
            RC: Datable + Serializable,
            Self: Datable + Serializable
{
    /// Returns the store prefix of the implementor.
    fn store_prefix() -> u64;

    /// Returns the store key of the implementor.
    fn store_key(&self) -> Result<K>;

    /// Retrieves the store value of the implementor.
    fn store_value(&self) -> Result<V>;

    /// Retrieves an instance of the implementor from a store value.
    fn from_store_value(value: &[u8]) -> Result<Self> {
        Self::from_bytes(value)
    }
    
    /// Counts the store items starting from the `from` key until, not included, the `to` key.
    fn store_count(store: &mut St, from: &Option<K>, to: &Option<K>) -> Result<u64> {
        let permission = Permission::Read;

        let session = store.session(&permission)?;

        from.check()?;
        to.check()?;

        if let Some(from) = from {
            if let Some(to) = to {
                if from >= to {
                    return Err(String::from("invalid range"));
                } 
            }
        }

        let store_from = if let Some(k) = from {
            let mut from_key = Vec::new();

            let prefix: [u8; 8] = unsafe { mem::transmute(Self::store_prefix()) };
            from_key.extend_from_slice(&prefix[..]);
            from_key.extend(&k.to_bytes()?);

            Some(from_key)
        } else {
            None
        };

        let store_to = if let Some(k) = to {
            let mut to_key = Vec::new();

            let prefix: [u8; 8] = unsafe { mem::transmute(Self::store_prefix()) };
            to_key.extend_from_slice(&prefix[..]);
            to_key.extend(&k.to_bytes()?);

            Some(to_key)
        } else {
            None
        };

        store.count(&session, &store_from, &store_to)
    }
    
    /// Lists the store items starting from the `from` key until, not included, the `to` key.
    fn store_list(store: &mut St,
                  from: &Option<K>,
                  to: &Option<K>,
                  count: &Option<u64>)
        -> Result<Vec<Self>>
    {
        let permission = Permission::Read;

        let session = store.session(&permission)?;

        from.check()?;
        to.check()?;

        if let Some(from) = from {
            if let Some(to) = to {
                if from >= to {
                    return Err(String::from("invalid range"));
                } 
            }
        }

        if let Some(count) = count {
            if *count == 0 {
                return Err(String::from("invalid count"));
            }
        }

        let store_from = if let Some(k) = from {
            let mut from_key = Vec::new();

            let prefix: [u8; 8] = unsafe { mem::transmute(Self::store_prefix()) };
            from_key.extend_from_slice(&prefix[..]);
            from_key.extend(&k.to_bytes()?);

            Some(from_key)
        } else {
            None
        };

        let store_to = if let Some(k) = to {
            let mut to_key = Vec::new();

            let prefix: [u8; 8] = unsafe { mem::transmute(Self::store_prefix()) };
            to_key.extend_from_slice(&prefix[..]);
            to_key.extend(&k.to_bytes()?);

            Some(to_key)
        } else {
            None
        };

        let mut list = Vec::new();

        for value in store.list(&session, &store_from, &store_to, count)?.iter() {
            list.push(Self::from_store_value(&value)?);
        }

        Ok(list)
    }
    
    /// Lookups an item from its key.
    fn store_lookup(store: &mut St, key: &K) -> Result<bool> {
        let permission = Permission::Read;

        let session = store.session(&permission)?;

        key.check()?;

        let mut store_key = Vec::new();

        let prefix: [u8; 8] = unsafe { mem::transmute(Self::store_prefix()) };
        store_key.extend_from_slice(&prefix[..]);
        store_key.extend(&key.to_bytes()?);

        store.lookup(&session, &store_key)
    }
    
    /// Retrieves an item from its key. The item should already exist in the store before the operation.
    fn store_get(store: &mut St, key: &K) -> Result<Self> {
        let permission = Permission::Read;

        let session = store.session(&permission)?;

        key.check()?;
        
        let mut store_key = Vec::new();

        let prefix: [u8; 8] = unsafe { mem::transmute(Self::store_prefix()) };
        store_key.extend_from_slice(&prefix[..]);
        store_key.extend(&key.to_bytes()?);

        let value = store.get(&session, &store_key)?;
        Self::from_store_value(&value)
    }
    
    /// Creates an item in the store. The item should not exist in the store before the operation.
    fn store_create(&self, store: &mut St) -> Result<()> {
        let permission = Permission::Write;

        let session = store.session(&permission)?;

        let key = self.store_key()?;

        let value = self.store_value()?;
        
        let mut store_key = Vec::new();

        let prefix: [u8; 8] = unsafe { mem::transmute(Self::store_prefix()) };
        store_key.extend_from_slice(&prefix[..]);
        store_key.extend(&key.to_bytes()?);

        let store_value = value.to_bytes()?;

        store.create(&session, &store_key, &store_value)
    }
    
    /// Updates the item in the store. The item should already exist in the store before the operation.
    fn store_update(&self, store: &mut St) -> Result<()> {
        let permission = Permission::Write;

        let session = store.session(&permission)?;

        let key = self.store_key()?;

        let value = self.store_value()?;
        
        let mut store_key = Vec::new();

        let prefix: [u8; 8] = unsafe { mem::transmute(Self::store_prefix()) };
        store_key.extend_from_slice(&prefix[..]);
        store_key.extend(&key.to_bytes()?);

        let store_value = value.to_bytes()?;

        store.update(&session, &store_key, &store_value)
    }
    
    /// Creates the item in the store if absent, update it if present.
    fn store_upsert(&self, store: &mut St) -> Result<()> {
        let permission = Permission::Write;

        let session = store.session(&permission)?;

        let key = self.store_key()?;

        let value = self.store_value()?;
        
        let mut store_key = Vec::new();

        let prefix: [u8; 8] = unsafe { mem::transmute(Self::store_prefix()) };
        store_key.extend_from_slice(&prefix[..]);
        store_key.extend(&key.to_bytes()?);

        let store_value = value.to_bytes()?;
        
        store.upsert(&session, &store_key, &store_value)
    }
    
    /// Deletes the item from the store. The item should already exist in the store before the operation.
    fn store_delete(&self, store: &mut St) -> Result<()> {
        let permission = Permission::Write;

        let session = store.session(&permission)?;

        let key = self.store_key()?;
        
        let mut store_key = Vec::new();

        let prefix: [u8; 8] = unsafe { mem::transmute(Self::store_prefix()) };
        store_key.extend_from_slice(&prefix[..]);
        store_key.extend(&key.to_bytes()?);
        
        store.delete(&session, &store_key)
    }

    /// Custom operation in the store.
    fn store_custom(store: &mut St, params: &PC, session: &Session<S>) -> Result<RC> {
        params.check()?;
        session.check()?;

        if session.is_expired()? {
            return Err(String::from("expired session"));
        }

        store.custom(session, params)
    }
}