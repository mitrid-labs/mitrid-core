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
    fn session_cb<P: Datable>(&mut self,
                              params: &P,
                              permission: &Permission,
                              cb: &Fn(&mut Self, &P, &Permission) -> Future<Session<S>>)
        -> Future<Session<S>>
    {
        match params.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        match permission.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        cb(self, params, permission)
    }
    
    fn count_cb<P: Datable>(&mut self,
                            params: &P,
                            session: &Session<S>,
                            from: &Option<K>,
                            to: &Option<K>,
                            cb: &Fn(&mut Self, &P, &Session<S>, &Option<K>, &Option<K>) -> Future<u64>)
        -> Future<u64>
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

        cb(self, params, session, from, to)
    }
    
    fn list_cb<P: Datable>(&mut self,
                           params: &P,
                           session: &Session<S>,
                           from: &Option<K>,
                           to: &Option<K>,
                           count: &Option<u64>,
                           cb: &Fn(&mut Self, &P, &Session<S>, &Option<K>, &Option<K>, &Option<u64>) -> Future<Vec<V>>)
        -> Future<Vec<V>>
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

        cb(self, params, session, from, to, count)
    }
    
    fn lookup_cb<P: Datable>(&mut self,
                             params: &P,
                             session: &Session<S>,
                             key: &K,
                             cb: &Fn(&mut Self, &P, &Session<S>, &K) -> Future<bool>)
        -> Future<bool>
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

        cb(self, params, session, key)
    }
    
    fn get_cb<P: Datable>(&mut self,
                          params: &P,
                          session: &Session<S>,
                          key: &K,
                          cb: &Fn(&mut Self, &P, &Session<S>, &K) -> Future<V>)
        -> Future<V>
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

        cb(self, params, session, key)
    }
    
    fn create_cb<P: Datable>(&mut self,
                             params: &P,
                             session: &Session<S>,
                             key: &K,
                             value: &V,
                             cb: &Fn(&mut Self, &P, &Session<S>, &K, &V) -> Future<()>)
        -> Future<()>
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

        match key.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        match value.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        cb(self, params, session, key, value)
    }
    
    fn update_cb<P: Datable>(&mut self,
                             params: &P,
                             session: &Session<S>,
                             key: &K,
                             value: &V,
                             cb: &Fn(&mut Self, &P, &Session<S>, &K, &V) -> Future<()>)
        -> Future<()>
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

        match key.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        match value.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        cb(self, params, session, key, value)
    }
    
    fn upsert_cb<P: Datable>(&mut self,
                             params: &P,
                             session: &Session<S>,
                             key: &K,
                             value: &V,
                             cb: &Fn(&mut Self, &P, &Session<S>, &K, &V) -> Future<()>)
        -> Future<()>
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

        match key.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        match value.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        cb(self, params, session, key, value)
    }
    
    fn delete_cb<P: Datable>(&mut self,
                             params: &P,
                             session: &Session<S>,
                             key: &K,
                             cb: &Fn(&mut Self, &P, &Session<S>, &K) -> Future<()>)
        -> Future<()>
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

        match key.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        cb(self, params, session, key)
    }
    
    fn custom_cb<P: Datable, R: Datable>(&mut self,
                                         params: &P,
                                         session: &Session<S>,
                                         cb: &Fn(&mut Self, &P, &Session<S>) -> Future<R>)
        -> Future<R>
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

        cb(self, params, session)
    }
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
                              permission: &Permission,
                              cb: &Fn(&mut St, &Par, &Permission) -> Future<Session<S>>)
        -> Future<Session<S>>
        where   Par: Datable,
                St: Store<S, K, V>
    {
        store.session_cb(params, permission, cb)
    }
    
    fn store_count<Par, St>(store: &mut St,
                            params: &Par,
                            session: &Session<S>,
                            from: &Option<K>,
                            to: &Option<K>,
                            cb: &Fn(&mut St, &Par, &Session<S>, &Option<K>, &Option<K>) -> Future<u64>)
        -> Future<u64>
        where   Par: Datable,
                St: Store<S, K, V>
    {
        store.count_cb(params, session, from, to, cb)
    }
    
    fn store_list<Par, St>(store: &mut St,
                           params: &Par,
                           session: &Session<S>,
                           from: &Option<K>,
                           to: &Option<K>,
                           count: &Option<u64>,
                           cb: &Fn(&mut St, &Par, &Session<S>, &Option<K>, &Option<K>, &Option<u64>) -> Future<Vec<V>>)
        -> Future<Vec<V>>
        where   Par: Datable,
                St: Store<S, K, V>
    {
        store.list_cb(params, session, from, to, count, cb)
    }
    
    fn store_lookup<Par, St>(store: &mut St,
                             params: &Par,
                             session: &Session<S>,
                             key: &K,
                             cb: &Fn(&mut St, &Par, &Session<S>, &K) -> Future<bool>)
        -> Future<bool>
        where   Par: Datable,
                St: Store<S, K, V>
    {
        store.lookup_cb(params, session, key, cb)
    }
    
    fn store_get<Par, St>(store: &mut St,
                          params: &Par,
                          session: &Session<S>,
                          key: &K,
                          cb: &Fn(&mut St, &Par, &Session<S>, &K) -> Future<V>)
        -> Future<V>
        where   Par: Datable,
                S: Datable,
                St: Store<S, K, V>
    {
        store.get_cb(params, session, key, cb)
    }
    
    fn store_create<Par, St>(&self,
                             store: &mut St,
                             params: &Par,
                             session: &Session<S>,
                             cb: &Fn(&mut St, &Par, &Session<S>, &K, &V) -> Future<()>)
        -> Future<()>
        where   Par: Datable,
                St: Store<S, K, V>
    {
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

        store.create_cb(params, session, &key, &value, cb)
    }
    
    fn store_update<Par, St>(&self,
                             store: &mut St,
                             params: &Par,
                             session: &Session<S>,
                             cb: &Fn(&mut St, &Par, &Session<S>, &K, &V) -> Future<()>)
        -> Future<()>
        where   Par: Datable,
                St: Store<S, K, V>
    {
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

        store.update_cb(params, session, &key, &value, cb)
    }
    
    fn store_upsert<Par, St>(&self,
                             store: &mut St,
                             params: &Par,
                             session: &Session<S>,
                             cb: &Fn(&mut St, &Par, &Session<S>, &K, &V) -> Future<()>)
        -> Future<()>
        where   Par: Datable,
                St: Store<S, K, V>
    {
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
        
        store.upsert_cb(params, session, &key, &value, cb)
    }
    
    fn store_delete<Par, St>(&self,
                             store: &mut St,
                             params: &Par,
                             session: &Session<S>,
                             cb: &Fn(&mut St, &Par, &Session<S>, &K) -> Future<()>)
        -> Future<()>
        where   Par: Datable,
                St: Store<S, K, V>
    {
        let key_res = self.store_key();

        match key_res {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)); }
        }

        let key = key_res.unwrap();
        
        store.delete_cb(params, session, &key, cb)
    }

    fn store_custom<Par, R, St>(store: &mut St,
                                params: &Par,
                                session: &Session<S>,
                                cb: &Fn(&mut St, &Par, &Session<S>) -> Future<R>)
        -> Future<R>
        where   Par: Datable,
                R: Datable,
                St: Store<S, K, V>
    {
        store.custom_cb(params, session, cb)
    }
}