use base::Future;
use base::VariableSize;
use base::Checkable;
use base::Datable;
use io::Permission;
use io::Session;
use io::Node;

pub trait Networkable<S, A, NP, K, V>
    where   S: Datable,
            A: Datable + VariableSize,
            NP: Datable,
            K: Datable,
            V: Datable
{
    fn session_cb<P: Datable>(params: &P,
                              address: &A,
                              nodes: &Vec<Node<A, NP>>,
                              permission: &Permission,
                              cb: &Fn(&P, &A, &Vec<Node<A, NP>>, &Permission) -> Future<Session<S>>)
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

            cb(params, address, nodes, permission)
    }
    
    fn count_cb<P: Datable>(params: &P,
                            address: &A,
                            nodes: &Vec<Node<A, NP>>,
                            from: &Option<K>,
                            to: &Option<K>,
                            cb: &Fn(&P, &A, &Vec<Node<A, NP>>, &Option<K>, &Option<K>) -> Future<u64>)
        -> Future<u64> {
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

        match from.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        match to.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        cb(params, address, nodes, from, to)
    }
    
    fn list_cb<P: Datable>(params: &P,
                           address: &A,
                           nodes: &Vec<Node<A, NP>>,
                           from: &Option<K>,
                           to: &Option<K>,
                           count: &Option<u64>,
                           cb: &Fn(&P, &A, &Vec<Node<A, NP>>, &Option<K>, &Option<K>, &Option<u64>) -> Future<Vec<V>>)
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

        cb(params, address, nodes, from, to, count)
    }
    
    fn lookup_cb<P: Datable>(params: &P,
                             address: &A,
                             nodes: &Vec<Node<A, NP>>,
                             key: &K,
                             cb: &Fn(&P, &A, &Vec<Node<A, NP>>, &K) -> Future<bool>)
        -> Future<bool>
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

        match key.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        cb(params, address, nodes, key)
    }
    
    fn get_cb<P: Datable>(params: &P,
                          address: &A,
                          nodes: &Vec<Node<A, NP>>,
                          key: &K,
                          cb: &Fn(&P, &A, &Vec<Node<A, NP>>, &K) -> Future<V>)
        -> Future<V>
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

        match key.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        cb(params, address, nodes, key)
    }
    
    fn create_cb<P: Datable>(params: &P,
                             address: &A,
                             nodes: &Vec<Node<A, NP>>,
                             key: &K,
                             value: &V,
                             cb: &Fn(&P, &A, &Vec<Node<A, NP>>, &K, &V) -> Future<()>)
        -> Future<()>
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

        match key.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        match value.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        cb(params, address, nodes, key, value)
    }
    
    fn update_cb<P: Datable>(params: &P,
                             address: &A,
                             nodes: &Vec<Node<A, NP>>,
                             key: &K,
                             value: &V,
                             cb: &Fn(&P, &A, &Vec<Node<A, NP>>, &K, &V) -> Future<()>)
        -> Future<()>
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

        match key.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        match value.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        cb(params, address, nodes, key, value)
    }
    
    fn upsert_cb<P: Datable>(params: &P,
                             address: &A,
                             nodes: &Vec<Node<A, NP>>,
                             key: &K,
                             value: &V,
                             cb: &Fn(&P, &A, &Vec<Node<A, NP>>, &K, &V) -> Future<()>)
        -> Future<()>
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

        match key.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        match value.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        cb(params, address, nodes, key, value)
    }
    
    fn delete_cb<P: Datable>(params: &P,
                             address: &A,
                             nodes: &Vec<Node<A, NP>>,
                             key: &K,
                             cb: &Fn(&P, &A, &Vec<Node<A, NP>>, &K) -> Future<()>)
        -> Future<()>
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

        match key.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        cb(params, address, nodes, key)
    }
    
    fn custom_cb<P: Datable, R: Datable>(params: &P,
                                         address: &A,
                                         nodes: &Vec<Node<A, NP>>,
                                         cb: &Fn(&P, &A, &Vec<Node<A, NP>>) -> Future<R>)
        -> Future<R>
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

        cb(params, address, nodes)
    }
}