#![allow(dead_code)]

use sodiumoxide::init;
use sodiumoxide::crypto::auth::hmacsha512::KEYBYTES;
use sodiumoxide::crypto::auth::hmacsha512::Key as _Key;
use sodiumoxide::crypto::auth::hmacsha512::Tag as _Tag;
use sodiumoxide::crypto::auth::hmacsha512::{gen_key, authenticate, verify};

use mitrid_core::base::Result;
use mitrid_core::base::{Sizable, ConstantSize};
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::base::Datable;
use mitrid_core::crypto::Authenticate;

use fixture::crypto::{hash_sha512::DIGEST_SIZE, Digest};

pub const TAG_SIZE: u64 = DIGEST_SIZE;

pub type Tag = Digest;

pub const AUTHKEY_SIZE: u64 = KEYBYTES as u64;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Serialize, Deserialize)]
pub struct AuthKey(Vec<u8>);

impl AuthKey {
    pub fn from_vec(buf: &Vec<u8>) -> Result<AuthKey> {
        if buf.len() != AUTHKEY_SIZE as usize {
            return Err(String::from("invalid length"));
        }

        Ok(AuthKey(buf.to_owned()))
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.clone()
    }

    pub fn from_slice(buf: &[u8]) -> Result<AuthKey> {
        if buf.len() != AUTHKEY_SIZE as usize {
            return Err(String::from("invalid length"));
        }

        Ok(AuthKey(buf.to_owned()))
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl Default for AuthKey {
    fn default() -> AuthKey {
        let mut _ks = Vec::new();
        
        for _ in 0..AUTHKEY_SIZE as usize {
            _ks.push(0);
        }

        AuthKey(_ks)
    }
}

impl Sizable for AuthKey {
    fn size(&self) -> u64 {
        self.0.size()
    }
}

impl ConstantSize for AuthKey {
    fn constant_size() -> u64 {
        AUTHKEY_SIZE
    }
}

impl Checkable for AuthKey {
    fn check(&self) -> Result<()> {
        self.0.check()?;
        self.check_size()
    }
}

impl Serializable for AuthKey {}

impl Datable for AuthKey {}

pub struct SHA512HMAC;

impl SHA512HMAC {
    pub fn genkey() -> Result<AuthKey> {
        init().unwrap();

        let _key = gen_key();
        AuthKey::from_slice(&_key.0[..])
    }

    pub fn authenticate(msg: &[u8], key: &AuthKey) -> Result<Tag> {
        init().unwrap();

        key.check()?;
        key.check_size()?;

        let _key = _Key::from_slice(key.as_slice()).unwrap();
        let _tag = authenticate(msg, &_key);

        Tag::from_slice(&_tag.0[..])
    }

    pub fn verify(msg: &[u8], key: &AuthKey, tag: &Tag) -> Result<bool> {
        init().unwrap();

        key.check()?;
        key.check_size()?;
        tag.check()?;
        tag.check_size()?;

        let _key = _Key::from_slice(key.as_slice()).unwrap();
        let _tag = _Tag::from_slice(tag.as_slice()).unwrap();

        Ok(verify(&_tag, msg, &_key))
    }

    pub fn check(msg: &[u8], key: &AuthKey, tag: &Tag) -> Result<()> {
        init().unwrap();

        key.check()?;
        key.check_size()?;
        tag.check()?;
        tag.check_size()?;

        if !Self::verify(msg, key, tag)? {
            return Err(String::from("invalid tag"));
        }

        Ok(())
    }
}

impl Authenticate<AuthKey, Tag> for SHA512HMAC {
    fn generate_key(&mut self) -> Result<AuthKey> {
        Self::genkey()
    }

    fn authenticate(&mut self, msg: &[u8], key: &AuthKey) -> Result<Tag> {
        Self::authenticate(msg, key)
    }

    fn verify(&mut self, msg: &[u8], key: &AuthKey, tag: &Tag) -> Result<bool> {
        Self::verify(msg, key, tag)
    }

    fn check(&mut self, msg: &[u8], key: &AuthKey, tag: &Tag) -> Result<()> {
        Self::check(msg, key, tag)
    }
}

#[test]
fn test_authkey_from_vec() {
    let mut buf = Vec::new();
    for _ in 0..AUTHKEY_SIZE-1 {
        buf.push(0);
    }

    let res = AuthKey::from_vec(&buf);
    assert!(res.is_err());

    buf.push(0);

    let res = AuthKey::from_vec(&buf);
    assert!(res.is_ok());
}

#[test]
fn test_hmac_sha512() {
    let mut msg = Vec::new();
    for _ in 0..500 {
        msg.push(0);
    }

    let mut auth = SHA512HMAC{};

    let res = auth.generate_key();
    assert!(res.is_ok());

    let key = res.unwrap();

    let res = auth.authenticate(&msg, &key);
    assert!(res.is_ok());

    let tag = res.unwrap();

    let res = auth.verify(&msg, &key, &tag);
    assert!(res.is_ok());
    assert!(res.unwrap());

    let res = auth.check(&msg, &key, &tag);
    assert!(res.is_ok());

    msg.push(0);

    let res = auth.verify(&msg, &key, &tag);
    assert!(res.is_ok());
    assert!(!res.unwrap());

    let res = auth.check(&msg, &key, &tag);
    assert!(res.is_err());
}