use mitrid_core::base::Sizable;
use mitrid_core::base::Checkable;
use mitrid_core::io::store::Store as BasicStore;
use mitrid_core::io::Permission;

use fixtures::io::store::*;

#[test]
fn test_store_size() {
    let mut store = Store::new();

    let size_a = store.size();
    assert_eq!(size_a, 0);

    let permission = Permission::Write;

    let session = store.session(&(), &permission).unwrap();

    let session_size = session.id.size() + session.size();
    let size_b = store.size();
    assert_eq!(size_b, size_a + session_size);

    let key = StoreKey::default();
    let value = StoreValue::default();

    let _ = store.create(&session, &(), &key, &value).unwrap();

    let item_size = key.size() + value.size();
    let size_c = store.size();
    assert_eq!(size_c, size_b + item_size);

    let _ = store.delete(&session, &(), &key).unwrap();
    let size_d = store.size();
    assert_eq!(size_d, size_c - item_size);
}

#[test]
fn test_store_check() {
    let mut store = Store::new();
    let res = store.check();
    assert!(res.is_ok());

    let permission = Permission::Write;

    let session = store.session(&(), &permission).unwrap();
    let res = store.check();
    assert!(res.is_ok());

    let key = StoreKey::default();
    let value = StoreValue::default();

    let _ = store.create(&session, &(), &key, &value).unwrap();
    let res = store.check();
    assert!(res.is_ok());

    let _ = store.delete(&session, &(), &key).unwrap();
    let res = store.check();
    assert!(res.is_ok());
}

#[test]
fn test_store_session() {
    let mut store = Store::new();

    let read_permission = Permission::Read;
    let res = store.session(&(), &read_permission);
    assert!(res.is_ok());
    let read_session = res.unwrap();
    assert!(read_session.permission == read_permission);

    let write_permission = Permission::Write;
    let res = store.session(&(), &write_permission);
    assert!(res.is_ok());
    let write_session = res.unwrap();
    assert!(write_session.permission == write_permission);
}

#[test]
fn test_store_count() {
    let mut store = Store::new();

    let write_permission = Permission::Write;
    let write_session = store.session(&(), &write_permission).unwrap();

    let key = StoreKey::default();
    let value = StoreValue::default();

    store.create(&write_session, &(), &key, &value).unwrap();

    let mut from = None;
    let mut to = None;

    let res = store.count(&write_session, &(), &from, &to);
    assert!(res.is_err());

    let read_permission = Permission::Read;
    let read_session = store.session(&(), &read_permission).unwrap();

    let res = store.count(&read_session, &(), &from, &to);
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 1);

    from = Some(key.clone());

    let res = store.count(&read_session, &(), &from, &to);
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 1);

    to = Some(key.clone());

    let res = store.count(&read_session, &(), &from, &to);
    assert!(res.is_err());
}

#[test]
fn test_store_list() {
    let mut store = Store::new();

    let write_permission = Permission::Write;
    let write_session = store.session(&(), &write_permission).unwrap();

    let key = StoreKey::default();
    let value = StoreValue::default();

    store.create(&write_session, &(), &key, &value).unwrap();

    let mut from = None;
    let mut to = None;
    let mut count = None;

    let res = store.list(&write_session, &(), &from, &to, &count);
    assert!(res.is_err());

    let read_permission = Permission::Read;
    let read_session = store.session(&(), &read_permission).unwrap();

    let res = store.list(&read_session, &(), &from, &to, &count);
    assert!(res.is_ok());

    let values = vec![value];

    let list = res.unwrap();
    assert_eq!(&list, &values);

    from = Some(key.clone());

    let res = store.list(&read_session, &(), &from, &to, &count);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(&list, &values);

    to = Some(key.clone());

    let res = store.list(&read_session, &(), &from, &to, &count);
    assert!(res.is_err());

    from = None;
    to = None;
    count = Some(1);

    let res = store.list(&read_session, &(), &from, &to, &count);
    println!("{:?}", &res);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(&list, &values);

    count = Some(0);

    let res = store.list(&read_session, &(), &from, &to, &count);
    assert!(res.is_err());
}

