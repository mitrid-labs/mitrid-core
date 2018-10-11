#![allow(dead_code)]

use sodiumoxide::crypto::sign::{SEEDBYTES, SECRETKEYBYTES, PUBLICKEYBYTES, SIGNATUREBYTES};
use sodiumoxide::crypto::sign::Seed;
use sodiumoxide::crypto::sign::SecretKey as _SecretKey;
use sodiumoxide::crypto::sign::PublicKey as _PublicKey;
use sodiumoxide::crypto::sign::Signature as _Signature;
use sodiumoxide::crypto::sign::{gen_keypair, keypair_from_seed};
use sodiumoxide::crypto::sign::{sign_detached, verify_detached};

use mitrid_core::base::Result;
use mitrid_core::base::{Sizable, FixedSize};
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::base::Datable;

pub const KEYSEED_SIZE: u64 = SEEDBYTES as u64;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Serialize, Deserialize)]
pub struct KeySeed(Vec<u8>);

impl KeySeed {
    pub fn from_vec(buf: &Vec<u8>) -> Result<KeySeed> {
        if buf.len() != KEYSEED_SIZE as usize {
            return Err(String::from("invalid length"));
        }

        Ok(KeySeed(buf.to_owned()))
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.clone()
    }

    pub fn from_slice(buf: &[u8]) -> Result<KeySeed> {
        if buf.len() != KEYSEED_SIZE as usize {
            return Err(String::from("invalid length"));
        }

        Ok(KeySeed(buf.to_owned()))
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl Default for KeySeed {
    fn default() -> KeySeed {
        let mut _ks = Vec::new();
        
        for _ in 0..KEYSEED_SIZE as usize {
            _ks.push(0);
        }

        KeySeed(_ks)
    }
}

pub const SECRETKEY_SIZE: u64 = SECRETKEYBYTES as u64;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Serialize, Deserialize)]
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

impl Default for SecretKey {
    fn default() -> SecretKey {
        let mut _sk = Vec::new();
        
        for _ in 0..SECRETKEY_SIZE as usize {
            _sk.push(0);
        }

        SecretKey(_sk)
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

pub const PUBLICKEY_SIZE: u64 = PUBLICKEYBYTES as u64;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Serialize, Deserialize)]
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

impl Default for PublicKey {
    fn default() -> PublicKey {
        let mut _pk = Vec::new();
        
        for _ in 0..PUBLICKEY_SIZE as usize {
            _pk.push(0);
        }

        PublicKey(_pk)
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

pub const SIGNATURE_SIZE: u64 = SIGNATUREBYTES as u64;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Serialize, Deserialize)]
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

impl Default for Signature {
    fn default() -> Signature {
        let mut _sig = Vec::new();
        
        for _ in 0..SIGNATURE_SIZE as usize {
            _sig.push(0);
        }

        Signature(_sig)
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
    pub fn keypair(seed: &Option<KeySeed>) -> Result<(PublicKey, SecretKey)> {
        let (_pk, _sk) = if let Some(ks) = seed {
            let key_seed = Seed::from_slice(ks.as_slice()).unwrap();
            keypair_from_seed(&key_seed)
        } else {
            gen_keypair()
        };

        let pk = PublicKey::from_slice(&_pk[..])?;
        let sk = SecretKey::from_slice(&_sk[..])?;

        Ok((pk, sk))
    }

    pub fn sign(msg: &[u8], sk: &SecretKey) -> Result<Signature> {
        sk.check()?;

        let _sk = _SecretKey::from_slice(sk.as_slice()).unwrap();

        let _sig = sign_detached(msg, &_sk);
        Signature::from_slice(&_sig[..])
    }

    pub fn verify(msg: &[u8], pk: &PublicKey, sig: &Signature) -> Result<bool> {
        pk.check()?;
        sig.check()?;

        let _pk = _PublicKey::from_slice(pk.as_slice()).unwrap();
        let _sig = _Signature::from_slice(sig.as_slice()).unwrap();

        let verified = verify_detached(&_sig, msg, &_pk);
        Ok(verified)
    }
}