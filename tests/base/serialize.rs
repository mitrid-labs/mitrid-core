use mitrid_core::base::Serializable;

#[test]
fn test_json() {
    let a = Some(String::from("string"));
    let json_a = a.to_json().unwrap();
    let b = Option::<String>::from_json(&json_a).unwrap();

    assert_eq!(a, b)
}

#[test]
fn test_bytes() {
    let a = Some(String::from("string"));
    let bytes_a = a.to_bytes().unwrap();
    let b = Option::<String>::from_bytes(&bytes_a).unwrap();

    assert_eq!(a, b)
}

#[test]
fn test_hex() {
    let a = Some(String::from("string"));
    let hex_a = a.to_hex().unwrap();
    let b = Option::<String>::from_hex(&hex_a).unwrap();

    assert_eq!(a, b)
}