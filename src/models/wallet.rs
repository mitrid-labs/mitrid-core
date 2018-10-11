//! # Wallet
//!
//! `wallet` is the module providing the type used for wallets (accounts) in the distributed ledger.

use base::Result;
use base::Checkable;
use base::Datable;
use base::Serializable;
use base::{Sizable, FixedSize};
use base::Evaluable;
use crypto::{Hashable, Signable};
use models::Meta;

/// Type used to represent a wallet (account) in the distributed ledger.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Wallet<D, Sk, Pk, Sig, P>
    where   D: Datable + FixedSize,
            Sk: Datable + FixedSize,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize,
            P: Datable
{
    /// Wallet id. It is the digest of the same wallet, but with a default `D` id.
    pub id: D,
    /// Wallet metadata.
    pub meta: Meta,
    /// Wallet secret key.
    pub secret_key: Sk,
    /// Wallet public key.
    pub public_key: Pk,
    /// Custom payload.
    pub payload: P,
    /// Signature of the wallet with a default id.
    pub signature: Sig,
}

impl<D, Sk, Pk, Sig, P> Wallet<D, Sk, Pk, Sig, P>
    where   D: Datable + FixedSize,
            Sk: Datable + FixedSize,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize,
            P: Datable
{
    /// Creates a new `Wallet`.
    pub fn new() -> Wallet<D, Sk, Pk, Sig, P> {
        Wallet::default()
    }

    /// Updates the `Wallet` size.
    pub fn update_size(&mut self) {
        let size = self.size();

        self.meta.set_size(size);
    }

    /// Sets the `Wallet`'s metadata.
    pub fn meta(mut self, meta: &Meta) -> Result<Wallet<D, Sk, Pk, Sig, P>> {
        meta.check()?;
        self.meta = meta.clone();

        self.update_size();

        Ok(self)
    }

    /// Sets the `Wallet`'s custom payload.
    pub fn payload(mut self, payload: &P) -> Result<Wallet<D, Sk, Pk, Sig, P>> {
        payload.check()?;

        self.payload = payload.clone();

        self.update_size();

        Ok(self)
    }

    /// Signs cryptographically the `Wallet`.
    pub fn sign<SP>(mut self,
                    params: &SP,
                    sk: &Sk,
                    pk: &Pk,
                    cb: &Fn(&Self, &SP, &Sk) -> Result<Sig>)
        -> Result<Wallet<D, Sk, Pk, Sig, P>>
        where   SP: Datable
    {
        params.check()?;
        sk.check()?;
        pk.check()?;

        self.signature = self.sign_cb(params, sk, cb)?;
        self.secret_key = sk.clone();
        self.public_key = pk.clone();

        self.update_size();

        Ok(self)
    }

    /// Verifies the cryptographic signature against the `Wallet`.
    pub fn verify_signature<SP>(&self,
                                params: &SP,
                                sig: &Sig,
                                pk: &Pk,
                                cb: &Fn(&Self, &SP, &Sig, &Pk) -> Result<bool>)
        -> Result<bool>
        where   SP: Datable,
                Sk: Datable + FixedSize
    {
        params.check()?;
        sig.check()?;
        pk.check()?;

        Signable::<SP, Sk, Pk, Sig>::verify_signature_cb(self, params, sig, pk, cb)
    }

    /// Checks the cryptographic signature against the `Wallet`.
    pub fn check_signature<SP>(&self,
                               params: &SP,
                               sig: &Sig,
                               pk: &Pk,
                               cb: &Fn(&Self, &SP, &Sig, &Pk) -> Result<bool>)
        -> Result<()>
        where   SP: Datable,
                Sk: Datable + FixedSize
    {
        params.check()?;
        sig.check()?;
        pk.check()?;

        Signable::<SP, Sk, Pk, Sig>::check_signature_cb(self, params, sig, pk, cb)
    }

    /// Finalizes the `Wallet`, building its id and returning it's complete form.
    pub fn finalize<HP: Datable>(mut self, params: &HP, cb: &Fn(&Self, &HP) -> Result<D>)
        -> Result<Wallet<D, Sk, Pk, Sig, P>>
    {
        params.check()?;

        self.update_size();

        self.id = self.digest(params, cb)?;

        self.check()?;

        Ok(self)
    }

    /// Hashes cryptographically the `Wallet`.
    pub fn digest<HP: Datable>(&self, params: &HP, cb: &Fn(&Self, &HP) -> Result<D>)
        -> Result<D>
    {
        params.check()?;

        self.digest_cb(params, cb)
    }

    /// Verifies the cryptographic digest against the `Wallet`'s digest.
    pub fn verify_digest<HP: Datable>(&self,
                                      params: &HP,
                                      digest: &D,
                                      cb: &Fn(&Self, &HP, &D) -> Result<bool>)
        -> Result<bool>
    {
        params.check()?;
        digest.check()?;

        self.verify_digest_cb(params, digest, cb)
    }

    /// Checks the cryptographic digest against the `Wallet`'s digest.
    pub fn check_digest<HP: Datable>(&self,
                                     params: &HP,
                                     digest: &D,
                                     cb: &Fn(&Self, &HP, &D) -> Result<()>)
        -> Result<()>
    {
        params.check()?;
        digest.check()?;

        self.check_digest_cb(params, digest, cb)
    }

    /// Evals the `Wallet`.
    pub fn eval<EP, R>(&self, params: &EP, cb: &Fn(&Self, &EP) -> Result<R>)
        -> Result<R>
        where   EP: Datable,
                R: Datable
    {
        params.check()?;

        self.eval_cb(params, cb)
    }
}