#[test]
fn test_store_lookup() {
    let mut store = Store::new();

    let write_permission = Permission::Write;
    let write_session = store.session(&(), &write_permission).unwrap();

    let mut key = StoreKey::default();
    let value = StoreValue::default();

    store.create(&write_session, &(), &key, &value).unwrap();

    let res = store.lookup(&write_session, &(), &key);
    assert!(res.is_err());

    let read_permission = Permission::Read;
    let read_session = store.session(&(), &read_permission).unwrap();

    let res = store.lookup(&read_session, &(), &key);
    assert!(res.is_ok());
    assert!(res.unwrap());

    key.push(1);

    let res = store.lookup(&read_session, &(), &key);
    assert!(res.is_ok());
    assert!(!res.unwrap());
}

#[test]
fn test_store_get() {
    let mut store = Store::new();

    let write_permission = Permission::Write;
    let write_session = store.session(&(), &write_permission).unwrap();

    let mut key = StoreKey::default();
    let value = StoreValue::default();

    store.create(&write_session, &(), &key, &value).unwrap();

    let res = store.get(&write_session, &(), &key);
    assert!(res.is_err());

    let read_permission = Permission::Read;
    let read_session = store.session(&(), &read_permission).unwrap();

    let res = store.get(&read_session, &(), &key);
    assert!(res.is_ok());
    
    let found_value = res.unwrap();
    assert_eq!(found_value, value);

    key.push(1);

    let res = store.get(&read_session, &(), &key);
    assert!(res.is_err());
}

#[test]
fn test_store_create() {
    let mut store = Store::new();

    let key = StoreKey::default();
    let value = StoreValue::default();

    let read_permission = Permission::Read;
    let read_session = store.session(&(), &read_permission).unwrap();

    let res = store.create(&read_session, &(), &key, &value);
    assert!(res.is_err());

    let write_permission = Permission::Write;
    let write_session = store.session(&(), &write_permission).unwrap();

    let res = store.create(&write_session, &(), &key, &value);
    assert!(res.is_ok());
}

#[test]
fn test_store_update() {
    let mut store = Store::new();

    let write_permission = Permission::Write;
    let write_session = store.session(&(), &write_permission).unwrap();

    let mut key = StoreKey::default();
    let mut value = StoreValue::default();

    store.create(&write_session, &(), &key, &value).unwrap();

    let read_permission = Permission::Read;
    let read_session = store.session(&(), &read_permission).unwrap();

    let found_value = store.get(&read_session, &(), &key).unwrap();
    assert_eq!(found_value, value);

    value.push(1);

    let res = store.update(&read_session, &(), &key, &value);
    assert!(res.is_err());

    let res = store.update(&write_session, &(), &key, &value);
    assert!(res.is_ok());

    let found_value = store.get(&read_session, &(), &key).unwrap();
    assert_eq!(found_value, value);

    key.push(1);

    let res = store.update(&write_session, &(), &key, &value);
    assert!(res.is_err());
}

#[test]
fn test_store_upsert() {
    let mut store = Store::new();

    let write_permission = Permission::Write;
    let write_session = store.session(&(), &write_permission).unwrap();

    let mut key = StoreKey::default();
    let mut value = StoreValue::default();

    store.create(&write_session, &(), &key, &value).unwrap();

    let read_permission = Permission::Read;
    let read_session = store.session(&(), &read_permission).unwrap();

    let found_value = store.get(&read_session, &(), &key).unwrap();
    assert_eq!(found_value, value);

    value.push(1);

    let res = store.upsert(&read_session, &(), &key, &value);
    assert!(res.is_err());

    let res = store.upsert(&write_session, &(), &key, &value);
    assert!(res.is_ok());

    let found_value = store.get(&read_session, &(), &key).unwrap();
    assert_eq!(found_value, value);

    key.push(1);

    let res = store.upsert(&write_session, &(), &key, &value);
    assert!(res.is_ok());

    let found_value = store.get(&read_session, &(), &key).unwrap();
    assert_eq!(found_value, value);
}

