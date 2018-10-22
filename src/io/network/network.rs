//! # Network
//!
//! `network` is the module providing the traits implemented by networking facilities and
//! types that can be sent and retrieved through a network.

use std::marker::PhantomData;

use base::Result;
use base::Future;
use base::Numerical;
use base::{ConstantSize, VariableSize};
use base::Checkable;
use base::Datable;
use base::Serializable;
use models::meta::Meta;
use models::coin::Coin;
use models::input::Input;
use models::output::Output;
use models::transaction::Transaction;
use models::blocknode::BlockNode;
use models::block::Block;
use models::blockgraph::BlockGraph;
use io::Permission;
use io::Session;
use io::Node;

/// Trait implemented by network transports.
pub trait Transport<A, D>
    where   A: Datable + VariableSize,
            D: Datable + Serializable
{
    /// Opens a connection to a network address.
    fn connect<P: Datable>(&mut self, params: &P, address: &A) -> Future<()>;

    /// Closes a connection to a network address.
    fn disconnect<P: Datable>(&mut self, params: &P, address: &A) -> Future<()>;

    /// Listen to connections incoming from a network address.
    fn listen<P: Datable>(&mut self, params: &P, address: &A) -> Future<()>;

    /// Sends data through a network connection.
    fn send<P: Datable>(&mut self, params: &P, data: &D) -> Future<()>;

    /// Receives data from a network connection.
    fn recv<P: Datable>(&mut self, params: &P) -> Future<D>;
}

pub enum Method {
    Ping,
    Session,
    Count,
    List,
    Lookup,
    Get,
    Create,
    Update,
    Upgrade,
    Delete,
    Custom,
}

pub enum Resource {
    None,
    Node,
    Coin,
    Input,
    Output,
    Transaction,
    BlockNode,
    Block,
    BlockGraph,
    Custom,
}

pub enum MessageData<Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   Ad: Datable + VariableSize,
            NP: Datable,
            D: Datable + ConstantSize,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            Pr: Datable,
            Am: Numerical,
            IP: Datable,
            OP: Datable,
            TP: Datable,
            BP: Datable,
            BGP: Datable,
            C: Datable
{
    None,
    Node(Node<Ad, NP>),
    Coin(Coin<D, Am>),
    Input(Input<D, Am, IP, Pk, Sig>),
    Output(Output<D, Pk, Am, OP>),
    Transaction(Transaction<D, Am, IP, Pk, Sig, OP, TP>),
    BlockNode(BlockNode<D>),
    Block(Block<D, Am, IP, Pk, Sig, OP, TP, BP, Pr>),
    BlockGraph(BlockGraph<D, BGP>),
    Custom(C),
    Error(String),
}

pub struct Message<M, R, D, S, A, NP, P>
    where   M: Datable,
            R: Datable,
            D: Datable + ConstantSize,
            S: Datable,
            A: Datable + VariableSize,
            NP: Datable,
            P: Datable
{
    method: PhantomData<T>,
    resource: PhantomData<R>,
    pub id: D,
    pub meta: Meta,
    pub nonce: u64,
    pub session: Session<S>,
    pub node: Node<A, NP>,
    pub data: MessageData,
}

