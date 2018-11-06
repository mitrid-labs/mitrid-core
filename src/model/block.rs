//! # Block
//!
//! `block` is the module providing the type used to represent the (non-cryptographical) commitment
//! to one or more `Transaction`s in the `BlockGraph`. Put differently, a `Block` is a bundle of
//! transactions confirmed by one or more nodes.

use base::Result;
use base::Checkable;
use base::Datable;
use base::Serializable;
use base::{Sizable, ConstantSize};
use base::Numerical;
use base::{Eval, EvalMut};
use base::Meta;
use crypto::{Hash, Prove};
use io::{Store, Storable};
use model::Transaction;
use model::BlockNode;

/// Type used to represent a bundle of confirmed `Transaction`s.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Block<D, A, IP, OP, TP, P, Pr>
    where   D: Ord + Datable + ConstantSize,
            A: Numerical,
            IP: Datable,
            OP: Datable,
            TP: Datable,
            P: Datable,
            Pr: Datable
{
    /// Block id. It is the digest of the same coin, but with a default `D` id.
    pub id: D,
    /// Block metadata.
    pub meta: Meta,
    /// Block's height.
    pub height: u64,
    /// Previous blocks length.
    pub prev_blocks_len: u64,
    /// Previous blocks.
    pub prev_blocks: Vec<BlockNode<D>>,
    /// Block's transactions length.
    pub transactions_len: u64,
    /// Block's transactions.
    pub transactions: Vec<Transaction<D, A, IP, OP, TP>>,
    /// Custom payload.
    pub payload: P,
    /// Proof of the block.
    pub proof: Pr,
}