#[test]
fn test_store_delete() {
    let mut store = Store::new();

    let write_permission = Permission::Write;
    let write_session = store.session(&(), &write_permission).unwrap();

    let key = StoreKey::default();
    let value = StoreValue::default();

    store.create(&write_session, &(), &key, &value).unwrap();

    let read_permission = Permission::Read;
    let read_session = store.session(&(), &read_permission).unwrap();

    let found = store.lookup(&read_session, &(), &key).unwrap();
    assert!(found);

    let res = store.delete(&read_session, &(), &key);
    assert!(res.is_err());

    let res = store.delete(&write_session, &(), &key);
    assert!(res.is_ok());

    let found = store.lookup(&read_session, &(), &key).unwrap();
    assert!(!found);
}

#[test]
fn test_custom_size() {
    let mut store = Store::new();
    let mut size = 0;

    let read_permission = Permission::Read;
    let read_session = store.session(&(), &read_permission).unwrap();
    size += read_session.id.size() + read_session.size();

    let write_permission = Permission::Write;
    let write_session = store.session(&(), &write_permission).unwrap();
    size += write_session.id.size() + write_session.size();

    let params = CustomParams::Size;

    let res = store.custom(&write_session, &params);
    assert!(res.is_err());

    let res = store.custom(&read_session, &params);
    assert!(res.is_ok());

    let result = res.unwrap();

    match result {
        CustomResult::Size(size_result) => {
            assert_eq!(size_result, size);
        },
        _ => panic!("invalid result"),
    }

    let key = StoreKey::default();
    let value = StoreValue::default();

    store.create(&write_session, &(), &key, &value).unwrap();
    size += key.size() + value.size();

    let res = store.custom(&read_session, &params);
    assert!(res.is_ok());

    let result = res.unwrap();

    match result {
        CustomResult::Size(size_result) => {
            assert_eq!(size_result, size);
        },
        _ => panic!("invalid result"),
    }

    store.delete(&write_session, &(), &key).unwrap();
    size -= key.size() + value.size();

    let res = store.custom(&read_session, &params);
    assert!(res.is_ok());

    let result = res.unwrap();

    match result {
        CustomResult::Size(size_result) => {
            assert_eq!(size_result, size);
        },
        _ => panic!("invalid result"),
    }
}

#[test]
fn test_custom_dump_sessions() {
    let mut store = Store::new();

    let permission = Permission::default();

    let session_a = store.session(&(), &permission).unwrap();
    let session_b = store.session(&(), &permission).unwrap();

    let params = CustomParams::Dump(DumpParams::Sessions);
    let res = store.custom(&session_a, &params);
    assert!(res.is_ok());

    let result_a = res.unwrap();

    let res = store.custom(&session_b, &params);
    assert!(res.is_ok());
    
    let result_b = res.unwrap();

    assert_eq!(result_a, result_b);

    match result_a {
        CustomResult::DumpSessions(dump) => {
            let res = dump.check();
            assert!(res.is_ok());

            assert_eq!(dump.count, 2);
            
            for (id, session) in dump.sessions {
                if id != session_a.id && id != session_b.id {
                    panic!("unknown session id");
                }

                if session != session_a && session != session_b {
                    panic!("unknown session");
                }
            }
        },
        _ => panic!("invalid result"),
    }
}

