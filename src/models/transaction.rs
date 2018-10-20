//! # Transaction
//!
//! `transaction` is the module providing the type used to produce new `Output`s from one or more input `Transaction`s.

use base::Result;
use base::Checkable;
use base::Datable;
use base::Serializable;
use base::{Sizable, ConstantSize};
use base::Numerical;
use base::Evaluable;
use crypto::{Hashable, Committable, Authenticatable};
use io::Storable;
use io::Network;
use models::Meta;
use models::Input;
use models::Output;

/// Type used to produce one or more `Output`s from one or more `Input`s.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Transaction<D, A, IP, Pk, Sig, OP, P>
    where   D: Datable + ConstantSize,
            A: Numerical,
            IP: Datable,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            OP: Datable,
            P: Datable
{
    /// Transaction id. It is the digest of the same coin, but with a default `D` id.
    pub id: D,
    /// Transaction metadata.
    pub meta: Meta,
    /// Transaction inputs length.
    pub inputs_len: u64,
    /// Transaction inputs.
    pub inputs: Vec<Input<D, A, IP, Pk, Sig>>,
    /// Transaction outputs length.
    pub outputs_len: u64,
    /// Transaction outputs.
    pub outputs: Vec<Output<D, Pk, A, OP>>,
    /// Custom payload.
    pub payload: P,
}

impl<D, A, IP, Pk, Sig, OP, P> Transaction<D, A, IP, Pk, Sig, OP, P>
    where   D: Datable + ConstantSize,
            A: Numerical,
            IP: Datable,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            OP: Datable,
            P: Datable
{
    /// Creates a new `Transaction`.
    pub fn new() -> Transaction<D, A, IP, Pk, Sig, OP, P> {
        let mut tx = Transaction::default();
        tx.update_size();
        tx
    }

    /// Updates the `Transaction` size.
    pub fn update_size(&mut self) {
        let size = self.size();

        self.meta.set_size(size);
    }

    /// Sets the `Transaction`'s metadata.
    pub fn meta(mut self, meta: &Meta) -> Result<Transaction<D, A, IP, Pk, Sig, OP, P>> {
        meta.check()?;
        self.meta = meta.clone();

        self.update_size();

        Ok(self)
    }

    /// Sets the `Transaction`s set of inputs and its lenght.
    pub fn inputs(mut self, inputs: &Vec<Input<D, A, IP, Pk, Sig>>,)
        -> Result<Transaction<D, A, IP, Pk, Sig, OP, P>>
    {
        inputs.check()?;

        self.inputs_len = inputs.len() as u64;
        self.inputs = inputs.clone();

        self.update_size();

        Ok(self)
    }

    /// Sets the `Transaction`s set of outputs and its lenght.
    pub fn outputs(mut self, outputs: &Vec<Output<D, Pk, A, OP>>,)
        -> Result<Transaction<D, A, IP, Pk, Sig, OP, P>>
    {
        outputs.check()?;

        self.outputs_len = outputs.len() as u64;
        self.outputs = outputs.clone();

        self.update_size();

        Ok(self)
    }

    /// Sets the `Transaction`'s custom payload.
    pub fn payload(mut self, payload: &P) -> Result<Transaction<D, A, IP, Pk, Sig, OP, P>> {
        payload.check()?;

        self.payload = payload.clone();

        self.update_size();

        Ok(self)
    }

    /// Finalizes the `Transaction`, building its id and returning it's complete form.
    pub fn finalize<HP: Datable>(mut self, params: &HP, cb: &Fn(&Self, &HP) -> Result<D>)
        -> Result<Transaction<D, A, IP, Pk, Sig, OP, P>>
    {
        params.check()?;

        self.update_size();

        self.id = self.digest(params, cb)?;

        self.check()?;

        Ok(self)
    }

    /// Hashes cryptographically the `Transaction`.
    pub fn digest<HP: Datable>(&self, params: &HP, cb: &Fn(&Self, &HP) -> Result<D>)
        -> Result<D>
    {
        params.check()?;

        let mut tx = self.clone();
        tx.id = D::default();

        tx.digest_cb(params, cb)
    }

    /// Verifies the cryptographic digest against the `Transaction`'s digest.
    pub fn verify_digest<HP: Datable>(&self,
                                      params: &HP,
                                      cb: &Fn(&Self, &HP, &D) -> Result<bool>)
        -> Result<bool>
    {
        params.check()?;

        let digest = self.id.clone();
        digest.check()?;

        let mut tx = self.clone();
        tx.id = D::default();
        tx.update_size();

        tx.verify_digest_cb(params, &digest, cb)
    }

    /// Checks the cryptographic digest against the `Transaction`'s digest.
    pub fn check_digest<HP: Datable>(&self,
                                     params: &HP,
                                     cb: &Fn(&Self, &HP, &D) -> Result<()>)
        -> Result<()>
    {
        params.check()?;

        let digest = self.id.clone();
        digest.check()?;

        let mut tx = self.clone();
        tx.id = D::default();
        tx.update_size();

        tx.check_digest_cb(params, &digest, cb)
    }

    /// Commits cryptographically the `Transaction`.
    pub fn commit<CP, C>(&self, params: &CP, cb: &Fn(&Self, &CP) -> Result<C>)
        -> Result<C>
        where   CP: Datable,
                C: Datable + ConstantSize
    {
        params.check()?;

        self.commit_cb(params, cb)
    }

    /// Verifies the cryptographic commitment against the `Transaction`'s commitment.
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

    /// Checks the cryptographic commitment against the `Transaction`'s commitment.
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

    /// Authenticates cryptographically the `Transaction`.
    pub fn authenticate<AP, T>(&self, params: &AP, cb: &Fn(&Self, &AP) -> Result<T>)
        -> Result<T>
        where   AP: Datable,
                T: Datable + ConstantSize
    {
        params.check()?;

        self.authenticate_cb(params, cb)
    }

    /// Verifies the cryptographic authentication of the `Transaction` against a tag.
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

    /// Checks the cryptographic authentication of the `Transaction` against a tag.
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

    /// Evals the `Transaction`.
    pub fn eval<EP, R>(&self, params: &EP, cb: &Fn(&Self, &EP) -> Result<R>)
        -> Result<R>
        where   EP: Datable,
                R: Datable
    {
        params.check()?;

        self.eval_cb(params, cb)
    }
}

