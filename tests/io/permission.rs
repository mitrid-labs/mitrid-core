use mitrid_core::base::Checkable;
use mitrid_core::base::Sizable;
use mitrid_core::base::Serializable;
use mitrid_core::io::Permission;

#[test]
fn test_permission_parse() {
    let permissions = vec!["none", "read", "write", "invalid"];

    for ref permission in permissions {
        let res = Permission::parse(permission);
        if permission != &"invalid" {
            assert!(res.is_ok());
        } else {
            assert!(res.is_err());
        }
    }
}

#[test]
fn test_permission_format() {
    let none = Permission::None;
    let none_string = format!("{}", none);
    let read = Permission::Read;
    let read_string = format!("{}", read);
    let write = Permission::Write;
    let write_string = format!("{}", write);

    assert_eq!(none_string, String::from("none"));
    assert_eq!(read_string, String::from("read"));
    assert_eq!(write_string, String::from("write"));
}

#[test]
fn test_permission_default() {
    let permission = Permission::default();

    assert_eq!(permission, Permission::None);
}

#[test]
fn test_permission_check() {
    let permission = Permission::default();
    let res = permission.check();

    assert!(res.is_ok());
}

#[test]
fn test_permission_size() {
    let permission = Permission::default();
    assert_eq!(permission.size(), 1);
}

#[test]
fn test_permission_json() {
    let permission_a = Permission::default();

    let res = permission_a.to_json();
    assert!(res.is_ok());

    let permission_json = res.unwrap();

    let res = Permission::from_json(&permission_json);
    assert!(res.is_ok());

    let permission_b = res.unwrap();

    assert_eq!(permission_a, permission_b);
}

#[test]
fn test_permission_bytes() {
    let permission_a = Permission::default();

    let res = permission_a.to_bytes();
    assert!(res.is_ok());

    let permission_bytes = res.unwrap();

    let res = Permission::from_bytes(&permission_bytes);
    assert!(res.is_ok());

    let permission_b = res.unwrap();

    assert_eq!(permission_a, permission_b);
}

#[test]
fn test_permission_hex() {
    let permission_a = Permission::default();

    let res = permission_a.to_hex();
    assert!(res.is_ok());

    let permission_hex = res.unwrap();

    let res = Permission::from_hex(&permission_hex);
    assert!(res.is_ok());

    let permission_b = res.unwrap();

    assert_eq!(permission_a, permission_b);
}