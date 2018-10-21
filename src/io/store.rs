use base::Result;
use base::Future;
use base::Checkable;
use base::Datable;
use base::Serializable;
use io::Permission;
use io::Session;

pub trait Store<S, K, V>
    where   S: Datable + Serializable,
            K: Datable + Serializable,
            V: Datable + Serializable
{
    fn session<P: Datable>(&mut self, params: &P, permission: &Permission)
        -> Future<Session<S>>;
    
    fn count<P: Datable>(&mut self,
                            params: &P,
                            session: &Session<S>,
                            from: &Option<K>,
                            to: &Option<K>) -> Future<u64>;
    
    fn list<P: Datable>(&mut self,
                        params: &P,
                        session: &Session<S>,
                        from: &Option<K>,
                        to: &Option<K>,
                        count: &Option<u64>)
        -> Future<Vec<V>>;
    
    fn lookup<P: Datable>(&mut self,
                          params: &P,
                          session: &Session<S>,
                          key: &K)
        -> Future<bool>;
    
    fn get<P: Datable>(&mut self,
                       params: &P,
                       session: &Session<S>,
                       key: &K)
        -> Future<V>;
    
    fn create<P: Datable>(&mut self,
                          params: &P,
                          session: &Session<S>,
                          key: &K,
                          value: &V)
        -> Future<()>;
    
    fn update<P: Datable>(&mut self,
                          params: &P,
                          session: &Session<S>,
                          key: &K,
                          value: &V)
        -> Future<()>;
    
    fn upsert<P: Datable>(&mut self,
                          params: &P,
                          session: &Session<S>,
                          key: &K,
                          value: &V)
        -> Future<()>;
    
    fn delete<P: Datable>(&mut self,
                          params: &P,
                          session: &Session<S>,
                          key: &K)
        -> Future<()>;
    
    fn custom<P: Datable, R: Datable>(&mut self,
                                      params: &P,
                                      session: &Session<S>)
        -> Future<R>;
}

pub trait Storable<S, K, V>
    where   S: Datable + Serializable,
            K: Datable + Serializable,
            V: Datable + Serializable,
            Self: Datable
{
    fn store_key(&self) -> Result<K>;

    fn store_value(&self) -> Result<V>;

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

        match from.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        match to.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        store.count(params, session, from, to)
    }
    
    fn store_list<Par, St>(store: &mut St,
                           params: &Par,
                           session: &Session<S>,
                           from: &Option<K>,
                           to: &Option<K>,
                           count: &Option<u64>)
        -> Future<Vec<V>>
        where   Par: Datable,
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

        match from.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        match to.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        store.list(params, session, from, to, count)
    }
    
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

        store.lookup(params, session, key)
    }
    
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

        if session.permission < Permission::Write {
            return Future::from_result(Err(String::from("invalid permission")));
        }

        let key_res = self.store_key();

        match key_res {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)); }
        }

        let key = key_res.unwrap();

        let value_res = self.store_value();

        match value_res {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)); }
        }

        let value = value_res.unwrap();

        store.create(params, session, &key, &value)
    }
    
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

        if session.permission < Permission::Write {
            return Future::from_result(Err(String::from("invalid permission")));
        }

        let key_res = self.store_key();

        match key_res {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)); }
        }

        let key = key_res.unwrap();

        let value_res = self.store_value();

        match value_res {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)); }
        }

        let value = value_res.unwrap();

        store.update(params, session, &key, &value)
    }
    
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

        if session.permission < Permission::Write {
            return Future::from_result(Err(String::from("invalid permission")));
        }

        let key_res = self.store_key();

        match key_res {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)); }
        }

        let key = key_res.unwrap();

        let value_res = self.store_value();

        match value_res {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)); }
        }

        let value = value_res.unwrap();
        
        store.upsert(params, session, &key, &value)
    }
    
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

        if session.permission < Permission::Write {
            return Future::from_result(Err(String::from("invalid permission")));
        }

        let key_res = self.store_key();

        match key_res {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)); }
        }

        let key = key_res.unwrap();
        
        store.delete(params, session, &key)
    }

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