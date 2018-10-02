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
    pub id: D,
    pub meta: Meta,
    pub height: u64,
    pub prev_blocks_len: u64,
    pub prev_blocks: Vec<BlockNode<D>>,
    pub transactions_len: u64,
    pub transactions: Vec<Transaction<D, A, IP, Pk, Sig, OP, TP>>,
    pub payload: P,
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
    pub fn new() -> Block<D, A, IP, Pk, Sig, OP, TP, P, Pr> {
        Block::default()
    }

    pub fn meta(mut self, meta: &Meta) -> Result<Block<D, A, IP, Pk, Sig, OP, TP, P, Pr>> {
        meta.check()?;
        self.meta = meta.clone();

        Ok(self)
    }

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

        Ok(self)
    }

    pub fn transactions(mut self, transactions: &Vec<Transaction<D, A, IP, Pk, Sig, OP, TP>>)
        -> Result<Block<D, A, IP, Pk, Sig, OP, TP, P, Pr>>
    {
        transactions.check()?;

        self.transactions_len = transactions.len() as u64;
        self.transactions = transactions.clone();

        Ok(self)
    }

    pub fn payload(mut self, payload: &P) -> Result<Block<D, A, IP, Pk, Sig, OP, TP, P, Pr>> {
        payload.check()?;

        self.payload = payload.clone();

        Ok(self)
    }

    pub fn prove<PrP: Datable>(mut self, params: &PrP, cb: &Fn(&Self, &PrP) -> Result<Pr>)
        -> Result<Block<D, A, IP, Pk, Sig, OP, TP, P, Pr>>
    {
        params.check()?;

        self.proof = self.prove_cb(params, cb)?;

        Ok(self)
    }

    pub fn verify_proof<PrP: Datable>(&self,
                                      params: &PrP,
                                      proof: &Pr,
                                      cb: &Fn(&Self, &PrP, &Pr) -> Result<bool>)
        -> Result<bool>
    {
        params.check()?;
        proof.check()?;

        self.verify_proof_cb(params, proof, cb)
    }

    pub fn check_proof<PrP: Datable>(&self,
                                     params: &PrP,
                                     proof: &Pr,
                                     cb: &Fn(&Self, &PrP, &Pr) -> Result<bool>)
        -> Result<()>
    {
        params.check()?;
        proof.check()?;

        self.check_proof_cb(params, proof, cb)
    }

    pub fn finalize<HP: Datable>(mut self, params: &HP, cb: &Fn(&Self, &HP) -> Result<D>)
        -> Result<Block<D, A, IP, Pk, Sig, OP, TP, P, Pr>>
    {
        params.check()?;

        self.id = self.digest(params, cb)?;

        self.check()?;

        Ok(self)
    }

    pub fn digest<HP: Datable>(&self, params: &HP, cb: &Fn(&Self, &HP) -> Result<D>)
        -> Result<D>
    {
        params.check()?;

        self.digest_cb(params, cb)
    }

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

    pub fn check_digest<HP: Datable>(&self,
                                     params: &HP,
                                     digest: &D,
                                     cb: &Fn(&Self, &HP, &D) -> Result<bool>)
        -> Result<()>
    {
        params.check()?;
        digest.check()?;

        self.check_digest_cb(params, digest, cb)
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
        
        if self.meta.size != self.size() {
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

impl<RP, D, A, IP, Pk, Sig, OP, TP, P, Pr> Evaluable<RP, D> for Block<D, A, IP, Pk, Sig, OP, TP, P, Pr>
    where   RP: Datable,
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