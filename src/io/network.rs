use base::Result;
use base::Future;
use base::VariableSize;
use base::Checkable;
use base::Datable;
use base::Serializable;
use io::Permission;
use io::Session;
use io::Node;

pub trait Network<S, A, NP, K, V>
    where   S: Datable + Serializable,
            A: Datable + VariableSize + Serializable,
            NP: Datable + Serializable,
            K: Datable + Serializable,
            V: Datable + Serializable
{
    fn session_cb<P: Datable>(&mut self,
                              params: &P,
                              address: &A,
                              nodes: &Vec<Node<A, NP>>,
                              permission: &Permission,
                              cb: &Fn(&mut Self, &P, &A, &Vec<Node<A, NP>>, &Permission) -> Future<Session<S>>)
        -> Future<Session<S>> {
            match params.check() {
                Ok(_) => {},
                Err(e) => { return Future::from_result(Err(e)) },
            }

            match address.check() {
                Ok(_) => {},
                Err(e) => { return Future::from_result(Err(e)) },
            }

            match nodes.check() {
                Ok(_) => {},
                Err(e) => { return Future::from_result(Err(e)) },
            }

            match permission.check() {
                Ok(_) => {},
                Err(e) => { return Future::from_result(Err(e)) },
            }

            cb(self, params, address, nodes, permission)
    }
    
    fn count_cb<P: Datable>(&mut self,
                            params: &P,
                            session: &Session<S>,
                            address: &A,
                            nodes: &Vec<Node<A, NP>>,
                            from: &Option<K>,
                            to: &Option<K>,
                            cb: &Fn(&mut Self, &P, &Session<S>, &A, &Vec<Node<A, NP>>, &Option<K>, &Option<K>) -> Future<u64>)
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

        match address.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        match nodes.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        match from.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        match to.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        cb(self, params, session, address, nodes, from, to)
    }
    
    fn list_cb<P: Datable>(&mut self,
                           params: &P,
                           session: &Session<S>,
                           address: &A,
                           nodes: &Vec<Node<A, NP>>,
                           from: &Option<K>,
                           to: &Option<K>,
                           count: &Option<u64>,
                           cb: &Fn(&mut Self, &P, &Session<S>, &A, &Vec<Node<A, NP>>, &Option<K>, &Option<K>, &Option<u64>) -> Future<Vec<V>>)
        -> Future<Vec<V>>
    {
        match params.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        match address.check() {
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

        match nodes.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        match from.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        match to.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        cb(self, params, session, address, nodes, from, to, count)
    }
    
    fn lookup_cb<P: Datable>(&mut self,
                             params: &P,
                             session: &Session<S>,
                             address: &A,
                             nodes: &Vec<Node<A, NP>>,
                             key: &K,
                             cb: &Fn(&mut Self, &P, &Session<S>, &A, &Vec<Node<A, NP>>, &K) -> Future<bool>)
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

        match address.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        match nodes.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        match key.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        cb(self, params, session, address, nodes, key)
    }
    
    fn get_cb<P: Datable>(&mut self,
                          params: &P,
                          session: &Session<S>,
                          address: &A,
                          nodes: &Vec<Node<A, NP>>,
                          key: &K,
                          cb: &Fn(&mut Self, &P, &Session<S>, &A, &Vec<Node<A, NP>>, &K) -> Future<V>)
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

        match address.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        match nodes.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        match key.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        cb(self, params, session, address, nodes, key)
    }
    
    fn create_cb<P: Datable>(&mut self,
                             params: &P,
                             session: &Session<S>,
                             address: &A,
                             nodes: &Vec<Node<A, NP>>,
                             key: &K,
                             value: &V,
                             cb: &Fn(&mut Self, &P, &Session<S>, &A, &Vec<Node<A, NP>>, &K, &V) -> Future<()>)
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

        match address.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        match nodes.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        match key.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        match value.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        cb(self, params, session, address, nodes, key, value)
    }
    
    fn update_cb<P: Datable>(&mut self,
                             params: &P,
                             session: &Session<S>,
                             address: &A,
                             nodes: &Vec<Node<A, NP>>,
                             key: &K,
                             value: &V,
                             cb: &Fn(&mut Self, &P, &Session<S>, &A, &Vec<Node<A, NP>>, &K, &V) -> Future<()>)
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

        match address.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        match nodes.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        match key.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        match value.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        cb(self, params, session, address, nodes, key, value)
    }
    
    fn upsert_cb<P: Datable>(&mut self,
                             params: &P,
                             session: &Session<S>,
                             address: &A,
                             nodes: &Vec<Node<A, NP>>,
                             key: &K,
                             value: &V,
                             cb: &Fn(&mut Self, &P, &Session<S>, &A, &Vec<Node<A, NP>>, &K, &V) -> Future<()>)
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

        match address.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        match nodes.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        match key.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        match value.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        cb(self, params, session, address, nodes, key, value)
    }
    
    fn delete_cb<P: Datable>(&mut self,
                             params: &P,
                             session: &Session<S>,
                             address: &A,
                             nodes: &Vec<Node<A, NP>>,
                             key: &K,
                             cb: &Fn(&mut Self, &P, &Session<S>, &A, &Vec<Node<A, NP>>, &K) -> Future<()>)
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

        match address.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        match nodes.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        match key.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        cb(self, params, session, address, nodes, key)
    }
    
    fn custom_cb<P: Datable, R: Datable>(&mut self,
                                         params: &P,
                                         session: &Session<S>,
                                         address: &A,
                                         nodes: &Vec<Node<A, NP>>,
                                         cb: &Fn(&mut Self, &P, &Session<S>, &A, &Vec<Node<A, NP>>) -> Future<R>)
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

        match address.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        match nodes.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        cb(self, params, session, address, nodes)
    }
}