impl<HP, D, Sk, Pk, Sig, P> Hashable<HP, D> for Wallet<D, Sk, Pk, Sig, P>
    where   HP: Datable,
            D: Datable + FixedSize,
            Sk: Datable + FixedSize,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize,
            P: Datable
{}

impl<SP, D, Sk, Pk, Sig, P> Signable<SP, Sk, Pk, Sig> for Wallet<D, Sk, Pk, Sig, P>
    where   SP: Datable,
            D: Datable + FixedSize,
            Sk: Datable + FixedSize,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize,
            P: Datable
{}

impl<D, Sk, Pk, Sig, P> Sizable for Wallet<D, Sk, Pk, Sig, P>
    where   D: Datable + FixedSize,
            Sk: Datable + FixedSize,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize,
            P: Datable
{
    fn size(&self) -> u64 {
        self.id.size() +
            self.meta.size() +
            self.secret_key.size() +
            self.public_key.size() +
            self.payload.size() +
            self.signature.size()
    }
}

impl<D, Sk, Pk, Sig, P> Checkable for Wallet<D, Sk, Pk, Sig, P>
    where   D: Datable + FixedSize,
            Sk: Datable + FixedSize,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize,
            P: Datable
{
    fn check(&self) -> Result<()> {
        self.id.check()?;
        self.id.check_size()?;
        self.meta.check()?;
        
        if self.meta.get_size() != self.size() {
            return Err(String::from("invalid meta size"));
        }
        
        self.secret_key.check()?;
        self.secret_key.check_size()?;
        self.public_key.check()?;
        self.public_key.check_size()?;
        self.payload.check()?;
        self.signature.check()?;
        self.signature.check_size()?;

        Ok(())
    }
}

impl<D, Sk, Pk, Sig, P> Serializable for Wallet<D, Sk, Pk, Sig, P>
    where   D: Datable + FixedSize + Serializable,
            Sk: Datable + FixedSize + Serializable,
            Pk: Datable + FixedSize + Serializable,
            Sig: Datable + FixedSize + Serializable,
            P: Datable + Serializable
{}

impl<D, Sk, Pk, Sig, P> Datable for Wallet<D, Sk, Pk, Sig, P>
    where   D: Datable + FixedSize,
            Sk: Datable + FixedSize,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize,
            P: Datable
{}

impl<D, Sk, Pk, Sig, P> Evaluable for Wallet<D, Sk, Pk, Sig, P>
    where   D: Datable + FixedSize,
            Sk: Datable + FixedSize,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize,
            P: Datable
{}