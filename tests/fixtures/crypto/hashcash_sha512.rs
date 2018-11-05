#![allow(dead_code)]

use sodiumoxide::init;

use mitrid_core::base::Result;
use mitrid_core::base::Checkable;
use mitrid_core::crypto::Prove;

use std::mem;
use std::u32;

use fixtures::crypto::SHA512;

// Already Datable in mitrid_core
pub type Proof = Option<u32>;

// NB: loosely hashcash
pub struct HashCash {
    pub bits: u32,
}

impl HashCash {
    pub fn prove(msg: &[u8], bits: u32) -> Result<Proof> {
        init().unwrap();

        let base_digest = SHA512::digest(msg)?;
        let mut nonce: u32 = 0;
        let mut found = false;

        while !found {
            let nonce_arr: [u8; 4] = unsafe { mem::transmute(nonce) };
            let mut msg = Vec::new();
            msg.extend_from_slice(&nonce_arr[..]);
            msg.extend_from_slice(base_digest.as_slice());

            let digest = SHA512::digest(&msg)?;
            let _digest = digest.as_slice();
            let mut __digest = [0u8; 4];
            for i in 0..4 {
                __digest[i] = _digest[i];
            }
            
            let leading: u32 = unsafe { mem::transmute(__digest) };
            if leading.leading_zeros() >= bits {
                found = true;
            } else {
                if nonce == u32::MAX {
                    break;
                }

                nonce += 1;
            }
        }

        if !found {
            Ok(None)
        } else {
            Ok(Some(nonce))
        }

    }

    pub fn verify(msg: &[u8], bits: u32, proof: &Proof) -> Result<bool> {
        init().unwrap();

        proof.check()?;

        if let Some(nonce) = proof {
            let nonce_arr: [u8; 4] = unsafe { mem::transmute(*nonce) };

            let base_digest = SHA512::digest(msg)?;
            
            let mut msg = Vec::new();
            msg.extend_from_slice(&nonce_arr[..]);
            msg.extend_from_slice(base_digest.as_slice());

            let digest = SHA512::digest(&msg)?;
            let _digest = digest.as_slice();
            let mut __digest = [0u8; 4];
            for i in 0..4 {
                __digest[i] = _digest[i];
            }

            let leading: u32 = unsafe { mem::transmute(__digest) };

            if leading.leading_zeros() < bits {
                Ok(false)
            } else {
                Ok(true)
            }
        } else {
            return Ok(false);
        }
    }

    pub fn check(msg: &[u8], bits: u32, proof: &Proof) -> Result<()> {
        init().unwrap();

        proof.check()?;

        if !Self::verify(msg, bits, proof)? {
            return Err(format!("invalid proof"));
        }

        Ok(())
    }
}

pub struct HashCash2 {
    pub bits: u32,
}

impl HashCash2 {
    pub fn new(bits: u32) -> HashCash {
        HashCash {
            bits: bits
        }
    }

    pub fn prove(&self, msg: &[u8]) -> Result<Proof> {
        init().unwrap();

        let base_digest = SHA512::digest(msg)?;
        let mut nonce: u32 = 0;
        let mut found = false;
        let bits = self.bits;

        while !found {
            let nonce_arr: [u8; 4] = unsafe { mem::transmute(nonce) };
            let mut msg = Vec::new();
            msg.extend_from_slice(&nonce_arr[..]);
            msg.extend_from_slice(base_digest.as_slice());

            let digest = SHA512::digest(&msg)?;
            let _digest = digest.as_slice();
            let mut __digest = [0u8; 4];
            for i in 0..4 {
                __digest[i] = _digest[i];
            }
            
            let leading: u32 = unsafe { mem::transmute(__digest) };
            if leading.leading_zeros() >= bits {
                found = true;
            } else {
                if nonce == u32::MAX {
                    break;
                }

                nonce += 1;
            }
        }

        if !found {
            Ok(None)
        } else {
            Ok(Some(nonce))
        }

    }

    pub fn verify(&self, msg: &[u8], proof: &Proof) -> Result<bool> {
        init().unwrap();

        proof.check()?;

        let bits = self.bits;

        if let Some(nonce) = proof {
            let nonce_arr: [u8; 4] = unsafe { mem::transmute(*nonce) };

            let base_digest = SHA512::digest(msg)?;
            
            let mut msg = Vec::new();
            msg.extend_from_slice(&nonce_arr[..]);
            msg.extend_from_slice(base_digest.as_slice());

            let digest = SHA512::digest(&msg)?;
            let _digest = digest.as_slice();
            let mut __digest = [0u8; 4];
            for i in 0..4 {
                __digest[i] = _digest[i];
            }

            let leading: u32 = unsafe { mem::transmute(__digest) };

            if leading.leading_zeros() < bits {
                Ok(false)
            } else {
                Ok(true)
            }
        } else {
            return Ok(false);
        }
    }

    pub fn check(&self, msg: &[u8], proof: &Proof) -> Result<()> {
        init().unwrap();

        proof.check()?;

        if !self.verify(msg, proof)? {
            return Err(format!("invalid proof"));
        }

        Ok(())
    }
}

impl Prove<Proof> for HashCash2 {
    fn prove(&mut self, msg: &[u8]) -> Result<Proof> {
        (self as &HashCash2).prove(msg)
    }

    fn verify(&mut self, msg: &[u8], proof: &Proof) -> Result<bool> {
        (self as &HashCash2).verify(msg, proof)
    }

    fn check(&mut self, msg: &[u8], proof: &Proof) -> Result<()> {
       (self as &HashCash2).check(msg, proof)
    }
}