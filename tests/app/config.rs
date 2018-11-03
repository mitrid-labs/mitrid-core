use tempfile::tempdir;
use mitrid_core::base::Sizable;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::utils::{Version, Stage};
use mitrid_core::app::Config as BasicConfig;

use std::fs::{self, File};

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
fn test_config_accessors() {
    let chain: String = "chain".into();
    let version = Version::default();
    let stage = Stage::default();
    let pswd_hash = Digest::default();
    let buffer_size = 1024;
    let max_threads = 8;
    let addresses = vec![Address::default()];
    let seed = vec![Address::default()];

    let config = Config::new(&chain, &version, &stage, &pswd_hash, buffer_size, max_threads, &addresses, &seed).unwrap();

    assert_eq!(config.chain(), chain);
    assert_eq!(config.version(), version);
    assert_eq!(config.stage(), stage);
    assert_eq!(config.pswd_hash(), pswd_hash);
    assert_eq!(config.buffer_size(), buffer_size);
    assert_eq!(config.max_threads(), max_threads);
    assert_eq!(config.addresses(), addresses);
    assert_eq!(config.seed(), seed);
    assert_eq!(&config.manager_params(), &());
    assert_eq!(&config.store_params(), &());
    assert_eq!(&config.server_params(), &());
    assert_eq!(&config.client_params(), &());
    assert_eq!(&config.custom_params(), &());
}

#[test]
fn test_config_json_file() {
    let config_file_name = "config.json";
    let temp_dir = tempdir().unwrap();

    let config_path = format!("{}", temp_dir.path().join(config_file_name).to_str().unwrap());
    let file = File::create(&config_path).unwrap();

    let chain: String = "chain".into();
    let version = Version::default();
    let stage = Stage::default();
    let pswd_hash = Digest::default();
    let buffer_size = 1024;
    let max_threads = 8;
    let addresses = vec![Address::default()];
    let seed = vec![Address::default()];

    let config = Config::new(&chain, &version, &stage, &pswd_hash, buffer_size, max_threads, &addresses, &seed).unwrap();

    let res = config.write_json_file(&config_path.clone());
    assert!(res.is_ok());

    let res = Config::read_json_file(&config_path.clone());
    assert!(res.is_ok());

    let read_config = res.unwrap();
    let config_json_a = read_config.to_json().unwrap();

    let buf = fs::read(&config_path).unwrap();
    let config_json_b = String::from_utf8(buf).unwrap();

    let config_json_c = config.to_json().unwrap();

    assert_eq!(config_json_a, config_json_b);
    assert_eq!(config_json_a, config_json_c);

    drop(file);
    temp_dir.close().unwrap();
}

#[test]
fn test_config_bytes_file() {
    let config_file_name = "config.bin";
    let temp_dir = tempdir().unwrap();

    let config_path = format!("{}", temp_dir.path().join(config_file_name).to_str().unwrap());
    let file = File::create(&config_path).unwrap();

    let chain: String = "chain".into();
    let version = Version::default();
    let stage = Stage::default();
    let pswd_hash = Digest::default();
    let buffer_size = 1024;
    let max_threads = 8;
    let addresses = vec![Address::default()];
    let seed = vec![Address::default()];

    let config = Config::new(&chain, &version, &stage, &pswd_hash, buffer_size, max_threads, &addresses, &seed).unwrap();

    let res = config.write_binary_file(&config_path.clone());
    assert!(res.is_ok());

    let res = Config::read_binary_file(&config_path.clone());
    assert!(res.is_ok());

    let read_config = res.unwrap();
    let config_bytes_a = read_config.to_bytes().unwrap();

    let config_bytes_b = fs::read(&config_path).unwrap();

    let config_bytes_c = config.to_bytes().unwrap();

    assert_eq!(config_bytes_a, config_bytes_b);
    assert_eq!(config_bytes_a, config_bytes_c);

    drop(file);
    temp_dir.close().unwrap();
}

#[test]
fn test_config_hex_file() {
    let config_file_name = "config.hex";
    let temp_dir = tempdir().unwrap();

    let config_path = format!("{}", temp_dir.path().join(config_file_name).to_str().unwrap());
    let file = File::create(&config_path).unwrap();

    let chain: String = "chain".into();
    let version = Version::default();
    let stage = Stage::default();
    let pswd_hash = Digest::default();
    let buffer_size = 1024;
    let max_threads = 8;
    let addresses = vec![Address::default()];
    let seed = vec![Address::default()];

    let config = Config::new(&chain, &version, &stage, &pswd_hash, buffer_size, max_threads, &addresses, &seed).unwrap();

    let res = config.write_hex_file(&config_path.clone());
    assert!(res.is_ok());

    let res = Config::read_hex_file(&config_path.clone());
    assert!(res.is_ok());

    let read_config = res.unwrap();
    let config_hex_a = read_config.to_hex().unwrap();

    let buf = fs::read(&config_path).unwrap();
    let config_hex_b = String::from_utf8(buf).unwrap();

    let config_hex_c = config.to_hex().unwrap();

    assert_eq!(config_hex_a, config_hex_b);
    assert_eq!(config_hex_a, config_hex_c);

    drop(file);
    temp_dir.close().unwrap();
}