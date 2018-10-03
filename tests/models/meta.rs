use mitrid_core::models::Meta;
use mitrid_core::models::Stage;
use mitrid_core::utils::Version;
use mitrid_core::base::Checkable;
use mitrid_core::base::Sizable;
use mitrid_core::base::Serializable;

#[test]
fn test_meta_new() {
    let name = "name";
    let chain = "chain";
    let valid_version = Version::default();
    let stage = Stage::default();
    let size = 1_000_000;
    
    let res = Meta::new(name.into(), chain.into(), valid_version, stage, size);
    assert!(res.is_ok());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let res = Meta::new(name.into(), chain.into(), invalid_version, stage, size);
    assert!(res.is_err());
}

#[test]
fn test_meta_check() {
    let name = "name";
    let chain = "chain";
    let valid_version = Version::default();
    let stage = Stage::default();
    let size = 1_000_000;
    
    let res = Meta::new(name.into(), chain.into(), valid_version, stage, size);
    assert!(res.is_ok());

    let mut meta = res.unwrap();
    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();
    meta.version = invalid_version;

    let res = meta.check();
    assert!(res.is_err());
}

#[test]
fn test_meta_size() {
    let meta = Meta::default();
    let meta_size = meta.name.size() +
                    meta.chain.size() +
                    meta.version.size() +
                    meta.stage.size() +
                    meta.size.size();
    assert_eq!(meta.size(), meta_size)
}

#[test]
fn test_meta_serialize_json() {
    let meta_a = Meta::default();
    let res = meta_a.to_json();
    assert!(res.is_ok());

    let meta_a_json = res.unwrap();
    let res = Meta::from_json(&meta_a_json);
    assert!(res.is_ok());

    let meta_b = res.unwrap();
    assert_eq!(meta_a, meta_b);
}

#[test]
fn test_meta_serialize_bytes() {
    let meta_a = Meta::default();
    let res = meta_a.to_bytes();
    assert!(res.is_ok());

    let meta_a_bytes = res.unwrap();
    let res = Meta::from_bytes(&meta_a_bytes);
    assert!(res.is_ok());

    let meta_b = res.unwrap();
    assert_eq!(meta_a, meta_b);
}

#[test]
fn test_meta_serialize_hex() {
    let meta_a = Meta::default();
    let res = meta_a.to_hex();
    assert!(res.is_ok());

    let meta_a_hex = res.unwrap();
    let res = Meta::from_hex(&meta_a_hex);
    assert!(res.is_ok());

    let meta_b = res.unwrap();
    assert_eq!(meta_a, meta_b);
}