//! # Network
//!
//! `network` is the module providing the traits implemented by networking facilities and
//! types that can be sent and retrieved through a network.

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
    /// Requests a new `Session` from the called network nodes.
    /// The request is sent to one or more nodes in the network. An empty list if from any node.
    fn session_req<P: Datable>(&mut self,
                               params: &P,
                               address: &A,
                               nodes: &Vec<Node<A, NP>>,
                               permission: &Permission)
        -> Future<Session<S>>;
    
    /// Returns a new `Session` to the calling network node.
    fn session_res<P: Datable>(&mut self,
                               params: &P,
                               address: &A,
                               result: &Option<Session<S>>,
                               error: &Option<String>)
        -> Future<()>;

    /// Requests the count of the items starting from the `from` key until, not included, the `to` key.
    /// The request is sent to one or more nodes in the network. An empty list if from any node.
    fn count_req<P: Datable>(&mut self,
                             params: &P,
                             session: &Session<S>,
                             address: &A,
                             nodes: &Vec<Node<A, NP>>,
                             from: &Option<K>,
                             to: &Option<K>)
        -> Future<u64>;

    /// Returns a count of items to the calling network node.
    fn count_res<P: Datable>(&mut self,
                             params: &P,
                             session: &Session<S>,
                             address: &A,
                             result: &Option<u64>,
                             error: &Option<String>)
        -> Future<()>;
    
    /// Requests the list of the items starting from the `from` key until, not included, the `to` key.
    /// The request is sent to one or more nodes in the network. An empty list if from any node.
    fn list_req<P: Datable>(&mut self,
                            params: &P,
                            session: &Session<S>,
                            address: &A,
                            nodes: &Vec<Node<A, NP>>,
                            from: &Option<K>,
                            to: &Option<K>,
                            count: &Option<u64>)
        -> Future<Vec<V>>;
    
    /// Returns a list of items to the calling network node.
    fn list_res<P: Datable>(&mut self,
                            params: &P,
                            session: &Session<S>,
                            address: &A,
                            result: &Option<Vec<V>>,
                            error: &Option<String>)
        -> Future<()>;
    
    /// Request the lookup of an item from the called nodes using its key. The request is sent to one
    /// or more nodes in the network. An empty list if from any node.
    fn lookup_req<P: Datable>(&mut self,
                          params: &P,
                          session: &Session<S>,
                          address: &A,
                          nodes: &Vec<Node<A, NP>>,
                          key: &K)
        -> Future<bool>;
    
    /// Returns the lookup of an item to the calling network node.
    fn lookup_res<P: Datable>(&mut self,
                              params: &P,
                              session: &Session<S>,
                              address: &A,
                              result: &Option<bool>,
                              error: &Option<String>)
        -> Future<()>;
    
    /// Requests an item from the called nodes using its key. The request is sent to one or more
    /// nodes in the network. An empty list if from any node.
    fn get_req<P: Datable>(&mut self,
                           params: &P,
                           session: &Session<S>,
                           address: &A,
                           nodes: &Vec<Node<A, NP>>,
                           key: &K)
        -> Future<V>;
    
    /// Returns an item to the calling network node.
    fn get_res<P: Datable>(&mut self,
                       params: &P,
                       session: &Session<S>,
                       address: &A,
                       result: &Option<V>,
                       error: &Option<String>)
        -> Future<()>;
    
    /// Requests the creation of an item to the called nodes. The item should not exist in the network stores
    /// before the operation. The request is sent to one or more nodes in the network.
    /// An empty list if from any node.
    fn create_req<P: Datable>(&mut self,
                          params: &P,
                          session: &Session<S>,
                          address: &A,
                          nodes: &Vec<Node<A, NP>>,
                          key: &K,
                          value: &V)
        -> Future<()>;
    
    /// Returns the result of the creation of an item from the calling network node.
    fn create_res<P: Datable>(&mut self,
                              params: &P,
                              session: &Session<S>,
                              address: &A,
                              error: &Option<String>)
        -> Future<()>;
    
    /// Requests the update of an item to the called nodes. The item should already exist
    /// in the network stores before the operation. The request is sent to one or more
    /// nodes in the network. An empty list if from any node.
    fn update_req<P: Datable>(&mut self,
                          params: &P,
                          session: &Session<S>,
                          address: &A,
                          nodes: &Vec<Node<A, NP>>,
                          key: &K,
                          value: &V)
        -> Future<()>;
    
    /// Returns the result of the update of an item to the calling network node.
    fn update_res<P: Datable>(&mut self,
                              params: &P,
                              session: &Session<S>,
                              address: &A,
                              error: &Option<String>)
        -> Future<()>;
    
    /// Requests the creation of an item to the called nodes if absent, update if present.
    /// The request is sent to one or more nodes in the network. An empty list if from any node.
    fn upsert_req<P: Datable>(&mut self,
                              params: &P,
                              session: &Session<S>,
                              address: &A,
                              nodes: &Vec<Node<A, NP>>,
                              key: &K,
                              value: &V)
        -> Future<()>;
    
    /// Returns the result of the creation of an item to the calling network node if absent,
    /// update if present.
    fn upsert_res<P: Datable>(&mut self,
                              params: &P,
                              session: &Session<S>,
                              address: &A,
                              error: &Option<String>)
        -> Future<()>;
    
    /// Requests the deletion of an item to the called network nodes. The item should
    /// already exist in the node store before the operation. The request is sent to one
    /// or more nodes in the network. An empty list if from any node.
    fn delete_req<P: Datable>(&mut self,
                              params: &P,
                              session: &Session<S>,
                              address: &A,
                              nodes: &Vec<Node<A, NP>>,
                              key: &K)
        -> Future<()>;
    
    /// Returns the result of the deletion of an item to the calling network node.
    fn delete_res<P: Datable>(&mut self,
                              params: &P,
                              session: &Session<S>,
                              address: &A,
                              error: &Option<String>)
        -> Future<()>;
    
    /// Requests a custom operation to the called network nodes.
    /// The request is sent to one or more nodes in the network. An empty list if from any node.
    fn custom_req<P: Datable, R: Datable>(&mut self,
                                      params: &P,
                                      session: &Session<S>,
                                      address: &A,
                                      nodes: &Vec<Node<A, NP>>)
        -> Future<R>;
    
    /// Returns the result of a custom operation to the calling network node.
    fn custom_res<P: Datable, R: Datable>(&mut self,
                                          params: &P,
                                          session: &Session<S>,
                                          address: &A,
                                          result: &Option<R>,
                                          error: &Option<String>)
        -> Future<()>;
}

