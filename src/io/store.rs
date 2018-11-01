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
pub trait Store<S, K, V, P, PC, RC>
    where   S: Datable + Serializable,
            K: Ord + Datable + Serializable,
            V: Datable + Serializable ,
            P: Datable,
            PC: Datable + Serializable,
            RC: Datable + Serializable,
            Self: 'static + Send + Sync + Checkable
{
    /// Retrieves a new `Session` from the store.
    fn session(&mut self, params: &P, permission: &Permission)
        -> Result<Session<S>>;
    
    /// Counts the store items starting from the `from` key until, not included, the `to` key.
    fn count(&mut self,
             session: &Session<S>,
             params: &P,
             from: &Option<K>,
             to: &Option<K>)
        -> Result<u64>;
    
    /// Lists the store items starting from the `from` key until, not included, the `to` key.
    fn list(&mut self,
            session: &Session<S>,
            params: &P,
            from: &Option<K>,
            to: &Option<K>,
            count: &Option<u64>)
        -> Result<Vec<V>>;
    
    /// Lookups an item from its key.
    fn lookup(&mut self,
              session: &Session<S>,
              params: &P,
              key: &K)
        -> Result<bool>;
    
    /// Retrieves an item from its key. The item should already exist in the store before the operation.
    fn get(&mut self,
           session: &Session<S>,
           params: &P,
           key: &K)
        -> Result<V>;
    
    /// Creates an item in the store. The item should not exist in the store before the operation.
    fn create(&mut self,
              session: &Session<S>,
              params: &P,
              key: &K,
              value: &V)
        -> Result<()>;
    
    /// Updates an item in the store. The item should already exist in the store before the operation.
    fn update(&mut self,
              session: &Session<S>,
              params: &P,
              key: &K,
              value: &V)
        -> Result<()>;
    
    /// Creates an item in the store if absent, update it if present.
    fn upsert(&mut self,
              session: &Session<S>,
              params: &P,
              key: &K,
              value: &V)
        -> Result<()>;
    
    /// Deletes an item from the store. The item should already exist in the store before the operation.
    fn delete(&mut self,
              session: &Session<S>,
              params: &P,
              key: &K)
        -> Result<()>;
    
    /// Custom operation in the store.
    fn custom(&mut self,
              session: &Session<S>,
              params: &PC)
        -> Result<RC>;
}

/// Trait implemented by types that can be stored and retrieved from a store.
pub trait Storable<St, S, K, V, P, PC, RC>
    where   St: Store<S, K, V, P, PC, RC>,
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
    fn from_store_value(value: &V) -> Result<Self> {
        let buf = value.to_bytes()?;
        Self::from_bytes(&buf)
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

        store.count(&session, params, from, to)
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

        let mut list = Vec::new();

        for value in store.list(&session, params, from, to, count)?.iter() {
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

        store.lookup(&session, params, key)
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

        let value = store.get(&session, params, key)?;
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

        store.create(&session, params, &key, &value)
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

        store.update(&session, params, &key, &value)
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
        
        store.upsert(&session, params, &key, &value)
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
        
        store.delete(&session, params, &key)
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