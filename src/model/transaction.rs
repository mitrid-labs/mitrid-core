//! # Transaction
//!
//! `transaction` is the module providing the type used to produce new `Output`s from one or more input `Transaction`s.

use base::Result;
use base::Checkable;
use base::Datable;
use base::Serializable;
use base::{Sizable, ConstantSize};
use base::{Eval, EvalMut};
use base::Numerical;
use base::Meta;
use crypto::Hash;
use io::{Store, Storable};
use model::Input;
use model::Output;

/// Type used to produce one or more `Output`s from one or more `Input`s.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Transaction<D, A, IP, OP, P>
    where   D: Ord + Datable + ConstantSize,
            A: Numerical,
            IP: Datable,
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
    pub inputs: Vec<Input<D, A, IP>>,
    /// Transaction outputs length.
    pub outputs_len: u64,
    /// Transaction outputs.
    pub outputs: Vec<Output<D, A, OP>>,
    /// Custom payload.
    pub payload: P,
}

impl<D, A, IP, OP, P> Transaction<D, A, IP, OP, P>
    where   D: Ord + Datable + ConstantSize,
            A: Numerical,
            IP: Datable,
            OP: Datable,
            P: Datable,
            Self: Serializable
{
    /// Creates a new `Transaction`.
    pub fn new() -> Self {
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
    pub fn meta(mut self, meta: &Meta) -> Result<Self> {
        meta.check()?;
        self.meta = meta.clone();

        self.update_size();

        Ok(self)
    }

    /// Sets the `Transaction`s set of inputs and its lenght.
    pub fn inputs(mut self, inputs: &Vec<Input<D, A, IP>>,) -> Result<Self>
    {
        inputs.check()?;

        self.inputs_len = inputs.len() as u64;
        self.inputs = inputs.clone();

        self.update_size();

        Ok(self)
    }

    /// Sets the `Transaction`s set of outputs and its lenght.
    pub fn outputs(mut self, outputs: &Vec<Output<D, A, OP>>,) -> Result<Self>
    {
        outputs.check()?;

        self.outputs_len = outputs.len() as u64;
        self.outputs = outputs.clone();

        self.update_size();

        Ok(self)
    }

    /// Sets the `Transaction`'s custom payload.
    pub fn payload(mut self, payload: &P) -> Result<Self> {
        payload.check()?;

        self.payload = payload.clone();

        self.update_size();

        Ok(self)
    }

    /// Finalizes the `Transaction`, building its id and returning it's complete form.
    pub fn finalize<H: Hash<D>>(mut self, hasher: &mut H) -> Result<Self> {
        let msg = self.to_bytes()?;
        self.id = hasher.digest(&msg)?;

        self.update_size();

        self.check()?;

        Ok(self)
    }

    /// Hashes cryptographically the `Transaction`.
    pub fn digest<H: Hash<D>>(&self, hasher: &mut H) -> Result<D> {
        let mut transaction = self.clone();
        transaction.id = D::default();
        transaction.update_size();

        let msg = transaction.to_bytes()?;
        hasher.digest(&msg)
    }

    /// Verifies the cryptographic digest against the `Transaction`'s digest.
    pub fn verify_digest<H: Hash<D>>(&self, hasher: &mut H) -> Result<bool> {
        let digest = self.id.clone();
        digest.check()?;

        let mut transaction = self.clone();
        transaction.id = D::default();
        transaction.update_size();

        let msg = transaction.to_bytes()?;
        hasher.verify(&msg, &digest)
    }

    /// Checks the cryptographic digest against the `Transaction`'s digest.
    pub fn check_digest<H: Hash<D>>(&self, hasher: &mut H) -> Result<()> {
        let digest = self.id.clone();
        digest.check()?;

        let mut transaction = self.clone();
        transaction.id = D::default();
        transaction.update_size();

        let msg = transaction.to_bytes()?;
        hasher.check(&msg, &digest)
    }

    /// Evals the `Transaction`.
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

    /// Evals mutably the `Transaction`.
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

impl<D, A, IP, OP, P> Sizable for Transaction<D, A, IP, OP, P>
    where   D: Ord + Datable + ConstantSize,
            A: Numerical,
            IP: Datable,
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

impl<D, A, IP, OP, P> Checkable for Transaction<D, A, IP, OP, P>
    where   D: Ord + Datable + ConstantSize,
            A: Numerical,
            IP: Datable,
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

impl<D, A, IP, OP, P> Serializable for Transaction<D, A, IP, OP, P>
    where   D: Ord + Datable + ConstantSize + Serializable,
            A: Numerical + Serializable,
            IP: Datable + Serializable,
            OP: Datable + Serializable,
            P: Datable + Serializable
{}

impl<D, A, IP, OP, P> Datable for Transaction<D, A, IP, OP, P>
    where   D: Ord + Datable + ConstantSize,
            A: Numerical,
            IP: Datable,
            OP: Datable,
            P: Datable
{}

pub const TRANSACTION_STORE_PREFIX: u64 = 3;

impl<St, S, D, A, IP, OP, P>
    Storable<St, S, D, Transaction<D, A, IP, OP, P>>
    for Transaction<D, A, IP, OP, P>
    where   St: Store<S>,
            S: Datable + Serializable,
            D: Ord + Datable + ConstantSize + Serializable,
            A: Numerical + Serializable,
            IP: Datable + Serializable,
            OP: Datable + Serializable,
            P: Datable + Serializable
{
    fn store_prefix() -> u64 {
        TRANSACTION_STORE_PREFIX
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