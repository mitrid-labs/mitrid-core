#![allow(dead_code)]

use sodiumoxide::init;

use mitrid_core::base::Result;

use fixtures::crypto::{Digest, SHA512};

pub type Commitment = Digest;

pub struct SHA512Commit;

impl SHA512Commit {
    pub fn commit(msg: &[u8]) -> Result<Commitment> {
        init().unwrap();

        SHA512::digest(msg)
    }

    pub fn verify(msg: &[u8], commitment: &Commitment) -> Result<bool> {
        init().unwrap();

        SHA512::verify(msg, commitment)
    }

    pub fn check(msg: &[u8], commitment: &Commitment) -> Result<()> {
        init().unwrap();

        if !Self::verify(msg, commitment)? {
            return Err(String::from("invalid commitment"));
        }

        Ok(())
    }
}

#[test]
fn test_sha512_commit() {
    let mut msg = Vec::new();
    for _ in 0..500 {
        msg.push(0);
    }

    let res = SHA512Commit::commit(&msg);
    assert!(res.is_ok());

    let commitment = res.unwrap();

    let res = SHA512Commit::verify(&msg, &commitment);
    assert!(res.is_ok());
    assert!(res.unwrap());

    let res = SHA512Commit::check(&msg, &commitment);
    assert!(res.is_ok());

    msg.push(0);

    let res = SHA512Commit::verify(&msg, &commitment);
    assert!(res.is_ok());
    assert!(!res.unwrap());

    let res = SHA512Commit::check(&msg, &commitment);
    assert!(res.is_err());
}