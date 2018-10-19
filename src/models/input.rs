//! # Input
//!
//! `input` is the module providing the type used to bind as inputs one or more `Input`s
//! in a `Transaction`.

use base::Result;
use base::Checkable;
use base::Datable;
use base::Serializable;
use base::{Sizable, ConstantSize, VariableSize};
use base::Numerical;
use base::Evaluable;
use crypto::{Hashable, Signable, Committable, Authenticatable};
use io::Storable;
use io::Networkable;
use models::Meta;
use models::Coin;

/// Type used to bind one or more `Input`s to a `Transaction` as one of its inputs.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Input<D, A, P, Pk, Sig>
    where   D: Datable + ConstantSize,
            A: Numerical,
            P: Datable,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize
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
    where   D: Datable + ConstantSize,
            A: Numerical,
            P: Datable,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize
{
    /// Creates a new `Input`.
    pub fn new() -> Input<D, A, P, Pk, Sig> {
        let mut input = Input::default();
        input.update_size();
        input
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
                Sk: Datable + ConstantSize
    {
        params.check()?;
        sk.check()?;
        pk.check()?;

        self.public_key = pk.clone();
        
        self.signature = self.sign_cb(params, sk, cb)?;

        self.update_size();

        Ok(self)
    }

    /// Verifies the cryptographic signature against the `Input`.
    pub fn verify_signature<SP, Sk>(&self,
                                    params: &SP,
                                    cb: &Fn(&Self, &SP, &Pk, &Sig) -> Result<bool>)
        -> Result<bool>
        where   SP: Datable,
                Sk: Datable + ConstantSize
    {
        params.check()?;

        let pk = self.public_key.clone();
        pk.check()?;

        let sig = self.signature.clone();
        sig.check()?;

        let mut input = self.clone();
        input.signature = Sig::default();
        input.id = D::default();
        input.update_size();

        Signable::<SP, Sk, Pk, Sig>::verify_signature_cb(&input, params, &pk, &sig, cb)
    }

    /// Checks the cryptographic signature against the `Input`.
    pub fn check_signature<SP, Sk>(&self,
                                   params: &SP,
                                   cb: &Fn(&Self, &SP, &Pk, &Sig) -> Result<()>)
        -> Result<()>
        where   SP: Datable,
                Sk: Datable + ConstantSize
    {
        params.check()?;

        let pk = self.public_key.clone();
        pk.check()?;

        let sig = self.signature.clone();
        sig.check()?;

        let mut input = self.clone();
        input.signature = Sig::default();
        input.id = D::default();
        input.update_size();

        Signable::<SP, Sk, Pk, Sig>::check_signature_cb(&input, params, &pk, &sig, cb)
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

        let mut input = self.clone();
        input.id = D::default();

        input.digest_cb(params, cb)
    }

    /// Verifies the cryptographic digest against the `Input`'s digest.
    pub fn verify_digest<HP: Datable>(&self,
                                      params: &HP,
                                      cb: &Fn(&Self, &HP, &D) -> Result<bool>)
        -> Result<bool>
    {
        params.check()?;

        let digest = self.id.clone();
        digest.check()?;

        let mut input = self.clone();
        input.id = D::default();
        input.update_size();

        input.verify_digest_cb(params, &digest, cb)
    }

    /// Checks the cryptographic digest against the `Input`'s digest.
    pub fn check_digest<HP: Datable>(&self,
                                     params: &HP,
                                     cb: &Fn(&Self, &HP, &D) -> Result<()>)
        -> Result<()>
    {
        params.check()?;

        let digest = self.id.clone();
        digest.check()?;

        let mut input = self.clone();
        input.id = D::default();
        input.update_size();

        input.check_digest_cb(params, &digest, cb)
    }

    /// Commits cryptographically the `Input`.
    pub fn commit<CP, C>(&self, params: &CP, cb: &Fn(&Self, &CP) -> Result<C>)
        -> Result<C>
        where   CP: Datable,
                C: Datable + ConstantSize
    {
        params.check()?;

        self.commit_cb(params, cb)
    }

    /// Verifies the cryptographic commitment against the `Input`'s commitment.
    pub fn verify_commitment<CP, C>(&self,
                                    params: &CP,
                                    commitment: &C,
                                    cb: &Fn(&Self, &CP, &C) -> Result<bool>)
        -> Result<bool>
        where   CP: Datable,
                C: Datable + ConstantSize
    {
        params.check()?;
        commitment.check()?;

        self.verify_commitment_cb(params, commitment, cb)
    }

    /// Checks the cryptographic commitment against the `Input`'s commitment.
    pub fn check_commitment<CP, C>(&self,
                                   params: &CP,
                                   commitment: &C,
                                   cb: &Fn(&Self, &CP, &C) -> Result<()>)
        -> Result<()>
        where   CP: Datable,
                C: Datable + ConstantSize
    {
        params.check()?;
        commitment.check()?;

        self.check_commitment_cb(params, commitment, cb)
    }

    /// Authenticates cryptographically the `Input`.
    pub fn authenticate<AP, T>(&self, params: &AP, cb: &Fn(&Self, &AP) -> Result<T>)
        -> Result<T>
        where   AP: Datable,
                T: Datable + ConstantSize
    {
        params.check()?;

        self.authenticate_cb(params, cb)
    }

    /// Verifies the cryptographic authentication of the `Input` against a tag.
    pub fn verify_tag<AP, T>(&self,
                             params: &AP,
                             tag: &T,
                             cb: &Fn(&Self, &AP, &T) -> Result<bool>)
        -> Result<bool>
        where   AP: Datable,
                T: Datable + ConstantSize
    {
        params.check()?;
        tag.check()?;

        self.verify_tag_cb(params, tag, cb)
    }

    /// Checks the cryptographic authentication of the `Input` against a tag.
    pub fn check_tag<AP, T>(&self,
                            params: &AP,
                            tag: &T,
                            cb: &Fn(&Self, &AP, &T) -> Result<()>)
        -> Result<()>
        where   AP: Datable,
                T: Datable + ConstantSize
    {
        params.check()?;
        tag.check()?;

        self.check_tag_cb(params, tag, cb)
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
            D: Datable + ConstantSize,
            A: Numerical,
            P: Datable,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize
{}

impl<SP, Sk, D, A, P, Pk, Sig> Signable<SP, Sk, Pk, Sig> for Input<D, A, P, Pk, Sig>
    where   SP: Datable,
            Sk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            D: Datable + ConstantSize,
            A: Numerical,
            P: Datable,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize
{}

impl<CP, C, D, A, P, Pk, Sig> Committable<CP, C> for Input<D, A, P, Pk, Sig>
    where   CP: Datable,
            C: Datable + ConstantSize,
            D: Datable + ConstantSize,
            A: Numerical,
            P: Datable,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize
{}

impl<AP, T, D, A, P, Pk, Sig> Authenticatable<AP, T> for Input<D, A, P, Pk, Sig>
    where   AP: Datable,
            T: Datable + ConstantSize,
            D: Datable + ConstantSize,
            A: Numerical,
            P: Datable,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize
{}

impl<D, A, P, Pk, Sig> Sizable for Input<D, A, P, Pk, Sig>
    where   D: Datable + ConstantSize,
            A: Numerical,
            P: Datable,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize
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
    where   D: Datable + ConstantSize,
            A: Numerical,
            P: Datable,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize
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
    where   D: Datable + ConstantSize + Serializable,
            A: Numerical + Serializable,
            P: Datable + Serializable,
            Pk: Datable + ConstantSize + Serializable,
            Sig: Datable + ConstantSize + Serializable
{}

impl<D, A, P, Pk, Sig> Datable for Input<D, A, P, Pk, Sig>
    where   D: Datable + ConstantSize,
            A: Numerical,
            P: Datable,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize
{}

impl<D, A, P, Pk, Sig> Evaluable for Input<D, A, P, Pk, Sig>
    where   D: Datable + ConstantSize,
            A: Numerical,
            P: Datable,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize
{}

impl<S, D, A, P, Pk, Sig> Storable<S, D, Input<D, A, P, Pk, Sig>> for Input<D, A, P, Pk, Sig>
    where   S: Datable,
            D: Datable + ConstantSize,
            A: Numerical,
            P: Datable,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize
{}

impl<S, NA, NP, D, A, P, Pk, Sig> Networkable<S, NA, NP, D, Input<D, A, P, Pk, Sig>> for Input<D, A, P, Pk, Sig>
    where   S: Datable,
            NA: Datable + VariableSize,
            NP: Datable,
            D: Datable + ConstantSize,
            A: Numerical,
            P: Datable,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize
{}