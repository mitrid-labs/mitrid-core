use mitrid_core::base::Sizable;
use mitrid_core::base::Serializable;
use mitrid_core::io::network::client::OnError;

#[test]
fn test_on_error_new_ignore() {
    let on_error = OnError::new_ignore();
    assert_eq!(on_error, OnError::Ignore);
}

#[test]
fn test_on_error_new_fail() {
    let on_error = OnError::new_fail();
    assert_eq!(on_error, OnError::Fail);
}

#[test]
fn test_on_error_new_retry_and_ignore() {
    let times = 100;
    let on_error = OnError::new_retry_and_ignore(times);
    assert_eq!(on_error, OnError::RetryAndIgnore(times));
}

#[test]
fn test_on_error_new_retry_and_fail() {
    let times = 100;
    let on_error = OnError::new_retry_and_ignore(times);
    assert_eq!(on_error, OnError::RetryAndIgnore(times));
}

#[test]
fn test_on_error_default() {
    let on_error = OnError::default();
    assert_eq!(on_error, OnError::Ignore);
}

#[test]
fn test_on_error_size() {
    let on_error = OnError::default();
    assert_eq!(on_error.size(), 0u8.size());

    let times = 3;

    let on_error = OnError::RetryAndIgnore(times);
    assert_eq!(on_error.size(), times.size());

    let on_error = OnError::RetryAndFail(times);
    assert_eq!(on_error.size(), times.size());
}

#[test]
fn test_on_error_serialize_json() {
    let on_error_a = OnError::default();
    let res = on_error_a.to_json();
    assert!(res.is_ok());

    let on_error_a_json = res.unwrap();
    let res = OnError::from_json(&on_error_a_json);
    assert!(res.is_ok());

    let on_error_b = res.unwrap();
    assert_eq!(on_error_a, on_error_b);
}

#[test]
fn test_on_error_serialize_bytes() {
    let on_error_a = OnError::default();
    let res = on_error_a.to_bytes();
    assert!(res.is_ok());

    let on_error_a_bytes = res.unwrap();
    let res = OnError::from_bytes(&on_error_a_bytes);
    assert!(res.is_ok());

    let on_error_b = res.unwrap();
    assert_eq!(on_error_a, on_error_b);
}

#[test]
fn test_on_error_serialize_hex() {
    let on_error_a = OnError::default();
    let res = on_error_a.to_hex();
    assert!(res.is_ok());

    let on_error_a_hex = res.unwrap();
    let res = OnError::from_hex(&on_error_a_hex);
    assert!(res.is_ok());

    let on_error_b = res.unwrap();
    assert_eq!(on_error_a, on_error_b);
}