impl<HP, D, A, IP, Pk, Sig, OP, P> Hashable<HP, D> for Transaction<D, A, IP, Pk, Sig, OP, P>
    where   HP: Datable,
            D: Datable + ConstantSize,
            A: Numerical,
            IP: Datable,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            OP: Datable,
            P: Datable
{}

impl<CP, C, D, A, IP, Pk, Sig, OP, P> Committable<CP, C> for Transaction<D, A, IP, Pk, Sig, OP, P>
    where   CP: Datable,
            C: Datable + ConstantSize,
            D: Datable + ConstantSize,
            A: Numerical,
            IP: Datable,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            OP: Datable,
            P: Datable
{}

impl<AP, T, D, A, IP, Pk, Sig, OP, P> Authenticatable<AP, T> for Transaction<D, A, IP, Pk, Sig, OP, P>
    where   AP: Datable,
            T: Datable + ConstantSize,
            D: Datable + ConstantSize,
            A: Numerical,
            IP: Datable,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            OP: Datable,
            P: Datable
{}

impl<D, A, IP, Pk, Sig, OP, P> Sizable for Transaction<D, A, IP, Pk, Sig, OP, P>
    where   D: Datable + ConstantSize,
            A: Numerical,
            IP: Datable,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            OP: Datable,
            P: Datable
{
    fn size(&self) -> u64 {
        self.id.size() +
            self.meta.size() +
            self.inputs_len.size() +
            self.inputs.size() +
            self.outputs_len.size() +
            self.outputs.size() +
            self.payload.size()
    }
}

impl<D, A, IP, Pk, Sig, OP, P> Checkable for Transaction<D, A, IP, Pk, Sig, OP, P>
    where   D: Datable + ConstantSize,
            A: Numerical,
            IP: Datable,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            OP: Datable,
            P: Datable
{
    fn check(&self) -> Result<()> {
        self.id.check()?;
        self.id.check_size()?;
        self.meta.check()?;
        
        if self.meta.get_size() != self.size() {
            return Err(String::from("invalid meta size"));
        }

        self.inputs_len.check()?;
        self.inputs.check()?;

        if self.inputs.len() != self.inputs_len as usize {
            return Err(String::from("invalid inputs length"));
        }

        self.outputs_len.check()?;
        self.outputs.check()?;

        if self.outputs.len() != self.outputs_len as usize {
            return Err(String::from("invalid outputs length"));
        }

        self.payload.check()?;

        Ok(())
    }
}

impl<D, A, IP, Pk, Sig, OP, P> Serializable for Transaction<D, A, IP, Pk, Sig, OP, P>
    where   D: Datable + ConstantSize + Serializable,
            A: Numerical + Serializable,
            IP: Datable + Serializable,
            Pk: Datable + ConstantSize + Serializable,
            Sig: Datable + ConstantSize + Serializable,
            OP: Datable + Serializable,
            P: Datable + Serializable
{}

impl<D, A, IP, Pk, Sig, OP, P> Datable for Transaction<D, A, IP, Pk, Sig, OP, P>
    where   D: Datable + ConstantSize,
            A: Numerical,
            IP: Datable,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            OP: Datable,
            P: Datable
{}

impl<D, A, IP, Pk, Sig, OP, P> Evaluable for Transaction<D, A, IP, Pk, Sig, OP, P>
    where   D: Datable + ConstantSize,
            A: Numerical,
            IP: Datable,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            OP: Datable,
            P: Datable
{}

impl<S, D, A, IP, Pk, Sig, OP, P> Storable<S, D, Transaction<D, A, IP, Pk, Sig, OP, P>>
    for Transaction<D, A, IP, Pk, Sig, OP, P>
    where   S: Datable + Serializable,
            D: Datable + ConstantSize + Serializable,
            A: Numerical + Serializable,
            IP: Datable + Serializable,
            Pk: Datable + ConstantSize + Serializable,
            Sig: Datable + ConstantSize + Serializable,
            OP: Datable + Serializable,
            P: Datable + Serializable
{
    fn store_key(&self) -> Result<D> {
        self.id.check()?;

        Ok(self.id.clone())
    }

    fn store_value(&self) -> Result<Transaction<D, A, IP, Pk, Sig, OP, P>> {
        self.check()?;

        Ok(self.clone())
    }
}

pub type TransactionNetwork<S, NA, NP, D, A, IP, Pk, Sig, OP, P>
    = Network<S, NA, NP, D, Transaction<D, A, IP, Pk, Sig, OP, P>>;