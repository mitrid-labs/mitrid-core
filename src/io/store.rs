use base::Future;
use base::Checkable;
use base::Datable;
use io::Permission;
use io::Session;

pub trait Storable<S, K, V>
    where   S: Datable,
            K: Datable,
            V: Datable
{
    fn session_cb<P: Datable>(params: &P,
                              permission: &Permission,
                              cb: &Fn(&P, &Permission) -> Future<Session<S>>)
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

        cb(params, permission)
    }
    
    fn count_cb<P: Datable>(params: &P,
                            from: &Option<K>,
                            to: &Option<K>,
                            cb: &Fn(&P, &Option<K>, &Option<K>) -> Future<u64>)
        -> Future<u64>
    {
        match params.check() {
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

        cb(params, from, to)
    }
    
    fn list_cb<P: Datable>(params: &P,
                           from: &Option<K>,
                           to: &Option<K>,
                           count: &Option<u64>,
                           cb: &Fn(&P, &Option<K>, &Option<K>, &Option<u64>) -> Future<Vec<V>>)
        -> Future<Vec<V>>
    {
        match params.check() {
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

        cb(params, from, to, count)
    }
    
    fn lookup_cb<P: Datable>(params: &P,
                             key: &K,
                             cb: &Fn(&P, &K) -> Future<bool>)
        -> Future<bool>
    {
        match params.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        match key.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        cb(params, key)
    }
    
    fn get_cb<P: Datable>(params: &P,
                          key: &K,
                          cb: &Fn(&P, &K) -> Future<V>)
        -> Future<V>
    {
        match params.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        match key.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        cb(params, key)
    }
    
    fn create_cb<P: Datable>(params: &P,
                             key: &K,
                             value: &V,
                             cb: &Fn(&P, &K, &V) -> Future<()>)
        -> Future<()>
    {
        match params.check() {
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

        cb(params, key, value)
    }
    
    fn update_cb<P: Datable>(params: &P,
                             key: &K,
                             value: &V,
                             cb: &Fn(&P, &K, &V) -> Future<()>)
        -> Future<()>
    {
        match params.check() {
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

        cb(params, key, value)
    }
    
    fn upsert_cb<P: Datable>(params: &P,
                             key: &K,
                             value: &V,
                             cb: &Fn(&P, &K, &V) -> Future<()>)
        -> Future<()>
    {
        match params.check() {
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

        cb(params, key, value)
    }
    
    fn delete_cb<P: Datable>(params: &P,
                             key: &K,
                             cb: &Fn(&P, &K) -> Future<()>)
        -> Future<()>
    {
        match params.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        match key.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        cb(params, key)
    }
    
    fn custom_cb<P: Datable, R: Datable>(params: &P,
                                         cb: &Fn(&P) -> Future<R>)
        -> Future<R>
    {
        match params.check() {
            Ok(_) => {},
            Err(e) => { return Future::from_result(Err(e)) },
        }

        cb(params)
    }
}