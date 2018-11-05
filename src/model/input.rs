//! # Input
//!
//! `input` is the module providing the type used to bind as inputs one or more `Input`s
//! in a `Input`.

use base::Result;
use base::Checkable;
use base::Datable;
use base::Serializable;
use base::{Sizable, ConstantSize};
use base::{Eval, EvalMut};
use base::Numerical;
use base::Meta;
use crypto::{Hash, Sign};
use io::{Store, Storable};
use model::Coin;

/// Type used to bind one or more `Input`s to a `Input` as one of its inputs.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Input<D, A, P, Pk, Sig>
    where   D: Ord + Datable + ConstantSize,
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
    where   D: Ord + Datable + ConstantSize,
            A: Numerical,
            P: Datable,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            Self: Serializable
{
    /// Creates a new `Input`.
    pub fn new() -> Self {
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
    pub fn meta(mut self, meta: &Meta) -> Result<Self> {
        meta.check()?;
        self.meta = meta.clone();

        self.update_size();

        Ok(self)
    }

    /// Sets the `Input`s coin.
    pub fn coin(mut self, coin: &Coin<D, A>,) -> Result<Self> {
        coin.check()?;

        self.coin = coin.clone();

        self.update_size();

        Ok(self)
    }

    /// Sets the `Input`'s custom payload.
    pub fn payload(mut self, payload: &P) -> Result<Self> {
        payload.check()?;

        self.payload = payload.clone();

        self.update_size();

        Ok(self)
    }

    /// Signs cryptographically the `Input`.
    pub fn sign<Sgnr, Seed, Sk>(mut self, sk: &Sk, pk: &Pk, signer: &mut Sgnr) -> Result<Self>
        where   Sgnr: Sign<Seed, Pk, Sk, Sig>,
                Seed: Datable + ConstantSize,
                Sk: Datable + ConstantSize,
    {
        sk.check()?;
        pk.check()?;

        self.public_key = pk.clone();
        
        let mut input = self.clone();
        input.signature = Sig::default();
        input.id = D::default();

        let msg = input.to_bytes()?;
        self.signature = signer.sign(&msg, sk)?;

        self.update_size();

        Ok(self)
    }

    /// Verifies the cryptographic signature against the `Input`.
    pub fn verify_signature<Sgnr, Seed, Sk>(&self, signer: &mut Sgnr) -> Result<bool>
        where   Sgnr: Sign<Seed, Pk, Sk, Sig>,
                Seed: Datable + ConstantSize,
                Sk: Datable + ConstantSize,
    {
        let pk = self.public_key.clone();
        pk.check()?;

        let sig = self.signature.clone();
        sig.check()?;

        let mut input = self.clone();
        input.signature = Sig::default();
        input.id = D::default();
        input.update_size();

        let msg = input.to_bytes()?;

        signer.verify(&msg, &pk, &sig)
    }

    /// Checks the cryptographic signature against the `Input`.
    pub fn check_signature<Sgnr, Seed, Sk>(&self, signer: &mut Sgnr) -> Result<()>
        where   Sgnr: Sign<Seed, Pk, Sk, Sig>,
                Seed: Datable + ConstantSize,
                Sk: Datable + ConstantSize,
    {
        let pk = self.public_key.clone();
        pk.check()?;

        let sig = self.signature.clone();
        sig.check()?;

        let mut input = self.clone();
        input.signature = Sig::default();
        input.id = D::default();
        input.update_size();

        let msg = input.to_bytes()?;

        signer.check(&msg, &pk, &sig)
    }

    /// Finalizes the `Input`, building its id and returning it's complete form.
    pub fn finalize<H: Hash<D>>(mut self, hasher: &mut H) -> Result<Self> {
        let msg = self.to_bytes()?;
        self.id = hasher.digest(&msg)?;

        self.update_size();

        self.check()?;

        Ok(self)
    }

    /// Hashes cryptographically the `Input`.
    pub fn digest<H: Hash<D>>(&self, hasher: &mut H) -> Result<D> {
        let mut input = self.clone();
        input.id = D::default();
        input.update_size();

        let msg = input.to_bytes()?;
        hasher.digest(&msg)
    }

    /// Verifies the cryptographic digest against the `Input`'s digest.
    pub fn verify_digest<H: Hash<D>>(&self, hasher: &mut H) -> Result<bool> {
        let digest = self.id.clone();
        digest.check()?;

        let mut input = self.clone();
        input.id = D::default();
        input.update_size();

        let msg = input.to_bytes()?;
        hasher.verify(&msg, &digest)
    }

    /// Checks the cryptographic digest against the `Input`'s digest.
    pub fn check_digest<H: Hash<D>>(&self, hasher: &mut H) -> Result<()> {
        let digest = self.id.clone();
        digest.check()?;

        let mut input = self.clone();
        input.id = D::default();
        input.update_size();

        let msg = input.to_bytes()?;
        hasher.check(&msg, &digest)
    }

    /// Evals the `Input`.
    pub fn eval<Ev, EP, ER>(&self, params: &EP, evaluator: &Ev)
        -> Result<ER>
        where   Ev: Eval<Self, EP, ER>,
                EP: Datable,
                ER: Datable
    {
        self.check()?;
        params.check()?;

        evaluator.eval(self, params)
    }

    /// Evals mutably the `Input`.
    pub fn eval_mut<EvM, EP, ER>(&mut self, params: &EP, evaluator: &mut EvM)
        -> Result<ER>
        where   EvM: EvalMut<Self, EP, ER>,
                EP: Datable,
                ER: Datable
    {
        self.check()?;
        params.check()?;

        let result = evaluator.eval_mut(self, params)?;
        self.update_size();

        self.check()?;

        Ok(result)
    }
}

impl<D, A, P, Pk, Sig> Sizable for Input<D, A, P, Pk, Sig>
    where   D: Ord + Datable + ConstantSize,
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
    where   D: Ord + Datable + ConstantSize,
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
    where   D: Ord + Datable + ConstantSize + Serializable,
            A: Numerical + Serializable,
            P: Datable + Serializable,
            Pk: Datable + ConstantSize + Serializable,
            Sig: Datable + ConstantSize + Serializable
{}

impl<D, A, P, Pk, Sig> Datable for Input<D, A, P, Pk, Sig>
    where   D: Ord + Datable + ConstantSize,
            A: Numerical,
            P: Datable,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize
{}

pub const INPUT_STORE_PREFIX: u64 = 1;

impl<St, S, D, A, P, Pk, Sig>
    Storable<St, S, D, Input<D, A, P, Pk, Sig>>
    for Input<D, A, P, Pk, Sig>
    where   St: Store<S>,
            S: Datable + Serializable,
            D: Ord + Datable + ConstantSize + Serializable,
            A: Numerical + Serializable,
            P: Datable + Serializable,
            Pk: Datable + ConstantSize + Serializable,
            Sig: Datable + ConstantSize + Serializable
{
    fn store_prefix() -> u64 {
        INPUT_STORE_PREFIX
    }

    fn store_key(&self) -> Result<D> {
        self.id.check()?;

        Ok(self.id.clone())
    }

    fn store_value(&self) -> Result<Self> {
        self.check()?;

        Ok(self.clone())
    }
}