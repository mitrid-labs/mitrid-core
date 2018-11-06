use sodiumoxide::init;
use sodiumoxide::crypto::sign::{SEEDBYTES, SECRETKEYBYTES, PUBLICKEYBYTES, SIGNATUREBYTES};
use sodiumoxide::crypto::sign::Seed as _Seed;
use sodiumoxide::crypto::sign::SecretKey as _SecretKey;
use sodiumoxide::crypto::sign::PublicKey as _PublicKey;
use sodiumoxide::crypto::sign::Signature as _Signature;
use sodiumoxide::crypto::sign::{gen_keypair, keypair_from_seed};
use sodiumoxide::crypto::sign::{sign_detached, verify_detached};

use mitrid_core::base::Result;
use mitrid_core::base::{Sizable, ConstantSize};
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::base::Datable;
use mitrid_core::crypto::Sign;

pub const SEED_SIZE: u64 = SEEDBYTES as u64;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Serialize, Deserialize)]
pub struct Seed(Vec<u8>);

impl Seed {
    pub fn from_vec(buf: &Vec<u8>) -> Result<Seed> {
        if buf.len() != SEED_SIZE as usize {
            return Err(String::from("invalid length"));
        }

        Ok(Seed(buf.to_owned()))
    }

    pub fn from_slice(buf: &[u8]) -> Result<Seed> {
        if buf.len() != SEED_SIZE as usize {
            return Err(String::from("invalid length"));
        }

        Ok(Seed(buf.to_owned()))
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl Default for Seed {
    fn default() -> Seed {
        let mut _ks = Vec::new();
        
        for _ in 0..SEED_SIZE as usize {
            _ks.push(0);
        }

        Seed(_ks)
    }
}

impl Sizable for Seed {
    fn size(&self) -> u64 {
        self.0.size()
    }
}

impl ConstantSize for Seed {
    fn constant_size() -> u64 {
        SEED_SIZE
    }
}

impl Checkable for Seed {
    fn check(&self) -> Result<()> {
        self.0.check()?;
        self.check_size()
    }
}

impl Serializable for Seed {}

impl Datable for Seed {}

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

impl ConstantSize for SecretKey {
    fn constant_size() -> u64 {
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

impl ConstantSize for PublicKey {
    fn constant_size() -> u64 {
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

impl ConstantSize for Signature {
    fn constant_size() -> u64 {
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
    pub fn keypair(seed: Option<Seed>) -> Result<(PublicKey, SecretKey)> {
        init().unwrap();

        seed.check()?;

        let (_pk, _sk) = if let Some(ref seed) = seed {
            seed.check()?;
            seed.check_size()?;

            let _seed = _Seed::from_slice(seed.as_slice()).unwrap();
            keypair_from_seed(&_seed)
        } else {
            gen_keypair()
        };

        let pk = PublicKey::from_slice(&_pk[..])?;
        let sk = SecretKey::from_slice(&_sk[..])?;

        Ok((pk, sk))
    }

    pub fn sign(msg: &[u8], sk: &SecretKey) -> Result<Signature> {
        init().unwrap();

        sk.check()?;
        sk.check_size()?;

        let _sk = _SecretKey::from_slice(sk.as_slice()).unwrap();

        let _sig = sign_detached(msg, &_sk);
        Signature::from_slice(&_sig[..])
    }

    pub fn verify(msg: &[u8], pk: &PublicKey, sig: &Signature) -> Result<bool> {
        init().unwrap();

        pk.check()?;
        pk.check_size()?;
        sig.check()?;
        sig.check_size()?;

        let _pk = _PublicKey::from_slice(pk.as_slice()).unwrap();
        let _sig = _Signature::from_slice(sig.as_slice()).unwrap();

        let verified = verify_detached(&_sig, msg, &_pk);
        Ok(verified)
    }

    pub fn check(msg: &[u8], pk: &PublicKey, sig: &Signature) -> Result<()> {
        init().unwrap();

        pk.check()?;
        pk.check_size()?;
        sig.check()?;
        sig.check_size()?;

        if !Self::verify(msg, pk, sig)? {
            return Err(String::from("invalid signature"));
        }

        Ok(())
    }
}

impl Sign<Seed, PublicKey, SecretKey, Signature> for Ed25519 {
    fn generate_keys(&mut self, seed: Option<Seed>) -> Result<(PublicKey, SecretKey)> {
        Self::keypair(seed)
    }

    fn sign(&mut self, msg: &[u8], sk: &SecretKey) -> Result<Signature> {
        Self::sign(msg, sk)
    }

    fn verify(&mut self, msg: &[u8], pk: &PublicKey, sig: &Signature) -> Result<bool> {
        Self::verify(msg, pk, sig)
    }

    fn check(&mut self, msg: &[u8], pk: &PublicKey, sig: &Signature) -> Result<()> {
        Self::check(msg, pk, sig)
    }
}

#[test]
fn test_keyseed_from_vec() {
    let mut buf = Vec::new();
    for _ in 0..SEED_SIZE-1 {
        buf.push(0);
    }

    let res = Seed::from_vec(&buf);
    assert!(res.is_err());

    buf.push(0);

    let res = Seed::from_vec(&buf);
    assert!(res.is_ok());
}

#[test]
fn test_digest_from_slice() {
    let mut buf = Vec::new();
    for _ in 0..SEED_SIZE-1 {
        buf.push(0);
    }

    let res = Seed::from_slice(&buf);
    assert!(res.is_err());

    buf.push(0);

    let res = Seed::from_slice(&buf);
    assert!(res.is_ok());
}

#[test]
fn test_secret_key_from_vec() {
    let mut buf = Vec::new();
    for _ in 0..SECRETKEY_SIZE-1 {
        buf.push(0);
    }

    let res = SecretKey::from_vec(&buf);
    assert!(res.is_err());

    buf.push(0);

    let res = SecretKey::from_vec(&buf);
    assert!(res.is_ok());
}

#[test]
fn test_secret_key_from_slice() {
    let mut buf = Vec::new();
    for _ in 0..SECRETKEY_SIZE-1 {
        buf.push(0);
    }

    let res = SecretKey::from_slice(&buf);
    assert!(res.is_err());

    buf.push(0);

    let res = SecretKey::from_slice(&buf);
    assert!(res.is_ok());
}

#[test]
fn test_public_key_from_vec() {
    let mut buf = Vec::new();
    for _ in 0..PUBLICKEY_SIZE-1 {
        buf.push(0);
    }

    let res = PublicKey::from_vec(&buf);
    assert!(res.is_err());

    buf.push(0);

    let res = PublicKey::from_vec(&buf);
    assert!(res.is_ok());
}

#[test]
fn test_public_key_from_slice() {
    let mut buf = Vec::new();
    for _ in 0..PUBLICKEY_SIZE-1 {
        buf.push(0);
    }

    let res = PublicKey::from_slice(&buf);
    assert!(res.is_err());

    buf.push(0);

    let res = PublicKey::from_slice(&buf);
    assert!(res.is_ok());
}

#[test]
fn test_signature_from_vec() {
    let mut buf = Vec::new();
    for _ in 0..SIGNATURE_SIZE-1 {
        buf.push(0);
    }

    let res = Signature::from_vec(&buf);
    assert!(res.is_err());

    buf.push(0);

    let res = Signature::from_vec(&buf);
    assert!(res.is_ok());
}

#[test]
fn test_signature_from_slice() {
    let mut buf = Vec::new();
    for _ in 0..SIGNATURE_SIZE-1 {
        buf.push(0);
    }

    let res = Signature::from_slice(&buf);
    assert!(res.is_err());

    buf.push(0);

    let res = Signature::from_slice(&buf);
    assert!(res.is_ok());
}

#[test]
fn test_sign_ed25519() {
    let mut msg = Vec::new();
    for _ in 0..500 {
        msg.push(0);
    }

    let mut sign = Ed25519{};

    let res = sign.generate_keys(None);
    assert!(res.is_ok());

    let (pk, sk) = res.unwrap();

    let res = sign.sign(&msg, &sk);
    assert!(res.is_ok());

    let sig = res.unwrap();

    let res = sign.verify(&msg, &pk, &sig);
    assert!(res.is_ok());
    assert!(res.unwrap());

    let res = sign.check(&msg, &pk, &sig);
    assert!(res.is_ok());

    msg.push(0);

    let res = sign.verify(&msg, &pk, &sig);
    assert!(res.is_ok());
    assert!(!res.unwrap());

    let res = sign.check(&msg, &pk, &sig);
    assert!(res.is_err());
}