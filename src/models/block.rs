//! # Block
//!
//! `block` is the module providing the type used to represent the (non-cryptographical) commitment
//! to one or more `Transaction`s in the `BlockGraph`. Put differently, a `Block` is a bundle of
//! transactions confirmed by one or more nodes.

use base::Result;
use base::Checkable;
use base::Datable;
use base::Serializable;
use base::{Sizable, FixedSize};
use base::Numerical;
use base::Evaluable;
use crypto::{Hashable, Provable};
use models::Meta;
use models::Transaction;
use models::BlockNode;

/// Type used to represent a bundle of confirmed `Transaction`s.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Block<D, A, IP, Pk, Sig, OP, TP, P, Pr>
    where   D: Datable + FixedSize,
            A: Numerical,
            IP: Datable,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize,
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
    pub transactions: Vec<Transaction<D, A, IP, Pk, Sig, OP, TP>>,
    /// Custom payload.
    pub payload: P,
    /// Proof of the block.
    pub proof: Pr,
}

impl<D, A, IP, Pk, Sig, OP, TP, P, Pr> Block<D, A, IP, Pk, Sig, OP, TP, P, Pr>
    where   D: Datable + FixedSize,
            A: Numerical,
            IP: Datable,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize,
            OP: Datable,
            TP: Datable,
            P: Datable,
            Pr: Datable
{
    /// Creates a new `Block`.
    pub fn new() -> Block<D, A, IP, Pk, Sig, OP, TP, P, Pr> {
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
    pub fn meta(mut self, meta: &Meta) -> Result<Block<D, A, IP, Pk, Sig, OP, TP, P, Pr>> {
        meta.check()?;
        self.meta = meta.clone();

        self.update_size();

        Ok(self)
    }

    /// Sets the `Block`s set of previous blocks and its lenght.
    pub fn prev_blocks(mut self, prev_blocks: &Vec<BlockNode<D>>)
        -> Result<Block<D, A, IP, Pk, Sig, OP, TP, P, Pr>>
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
    pub fn transactions(mut self, transactions: &Vec<Transaction<D, A, IP, Pk, Sig, OP, TP>>)
        -> Result<Block<D, A, IP, Pk, Sig, OP, TP, P, Pr>>
    {
        transactions.check()?;

        self.transactions_len = transactions.len() as u64;
        self.transactions = transactions.clone();

        self.update_size();

        Ok(self)
    }

    /// Sets the `Block`'s custom payload.
    pub fn payload(mut self, payload: &P) -> Result<Block<D, A, IP, Pk, Sig, OP, TP, P, Pr>> {
        payload.check()?;

        self.payload = payload.clone();

        self.update_size();

        Ok(self)
    }

    /// Proves cryptographically the `Block`.
    pub fn prove<PrP: Datable>(mut self, params: &PrP, cb: &Fn(&Self, &PrP) -> Result<Pr>)
        -> Result<Block<D, A, IP, Pk, Sig, OP, TP, P, Pr>>
    {
        params.check()?;

        self.proof = self.prove_cb(params, cb)?;

        self.update_size();

        Ok(self)
    }

    /// Verifies the cryptographic proof against the `Block`.
    pub fn verify_proof<PrP: Datable>(&self,
                                      params: &PrP,
                                      cb: &Fn(&Self, &PrP, &Pr) -> Result<bool>)
        -> Result<bool>
    {
        params.check()?;

        let proof = self.proof.clone();
        proof.check()?;

        self.verify_proof_cb(params, &proof, cb)
    }

    /// Checks the cryptographic proof against the `Block`.
    pub fn check_proof<PrP: Datable>(&self,
                                     params: &PrP,
                                     cb: &Fn(&Self, &PrP, &Pr) -> Result<()>)
        -> Result<()>
    {
        params.check()?;

        let proof = self.proof.clone();
        proof.check()?;

        self.check_proof_cb(params, &proof, cb)
    }

    /// Finalizes the `Block`, building its id and returning it's complete form.
    pub fn finalize<HP: Datable>(mut self, params: &HP, cb: &Fn(&Self, &HP) -> Result<D>)
        -> Result<Block<D, A, IP, Pk, Sig, OP, TP, P, Pr>>
    {
        params.check()?;

        self.update_size();

        self.id = self.digest(params, cb)?;

        self.check()?;

        Ok(self)
    }

    /// Hashes cryptographically the `Block`.
    pub fn digest<HP: Datable>(&self, params: &HP, cb: &Fn(&Self, &HP) -> Result<D>)
        -> Result<D>
    {
        params.check()?;

        let mut block = self.clone();
        block.id = D::default();

        block.digest_cb(params, cb)
    }

    /// Verifies the cryptographic digest against the `Block`'s digest.
    pub fn verify_digest<HP: Datable>(&self,
                                      params: &HP,
                                      cb: &Fn(&Self, &HP, &D) -> Result<bool>)
        -> Result<bool>
    {
        params.check()?;

        let digest = self.id.clone();
        digest.check()?;

        let mut block = self.clone();
        block.id = D::default();

        block.verify_digest_cb(params, &digest, cb)
    }

   /// Checks the cryptographic digest against the `Block`'s digest.
    pub fn check_digest<HP: Datable>(&self,
                                     params: &HP,
                                     cb: &Fn(&Self, &HP, &D) -> Result<()>)
        -> Result<()>
    {
        params.check()?;

        let digest = self.id.clone();
        digest.check()?;

        let mut block = self.clone();
        block.id = D::default();

        block.check_digest_cb(params, &digest, cb)
    }

    /// Evals the `Block`.
    pub fn eval<EP, R>(&self, params: &EP, cb: &Fn(&Self, &EP) -> Result<R>)
        -> Result<R>
        where   EP: Datable,
                R: Datable
    {
        params.check()?;

        self.eval_cb(params, cb)
    }
}

impl<HP, D, A, IP, Pk, Sig, OP, TP, P, Pr> Hashable<HP, D> for Block<D, A, IP, Pk, Sig, OP, TP, P, Pr>
    where   HP: Datable,
            D: Datable + FixedSize,
            A: Numerical,
            IP: Datable,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize,
            OP: Datable,
            TP: Datable,
            P: Datable,
            Pr: Datable
{}

impl<PrP, D, A, IP, Pk, Sig, OP, TP, P, Pr> Provable<PrP, Pr> for Block<D, A, IP, Pk, Sig, OP, TP, P, Pr>
    where   PrP: Datable,
            D: Datable + FixedSize,
            A: Numerical,
            IP: Datable,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize,
            OP: Datable,
            TP: Datable,
            P: Datable,
            Pr: Datable
{}

impl<D, A, IP, Pk, Sig, OP, TP, P, Pr> Sizable for Block<D, A, IP, Pk, Sig, OP, TP, P, Pr>
    where   D: Datable + FixedSize,
            A: Numerical,
            IP: Datable,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize,
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

impl<D, A, IP, Pk, Sig, OP, TP, P, Pr> Checkable for Block<D, A, IP, Pk, Sig, OP, TP, P, Pr>
    where   D: Datable + FixedSize,
            A: Numerical,
            IP: Datable,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize,
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

impl<D, A, IP, Pk, Sig, OP, TP, P, Pr> Serializable for Block<D, A, IP, Pk, Sig, OP, TP, P, Pr>
    where   D: Datable + FixedSize + Serializable,
            A: Numerical + Serializable,
            IP: Datable + Serializable,
            Pk: Datable + FixedSize + Serializable,
            Sig: Datable + FixedSize + Serializable,
            OP: Datable + Serializable,
            TP: Datable + Serializable,
            P: Datable + Serializable,
            Pr: Datable + Serializable
{}

impl<D, A, IP, Pk, Sig, OP, TP, P, Pr> Datable for Block<D, A, IP, Pk, Sig, OP, TP, P, Pr>
    where   D: Datable + FixedSize,
            A: Numerical,
            IP: Datable,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize,
            OP: Datable,
            TP: Datable,
            P: Datable,
            Pr: Datable
{}

impl<D, A, IP, Pk, Sig, OP, TP, P, Pr> Evaluable for Block<D, A, IP, Pk, Sig, OP, TP, P, Pr>
    where   D: Datable + FixedSize,
            A: Numerical,
            IP: Datable,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize,
            OP: Datable,
            TP: Datable,
            P: Datable,
            Pr: Datable
{}
