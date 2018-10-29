//! # Store
//!
//! `store` is the module providing the traits implemented by stores and by types that
//! can be stored in and retrieved from a store.

use base::Result;
use base::Future;
use base::Stream;
use base::Checkable;
use base::Datable;
use base::Serializable;
use io::Permission;
use io::Session;

/// Trait representing the operations implemented by a store.
pub trait Store<S, K, V>
    where   S: Datable + Serializable,
            K: Datable + Serializable,
            V: Datable + Serializable,
            Self: 'static + Clone + Send + Sync
{
    /// Retrieves a new `Session` from the store.
    fn session<P: Datable>(&mut self, params: &P, permission: &Permission)
        -> Future<Session<S>>;
    
    /// Counts the store items starting from the `from` key until, not included, the `to` key.
    fn count<P: Datable>(&mut self,
                         params: &P,
                         session: &Session<S>,
                         from: &Option<K>,
                         to: &Option<K>) -> Future<u64>;
    
    /// Lists the store items starting from the `from` key until, not included, the `to` key.
    fn list<P: Datable>(&mut self,
                        params: &P,
                        session: &Session<S>,
                        from: &Option<K>,
                        to: &Option<K>,
                        count: &Option<u64>)
        -> Stream<Vec<V>>;
    
    /// Lookups an item from its key.
    fn lookup<P: Datable>(&mut self,
                          params: &P,
                          session: &Session<S>,
                          key: &K)
        -> Future<bool>;
    
    /// Retrieves an item from its key. The item should already exist in the store before the operation.
    fn get<P: Datable>(&mut self,
                       params: &P,
                       session: &Session<S>,
                       key: &K)
        -> Future<V>;
    
    /// Creates an item in the store. The item should not exist in the store before the operation.
    fn create<P: Datable>(&mut self,
                          params: &P,
                          session: &Session<S>,
                          key: &K,
                          value: &V)
        -> Future<()>;
    
    /// Updates an item in the store. The item should already exist in the store before the operation.
    fn update<P: Datable>(&mut self,
                          params: &P,
                          session: &Session<S>,
                          key: &K,
                          value: &V)
        -> Future<()>;
    
    /// Creates an item in the store if absent, update it if present.
    fn upsert<P: Datable>(&mut self,
                          params: &P,
                          session: &Session<S>,
                          key: &K,
                          value: &V)
        -> Future<()>;
    
    /// Deletes an item from the store. The item should already exist in the store before the operation.
    fn delete<P: Datable>(&mut self,
                          params: &P,
                          session: &Session<S>,
                          key: &K)
        -> Future<()>;
    
    /// Custom operation in the store.
    fn custom<P: Datable, R: Datable>(&mut self,
                                      params: &P,
                                      session: &Session<S>)
        -> Future<R>;
}

