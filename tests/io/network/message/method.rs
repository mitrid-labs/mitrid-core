use mitrid_core::base::Sizable;
use mitrid_core::base::Serializable;
use mitrid_core::io::Permission;
use mitrid_core::io::network::Method;

#[test]
fn test_method_parse() {
    let valid_method_strs = vec!["ping",
                                 "session",
                                 "count",
                                 "list",
                                 "lookup",
                                 "get",
                                 "create",
                                 "update",
                                 "upsert",
                                 "delete",
                                 "eval",
                                 "evalmut"];

    let invalid_method_str = "put";

    for method_str in valid_method_strs.iter() {
        let res = Method::parse(method_str);
        assert!(res.is_ok());
    }

    let res = Method::parse(invalid_method_str);
    assert!(res.is_err());
}

#[test]
fn test_method_display() {
    let method_strs = vec!["ping",
                           "session",
                           "count",
                           "list",
                           "lookup",
                           "get",
                           "create",
                           "update",
                           "upsert",
                           "delete",
                           "eval",
                           "evalmut"];

    for method_str in method_strs.iter() {
        let method = Method::parse(method_str).unwrap();
        assert_eq!(format!("{}", method), String::from(*method_str));
    }
}

#[test]
fn test_method_check_permission() {
    let none_permission = Permission::None;
    let read_permission = Permission::Read;
    let write_permission = Permission::Write;

    let none_method_strs = vec!["ping", "session"];
    let read_method_strs = vec!["session", "count", "list", "lookup", "list", "eval"];
    let write_method_strs = vec!["session", "create", "update", "upsert", "delete", "evalmut"];

    for method_str in none_method_strs.iter() {
        let method = Method::parse(method_str).unwrap();

        let res = method.check_permission(&none_permission);
        assert!(res.is_ok());

        let res = method.check_permission(&read_permission);
        if method_str != &"session" {
            assert!(res.is_err());
        } else {
            assert!(res.is_ok());
        }

        let res = method.check_permission(&write_permission);
        if method_str != &"session" {
            assert!(res.is_err());
        } else {
            assert!(res.is_ok());
        }
    }

    for method_str in read_method_strs.iter() {
        let method = Method::parse(method_str).unwrap();

        let res = method.check_permission(&read_permission);
        assert!(res.is_ok());

        let res = method.check_permission(&write_permission);
        if method_str != &"session" {
            assert!(res.is_err());
        } else {
            assert!(res.is_ok());
        }

        let res = method.check_permission(&none_permission);
        if method_str != &"session" {
            assert!(res.is_err());
        } else {
            assert!(res.is_ok());
        }
    }

    for method_str in write_method_strs.iter() {
        let method = Method::parse(method_str).unwrap();

        let res = method.check_permission(&write_permission);
        assert!(res.is_ok());

        let res = method.check_permission(&none_permission);
        if method_str != &"session" {
            assert!(res.is_err());
        } else {
            assert!(res.is_ok());
        }

        let res = method.check_permission(&read_permission);
        if method_str != &"session" {
            assert!(res.is_err());
        } else {
            assert!(res.is_ok());
        }
    }
}

#[test]
fn test_method_default() {
    let method = Method::default();
    assert_eq!(method, Method::Ping);
}

#[test]
fn test_method_size() {
    let method = Method::default();
    let method_size = (method as u8).size();
    assert_eq!(method.size(), method_size)
}

#[test]
fn test_method_serialize_json() {
    let method_a = Method::default();
    let res = method_a.to_json();
    assert!(res.is_ok());

    let method_a_json = res.unwrap();
    let res = Method::from_json(&method_a_json);
    assert!(res.is_ok());

    let method_b = res.unwrap();
    assert_eq!(method_a, method_b);
}

#[test]
fn test_method_serialize_bytes() {
    let method_a = Method::default();
    let res = method_a.to_bytes();
    assert!(res.is_ok());

    let method_a_bytes = res.unwrap();
    let res = Method::from_bytes(&method_a_bytes);
    assert!(res.is_ok());

    let method_b = res.unwrap();
    assert_eq!(method_a, method_b);
}

#[test]
fn test_method_serialize_hex() {
    let method_a = Method::default();
    let res = method_a.to_hex();
    assert!(res.is_ok());

    let method_a_hex = res.unwrap();
    let res = Method::from_hex(&method_a_hex);
    assert!(res.is_ok());

    let method_b = res.unwrap();
    assert_eq!(method_a, method_b);
}