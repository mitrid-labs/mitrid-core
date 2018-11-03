use mitrid_core::base::Sizable;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::utils::{Version, Stage};

use fixtures::crypto::Digest;
use fixtures::io::network::Address;
use fixtures::app::Config;

#[test]
fn test_config_new() {
    let chain = "chain";
    let version = Version::default();
    let stage = Stage::default();
    let pswd_hash = Digest::default();
    let buffer_size = 1024;
    let max_threads = 8;
    let addresses = vec![Address::default()];
    let seed = vec![Address::default()];

    let res = Config::new(&chain, &version, &stage, &pswd_hash, buffer_size, max_threads, &addresses, &seed);
    assert!(res.is_ok());
}

#[test]
fn test_config_size() {
    let config = Config::default();

    let config_size = config.size();
    let expected_size = config.chain.size() +
                        config.version.size() +
                        config.stage.size() +
                        config.pswd_hash.size() +
                        config.buffer_size.size() +
                        config.max_threads.size() +
                        config.addresses.size() +
                        config.seed.size();

    assert_eq!(config_size, expected_size);
}

#[test]
fn test_config_check() {
    let chain = "chain";
    let version = Version::default();
    let stage = Stage::default();
    let pswd_hash = Digest::default();
    let buffer_size = 1024;
    let max_threads = 8;
    let addresses = vec![Address::default()];
    let seed = vec![Address::default()];

    let mut config = Config::new(&chain, &version, &stage, &pswd_hash, buffer_size, max_threads, &addresses, &seed).unwrap();
    let res = config.check();
    assert!(res.is_ok());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    config.version = invalid_version;
    let res = config.check();
    assert!(res.is_err());
}

#[test]
fn test_config_json() {
    let config_a = Config::default();

    let res = config_a.to_json();
    assert!(res.is_ok());

    let config_json = res.unwrap();

    let res = Config::from_json(&config_json);
    assert!(res.is_ok());

    let config_b = res.unwrap();

    assert_eq!(config_a, config_b);
}

#[test]
fn test_config_bytes() {
    let config_a = Config::default();

    let res = config_a.to_bytes();
    assert!(res.is_ok());

    let config_bytes = res.unwrap();

    let res = Config::from_bytes(&config_bytes);
    assert!(res.is_ok());

    let config_b = res.unwrap();

    assert_eq!(config_a, config_b);
}

#[test]
fn test_config_hex() {
    let config_a = Config::default();

    let res = config_a.to_hex();
    assert!(res.is_ok());

    let config_hex = res.unwrap();

    let res = Config::from_hex(&config_hex);
    assert!(res.is_ok());

    let config_b = res.unwrap();

    assert_eq!(config_a, config_b);
}

#[test]
fn test_config_accessors() {}

#[test]
fn test_config_json_file() {}

#[test]
fn test_config_bytes_file() {}

#[test]
fn test_config_hex_file() {}