/// Trait implemented by types that can be stored and retrieved from a store.
pub trait Storable<S, K, V>
    where   S: Datable + Serializable,
            K: Datable + Serializable,
            V: Datable + Serializable,
            Self: Datable
{
    /// Returns the store key of the item.
    fn store_key(&self) -> Result<K>;

    /// Retrieves the store value of the item.
    fn store_value(&self) -> Result<V>;

    /// Retrieves a new session from the store.
    fn store_session<Par, St>(store: &mut St,
                              params: &Par,
                              permission: &Permission)
        -> Future<Session<S>>
        where   Par: Datable,
                St: Store<S, K, V>
    {
        match params.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        match permission.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        store.session(params, permission)
    }
    
    /// Counts the store items starting from the `from` key until, not included, the `to` key.
    fn store_count<Par, St>(store: &mut St,
                            params: &Par,
                            session: &Session<S>,
                            from: &Option<K>,
                            to: &Option<K>)
        -> Future<u64>
        where   Par: Datable,
                St: Store<S, K, V>
    {
        match params.check() {
            Ok(_) => {},
            Err(e) => { return Err(e).into() },
        }

        match session.check() {
            Ok(_) => {},
            Err(e) => { return Err(e).into() },
        }

        match session.is_expired() {
            Ok(expired) => {
                if expired {
                    return Err(String::from("expired session")).into();
                }
            },
            Err(e) => {
                return Err(e).into();
            }
        }

        if session.permission > Permission::Read {
            return Err(String::from("invalid permission")).into();
        }

        match from.check() {
            Ok(_) => {},
            Err(e) => { return Err(e).into() },
        }

        match to.check() {
            Ok(_) => {},
            Err(e) => { return Err(e).into() },
        }

        store.count(params, session, from, to)
    }
    
    /// Lists the store items starting from the `from` key until, not included, the `to` key.
    fn store_list<Par, St>(store: &mut St,
                           params: &Par,
                           session: &Session<S>,
                           from: &Option<K>,
                           to: &Option<K>,
                           count: &Option<u64>)
        -> Stream<Vec<V>>
        where   Par: Datable,
                St: Store<S, K, V>
    {
        match params.check() {
            Ok(_) => {},
            Err(e) => { return Stream::from_result(Err(e)) },
        }

        match session.check() {
            Ok(_) => {},
            Err(e) => { return Stream::from_result(Err(e)) },
        }

        match session.is_expired() {
            Ok(expired) => {
                if expired {
                    return Stream::from_result(Err(String::from("expired session")));
                }
            },
            Err(e) => {
                return Stream::from_result(Err(e));
            }
        }

        if session.permission > Permission::Read {
            return Stream::from_result(Err(String::from("invalid permission")));
        }

        match from.check() {
            Ok(_) => {},
            Err(e) => { return Stream::from_result(Err(e)) },
        }

        match to.check() {
            Ok(_) => {},
            Err(e) => { return Stream::from_result(Err(e)) },
        }

        store.list(params, session, from, to, count)
    }
    
    /// Lookups an item from its key.
    fn store_lookup<Par, St>(store: &mut St,
                             params: &Par,
                             session: &Session<S>,
                             key: &K)
        -> Future<bool>
        where   Par: Datable,
                St: Store<S, K, V>
    {
        match params.check() {
            Ok(_) => {},
            Err(e) => { return Err(e).into() },
        }

        match session.check() {
            Ok(_) => {},
            Err(e) => { return Err(e).into() },
        }

        match session.is_expired() {
            Ok(expired) => {
                if expired {
                    return Err(String::from("expired session")).into();
                }
            },
            Err(e) => {
                return Err(e).into();
            }
        }

        if session.permission > Permission::Read {
            return Err(String::from("invalid permission")).into();
        }

        match key.check() {
            Ok(_) => {},
            Err(e) => { return Err(e).into() },
        }

        store.lookup(params, session, key)
    }
    
    /// Retrieves an item from its key. The item should already exist in the store before the operation.
    fn store_get<Par, St>(store: &mut St,
                          params: &Par,
                          session: &Session<S>,
                          key: &K)
        -> Future<V>
        where   Par: Datable,
                S: Datable,
                St: Store<S, K, V>
    {
        match params.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        match session.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        match session.is_expired() {
            Ok(expired) => {
                if expired {
                    return Future::from_result(Err(String::from("expired session")));
                }
            },
            Err(e) => {
                return Future::from_result(Err(e));
            }
        }

        if session.permission > Permission::Read {
            return Future::from_result(Err(String::from("invalid permission")));
        }

        match key.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        store.get(params, session, key)
    }
    
    /// Creates an item in the store. The item should not exist in the store before the operation.
    fn store_create<Par, St>(&self,
                             store: &mut St,
                             params: &Par,
                             session: &Session<S>)
        -> Future<()>
        where   Par: Datable,
                St: Store<S, K, V>
    {
        match params.check() {
            Ok(_) => {},
            Err(e) => { return Err(e).into() },
        }

        match session.check() {
            Ok(_) => {},
            Err(e) => { return Err(e).into() },
        }

        match session.is_expired() {
            Ok(expired) => {
                if expired {
                    return Err(String::from("expired session")).into();
                }
            },
            Err(e) => {
                return Err(e).into();
            }
        }

        if session.permission < Permission::Write {
            return Err(String::from("invalid permission")).into();
        }

        let key_res = self.store_key();

        match key_res {
            Ok(_) => {},
            Err(e) => { return Err(e).into(); }
        }

        let key = key_res.unwrap();

        let value_res = self.store_value();

        match value_res {
            Ok(_) => {},
            Err(e) => { return Err(e).into(); }
        }

        let value = value_res.unwrap();

        store.create(params, session, &key, &value)
    }
    
    /// Updates the item in the store. The item should already exist in the store before the operation.
    fn store_update<Par, St>(&self,
                             store: &mut St,
                             params: &Par,
                             session: &Session<S>)
        -> Future<()>
        where   Par: Datable,
                St: Store<S, K, V>
    {
        match params.check() {
            Ok(_) => {},
            Err(e) => { return Err(e).into() },
        }

        match session.check() {
            Ok(_) => {},
            Err(e) => { return Err(e).into() },
        }

        match session.is_expired() {
            Ok(expired) => {
                if expired {
                    return Err(String::from("expired session")).into();
                }
            },
            Err(e) => {
                return Err(e).into();
            }
        }

        if session.permission < Permission::Write {
            return Err(String::from("invalid permission")).into();
        }

        let key_res = self.store_key();

        match key_res {
            Ok(_) => {},
            Err(e) => { return Err(e).into(); }
        }

        let key = key_res.unwrap();

        let value_res = self.store_value();

        match value_res {
            Ok(_) => {},
            Err(e) => { return Err(e).into(); }
        }

        let value = value_res.unwrap();

        store.update(params, session, &key, &value)
    }
    
    /// Creates the item in the store if absent, update it if present.
    fn store_upsert<Par, St>(&self,
                             store: &mut St,
                             params: &Par,
                             session: &Session<S>)
        -> Future<()>
        where   Par: Datable,
                St: Store<S, K, V>
    {
        match params.check() {
            Ok(_) => {},
            Err(e) => { return Err(e).into() },
        }

        match session.check() {
            Ok(_) => {},
            Err(e) => { return Err(e).into() },
        }

        match session.is_expired() {
            Ok(expired) => {
                if expired {
                    return Err(String::from("expired session")).into();
                }
            },
            Err(e) => {
                return Err(e).into();
            }
        }

        if session.permission < Permission::Write {
            return Err(String::from("invalid permission")).into();
        }

        let key_res = self.store_key();

        match key_res {
            Ok(_) => {},
            Err(e) => { return Err(e).into(); }
        }

        let key = key_res.unwrap();

        let value_res = self.store_value();

        match value_res {
            Ok(_) => {},
            Err(e) => { return Err(e).into(); }
        }

        let value = value_res.unwrap();
        
        store.upsert(params, session, &key, &value)
    }
    
    /// Deletes the item from the store. The item should already exist in the store before the operation.
    fn store_delete<Par, St>(&self,
                             store: &mut St,
                             params: &Par,
                             session: &Session<S>)
        -> Future<()>
        where   Par: Datable,
                St: Store<S, K, V>
    {
        match params.check() {
            Ok(_) => {},
            Err(e) => { return Err(e).into() },
        }

        match session.check() {
            Ok(_) => {},
            Err(e) => { return Err(e).into() },
        }

        match session.is_expired() {
            Ok(expired) => {
                if expired {
                    return Err(String::from("expired session")).into();
                }
            },
            Err(e) => {
                return Err(e).into();
            }
        }

        if session.permission < Permission::Write {
            return Err(String::from("invalid permission")).into();
        }

        let key_res = self.store_key();

        match key_res {
            Ok(_) => {},
            Err(e) => { return Err(e).into(); }
        }

        let key = key_res.unwrap();
        
        store.delete(params, session, &key)
    }

    /// Custom operation in the store.
    fn store_custom<Par, R, St>(store: &mut St,
                                params: &Par,
                                session: &Session<S>)
        -> Future<R>
        where   Par: Datable,
                R: Datable,
                St: Store<S, K, V>
    {
        match params.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        match session.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        match session.is_expired() {
            Ok(expired) => {
                if expired {
                    return Future::from_result(Err(String::from("expired session")));
                }
            },
            Err(e) => {
                return Future::from_result(Err(e));
            }
        }

        store.custom(params, session)
    }
}