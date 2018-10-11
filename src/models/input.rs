//! # Input
//!
//! `input` is the module providing the type used to bind as inputs one or more `Coin`s
//! in a `Transaction`.

use base::Result;
use base::Checkable;
use base::Datable;
use base::Serializable;
use base::{Sizable, FixedSize};
use base::Numerical;
use base::Evaluable;
use crypto::{Hashable, Signable};
use models::Meta;
use models::Coin;

/// Type used to bind one or more `Coin`s to a `Transaction` as one of its inputs.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Input<D, A, P, Pk, Sig>
    where   D: Datable + FixedSize,
            A: Numerical,
            P: Datable,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize
{
    /// Input id. It is the digest of the same input, but with a default `D` id.
    pub id: D,
    /// Input metadata.
    pub meta: Meta,
    /// Bound coin.
    pub coin: Coin<D, A>,
    /// Custom payload.
    pub payload: P,
    /// Signature public key.
    pub public_key: Pk,
    /// Signature of the input with a default id.
    pub signature: Sig,
}

impl<D, A, P, Pk, Sig> Input<D, A, P, Pk, Sig>
    where   D: Datable + FixedSize,
            A: Numerical,
            P: Datable,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize
{
    /// Creates a new `Input`.
    pub fn new() -> Input<D, A, P, Pk, Sig> {
        Input::default()
    }

    /// Updates the `Input` size.
    pub fn update_size(&mut self) {
        let size = self.size();

        self.meta.set_size(size);
    }

    /// Sets the `Input`'s metadata.
    pub fn meta(mut self, meta: &Meta) -> Result<Input<D, A, P, Pk, Sig>> {
        meta.check()?;
        self.meta = meta.clone();

        self.update_size();

        Ok(self)
    }

    /// Sets the `Input`s coin.
    pub fn coin(mut self, coin: &Coin<D, A>,) -> Result<Input<D, A, P, Pk, Sig>> {
        coin.check()?;

        self.coin = coin.clone();

        self.update_size();

        Ok(self)
    }

    /// Sets the `Input`'s custom payload.
    pub fn payload(mut self, payload: &P) -> Result<Input<D, A, P, Pk, Sig>> {
        payload.check()?;

        self.payload = payload.clone();

        self.update_size();

        Ok(self)
    }

    /// Signs cryptographically the `Input`.
    pub fn sign<SP, Sk>(mut self,
                        params: &SP,
                        sk: &Sk,
                        pk: &Pk,
                        cb: &Fn(&Self, &SP, &Sk) -> Result<Sig>)
        -> Result<Input<D, A, P, Pk, Sig>>
        where   SP: Datable,
                Sk: Datable + FixedSize
    {
        params.check()?;
        sk.check()?;
        pk.check()?;

        self.signature = self.sign_cb(params, sk, cb)?;
        self.public_key = pk.clone();

        self.update_size();

        Ok(self)
    }

    /// Verifies the cryptographic signature against the `Input`.
    pub fn verify_signature<SP, Sk>(&self,
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

    /// Checks the cryptographic signature against the `Input`.
    pub fn check_signature<SP, Sk>(&self,
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

    /// Finalizes the `Input`, building its id and returning it's complete form.
    pub fn finalize<HP: Datable>(mut self, params: &HP, cb: &Fn(&Self, &HP) -> Result<D>)
        -> Result<Input<D, A, P, Pk, Sig>>
    {
        params.check()?;

        self.update_size();

        self.id = self.digest(params, cb)?;

        self.check()?;

        Ok(self)
    }

    /// Hashes cryptographically the `Input`.
    pub fn digest<HP: Datable>(&self, params: &HP, cb: &Fn(&Self, &HP) -> Result<D>)
        -> Result<D>
    {
        params.check()?;

        self.digest_cb(params, cb)
    }

    /// Verifies the cryptographic digest against the `Input`'s digest.
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

    /// Checks the cryptographic digest against the `Input`'s digest.
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

    /// Evals the `Input`.
    pub fn eval<EP, R>(&self, params: &EP, cb: &Fn(&Self, &EP) -> Result<R>)
        -> Result<R>
        where   EP: Datable,
                R: Datable
    {
        params.check()?;

        self.eval_cb(params, cb)
    }
}

impl<HP, D, A, P, Pk, Sig> Hashable<HP, D> for Input<D, A, P, Pk, Sig>
    where   HP: Datable,
            D: Datable + FixedSize,
            A: Numerical,
            P: Datable,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize
{}

impl<SP, Sk, D, A, P, Pk, Sig> Signable<SP, Sk, Pk, Sig> for Input<D, A, P, Pk, Sig>
    where   SP: Datable,
            Sk: Datable + FixedSize,
            Sig: Datable + FixedSize,
            D: Datable + FixedSize,
            A: Numerical,
            P: Datable,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize
{}

impl<D, A, P, Pk, Sig> Sizable for Input<D, A, P, Pk, Sig>
    where   D: Datable + FixedSize,
            A: Numerical,
            P: Datable,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize
{
    fn size(&self) -> u64 {
        self.id.size() +
            self.meta.size() +
            self.coin.size() +
            self.payload.size() +
            self.public_key.size() +
            self.signature.size()
    }
}

impl<D, A, P, Pk, Sig> Checkable for Input<D, A, P, Pk, Sig>
    where   D: Datable + FixedSize,
            A: Numerical,
            P: Datable,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize
{
    fn check(&self) -> Result<()> {
        self.id.check()?;
        self.id.check_size()?;
        self.meta.check()?;
        
        if self.meta.get_size() != self.size() {
            return Err(String::from("invalid meta size"));
        }
        
        self.coin.check()?;

        self.payload.check()?;
        self.public_key.check()?;
        self.public_key.check_size()?;
        self.signature.check()?;
        self.signature.check_size()?;

        Ok(())
    }
}

impl<D, A, P, Pk, Sig> Serializable for Input<D, A, P, Pk, Sig>
    where   D: Datable + FixedSize + Serializable,
            A: Numerical + Serializable,
            P: Datable + Serializable,
            Pk: Datable + FixedSize + Serializable,
            Sig: Datable + FixedSize + Serializable
{}

impl<D, A, P, Pk, Sig> Datable for Input<D, A, P, Pk, Sig>
    where   D: Datable + FixedSize,
            A: Numerical,
            P: Datable,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize
{}

impl<D, A, P, Pk, Sig> Evaluable for Input<D, A, P, Pk, Sig>
    where   D: Datable + FixedSize,
            A: Numerical,
            P: Datable,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize
{}