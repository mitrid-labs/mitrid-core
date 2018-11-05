use mitrid_core::base::Checkable;
use mitrid_core::base::Sizable;
use mitrid_core::base::Serializable;
use mitrid_core::util::Timestamp;
use mitrid_core::io::Permission;

use fixture::io::Session;

#[test]
fn test_session_new() {
    let id = 0;
    let permission = Permission::default();
    let timestamp = Timestamp::now().unwrap();
    let res = Session::new(id, &permission, &timestamp, &());
    assert!(res.is_ok());
}

#[test]
fn test_session_is_expired() {
    let id = 0;
    let permission = Permission::default();
    let timestamp = Timestamp::now().unwrap();

    let mut session = Session::new(id, &permission, &timestamp, &()).unwrap();

    let res = session.is_expired();
    assert!(res.is_ok());
    assert!(res.unwrap());

    session.expires_at *= 2;

    let res = session.is_expired();
    assert!(res.is_ok());
    assert!(!res.unwrap());
}

#[test]
fn test_session_check() {
    let id = 0;
    let permission = Permission::default();
    let timestamp = Timestamp::now().unwrap();

    let session = Session::new(id, &permission, &timestamp, &()).unwrap();

    let res = session.check();
    assert!(res.is_ok());
}

#[test]
fn test_session_size() {
    let id = 0;
    let permission = Permission::default();
    let timestamp = Timestamp::now().unwrap();

    let session = Session::new(id, &permission, &timestamp, &()).unwrap();

    let session_size = session.id.size() +
                        session.permission.size() +
                        session.expires_at.size() +
                        session.payload.size();

    assert_eq!(session.size(), session_size);
}

#[test]
fn test_session_json() {
    let session_a = Session::default();

    let res = session_a.to_json();
    assert!(res.is_ok());

    let session_json = res.unwrap();

    let res = Session::from_json(&session_json);
    assert!(res.is_ok());

    let session_b = res.unwrap();

    assert_eq!(session_a, session_b);
}

#[test]
fn test_session_bytes() {
    let session_a = Session::default();

    let res = session_a.to_bytes();
    assert!(res.is_ok());

    let session_bytes = res.unwrap();

    let res = Session::from_bytes(&session_bytes);
    assert!(res.is_ok());

    let session_b = res.unwrap();

    assert_eq!(session_a, session_b);
}

#[test]
fn test_session_hex() {
    let session_a = Session::default();

    let res = session_a.to_hex();
    assert!(res.is_ok());

    let session_hex = res.unwrap();

    let res = Session::from_hex(&session_hex);
    assert!(res.is_ok());

    let session_b = res.unwrap();

    assert_eq!(session_a, session_b);
}