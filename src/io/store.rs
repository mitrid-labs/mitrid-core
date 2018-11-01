//! # Store
//!
//! `store` is the module providing the traits implemented by stores and by types that
//! can be stored in and retrieved from a store.

use base::Result;
use base::Checkable;
use base::Datable;
use base::Serializable;
use io::Permission;
use io::Session;

/// Trait representing the operations implemented by a store.
pub trait Store<S, P, PC, RC>
    where   S: Datable + Serializable,
            P: Datable,
            PC: Datable + Serializable,
            RC: Datable + Serializable,
            Self: 'static + Send + Sync + Checkable
{
    /// Retrieves a new `Session` from the store.
    fn session(&mut self, params: &P, permission: &Permission)
        -> Result<Session<S>>;
    
    /// Counts the store items starting from the `from` key until, not included, the `to` key.
    fn count(&self,
             session: &Session<S>,
             params: &P,
            from: &Option<Vec<u8>>,
            to: &Option<Vec<u8>>)
        -> Result<u64>;
    
    /// Lists the store items starting from the `from` key until, not included, the `to` key.
    fn list(&self,
            session: &Session<S>,
            params: &P,
            from: &Option<Vec<u8>>,
            to: &Option<Vec<u8>>,
            count: &Option<u64>)
        -> Result<Vec<Vec<u8>>>;
    
    /// Lookups an item from its key.
    fn lookup(&self,
              session: &Session<S>,
              params: &P,
              key: &[u8])
        -> Result<bool>;
    
    /// Retrieves an item from its key. The item should already exist in the store before the operation.
    fn get(&self,
           session: &Session<S>,
           params: &P,
           key: &[u8])
        -> Result<Vec<u8>>;
    
    /// Creates an item in the store. The item should not exist in the store before the operation.
    fn create(&mut self,
              session: &Session<S>,
              params: &P,
              key: &[u8],
              value: &[u8])
        -> Result<()>;
    
    /// Updates an item in the store. The item should already exist in the store before the operation.
    fn update(&mut self,
              session: &Session<S>,
              params: &P,
              key: &[u8],
              value: &[u8])
        -> Result<()>;
    
    /// Creates an item in the store if absent, update it if present.
    fn upsert(&mut self,
              session: &Session<S>,
              params: &P,
              key: &[u8],
              value: &[u8])
        -> Result<()>;
    
    /// Deletes an item from the store. The item should already exist in the store before the operation.
    fn delete(&mut self,
              session: &Session<S>,
              params: &P,
              key: &[u8])
        -> Result<()>;
    
    /// Custom operation in the store.
    fn custom(&mut self,
              session: &Session<S>,
              params: &PC)
        -> Result<RC>;
}

