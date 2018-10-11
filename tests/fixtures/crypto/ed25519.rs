#![allow(dead_code)]

use crypto::ed25519;

use mitrid_core::base::Result;
use mitrid_core::base::{Sizable, FixedSize};
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::base::Datable;

pub const SECRETKEY_SIZE: u64 = 64;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash, Serialize, Deserialize)]
pub struct SecretKey(Vec<u8>);

impl SecretKey {
    pub fn from_vec(buf: &Vec<u8>) -> Result<SecretKey> {
        if buf.len() != SECRETKEY_SIZE as usize {
            return Err(String::from("invalid length"));
        }

        Ok(SecretKey(buf.to_owned()))
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.clone()
    }

    pub fn from_slice(buf: &[u8]) -> Result<SecretKey> {
        if buf.len() != SECRETKEY_SIZE as usize {
            return Err(String::from("invalid length"));
        }

        Ok(SecretKey(buf.to_owned()))
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl Sizable for SecretKey {
    fn size(&self) -> u64 {
        self.0.size()
    }
}

impl FixedSize for SecretKey {
    fn required_size() -> u64 {
        SECRETKEY_SIZE
    }
}

impl Checkable for SecretKey {
    fn check(&self) -> Result<()> {
        self.0.check()?;
        self.check_size()
    }
}

impl Serializable for SecretKey {}

impl Datable for SecretKey {}

pub const PUBLICKEY_SIZE: u64 = 32;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash, Serialize, Deserialize)]
pub struct PublicKey(Vec<u8>);

impl PublicKey {
    pub fn from_vec(buf: &Vec<u8>) -> Result<PublicKey> {
        if buf.len() != PUBLICKEY_SIZE as usize {
            return Err(String::from("invalid length"));
        }

        Ok(PublicKey(buf.to_owned()))
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.clone()
    }

    pub fn from_slice(buf: &[u8]) -> Result<PublicKey> {
        if buf.len() != PUBLICKEY_SIZE as usize {
            return Err(String::from("invalid length"));
        }

        Ok(PublicKey(buf.to_owned()))
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl Sizable for PublicKey {
    fn size(&self) -> u64 {
        self.0.size()
    }
}

impl FixedSize for PublicKey {
    fn required_size() -> u64 {
        PUBLICKEY_SIZE
    }
}

impl Checkable for PublicKey {
    fn check(&self) -> Result<()> {
        self.0.check()?;
        self.check_size()
    }
}

impl Serializable for PublicKey {}

impl Datable for PublicKey {}

pub const SIGNATURE_SIZE: u64 = 32;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Signature(Vec<u8>);

impl Signature {
    pub fn from_vec(buf: &Vec<u8>) -> Result<Signature> {
        if buf.len() != SIGNATURE_SIZE as usize {
            return Err(String::from("invalid length"));
        }

        Ok(Signature(buf.to_owned()))
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.clone()
    }

    pub fn from_slice(buf: &[u8]) -> Result<Signature> {
        if buf.len() != SIGNATURE_SIZE as usize {
            return Err(String::from("invalid length"));
        }

        Ok(Signature(buf.to_owned()))
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl Sizable for Signature {
    fn size(&self) -> u64 {
        self.0.size()
    }
}

impl FixedSize for Signature {
    fn required_size() -> u64 {
        SIGNATURE_SIZE
    }
}

impl Checkable for Signature {
    fn check(&self) -> Result<()> {
        self.0.check()?;
        self.check_size()
    }
}

impl Serializable for Signature {}

impl Datable for Signature {}

pub struct Ed25519 {}

impl Ed25519 {
    pub fn keypair(seed: &[u8]) -> Result<(PublicKey, SecretKey)> {
        let (_pk, _sk) = ed25519::keypair(seed);

        let pk = PublicKey::from_slice(&_pk[..])?;
        let sk = SecretKey::from_slice(&_sk[..])?;

        Ok((pk, sk))
    }

    pub fn sign(msg: &[u8], sk: &SecretKey) -> Result<Signature> {
        sk.check()?;

        let _sig = ed25519::signature(msg, sk.as_slice());
        Signature::from_slice(&_sig[..])
    }

    pub fn verify(msg: &[u8], pk: &PublicKey, sig: &Signature) -> Result<bool> {
        pk.check()?;
        sig.check()?;

        let verified = ed25519::verify(msg, pk.as_slice(), sig.as_slice());
        Ok(verified)
    }
}