pub trait Networkable<S, A, NP, K, V>
    where   S: Datable + Serializable,
            A: Datable + VariableSize + Serializable,
            NP: Datable + Serializable,
            K: Datable + Serializable,
            V: Datable + Serializable,
            Self: Datable
{
    /// Returns the network key of the item.
    fn network_key(&self) -> Result<K>;

    /// Returns the network value of the item.
    fn network_value(&self) -> Result<V>;

    /// Requests a new `Session` from the called network nodes.
    /// The request is sent to one or more nodes in the network. An empty list if from any node.
    fn network_session_req<Par, Net>(network: &mut Net,
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

        network.session_req(params, address, nodes, permission)
    }
    
    /// Requests the count of the items starting from the `from` key until, not included, the `to` key.
    /// The request is sent to one or more nodes in the network. An empty list if from any node.
    fn network_count_req<Par, Net>(network: &mut Net,
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

        network.count_req(params, session, address, nodes, from, to)
    }
    
    /// Requests the list of the items starting from the `from` key until, not included, the `to` key.
    /// The request is sent to one or more nodes in the network. An empty list if from any node.
    fn network_list_req<Par, Net>(network: &mut Net,
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

        network.list_req(params, session, address, nodes, from, to, count)
    }
    
    /// Request the lookup of an item from the called nodes using its key. The request is sent to one
    /// or more nodes in the network. An empty list if from any node.
    fn network_lookup_req<Par, Net>(network: &mut Net,
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
   
        network.lookup_req(params, session, address, nodes, key)
    }
    
    /// Requests an item from the called nodes using its key. The request is sent to one or more
    /// nodes in the network. An empty list if from any node.
    fn network_get_req<Par, Net>(network: &mut Net,
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

        network.get_req(params, session, address, nodes, key)
    }
    
    /// Requests the creation of an item to the called nodes. The item should not exist in the network stores
    /// before the operation. The request is sent to one or more nodes in the network.
    /// An empty list if from any node.
    fn network_create_req<Par, Net>(&self,
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

        network.create_req(params, session, address, nodes, &key, &value)
    }
    
    /// Requests the update of an item to the called nodes. The item should already exist
    /// in the network stores before the operation. The request is sent to one or more
    /// nodes in the network. An empty list if from any node.
    fn network_update_req<Par, Net>(&self,
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

        network.update_req(params, session, address, nodes, &key, &value)
    }
    
    /// Requests the creation of an item to the called nodes if absent, update if present.
    /// The request is sent to one or more nodes in the network. An empty list if from any node.
    fn network_upsert_req<Par, Net>(&self,
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
        
        network.upsert_req(params, session, address, nodes, &key, &value)
    }
    
    /// Requests the deletion of an item to the called network nodes. The item should
    /// already exist in the node store before the operation. The request is sent to one
    /// or more nodes in the network. An empty list if from any node.
    fn network_delete_req<Par, Net>(&self,
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
        
        network.delete_req(params, session, address, nodes, &key)
    }

    /// Requests a custom operation to the called network nodes.
    /// The request is sent to one or more nodes in the network. An empty list if from any node.
    fn network_custom_req<Par, R, Net>(network: &mut Net,
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
        
        network.custom_req(params, session, address, nodes)
    }
}