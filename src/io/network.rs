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
    fn session<P: Datable>(&mut self,
                           params: &P,
                           address: &A,
                           nodes: &Vec<Node<A, NP>>,
                           permission: &Permission)
        -> Future<Session<S>>;
    
    fn count<P: Datable>(&mut self,
                         params: &P,
                         session: &Session<S>,
                         address: &A,
                         nodes: &Vec<Node<A, NP>>,
                         from: &Option<K>,
                         to: &Option<K>)
        -> Future<u64>;
    
    fn list<P: Datable>(&mut self,
                           params: &P,
                           session: &Session<S>,
                           address: &A,
                           nodes: &Vec<Node<A, NP>>,
                           from: &Option<K>,
                           to: &Option<K>,
                           count: &Option<u64>)
        -> Future<Vec<V>>;
    
    fn lookup<P: Datable>(&mut self,
                          params: &P,
                          session: &Session<S>,
                          address: &A,
                          nodes: &Vec<Node<A, NP>>,
                          key: &K)
        -> Future<bool>;
    
    fn get<P: Datable>(&mut self,
                       params: &P,
                       session: &Session<S>,
                       address: &A,
                       nodes: &Vec<Node<A, NP>>,
                       key: &K)
        -> Future<V>;
    
    fn create<P: Datable>(&mut self,
                             params: &P,
                             session: &Session<S>,
                             address: &A,
                             nodes: &Vec<Node<A, NP>>,
                             key: &K,
                             value: &V)
        -> Future<()>;
    
    fn update<P: Datable>(&mut self,
                             params: &P,
                             session: &Session<S>,
                             address: &A,
                             nodes: &Vec<Node<A, NP>>,
                             key: &K,
                             value: &V)
        -> Future<()>;
    
    fn upsert<P: Datable>(&mut self,
                             params: &P,
                             session: &Session<S>,
                             address: &A,
                             nodes: &Vec<Node<A, NP>>,
                             key: &K,
                             value: &V)
        -> Future<()>;
    
    fn delete<P: Datable>(&mut self,
                             params: &P,
                             session: &Session<S>,
                             address: &A,
                             nodes: &Vec<Node<A, NP>>,
                             key: &K)
        -> Future<()>;
    
    fn custom<P: Datable, R: Datable>(&mut self,
                                         params: &P,
                                         session: &Session<S>,
                                         address: &A,
                                         nodes: &Vec<Node<A, NP>>)
        -> Future<R>;
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
                                 permission: &Permission)
        -> Future<Session<S>>
        where   Par: Datable,
                Net: Network<S, A, NP, K, V>
    {
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

        network.session(params, address, nodes, permission)
    }
    
    fn network_count<Par, Net>(network: &mut Net,
                               params: &Par,
                               session: &Session<S>,
                               address: &A,
                               nodes: &Vec<Node<A, NP>>,
                               from: &Option<K>,
                               to: &Option<K>)
        -> Future<u64>
        where   Par: Datable,
                Net: Network<S, A, NP, K, V>
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

        network.count(params, session, address, nodes, from, to)
    }
    
    fn network_list<Par, Net>(network: &mut Net,
                              params: &Par,
                              session: &Session<S>,
                              address: &A,
                              nodes: &Vec<Node<A, NP>>,
                              from: &Option<K>,
                              to: &Option<K>,
                              count: &Option<u64>)
        -> Future<Vec<V>>
        where   Par: Datable,
                Net: Network<S, A, NP, K, V>
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

        network.list(params, session, address, nodes, from, to, count)
    }
    
    fn network_lookup<Par, Net>(network: &mut Net,
                                params: &Par,
                                session: &Session<S>,
                                address: &A,
                                nodes: &Vec<Node<A, NP>>,
                                key: &K)
        -> Future<bool>
        where   Par: Datable,
                Net: Network<S, A, NP, K, V>
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
   
        network.lookup(params, session, address, nodes, key)
    }
    
    fn network_get<Par, Net>(network: &mut Net,
                             params: &Par,
                             session: &Session<S>,
                             address: &A,
                             nodes: &Vec<Node<A, NP>>,
                             key: &K)
        -> Future<V>
        where   Par: Datable,
                S: Datable,
                Net: Network<S, A, NP, K, V>
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

        network.get(params, session, address, nodes, key)
    }
    
    fn network_create<Par, Net>(&self,
                                network: &mut Net,
                                params: &Par,
                                session: &Session<S>,
                                address: &A,
                                nodes: &Vec<Node<A, NP>>)
        -> Future<()>
        where   Par: Datable,
                Net: Network<S, A, NP, K, V>
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

        network.create(params, session, address, nodes, &key, &value)
    }
    
    fn network_update<Par, Net>(&self,
                                network: &mut Net,
                                params: &Par,
                                session: &Session<S>,
                                address: &A,
                                nodes: &Vec<Node<A, NP>>)
        -> Future<()>
        where   Par: Datable,
                Net: Network<S, A, NP, K, V>
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

        network.update(params, session, address, nodes, &key, &value)
    }
    
    fn network_upsert<Par, Net>(&self,
                                network: &mut Net,
                                params: &Par,
                                session: &Session<S>,
                                address: &A,
                                nodes: &Vec<Node<A, NP>>)
        -> Future<()>
        where   Par: Datable,
                Net: Network<S, A, NP, K, V>
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
        
        network.upsert(params, session, address, nodes, &key, &value)
    }
    
    fn network_delete<Par, Net>(&self,
                                network: &mut Net,
                                params: &Par,
                                session: &Session<S>,
                                address: &A,
                                nodes: &Vec<Node<A, NP>>)
        -> Future<()>
        where   Par: Datable,
                Net: Network<S, A, NP, K, V>
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
        
        let key_res = self.network_key();

        match key_res {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)); }
        }

        let key = key_res.unwrap();
        
        network.delete(params, session, address, nodes, &key)
    }

    fn network_custom<Par, R, Net>(network: &mut Net,
                                   params: &Par,
                                   session: &Session<S>,
                                   address: &A,
                                   nodes: &Vec<Node<A, NP>>)
        -> Future<R>
        where   Par: Datable,
                R: Datable,
                Net: Network<S, A, NP, K, V>
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
        
        network.custom(params, session, address, nodes)
    }
}