impl<D, A, IP, OP, TP, P, Pr> Block<D, A, IP, OP, TP, P, Pr>
    where   D: Ord + Datable + ConstantSize,
            A: Numerical,
            IP: Datable,
            OP: Datable,
            TP: Datable,
            P: Datable,
            Pr: Datable,
            Self: Serializable
{
    /// Creates a new `Block`.
    pub fn new() -> Self {
        let mut block = Block::default();
        block.update_size();
        block
    }

    /// Updates the `Block` size.
    pub fn update_size(&mut self) {
        let size = self.size();

        self.meta.set_size(size);
    }

    /// Sets the `Block`'s metadata.
    pub fn meta(mut self, meta: &Meta) -> Result<Self> {
        meta.check()?;
        self.meta = meta.clone();

        self.update_size();

        Ok(self)
    }

    /// Sets the `Block`s set of previous blocks and its lenght.
    pub fn prev_blocks(mut self, prev_blocks: &Vec<BlockNode<D>>) -> Result<Self>
    {
        prev_blocks.check()?;

        let mut prev_height = 0;

        for prev_block in prev_blocks.clone() {
            if prev_block.block_height > prev_height {
                prev_height = prev_block.block_height;
            }
        }

        self.height = prev_height + 1;
        self.prev_blocks_len = prev_blocks.len() as u64;
        self.prev_blocks = prev_blocks.clone();

        self.update_size();

        Ok(self)
    }

    /// Sets the `Block`s set of transactions and its lenght.
    pub fn transactions(mut self, transactions: &Vec<Transaction<D, A, IP, OP, TP>>) -> Result<Self>
    {
        transactions.check()?;

        self.transactions_len = transactions.len() as u64;
        self.transactions = transactions.clone();

        self.update_size();

        Ok(self)
    }

    /// Sets the `Block`'s custom payload.
    pub fn payload(mut self, payload: &P) -> Result<Self> {
        payload.check()?;

        self.payload = payload.clone();

        self.update_size();

        Ok(self)
    }

    /// Proves cryptographically the `Block`.
    pub fn prove<Prv: Prove<Pr>>(mut self, prover: &mut Prv) -> Result<Self> {
        let mut block = self.clone();
        block.proof = Pr::default();
        block.id = D::default();
        block.update_size();

        let msg = block.to_bytes()?;
        self.proof = prover.prove(&msg)?;

        self.update_size();

        Ok(self)
    }

    /// Verifies the cryptographic proof against the `Block`.
    pub fn verify_proof<Prv: Prove<Pr>>(&self, prover: &mut Prv) -> Result<bool> {
        let proof = self.proof.clone();
        proof.check()?;

        let mut block = self.clone();
        block.proof = Pr::default();
        block.id = D::default();
        block.update_size();

        let msg = block.to_bytes()?;
        prover.verify(&msg, &proof)
    }

    /// Checks the cryptographic proof against the `Block`.
    pub fn check_proof<Prv: Prove<Pr>>(&self, prover: &mut Prv) -> Result<()> {
        let proof = self.proof.clone();
        proof.check()?;

        let mut block = self.clone();
        block.proof = Pr::default();
        block.id = D::default();
        block.update_size();

        let msg = block.to_bytes()?;
        prover.check(&msg, &proof)
    }

    /// Finalizes the `Block`, building its id and returning it's complete form.
    pub fn finalize<H: Hash<D>>(mut self, hasher: &mut H) -> Result<Self> {
        let msg = self.to_bytes()?;
        self.id = hasher.digest(&msg)?;

        self.update_size();

        self.check()?;

        Ok(self)
    }

    /// Hashes cryptographically the `Block`.
    pub fn digest<H: Hash<D>>(&self, hasher: &mut H) -> Result<D> {
        let mut block = self.clone();
        block.id = D::default();
        block.update_size();

        let msg = block.to_bytes()?;
        hasher.digest(&msg)
    }

    /// Verifies the cryptographic digest against the `Block`'s digest.
    pub fn verify_digest<H: Hash<D>>(&self, hasher: &mut H) -> Result<bool> {
        let digest = self.id.clone();
        digest.check()?;

        let mut block = self.clone();
        block.id = D::default();
        block.update_size();

        let msg = block.to_bytes()?;
        hasher.verify(&msg, &digest)
    }

    /// Checks the cryptographic digest against the `Block`'s digest.
    pub fn check_digest<H: Hash<D>>(&self, hasher: &mut H) -> Result<()> {
        let digest = self.id.clone();
        digest.check()?;

        let mut block = self.clone();
        block.id = D::default();
        block.update_size();

        let msg = block.to_bytes()?;
        hasher.check(&msg, &digest)
    }

    /// Evals the `Block`.
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

    /// Evals mutably the `Block`.
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

impl<D, A, IP, OP, TP, P, Pr> Sizable for Block<D, A, IP, OP, TP, P, Pr>
    where   D: Ord + Datable + ConstantSize,
            A: Numerical,
            IP: Datable,
            OP: Datable,
            TP: Datable,
            P: Datable,
            Pr: Datable
{
    fn size(&self) -> u64 {
        self.id.size() +
            self.meta.size() +
            self.height.size() +
            self.prev_blocks_len.size() +
            self.prev_blocks.size() +
            self.transactions_len.size() +
            self.transactions.size() +
            self.payload.size() +
            self.proof.size()
    }
}

impl<D, A, IP, OP, TP, P, Pr> Checkable for Block<D, A, IP, OP, TP, P, Pr>
    where   D: Ord + Datable + ConstantSize,
            A: Numerical,
            IP: Datable,
            OP: Datable,
            TP: Datable,
            P: Datable,
            Pr: Datable
{
    fn check(&self) -> Result<()> {
        self.id.check()?;
        self.id.check_size()?;
        self.meta.check()?;

        if self.meta.get_size() != self.size() {
            return Err(String::from("invalid meta size"));
        }

        self.height.check()?;
        self.prev_blocks_len.check()?;
        self.prev_blocks.check()?;

        if self.prev_blocks.len() != self.prev_blocks_len as usize {
            return Err(String::from("invalid previous blocks length"));
        }

        self.transactions_len.check()?;
        self.transactions.check()?;

        if self.transactions.len() != self.transactions_len as usize {
            return Err(String::from("invalid transactions length"));
        }

        self.payload.check()?;
        self.proof.check()?;

        Ok(())
    }
}

impl<D, A, IP, OP, TP, P, Pr> Serializable for Block<D, A, IP, OP, TP, P, Pr>
    where   D: Ord + Datable + ConstantSize + Serializable,
            A: Numerical + Serializable,
            IP: Datable + Serializable,
            OP: Datable + Serializable,
            TP: Datable + Serializable,
            P: Datable + Serializable,
            Pr: Datable + Serializable
{}

impl<D, A, IP, OP, TP, P, Pr> Datable for Block<D, A, IP, OP, TP, P, Pr>
    where   D: Ord + Datable + ConstantSize,
            A: Numerical,
            IP: Datable,
            OP: Datable,
            TP: Datable,
            P: Datable,
            Pr: Datable
{}

pub const BLOCK_STORE_PREFIX: u64 = 5;

impl<St, S, D, A, IP, OP, TP, P, Pr>
    Storable<St, S, D, Block<D, A, IP, OP, TP, P, Pr>>
    for Block<D, A, IP, OP, TP, P, Pr>
    where   St: Store<S>,
            S: Datable + Serializable,
            D: Ord + Datable + ConstantSize + Serializable,
            A: Numerical + Serializable,
            IP: Datable + Serializable,
            OP: Datable + Serializable,
            TP: Datable + Serializable,
            P: Datable + Serializable,
            Pr: Datable + Serializable
{
    fn store_prefix() -> u64 {
        BLOCK_STORE_PREFIX
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