use mitrid_core::utils::{Version, version::VERSION};
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;

#[test]
fn test_version_new() {
    let res = Version::new(1, 2, 3, "alpha", "build-01");
    assert!(res.is_ok());

    let res = Version::new(1, 2, 3, "///", "build-02");
    assert!(res.is_err());

    let res = Version::new(1, 2, 3, "beta", "àòlè@*)");
    assert!(res.is_err());
}

#[test]
fn test_version_default() {
    let version = Version::default();
    assert!(version.check().is_ok());

    assert!(&version.to_string() == VERSION);
}

#[test]
fn test_version_check_numeric() {
    let valid_numeric = "12345";
    
    let res = Version::check_numeric(valid_numeric);
    assert!(res.is_ok());

    let invalid_numeric_a = "1234.9";
    let res = Version::check_numeric(invalid_numeric_a);
    assert!(res.is_err());

    let invalid_numeric_b = "sdf8873439hf-pewrfjhdsjgvbcru";
    let res = Version::check_numeric(invalid_numeric_b);
    assert!(res.is_err());

}

#[test]
fn test_version_check_prerelease() {
    let valid_prerelease = "abc-DEF-ghj-";
    
    let res = Version::check_prerelease(valid_prerelease);
    assert!(res.is_ok());

    let invalid_prerelease_a = "1234";
    let res = Version::check_prerelease(invalid_prerelease_a);
    assert!(res.is_err());

    let invalid_prerelease_b = "!£$%";
    let res = Version::check_prerelease(invalid_prerelease_b);
    assert!(res.is_err());
}

#[test]
fn test_version_check_buildmeta() {
    let valid_buildmeta = "123-abc-DEF-";
    
    let res = Version::check_buildmeta(valid_buildmeta);
    assert!(res.is_ok());

    let invalid_buildmeta = "&/(.";
    let res = Version::check_buildmeta(invalid_buildmeta);
    assert!(res.is_err());
}

#[test]
fn test_version_check_semver() {
    let valid_semver = "1.10.1947-abcd-EFG+1A-bc-2";
    
    let res = Version::check_semver(valid_semver);
    assert!(res.is_ok());

    let invalid_semver_a = "1.10.194a";
    let res = Version::check_semver(invalid_semver_a);
    assert!(res.is_err());

    let invalid_semver_b = "1.10.1947-";
    let res = Version::check_semver(invalid_semver_b);
    assert!(res.is_err());
}

#[test]
fn test_version_parse() {
    let valid_version = "1.10.1947-abcd-EFG+1A-bc-2";

    let res = Version::parse(valid_version);
    assert!(res.is_ok());

    let invalid_version_a = "1.10.1947+";

    let res = Version::parse(invalid_version_a);
    assert!(res.is_err());

    let invalid_version_b = "a.10.1947";

    let res = Version::parse(invalid_version_b);
    assert!(res.is_err());

    let invalid_version_c = "1.b.1947";

    let res = Version::parse(invalid_version_c);
    assert!(res.is_err());

    let invalid_version_d = "1.10.c";

    let res = Version::parse(invalid_version_d);
    assert!(res.is_err());

    let invalid_version_e = "a.b.c";
    let res = Version::parse(invalid_version_e);
    assert!(res.is_err());
}

#[test]
fn test_version_to_string() {
    let valid_version = "1.10.1947-abcd-EFG+1A-bc-2";

    let version_a = Version::parse(valid_version).unwrap();
    let version_a_str = version_a.to_string();

    let version_b = Version::parse(&version_a_str).unwrap();
    assert_eq!(version_a, version_b);
}

#[test]
fn test_version_format() {
    let valid_version = "1.10.1947-abcd-EFG+1A-bc-2";

    let version_a = Version::parse(valid_version).unwrap();
    let version_a_str = format!("{}", version_a);

    let version_b = Version::parse(&version_a_str).unwrap();
    assert_eq!(version_a, version_b);
}

#[test]
fn test_version_ord() {
    let version_a = Version::parse("0.1.0").unwrap();
    let version_b = Version::parse("0.2.0").unwrap();
    let version_c = Version::parse("1.0.0").unwrap();
    let version_d = Version::parse("0.0.6").unwrap();
    let version_e = Version::parse("0.0.6-alpha").unwrap();
    let version_f = Version::parse("0.0.6-beta").unwrap();
    let version_g = Version::parse("0.0.6-beta+abuild").unwrap();

    assert!(version_a < version_b);
    assert!(version_a < version_c);
    assert!(version_b < version_c);
    assert!(version_d < version_a);
    assert!(version_d < version_b);
    assert!(version_d < version_c);
    assert!(version_e < version_d);
    assert!(version_e < version_f);
    assert!(version_f < version_d);
    assert!(version_g == version_f);
}

#[test]
fn test_version_is_compatible() {
    let version_a = Version::parse("1.0.2-alpha").unwrap();
    let version_b = Version::parse("1.0.2-beta").unwrap();
    let version_c = Version::parse("0.0.1+build-1947").unwrap();
    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "....".into();

    let res = version_a.is_compatible(&version_b);
    assert!(res.is_ok());
    assert!(res.unwrap());

    let res = version_b.is_compatible(&version_c);
    assert!(res.is_ok());
    assert!(!res.unwrap());

    let res = version_c.is_compatible(&invalid_version);
    assert!(res.is_err());
}

#[test]
fn test_version_serialize_json() {
    let version_a = Version::new(0, 0, 0, "alphabeta", "buildmeta").unwrap();
    let res = version_a.to_json();
    assert!(res.is_ok());

    let version_a_json = res.unwrap();
    let res = Version::from_json(&version_a_json);
    assert!(res.is_ok());

    let version_b = res.unwrap();
    assert_eq!(version_a, version_b);
}

#[test]
fn test_version_serialize_bytes() {
    let version_a = Version::new(0, 0, 0, "alphabeta", "buildmeta").unwrap();
    let res = version_a.to_bytes();
    assert!(res.is_ok());

    let version_a_bytes = res.unwrap();
    let res = Version::from_bytes(&version_a_bytes);
    assert!(res.is_ok());

    let version_b = res.unwrap();
    assert_eq!(version_a, version_b);
}

#[test]
fn test_version_serialize_hex() {
    let version_a = Version::new(0, 0, 0, "alphabeta", "buildmeta").unwrap();
    let res = version_a.to_hex();
    assert!(res.is_ok());

    let version_a_hex = res.unwrap();
    let res = Version::from_hex(&version_a_hex);
    assert!(res.is_ok());

    let version_b = res.unwrap();
    assert_eq!(version_a, version_b);
}