/// Trait implemented by types that can be stored and retrieved from a store.
pub trait Storable<St, S, K, V, P, PC, RC>
    where   St: Store<S, P, PC, RC>,
            S: Datable + Serializable,
            K: Ord + Datable + Serializable,
            V: Datable + Serializable,
            P: Datable,
            PC: Datable + Serializable,
            RC: Datable + Serializable,
            Self: Datable + Serializable
{
    /// Returns the store key of the implementor.
    fn store_key(&self) -> Result<K>;

    /// Retrieves the store value of the implementor.
    fn store_value(&self) -> Result<V>;

    /// Retrieves an instance of the implementor from a store value.
    fn from_store_value(value: &[u8]) -> Result<Self> {
        Self::from_bytes(value)
    }
    
    /// Counts the store items starting from the `from` key until, not included, the `to` key.
    fn store_count(store: &mut St,
                   params: &P,
                   from: &Option<K>,
                   to: &Option<K>)
        -> Result<u64>
    {
        params.check()?;

        let permission = Permission::Read;

        let session = store.session(&params, &permission)?;

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
           Some(k.to_bytes()?)
        } else {
            None
        };

        let store_to = if let Some(k) = to {
           Some(k.to_bytes()?)
        } else {
            None
        };

        store.count(&session, params, &store_from, &store_to)
    }
    
    /// Lists the store items starting from the `from` key until, not included, the `to` key.
    fn store_list(store: &mut St,
                  params: &P,
                  from: &Option<K>,
                  to: &Option<K>,
                  count: &Option<u64>)
        -> Result<Vec<Self>>
    {
        params.check()?;

        let permission = Permission::Read;

        let session = store.session(&params, &permission)?;

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
           Some(k.to_bytes()?)
        } else {
            None
        };

        let store_to = if let Some(k) = to {
           Some(k.to_bytes()?)
        } else {
            None
        };

        let mut list = Vec::new();

        for value in store.list(&session, params, &store_from, &store_to, count)?.iter() {
            list.push(Self::from_store_value(&value)?);
        }

        Ok(list)
    }
    
    /// Lookups an item from its key.
    fn store_lookup(store: &mut St,
                    params: &P,
                    key: &K)
        -> Result<bool>
    {
        params.check()?;

        let permission = Permission::Read;

        let session = store.session(&params, &permission)?;

        key.check()?;

        let store_key = key.to_bytes()?;

        store.lookup(&session, params, &store_key)
    }
    
    /// Retrieves an item from its key. The item should already exist in the store before the operation.
    fn store_get(store: &mut St,
                 params: &P,
                 key: &K)
        -> Result<Self>
    {
        params.check()?;

        let permission = Permission::Read;

        let session = store.session(&params, &permission)?;

        key.check()?;

        let store_key = key.to_bytes()?;

        let value = store.get(&session, params, &store_key)?;
        Self::from_store_value(&value)
    }
    
    /// Creates an item in the store. The item should not exist in the store before the operation.
    fn store_create(&self,
                    store: &mut St,
                    params: &P)
        -> Result<()>
    {
        params.check()?;

        let permission = Permission::Write;

        let session = store.session(&params, &permission)?;

        let key = self.store_key()?;

        let value = self.store_value()?;

        let store_key = key.to_bytes()?;

        let store_value = value.to_bytes()?;

        store.create(&session, params, &store_key, &store_value)
    }
    
    /// Updates the item in the store. The item should already exist in the store before the operation.
    fn store_update(&self,
                    store: &mut St,
                    params: &P,)
        -> Result<()>
    {
        params.check()?;

        let permission = Permission::Write;

        let session = store.session(&params, &permission)?;

        let key = self.store_key()?;

        let value = self.store_value()?;

        let store_key = key.to_bytes()?;

        let store_value = value.to_bytes()?;

        store.update(&session, params, &store_key, &store_value)
    }
    
    /// Creates the item in the store if absent, update it if present.
    fn store_upsert(&self,
                    store: &mut St,
                    params: &P)
        -> Result<()>
    {
        params.check()?;

        let permission = Permission::Write;

        let session = store.session(&params, &permission)?;

        let key = self.store_key()?;

        let value = self.store_value()?;

        let store_key = key.to_bytes()?;

        let store_value = value.to_bytes()?;
        
        store.upsert(&session, params, &store_key, &store_value)
    }
    
    /// Deletes the item from the store. The item should already exist in the store before the operation.
    fn store_delete(&self,
                    store: &mut St,
                    params: &P)
        -> Result<()>
    {
        params.check()?;

        let permission = Permission::Write;

        let session = store.session(&params, &permission)?;

        let key = self.store_key()?;

        let store_key = key.to_bytes()?; 
        
        store.delete(&session, params, &store_key)
    }

    /// Custom operation in the store.
    fn store_custom(store: &mut St,
                    session: &Session<S>,
                    params: &PC)
        -> Result<RC>
    {
        params.check()?;

        session.check()?;

        if session.is_expired()? {
            return Err(String::from("expired session"));
        }

        store.custom(session, params)
    }
}