use mitrid_core::app::Env as BasicEnv;

use std::env as std_env;

use fixture::app::Env;

#[test]
fn test_env_vars() {
    let key = String::from("key");
    let value = String::from("value");

    std_env::set_var(&key, &value);

    let env = Env{};

    let res = env.vars();
    assert!(res.is_ok());

    let vars = res.unwrap();

    if !vars.contains_key(&key) {
        panic!("key not found");
    }

    if let Some(found_value) = vars.get(&key) {
        assert_eq!(found_value, &value);
    } else {
        panic!("value not found");
    }
}

#[test]
fn test_env_current_dir() {
    let env = Env{};

    let cwd = std_env::current_dir().unwrap();
    let res = env.current_dir();
    let current_dir = res.unwrap();

    assert_eq!(cwd, current_dir)
}

#[test]
fn test_env_log_level() {
    let key = String::from("log_level");
    let value = String::from("info");

    std_env::set_var(&key, &value);

    let env = Env{};

    let res = env.log_level();
    assert!(res.is_ok());

    let log_level = res.unwrap();
    assert_eq!(format!("{}", log_level), value);
}

#[test]
fn test_env_log_file() {
    let key = String::from("log_file");
    let value = String::from("stderr");

    std_env::set_var(&key, &value);

    let env = Env{};

    let res = env.log_file();
    assert!(res.is_ok());

    let log_file = res.unwrap();
    assert_eq!(format!("{}", log_file), value);
}