use base::Result;
use base::Checkable;
use base::Datable;
use base::Serializable;
use base::{Sizable, FixedSize};
use base::Numerical;
use base::Runnable;
use crypto::Hashable;
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

impl<RP, D, A, IP, Pk, Sig, OP, TP, P, Pr> Runnable<RP, D> for Block<D, A, IP, Pk, Sig, OP, TP, P, Pr>
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