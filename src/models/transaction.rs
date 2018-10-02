use base::Result;
use base::Checkable;
use base::Datable;
use base::Serializable;
use base::{Sizable, FixedSize};
use base::Numerical;
use base::Evaluable;
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

impl<D, A, IP, Pk, Sig, OP, P> Transaction<D, A, IP, Pk, Sig, OP, P>
    where   D: Datable + FixedSize,
            A: Numerical,
            IP: Datable,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize,
            OP: Datable,
            P: Datable
{
    pub fn new() -> Transaction<D, A, IP, Pk, Sig, OP, P> {
        Transaction::default()
    }

    pub fn meta(mut self, meta: &Meta) -> Result<Transaction<D, A, IP, Pk, Sig, OP, P>> {
        meta.check()?;
        self.meta = meta.clone();

        Ok(self)
    }

    pub fn inputs(mut self, inputs: &Vec<Input<D, A, IP, Pk, Sig>>,)
        -> Result<Transaction<D, A, IP, Pk, Sig, OP, P>>
    {
        inputs.check()?;

        self.inputs_len = inputs.len() as u64;
        self.inputs = inputs.clone();

        Ok(self)
    }

    pub fn outputs(mut self, outputs: &Vec<Output<D, Pk, A, OP>>,)
        -> Result<Transaction<D, A, IP, Pk, Sig, OP, P>>
    {
        outputs.check()?;

        self.outputs_len = outputs.len() as u64;
        self.outputs = outputs.clone();

        Ok(self)
    }

    pub fn payload(mut self, payload: &P) -> Result<Transaction<D, A, IP, Pk, Sig, OP, P>> {
        payload.check()?;

        self.payload = payload.clone();

        Ok(self)
    }

    pub fn finalize<HP: Datable>(mut self, params: &HP, cb: &Fn(&Self, &HP) -> Result<D>)
        -> Result<Transaction<D, A, IP, Pk, Sig, OP, P>>
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

impl<RP, D, A, IP, Pk, Sig, OP, P> Evaluable<RP, D> for Transaction<D, A, IP, Pk, Sig, OP, P>
    where   RP: Datable,
            D: Datable + FixedSize,
            A: Numerical,
            IP: Datable,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize,
            OP: Datable,
            P: Datable
{}