pub trait Client<S, A, NP, K, V>
    where   S: Datable + Serializable,
            A: Datable + VariableSize + Serializable,
            NP: Datable + Serializable,
            K: Datable + Serializable,
            V: Datable + Serializable
{
    /// Requests a new `Session` from the called network nodes.
    /// The request is sent to one or more nodes in the network. An empty list if from any node.
    fn session<P: Datable>(&mut self,
                               params: &P,
                               address: &A,
                               nodes: &Vec<Node<A, NP>>,
                               permission: &Permission)
        -> Future<Message<Session<S>>>;

    /// Requests the count of the items starting from the `from` key until, not included, the `to` key.
    /// The request is sent to one or more nodes in the network. An empty list if from any node.
    fn count<P: Datable>(&mut self,
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
    fn list<P: Datable>(&mut self,
                            params: &P,
                            session: &Session<S>,
                            address: &A,
                            nodes: &Vec<Node<A, NP>>,
                            from: &Option<K>,
                            to: &Option<K>,
                            count: &Option<u64>)
        -> Future<Vec<V>>;
    
    /// Request the lookup of an item from the called nodes using its key. The request is sent to one
    /// or more nodes in the network. An empty list if from any node.
    fn lookup<P: Datable>(&mut self,
                          params: &P,
                          session: &Session<S>,
                          address: &A,
                          nodes: &Vec<Node<A, NP>>,
                          key: &K)
        -> Future<bool>;
    
    /// Requests an item from the called nodes using its key. The request is sent to one or more
    /// nodes in the network. An empty list if from any node.
    fn get<P: Datable>(&mut self,
                           params: &P,
                           session: &Session<S>,
                           address: &A,
                           nodes: &Vec<Node<A, NP>>,
                           key: &K)
        -> Future<V>;
    
    /// Requests the creation of an item to the called nodes. The item should not exist in the network stores
    /// before the operation. The request is sent to one or more nodes in the network.
    /// An empty list if from any node.
    fn create<P: Datable>(&mut self,
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
    fn update<P: Datable>(&mut self,
                          params: &P,
                          session: &Session<S>,
                          address: &A,
                          nodes: &Vec<Node<A, NP>>,
                          key: &K,
                          value: &V)
        -> Future<()>;
    
    /// Requests the creation of an item to the called nodes if absent, update if present.
    /// The request is sent to one or more nodes in the network. An empty list if from any node.
    fn upsert<P: Datable>(&mut self,
                              params: &P,
                              session: &Session<S>,
                              address: &A,
                              nodes: &Vec<Node<A, NP>>,
                              key: &K,
                              value: &V)
        -> Future<()>;
    
    /// Requests the deletion of an item to the called network nodes. The item should
    /// already exist in the node store before the operation. The request is sent to one
    /// or more nodes in the network. An empty list if from any node.
    fn delete<P: Datable>(&mut self,
                              params: &P,
                              session: &Session<S>,
                              address: &A,
                              nodes: &Vec<Node<A, NP>>,
                              key: &K)
        -> Future<()>;
    
    /// Requests a custom operation to the called network nodes.
    /// The request is sent to one or more nodes in the network. An empty list if from any node.
    fn custom<P: Datable, R: Datable>(&mut self,
                                      params: &P,
                                      session: &Session<S>,
                                      address: &A,
                                      nodes: &Vec<Node<A, NP>>)
        -> Future<R>;
}

/*
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
    
    /// Requests the count of the items starting from the `from` key until, not included, the `to` key.
    /// The request is sent to one or more nodes in the network. An empty list if from any node.
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
    
    /// Requests the list of the items starting from the `from` key until, not included, the `to` key.
    /// The request is sent to one or more nodes in the network. An empty list if from any node.
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
    
    /// Request the lookup of an item from the called nodes using its key. The request is sent to one
    /// or more nodes in the network. An empty list if from any node.
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
    
    /// Requests an item from the called nodes using its key. The request is sent to one or more
    /// nodes in the network. An empty list if from any node.
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
    
    /// Requests the creation of an item to the called nodes. The item should not exist in the network stores
    /// before the operation. The request is sent to one or more nodes in the network.
    /// An empty list if from any node.
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
    
    /// Requests the update of an item to the called nodes. The item should already exist
    /// in the network stores before the operation. The request is sent to one or more
    /// nodes in the network. An empty list if from any node.
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
    
    /// Requests the creation of an item to the called nodes if absent, update if present.
    /// The request is sent to one or more nodes in the network. An empty list if from any node.
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
    
    /// Requests the deletion of an item to the called network nodes. The item should
    /// already exist in the node store before the operation. The request is sent to one
    /// or more nodes in the network. An empty list if from any node.
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

    /// Requests a custom operation to the called network nodes.
    /// The request is sent to one or more nodes in the network. An empty list if from any node.
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
*/