#[test]
fn test_custom_dump_items() {
    let mut store = Store::new();

    let read_permission = Permission::Read;
    let read_session = store.session(&(), &read_permission).unwrap();

    let write_permission = Permission::Write;
    let write_session = store.session(&(), &write_permission).unwrap();

    let params = CustomParams::Dump(DumpParams::Items);

    let res = store.custom(&write_session, &params);
    assert!(res.is_err());

    let res = store.custom(&read_session, &params);
    assert!(res.is_ok());

    let result = res.unwrap();

    match result {
        CustomResult::DumpItems(dump) => {
            let res = dump.check();
            assert!(res.is_ok());

            assert_eq!(dump.count, 0);
        },
        _ => panic!("invalid result"),
    }

    let key = StoreKey::default();
    let value = StoreValue::default();

    store.create(&write_session, &(), &key, &value).unwrap();

    let res = store.custom(&read_session, &params);
    assert!(res.is_ok());

    let result = res.unwrap();

    match result {
        CustomResult::DumpItems(dump) => {
            let res = dump.check();
            assert!(res.is_ok());

            assert_eq!(dump.count, 1);
            assert_eq!(dump.items[0].0, key);
            assert_eq!(dump.items[0].1, value);
        },
        _ => panic!("invalid result"),
    }

    store.delete(&write_session, &(), &key).unwrap();

    let res = store.custom(&read_session, &params);
    assert!(res.is_ok());

    let result = res.unwrap();

    match result {
        CustomResult::DumpItems(dump) => {
            let res = dump.check();
            assert!(res.is_ok());

            assert_eq!(dump.count, 0);
        },
        _ => panic!("invalid result"),
    }
}

#[test]
fn test_custom_dump_all() {
    let mut store = Store::new();

    let read_permission = Permission::Read;
    let read_session = store.session(&(), &read_permission).unwrap();

    let write_permission = Permission::Write;
    let write_session = store.session(&(), &write_permission).unwrap();

    let params = CustomParams::Dump(DumpParams::All);

    let res = store.custom(&write_session, &params);
    assert!(res.is_err());

    let res = store.custom(&read_session, &params);
    assert!(res.is_ok());

    let result = res.unwrap();

    match result {
        CustomResult::DumpAll(dump) => {
            let res = dump.check();
            assert!(res.is_ok());

            assert_eq!(dump.sessions_count, 2);
            
            for (id, session) in dump.sessions {
                if id != read_session.id && id != write_session.id {
                    panic!("unknown session id");
                }

                if session != read_session && session != write_session {
                    panic!("unknown session");
                }
            }

            assert_eq!(dump.items_count, 0);
        },
        _ => panic!("invalid result"),
    }

    let key = StoreKey::default();
    let value = StoreValue::default();

    store.create(&write_session, &(), &key, &value).unwrap();

    let res = store.custom(&read_session, &params);
    assert!(res.is_ok());

    let result = res.unwrap();

    match result {
        CustomResult::DumpAll(dump) => {
            let res = dump.check();
            assert!(res.is_ok());

            assert_eq!(dump.sessions_count, 2);
            
            for (id, session) in dump.sessions {
                if id != read_session.id && id != write_session.id {
                    panic!("unknown session id");
                }

                if session != read_session && session != write_session {
                    panic!("unknown session");
                }
            }

            assert_eq!(dump.items_count, 1);
            assert_eq!(dump.items[0].0, key);
            assert_eq!(dump.items[0].1, value);
        },
        _ => panic!("invalid result"),
    }

    store.delete(&write_session, &(), &key).unwrap();

    let res = store.custom(&read_session, &params);
    assert!(res.is_ok());

    let result = res.unwrap();

    match result {
        CustomResult::DumpAll(dump) => {
            let res = dump.check();
            assert!(res.is_ok());

            assert_eq!(dump.sessions_count, 2);
            
            for (id, session) in dump.sessions {
                if id != read_session.id && id != write_session.id {
                    panic!("unknown session id");
                }

                if session != read_session && session != write_session {
                    panic!("unknown session");
                }
            }

            assert_eq!(dump.items_count, 0);
        },
        _ => panic!("invalid result"),
    }
}