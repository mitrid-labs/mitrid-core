use mitrid_core::util::regex;

const EMAIL_PATTERN: &str = "(^(?P<username>[a-zA-Z0-9_.+-]+)@(?P<host>[a-zA-Z0-9-]+)\\.(?P<domain>[a-zA-Z0-9-.]+)$)";

#[test]
fn test_regex_is_match() {
    let valid_email = "test@example.com";

    let res = regex::is_match(EMAIL_PATTERN, valid_email);
    assert!(res.is_ok());
    assert!(res.unwrap());

    let invalid_email = "test/@example.com";

    let res = regex::is_match(EMAIL_PATTERN, invalid_email);
    assert!(res.is_ok());
    assert!(!res.unwrap())
}

#[test]
fn test_regex_captures() {
    let valid_email = "test@example.com";

    let res = regex::captures(EMAIL_PATTERN, valid_email);
    assert!(res.is_ok());

    let captures = res.unwrap();

    let username_ = captures.get("username");
    assert!(username_.is_some());
    let username = username_.unwrap();
    assert_eq!(username, "test");
    
    let host_ = captures.get("host");
    assert!(host_.is_some());
    let host = host_.unwrap();
    assert_eq!(host, "example");
    
    let domain_ = captures.get("domain");
    assert!(domain_.is_some());
    let domain = domain_.unwrap();
    assert_eq!(domain, "com");

    let invalid_email = "test/@example.com";

    let res = regex::captures(EMAIL_PATTERN, invalid_email);
    assert!(res.is_err());
}