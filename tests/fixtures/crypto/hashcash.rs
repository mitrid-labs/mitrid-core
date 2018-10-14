use mitrid_core::base::Result;

use std::mem;
use std::u32;

use fixtures::crypto::SHA512;

// Already Datable in mitrid_core
pub type Proof = Option<u32>;

// NB: loosely hashcash
pub struct HashCash {}

impl HashCash {
    pub fn prove(msg: &[u8], bits: u32) -> Result<Proof> {
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
} 