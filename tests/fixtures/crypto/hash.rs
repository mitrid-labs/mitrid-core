#![allow(dead_code)]
#![allow(unused_variables)]

use crypto::digest::Digest as _Digest;
use crypto::sha3::Sha3;

use mitrid_core::base::Result;
use mitrid_core::base::{Sizable, FixedSize};
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::base::Datable;

pub const DIGEST_SIZE: u64 = 64;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Digest(Vec<u8>);

impl Digest {
    pub fn from_vec(buf: &Vec<u8>) -> Result<Digest> {
        if buf.len() != DIGEST_SIZE as usize {
            return Err(String::from("invalid length"));
        }

        Ok(Digest(buf.to_owned()))
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.clone()
    }

    pub fn from_slice(buf: &[u8]) -> Result<Digest> {
        if buf.len() != DIGEST_SIZE as usize {
            return Err(String::from("invalid length"));
        }

        Ok(Digest(buf.to_owned()))
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl Sizable for Digest {
    fn size(&self) -> u64 {
        self.0.size()
    }
}

impl FixedSize for Digest {
    fn required_size() -> u64 {
        DIGEST_SIZE
    }
}

impl Checkable for Digest {
    fn check(&self) -> Result<()> {
        self.0.check()?;
        self.check_size()
    }
}

impl Serializable for Digest {}

impl Datable for Digest {}

pub struct SHA512 {}

impl SHA512 {
    fn digest(msg: &[u8]) -> Result<Digest> {
        let mut hasher = Sha3::sha3_512();
        hasher.input(msg);

        let mut buf = Vec::new();
        hasher.result(&mut buf.as_mut_slice());

        Digest::from_vec(&buf)
    }

    pub fn verify_digest(msg: &[u8], digest: &Digest) -> Result<bool> {
        Ok(&Self::digest(msg)? == digest)
    }

    pub fn check_digest(msg: &[u8], digest: &Digest) -> Result<()> {
        if !Self::verify_digest(msg, digest)? {
            return Err(String::from("invalid digest"));
        }

        Ok(())
    }
}