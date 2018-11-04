use mitrid_core::base::Result;
use mitrid_core::base::Serializable;
use mitrid_core::io::Message as BasicMessage;

use fixtures::base::Payload;
use fixtures::crypto::{Digest, SHA512};
use fixtures::crypto::{Commitment, SHA512Commit};
use fixtures::crypto::{AuthKey, Tag, SHA512HMAC};
use fixtures::io::Address;

pub type Message = BasicMessage<(), Address, Payload, Digest, Payload>;

pub fn message_digest_cb(message: &Message, _: &()) -> Result<Digest> {
    let msg = message.to_bytes()?;
    SHA512::digest(&msg)
}

pub fn message_verify_digest_cb(message: &Message, _: &(), digest: &Digest) -> Result<bool> {
    let target = message_digest_cb(message, &())?;
    
    Ok(&target == digest)
}

pub fn message_check_digest_cb(message: &Message, _: &(), digest: &Digest) -> Result<()> {
    if !message_verify_digest_cb(message, &(), digest)? {
        return Err("invalid digest".into());
    }

    Ok(())
}

pub fn message_commit_cb(message: &Message, _: &()) -> Result<Commitment> {
    let msg = message.to_bytes()?;
    SHA512Commit::commit(&msg)
}

pub fn message_verify_commitment_cb(message: &Message, _: &(), commitment: &Commitment) -> Result<bool> {
    let msg = message.to_bytes()?;
    SHA512Commit::verify(&msg, commitment)
}

pub fn message_check_commitment_cb(message: &Message, _: &(), commitment: &Commitment) -> Result<()> {
    let msg = message.to_bytes()?;
    SHA512Commit::check(&msg, commitment)
}

pub fn message_authenticate_cb(message: &Message, key: &AuthKey) -> Result<Commitment> {
    let msg = message.to_bytes()?;
    
    SHA512HMAC::authenticate(&msg, &key)
}

pub fn message_verify_tag_cb(message: &Message, key: &AuthKey, tag: &Tag) -> Result<bool> {
    let msg = message.to_bytes()?;
    SHA512HMAC::verify(&msg, key, tag)
}

pub fn message_check_tag_cb(message: &Message, key: &AuthKey, tag: &Tag) -> Result<()> {
    let msg = message.to_bytes()?;
    SHA512HMAC::check(&msg, key, tag)
}