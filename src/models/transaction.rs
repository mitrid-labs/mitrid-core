use base::Result;
use base::Checkable;
use base::Datable;
use base::Serializable;
use base::{Sizable, FixedSize};
use base::Numerical;
use base::Runnable;
use crypto::Hashable;
use models::Meta;
use models::Input;
use models::Output;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Transaction<D, A, IP, Pk, Sig, OP, P>
    where   D: Datable + FixedSize,
            A: Numerical,
            IP: Datable,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize,
            OP: Datable,
            P: Datable
{
    pub id: D,
    pub meta: Meta,
    pub inputs_len: u64,
    pub inputs: Vec<Input<D, A, IP, Pk, Sig>>,
    pub outputs_len: u64,
    pub outputs: Vec<Output<D, Pk, A, OP>>,
    pub payload: P,
}

impl<RP, D, A, IP, Pk, Sig, OP, P> Runnable<RP, D> for Transaction<D, A, IP, Pk, Sig, OP, P>
    where   RP: Datable,
            D: Datable + FixedSize,
            A: Numerical,
            IP: Datable,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize,
            OP: Datable,
            P: Datable
{}

impl<HP, D, A, IP, Pk, Sig, OP, P> Hashable<HP, D> for Transaction<D, A, IP, Pk, Sig, OP, P>
    where   HP: Datable,
            D: Datable + FixedSize,
            A: Numerical,
            IP: Datable,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize,
            OP: Datable,
            P: Datable
{}

impl<D, A, IP, Pk, Sig, OP, P> Sizable for Transaction<D, A, IP, Pk, Sig, OP, P>
    where   D: Datable + FixedSize,
            A: Numerical,
            IP: Datable,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize,
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
    where   D: Datable + FixedSize,
            A: Numerical,
            IP: Datable,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize,
            OP: Datable,
            P: Datable
{
    fn check(&self) -> Result<()> {
        self.id.check()?;
        self.id.check_size()?;
        self.meta.check()?;
        
        if self.meta.size != self.size() {
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
    where   D: Datable + FixedSize + Serializable,
            A: Numerical + Serializable,
            IP: Datable + Serializable,
            Pk: Datable + FixedSize + Serializable,
            Sig: Datable + FixedSize + Serializable,
            OP: Datable + Serializable,
            P: Datable + Serializable
{}

impl<D, A, IP, Pk, Sig, OP, P> Datable for Transaction<D, A, IP, Pk, Sig, OP, P>
    where   D: Datable + FixedSize,
            A: Numerical,
            IP: Datable,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize,
            OP: Datable,
            P: Datable
{}