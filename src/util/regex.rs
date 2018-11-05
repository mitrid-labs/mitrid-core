//! # Regex
//!
//! `regex` is the module providing the regex utility functions.

use regex::Regex;

use std::collections::HashMap;

use base::Result;

/// Returns if the target string matches the regex pattern.
pub fn is_match(pattern: &str, target: &str) -> Result<bool> {
    let reg = Regex::new(pattern).map_err(|e| format!("{}", e))?;
    Ok(reg.is_match(target))
}

/// Returns the regex captures obtained from the target string against the regex pattern.
pub fn captures(pattern: &str, target: &str) -> Result<HashMap<String, String>> {
    let reg = Regex::new(pattern).map_err(|e| format!("{}", e))?;
    if !reg.is_match(target) {
        return Err(String::from("no match"));
    }

    let mut res = HashMap::<String, String>::new();

    let _captures = reg.captures(target);

    if _captures.is_none() {
        return Ok(res);
    }

    let captures = _captures.unwrap();

    for cap_name in reg.capture_names() {
        if cap_name.is_some() {
            let key = cap_name.unwrap();
            let mut value = "";
            if let Some(cap_match) = captures.name(key) {
                value = cap_match.as_str();
            }
            res.insert(String::from(key), String::from(value));
        }
    }

    Ok(res)
}