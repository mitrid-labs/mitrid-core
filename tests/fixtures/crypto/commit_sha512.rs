use sodiumoxide::init;

use mitrid_core::base::Result;
use mitrid_core::base::ConstantSize;
use mitrid_core::base::Checkable;
use mitrid_core::crypto::Commit;

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

        commitment.check()?;
        commitment.check_size()?;

        SHA512::verify(msg, commitment)
    }

    pub fn check(msg: &[u8], commitment: &Commitment) -> Result<()> {
        init().unwrap();

        commitment.check()?;
        commitment.check_size()?;

        if !Self::verify(msg, commitment)? {
            return Err(String::from("invalid commitment"));
        }

        Ok(())
    }
}

impl Commit<Commitment> for SHA512Commit {
    fn commit(&mut self, msg: &[u8]) -> Result<Commitment> {
        Self::commit(msg)
    }

    fn verify(&mut self, msg: &[u8], commitment: &Commitment) -> Result<bool> {
        Self::verify(msg, commitment)
    }

    fn check(&mut self, msg: &[u8], commitment: &Commitment) -> Result<()> {
        Self::check(msg, commitment)
    }
}

#[test]
fn test_commit_sha512() {
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

#[test]
fn test_commit_sha512_commit() {
    let mut msg = Vec::new();
    for _ in 0..500 {
        msg.push(0);
    }

    let mut commit = SHA512Commit{};

    let res = commit.commit(&msg);
    assert!(res.is_ok());

    let commitment = res.unwrap();

    let res = commit.verify(&msg, &commitment);
    assert!(res.is_ok());
    assert!(res.unwrap());

    let res = commit.check(&msg, &commitment);
    assert!(res.is_ok());

    msg.push(0);

    let res = commit.verify(&msg, &commitment);
    assert!(res.is_ok());
    assert!(!res.unwrap());

    let res = commit.check(&msg, &commitment);
    assert!(res.is_err());
}