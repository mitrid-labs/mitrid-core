use mitrid_core::base::Result;
use mitrid_core::base::Checkable;
use mitrid_core::base::Serializable;
use mitrid_core::models::Wallet as BaseWallet;

use fixtures::base::eval::*;
use fixtures::base::Payload;
use fixtures::crypto::{Digest, SHA512};
use fixtures::crypto::{SecretKey, PublicKey, Signature, Ed25519};
use fixtures::crypto::{Commitment, SHA512Commit};
use fixtures::crypto::{AuthKey, Tag, SHA512HMAC};

pub type Wallet = BaseWallet<Digest, SecretKey, PublicKey, Signature, Payload>;

pub fn wallet_digest_cb(wallet: &Wallet, _: &()) -> Result<Digest> {
    let msg = wallet.to_bytes()?;
    SHA512::digest(&msg)
}

pub fn wallet_verify_digest_cb(wallet: &Wallet, _: &(), digest: &Digest) -> Result<bool> {
    let target = wallet_digest_cb(wallet, &())?;
    
    Ok(&target == digest)
}

pub fn wallet_check_digest_cb(wallet: &Wallet, _: &(), digest: &Digest) -> Result<()> {
    if !wallet_verify_digest_cb(wallet, &(), digest)? {
        return Err("invalid digest".into());
    }

    Ok(())
}

pub fn wallet_sign_cb(wallet: &Wallet, _: &(), sk: &SecretKey) -> Result<Signature> {
    let msg = wallet.to_bytes()?;
    Ed25519::sign(&msg, sk)
}

pub fn wallet_verify_signature_cb(wallet: &Wallet, _: &(), pk: &PublicKey, sig: &Signature) -> Result<bool> {
    let msg = wallet.to_bytes()?;
    Ed25519::verify(&msg, pk, sig)
}

pub fn wallet_check_signature_cb(wallet: &Wallet, _: &(), pk: &PublicKey, sig: &Signature) -> Result<()> {
    if !wallet_verify_signature_cb(wallet, &(), pk, sig)? {
        return Err("invalid signature".into());
    }

    Ok(())
}

pub fn wallet_commit_cb(wallet: &Wallet, _: &()) -> Result<Commitment> {
    let msg = wallet.to_bytes()?;
    SHA512Commit::commit(&msg)
}

pub fn wallet_verify_commitment_cb(wallet: &Wallet, _: &(), commitment: &Commitment) -> Result<bool> {
    let msg = wallet.to_bytes()?;
    SHA512Commit::verify(&msg, commitment)
}

pub fn wallet_check_commitment_cb(wallet: &Wallet, _: &(), commitment: &Commitment) -> Result<()> {
    let msg = wallet.to_bytes()?;
    SHA512Commit::check(&msg, commitment)
}

pub fn wallet_authenticate_cb(wallet: &Wallet, key: &AuthKey) -> Result<Commitment> {
    let msg = wallet.to_bytes()?;

    SHA512HMAC::authenticate(&msg, &key)
}

pub fn wallet_verify_tag_cb(wallet: &Wallet, key: &AuthKey, tag: &Tag) -> Result<bool> {
    let msg = wallet.to_bytes()?;
    SHA512HMAC::verify(&msg, key, tag)
}

pub fn wallet_check_tag_cb(wallet: &Wallet, key: &AuthKey, tag: &Tag) -> Result<()> {
    let msg = wallet.to_bytes()?;
    SHA512HMAC::check(&msg, key, tag)
}

pub fn wallet_eval_cb(wallet: &Wallet, params: &EvalParams) -> Result<EvalReturn> {
    wallet.check()?;
    params.check()?;

    let s = wallet.payload.to_string();

    match params {
        &EvalParams::Const => {
            let res = EvalReturn::Const(s);
            Ok(res)
        },
        &EvalParams::IsEmpty => {
            let res = EvalReturn::IsEmpty(s.is_empty());
            Ok(res)
        },
        &EvalParams::ToUppercase => {
            let res = EvalReturn::ToUppercase(s.to_uppercase());
            Ok(res)
        },
        &EvalParams::ToLowercase => {
            let res = EvalReturn::ToLowercase(s.to_lowercase());
            Ok(res)
        },
    }
}