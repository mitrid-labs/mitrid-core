//! # Store
//!
//! `store` is the module providing the traits implemented by stores and by types that
//! can be stored in and retrieved from a store.

use std::mem;

use base::Result;
use base::Checkable;
use base::Datable;
use base::Serializable;
use base::{Eval, EvalMut};
use io::Permission;
use io::Session;

/// Trait representing the operations implemented by a store.
pub trait Store<S>
    where   S: Datable + Serializable,
            Self: 'static + Sized + Send + Sync + Checkable
{
    /// Retrieves a new `Session` from the store.
    fn session(&mut self, permission: &Permission) -> Result<Session<S>>;
    
    /// Counts the store items starting from the `from` key until, not included, the `to` key.
    fn count(&mut self, session: &Session<S>, from: Option<Vec<u8>>, to: Option<Vec<u8>>) -> Result<u64>;

    /// Counts the store items starting with the given prefix.
    fn count_prefix(&mut self,
                    session: &Session<S>,
                    prefix: &[u8])
        -> Result<u64>;
    
    /// Lists the store items starting from the `from` key until, not included, the `to` key.
    fn list(&mut self,
            session: &Session<S>,
            from: Option<Vec<u8>>,
            to: Option<Vec<u8>>,
            count: Option<u64>)
        -> Result<Vec<Vec<u8>>>;

    /// Lists the store items starting with the given prefix.
    fn list_prefix(&mut self,
                   session: &Session<S>,
                   prefix: &[u8],
                   count: Option<u64>)
        -> Result<Vec<Vec<u8>>>;
    
    /// Lookups an item from its key.
    fn lookup(&mut self, session: &Session<S>, key: &[u8]) -> Result<bool>;
    
    /// Retrieves an item from its key. The item should already exist in the store before the operation.
    fn get(&mut self, session: &Session<S>, key: &[u8]) -> Result<Vec<u8>>;
    
    /// Creates an item in the store. The item should not exist in the store before the operation.
    fn create(&mut self, session: &Session<S>, key: &[u8], value: &[u8]) -> Result<()>;
    
    /// Updates an item in the store. The item should already exist in the store before the operation.
    fn update(&mut self, session: &Session<S>, key: &[u8], value: &[u8]) -> Result<()>;
    
    /// Creates an item in the store if absent, update it if present.
    fn upsert(&mut self, session: &Session<S>, key: &[u8], value: &[u8]) -> Result<()>;
    
    /// Deletes an item from the store. The item should already exist in the store before the operation.
    fn delete(&mut self, session: &Session<S>, key: &[u8]) -> Result<()>;
    
    /// Eval operation in the store.
    fn eval<E, P, R>(&mut self, session: &Session<S>, params: &P, evaluator: &E) -> Result<R>
        where   E: Eval<Self, P, R>,
                P: Datable,
                R: Datable
    {
        session.check()?;
        params.check()?;

        if session.is_expired()? {
            return Err(String::from("expired session"));
        }

        if session.permission > Permission::Read {
            return Err(String::from("invalid permission")).into();
        }

        evaluator.eval(self, params)
    }
    
    /// Evals mutably in the store.
    fn eval_mut<E, P, R>(&mut self, session: &Session<S>, params: &P, evaluator: &mut E) -> Result<R>
        where   E: EvalMut<Self, P, R>,
                P: Datable,
                R: Datable
    {
        session.check()?;
        params.check()?;

        if session.is_expired()? {
            return Err(String::from("expired session"));
        }

        if session.permission < Permission::Write {
            return Err(String::from("invalid permission")).into();
        }

        evaluator.eval_mut(self, params)
    }
}

/// Trait implemented by types that can be stored and retrieved from a store.
pub trait Storable<St, S, K, V>
    where   St: Store<S>,
            S: Datable + Serializable,
            K: Ord + Datable + Serializable,
            V: Datable + Serializable,
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
    fn store_count(store: &mut St, from: Option<K>, to: Option<K>) -> Result<u64> {
        let permission = Permission::Read;

        let session = store.session(&permission)?;

        from.check()?;
        to.check()?;

        let mut prefix = Vec::new();
        let _prefix: [u8; 8] = unsafe { mem::transmute(Self::store_prefix()) };
        prefix.extend_from_slice(&_prefix[..]);

        if from.is_none() && to.is_none() {
            return store.count_prefix(&session, &prefix);
        }

        if let Some(ref from) = from {
            if let Some(ref to) = to {
                if from >= to {
                    return Err(String::from("invalid range"));
                } 
            }
        }

        let store_from = if let Some(k) = from {
            let mut from_key = Vec::new();
            from_key.extend_from_slice(&prefix);
            from_key.extend(&k.to_bytes()?);

            Some(from_key)
        } else {
            None
        };

        let store_to = if let Some(k) = to {
            let mut to_key = Vec::new();
            to_key.extend_from_slice(&prefix);
            to_key.extend(&k.to_bytes()?);

            Some(to_key)
        } else {
            None
        };

        store.count(&session, store_from, store_to)
    }
    
    /// Lists the store items starting from the `from` key until, not included, the `to` key.
    fn store_list(store: &mut St,
                  from: Option<K>,
                  to: Option<K>,
                  count: Option<u64>)
        -> Result<Vec<Self>>
    {
        let permission = Permission::Read;

        let session = store.session(&permission)?;

        from.check()?;
        to.check()?;

        if let Some(count) = count {
            if count == 0 {
                return Err(String::from("invalid count"));
            }
        }

        let mut list = Vec::new();

        let mut prefix = Vec::new();
        let _prefix: [u8; 8] = unsafe { mem::transmute(Self::store_prefix()) };
        prefix.extend_from_slice(&_prefix[..]);

        if from.is_none() && to.is_none() {
            for value in store.list_prefix(&session, &prefix, count)?.iter() {
                list.push(Self::from_store_value(&value)?);
            }

            return Ok(list)
        }

        if let Some(from) = from.clone() {
            if let Some(to) = to.clone() {
                if from >= to {
                    return Err(String::from("invalid range"));
                } 
            }
        }

        let store_from = if let Some(k) = from {
            let mut from_key = Vec::new();
            from_key.extend_from_slice(&prefix);
            from_key.extend(&k.to_bytes()?);

            Some(from_key)
        } else {
            None
        };

        let store_to = if let Some(k) = to {
            let mut to_key = Vec::new();
            to_key.extend_from_slice(&prefix);
            to_key.extend(&k.to_bytes()?);

            Some(to_key)
        } else {
            None
        };

        for value in store.list(&session, store_from, store_to, count)?.iter() {
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

    /// Eval operation in the store.
    fn store_eval<E, P, R>(store: &mut St, session: &Session<S>, params: &P, evaluator: &E)
        -> Result<R>
        where   E: Eval<St, P, R>,
                P: Datable,
                R: Datable
    {
        params.check()?;
        session.check()?;

        if session.is_expired()? {
            return Err(String::from("expired session"));
        }

        if session.permission > Permission::Read {
            return Err(String::from("invalid permission")).into();
        }

        store.eval(session, params, evaluator)
    }

    /// Evals mutably in the store.
    fn store_eval_mut<E, P, R>(store: &mut St, session: &Session<S>, params: &P, evaluator: &mut E)
        -> Result<R>
        where   E: EvalMut<St, P, R>,
                P: Datable,
                R: Datable
    {
        params.check()?;
        session.check()?;

        if session.is_expired()? {
            return Err(String::from("expired session"));
        }

        if session.permission < Permission::Write {
            return Err(String::from("invalid permission")).into();
        }

        store.eval_mut(session, params, evaluator)
    }
}