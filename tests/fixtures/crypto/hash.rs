#![allow(dead_code)]
#![allow(unused_variables)]

use mitrid_core::base::Result;
use mitrid_core::base::{Sizable, FixedSize};
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::base::Datable;
use mitrid_core::crypto::Hashable;

pub const DIGEST_SIZE: u64 = 64;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Digest(Vec<u8>);

impl Digest {
    pub fn from_vec(buf: &Vec<u8>) -> Digest {
        Digest(buf.to_owned())
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.clone()
    }

    pub fn from_slice(buf: &[u8]) -> Digest {
        Digest(buf.to_owned())
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

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash, Serialize, Deserialize)]
pub struct SHA512 {}

impl SHA512 {
    pub fn new() -> SHA512 {
        unreachable!()
    }

    pub fn update(&mut self, msg: &[u8]) {
        unreachable!()
    }

    pub fn digest(&self) -> Result<Digest> {
        unreachable!()
    }

    pub fn verify_digest(&self) -> Result<bool> {
        unreachable!()
    }

    pub fn check_digest(&self) -> Result<bool> {
        unreachable!()
    }
}

impl Sizable for SHA512 {
    fn size(&self) -> u64 {
        0
    }
}

impl Checkable for SHA512 {}

impl Serializable for SHA512 {}

impl Datable for SHA512 {}

impl Hashable<(), Digest> for SHA512 {}