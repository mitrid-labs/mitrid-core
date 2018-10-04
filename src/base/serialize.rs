//! # Serialize
//!
//! `serialize` is the module providing the trait implemented by types that can be serialized.

use serde::{Serialize, Deserialize};
use json;
use cbor;
use hex;

use base::result::Result;
use base::check::Checkable;

/// Trait implemented by types that can be serialized.
pub trait Serializable
    where   for<'a> Self: Serialize + Deserialize<'a> + Checkable
{
    /// Serializes the implementor into a json string.
    fn to_json(&self) -> Result<String> {
        self.check()?;

        json::to_string(&self).map_err(|e| format!("{}", e))
    }

    /// Deserializes a json string into the implementor type.
    fn from_json(s: &str) -> Result<Self> {
        let t: Self = json::from_str(s).map_err(|e| format!("{}", e))?;

        t.check()?;
        Ok(t)
    }

    /// Serializes the implementor into a byte vector. 
    fn to_bytes(&self) -> Result<Vec<u8>> {
        self.check()?;

        cbor::to_vec(self).map_err(|e| format!("{}", e))
    }

    /// Deserializes a byte vector into the implementor type.
    fn from_bytes(b: &[u8]) -> Result<Self> {
        let t: Self = cbor::from_slice(b).map_err(|e| format!("{}", e))?;

        t.check()?;
        Ok(t)
    }

    /// Serializes the implementor into a hex string.
    fn to_hex(&self) -> Result<String> {
        Ok(hex::encode(self.to_bytes()?))
    }

    /// Deserializes an hex string into the implementor type.
    fn from_hex(s: &str) -> Result<Self> {
        let _s = s.to_lowercase();
        let b = hex::decode(_s)
            .map_err(|e| format!("{}", e))?;

        Self::from_bytes(&b)
    }
}

impl Serializable for () {}

impl Serializable for u8 {}

impl Serializable for u32 {}

impl Serializable for u64 {}

impl Serializable for i8 {}

impl Serializable for i32 {}

impl Serializable for i64 {}

impl<T> Serializable for Vec<T>
    where   T: Serializable
{}

impl Serializable for String {}

impl<T> Serializable for Option<T>
    where   T: Serializable
{}

impl<T> Serializable for Box<T>
    where   T: Serializable
{}