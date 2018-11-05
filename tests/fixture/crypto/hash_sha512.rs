#![allow(dead_code)]

use sodiumoxide::init;
use sodiumoxide::crypto::hash::DIGESTBYTES;
use sodiumoxide::crypto::hash::hash;

use mitrid_core::base::Result;
use mitrid_core::base::{Sizable, ConstantSize};
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::base::Datable;
use mitrid_core::crypto::Hash;

pub const DIGEST_SIZE: u64 = DIGESTBYTES as u64;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Serialize, Deserialize)]
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

impl Default for Digest {
    fn default() -> Digest {
        let mut _digest = Vec::new();
        
        for _ in 0..DIGEST_SIZE as usize {
            _digest.push(0);
        }

        Digest(_digest)
    }
}

impl Sizable for Digest {
    fn size(&self) -> u64 {
        self.0.size()
    }
}

impl ConstantSize for Digest {
    fn constant_size() -> u64 {
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
    pub fn digest(msg: &[u8]) -> Result<Digest> {
        init().unwrap();

        Digest::from_slice(&hash(msg).0[..])
    }

    pub fn verify(msg: &[u8], digest: &Digest) -> Result<bool> {
        init().unwrap();

        digest.check()?;
        digest.check_size()?;

        Ok(&Self::digest(msg)? == digest)
    }

    pub fn check(msg: &[u8], digest: &Digest) -> Result<()> {
        init().unwrap();

        digest.check()?;
        digest.check_size()?;

        if !Self::verify(msg, digest)? {
            return Err(String::from("invalid digest"));
        }

        Ok(())
    }
}

impl Hash<Digest> for SHA512 {
    fn digest(&mut self, msg: &[u8]) -> Result<Digest> {
        Self::digest(msg)
    }

    fn verify(&mut self, msg: &[u8], digest: &Digest) -> Result<bool> {
        Self::verify(msg, digest)
    }

    fn check(&mut self, msg: &[u8], digest: &Digest) -> Result<()> {
        Self::check(msg, digest)
    }
}

pub type Hasher = SHA512;

#[test]
fn test_digest_from_vec() {
    let mut buf = Vec::new();
    for _ in 0..DIGEST_SIZE-1 {
        buf.push(0);
    }

    let res = Digest::from_vec(&buf);
    assert!(res.is_err());

    buf.push(0);

    let res = Digest::from_vec(&buf);
    assert!(res.is_ok());
}

#[test]
fn test_digest_from_slice() {
    let mut buf = Vec::new();
    for _ in 0..DIGEST_SIZE-1 {
        buf.push(0);
    }

    let res = Digest::from_slice(&buf);
    assert!(res.is_err());

    buf.push(0);

    let res = Digest::from_slice(&buf);
    assert!(res.is_ok());
}

#[test]
fn test_hash_sha512() {
    let mut msg = Vec::new();
    for _ in 0..500 {
        msg.push(0);
    }

    let mut hash = SHA512{};

    let res = hash.digest(&msg);
    assert!(res.is_ok());

    let digest = res.unwrap();

    let res = hash.verify(&msg, &digest);
    assert!(res.is_ok());
    assert!(res.unwrap());

    let res = hash.check(&msg, &digest);
    assert!(res.is_ok());

    msg.push(0);

    let res = hash.verify(&msg, &digest);
    assert!(res.is_ok());
    assert!(!res.unwrap());

    let res = hash.check(&msg, &digest);
    assert!(res.is_err());
}