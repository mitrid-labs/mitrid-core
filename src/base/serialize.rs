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

impl Serializable for bool {}

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

impl<A, B> Serializable for (A, B)
    where   A: Serializable,
            B: Serializable
{}

impl<A, B, C> Serializable for (A, B, C)
    where   A: Serializable,
            B: Serializable,
            C: Serializable
{}

impl<A, B, C, D> Serializable for (A, B, C, D)
    where   A: Serializable,
            B: Serializable,
            C: Serializable,
            D: Serializable
{}

impl<A, B, C, D, E> Serializable for (A, B, C, D, E)
    where   A: Serializable,
            B: Serializable,
            C: Serializable,
            D: Serializable,
            E: Serializable
{}

impl<A, B, C, D, E, F> Serializable for (A, B, C, D, E, F)
    where   A: Serializable,
            B: Serializable,
            C: Serializable,
            D: Serializable,
            E: Serializable,
            F: Serializable
{}