pub trait Networkable<S, A, NP, K, V>
    where   S: Datable + Serializable,
            A: Datable + VariableSize + Serializable,
            NP: Datable + Serializable,
            K: Datable + Serializable,
            V: Datable + Serializable,
            Self: Datable
{
    fn network_key(&self) -> Result<K>;

    fn network_value(&self) -> Result<V>;

    fn network_session<Par, Net>(network: &mut Net,
                                 params: &Par,
                                 address: &A,
                                 nodes: &Vec<Node<A, NP>>,
                                 permission: &Permission,
                                 cb: &Fn(&mut Net, &Par, &A, &Vec<Node<A, NP>>, &Permission) -> Future<Session<S>>)
        -> Future<Session<S>>
        where   Par: Datable,
                Net: Network<S, A, NP, K, V>
    {
        network.session_cb(params, address, nodes, permission, cb)
    }
    
    fn network_count<Par, Net>(network: &mut Net,
                               params: &Par,
                               session: &Session<S>,
                               address: &A,
                               nodes: &Vec<Node<A, NP>>,
                               from: &Option<K>,
                               to: &Option<K>,
                               cb: &Fn(&mut Net, &Par, &Session<S>, &A, &Vec<Node<A, NP>>, &Option<K>, &Option<K>) -> Future<u64>)
        -> Future<u64>
        where   Par: Datable,
                Net: Network<S, A, NP, K, V>
    {
        network.count_cb(params, session, address, nodes, from, to, cb)
    }
    
    fn network_list<Par, Net>(network: &mut Net,
                              params: &Par,
                              session: &Session<S>,
                              address: &A,
                              nodes: &Vec<Node<A, NP>>,
                              from: &Option<K>,
                              to: &Option<K>,
                              count: &Option<u64>,
                              cb: &Fn(&mut Net, &Par, &Session<S>, &A, &Vec<Node<A, NP>>, &Option<K>, &Option<K>, &Option<u64>) -> Future<Vec<V>>)
        -> Future<Vec<V>>
        where   Par: Datable,
                Net: Network<S, A, NP, K, V>
    {
        network.list_cb(params, session, address, nodes, from, to, count, cb)
    }
    
    fn network_lookup<Par, Net>(network: &mut Net,
                                params: &Par,
                                session: &Session<S>,
                                address: &A,
                                nodes: &Vec<Node<A, NP>>,
                                key: &K,
                                cb: &Fn(&mut Net, &Par, &Session<S>, &A, &Vec<Node<A, NP>>, &K) -> Future<bool>)
        -> Future<bool>
        where   Par: Datable,
                Net: Network<S, A, NP, K, V>
    {
        network.lookup_cb(params, session, address, nodes, key, cb)
    }
    
    fn network_get<Par, Net>(network: &mut Net,
                             params: &Par,
                             session: &Session<S>,
                             address: &A,
                             nodes: &Vec<Node<A, NP>>,
                             key: &K,
                             cb: &Fn(&mut Net, &Par, &Session<S>, &A, &Vec<Node<A, NP>>, &K) -> Future<V>)
        -> Future<V>
        where   Par: Datable,
                S: Datable,
                Net: Network<S, A, NP, K, V>
    {
        network.get_cb(params, session, address, nodes, key, cb)
    }
    
    fn network_create<Par, Net>(&self,
                                network: &mut Net,
                                params: &Par,
                                session: &Session<S>,
                                address: &A,
                                nodes: &Vec<Node<A, NP>>,
                                cb: &Fn(&mut Net, &Par, &Session<S>, &A, &Vec<Node<A, NP>>, &K, &V) -> Future<()>)
        -> Future<()>
        where   Par: Datable,
                Net: Network<S, A, NP, K, V>
    {
        let key_res = self.network_key();

        match key_res {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)); }
        }

        let key = key_res.unwrap();

        let value_res = self.network_value();

        match value_res {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)); }
        }

        let value = value_res.unwrap();

        network.create_cb(params, session, address, nodes, &key, &value, cb)
    }
    
    fn network_update<Par, Net>(&self,
                                network: &mut Net,
                                params: &Par,
                                session: &Session<S>,
                                address: &A,
                                nodes: &Vec<Node<A, NP>>,
                                cb: &Fn(&mut Net, &Par, &Session<S>, &A, &Vec<Node<A, NP>>, &K, &V) -> Future<()>)
        -> Future<()>
        where   Par: Datable,
                Net: Network<S, A, NP, K, V>
    {
        let key_res = self.network_key();

        match key_res {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)); }
        }

        let key = key_res.unwrap();

        let value_res = self.network_value();

        match value_res {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)); }
        }

        let value = value_res.unwrap();

        network.update_cb(params, session, address, nodes, &key, &value, cb)
    }
    
    fn network_upsert<Par, Net>(&self,
                                network: &mut Net,
                                params: &Par,
                                session: &Session<S>,
                                address: &A,
                                nodes: &Vec<Node<A, NP>>,
                                cb: &Fn(&mut Net, &Par, &Session<S>, &A, &Vec<Node<A, NP>>, &K, &V) -> Future<()>)
        -> Future<()>
        where   Par: Datable,
                Net: Network<S, A, NP, K, V>
    {
        let key_res = self.network_key();

        match key_res {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)); }
        }

        let key = key_res.unwrap();

        let value_res = self.network_value();

        match value_res {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)); }
        }

        let value = value_res.unwrap();
        
        network.upsert_cb(params, session, address, nodes, &key, &value, cb)
    }
    
    fn network_delete<Par, Net>(&self,
                                network: &mut Net,
                                params: &Par,
                                session: &Session<S>,
                                address: &A,
                                nodes: &Vec<Node<A, NP>>,
                                cb: &Fn(&mut Net, &Par, &Session<S>, &A, &Vec<Node<A, NP>>, &K) -> Future<()>)
        -> Future<()>
        where   Par: Datable,
                Net: Network<S, A, NP, K, V>
    {
        let key_res = self.network_key();

        match key_res {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)); }
        }

        let key = key_res.unwrap();
        
        network.delete_cb(params, session, address, nodes, &key, cb)
    }

    fn network_custom<Par, R, Net>(network: &mut Net,
                                   params: &Par,
                                   session: &Session<S>,
                                   address: &A,
                                   nodes: &Vec<Node<A, NP>>,
                                   cb: &Fn(&mut Net, &Par, &Session<S>, &A, &Vec<Node<A, NP>>) -> Future<R>)
        -> Future<R>
        where   Par: Datable,
                R: Datable,
                Net: Network<S, A, NP, K, V>
    {
        network.custom_cb(params, session, address, nodes, cb)
    }
}