use serde::{Serialize, Deserialize};
use json;
use cbor;
use hex;

use base::result::Result;
use base::check::Checkable;

pub trait Serializable
    where   for<'a> Self: Serialize + Deserialize<'a> + Checkable
{
    fn to_json(t: Self) -> Result<String> {
        t.check()?;

        json::to_string(&t).map_err(|e| format!("{}", e))
    }

    fn from_json(s: &str) -> Result<Self> {
        let t: Self = json::from_str(s).map_err(|e| format!("{}", e))?;

        t.check()?;
        Ok(t)
    }

    fn to_bytes(t: Self) -> Result<Vec<u8>> {
        t.check()?;

        cbor::to_vec(&t).map_err(|e| format!("{}", e))
    }

    fn from_bytes(b: &[u8]) -> Result<Self> {
        let t: Self = cbor::from_slice(b).map_err(|e| format!("{}", e))?;

        t.check()?;
        Ok(t)
    }

    fn to_hex(t: Self) -> Result<String> {
        Ok(hex::encode(Self::to_bytes(t)?))
    }

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