use base::Result;
use base::Checkable;
use base::Datable;
use base::Serializable;
use base::{Sizable, FixedSize};
use base::Numerical;
use base::Runnable;
use crypto::{Hashable, Signable};
use models::Meta;
use models::Coin;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Input<D, A, P, Pk, Sig>
    where   D: Datable + FixedSize,
            A: Numerical,
            P: Datable,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize
{
    pub id: D,
    pub meta: Meta,
    pub coins_len: u64,
    pub coins: Vec<Coin<D, A>>,
    pub payload: P,
    pub public_key: Pk,
    pub signature: Sig,
}

impl<D, A, P, Pk, Sig> Input<D, A, P, Pk, Sig>
    where   D: Datable + FixedSize,
            A: Numerical,
            P: Datable,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize
{
    pub fn new() -> Input<D, A, P, Pk, Sig> {
        Input::default()
    }

    pub fn meta(mut self, meta: &Meta) -> Result<Input<D, A, P, Pk, Sig>> {
        meta.check()?;
        self.meta = meta.clone();

        Ok(self)
    }

    pub fn coins(mut self, coins: &Vec<Coin<D, A>>,) -> Result<Input<D, A, P, Pk, Sig>> {
        coins.check()?;

        self.coins_len = coins.len() as u64;
        self.coins = coins.clone();

        Ok(self)
    }

    pub fn payload(mut self, payload: &P) -> Result<Input<D, A, P, Pk, Sig>> {
        payload.check()?;

        self.payload = payload.clone();

        Ok(self)
    }

    pub fn sign<SP, Sk>(mut self,
                        params: &SP,
                        sk: &Sk,
                        pk: &Pk,
                        cb: &Fn(&Self, &SP, &Sk) -> Result<Sig>)
        -> Result<Input<D, A, P, Pk, Sig>>
        where   SP: Datable,
                Sk: Datable + FixedSize
    {
        params.check()?;
        sk.check()?;
        pk.check()?;

        self.signature = self.sign_cb(params, sk, cb)?;
        self.public_key = pk.clone();

        Ok(self)
    }

    pub fn verify_signature<SP, Sk>(&self,
                                    params: &SP,
                                    sig: &Sig,
                                    pk: &Pk,
                                    cb: &Fn(&Self, &SP, &Sig, &Pk) -> Result<bool>)
        -> Result<bool>
        where   SP: Datable,
                Sk: Datable + FixedSize
    {
        params.check()?;
        sig.check()?;
        pk.check()?;

        Signable::<SP, Sk, Pk, Sig>::verify_signature_cb(self, params, sig, pk, cb)
    }

    pub fn check_signature<SP, Sk>(&self,
                                   params: &SP,
                                   sig: &Sig,
                                   pk: &Pk,
                                   cb: &Fn(&Self, &SP, &Sig, &Pk) -> Result<bool>)
        -> Result<()>
        where   SP: Datable,
                Sk: Datable + FixedSize
    {
        params.check()?;
        sig.check()?;
        pk.check()?;

        Signable::<SP, Sk, Pk, Sig>::check_signature_cb(self, params, sig, pk, cb)
    }

    pub fn finalize<HP: Datable>(mut self, params: &HP, cb: &Fn(&Self, &HP) -> Result<D>)
        -> Result<Input<D, A, P, Pk, Sig>>
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

impl<RP, D, A, P, Pk, Sig> Runnable<RP, D> for Input<D, A, P, Pk, Sig>
    where   RP: Datable,
            D: Datable + FixedSize,
            A: Numerical,
            P: Datable,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize
{}

impl<HP, D, A, P, Pk, Sig> Hashable<HP, D> for Input<D, A, P, Pk, Sig>
    where   HP: Datable,
            D: Datable + FixedSize,
            A: Numerical,
            P: Datable,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize
{}

impl<SP, Sk, D, A, P, Pk, Sig> Signable<SP, Sk, Pk, Sig> for Input<D, A, P, Pk, Sig>
    where   SP: Datable,
            Sk: Datable + FixedSize,
            Sig: Datable + FixedSize,
            D: Datable + FixedSize,
            A: Numerical,
            P: Datable,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize
{}

impl<D, A, P, Pk, Sig> Sizable for Input<D, A, P, Pk, Sig>
    where   D: Datable + FixedSize,
            A: Numerical,
            P: Datable,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize
{
    fn size(&self) -> u64 {
        self.id.size() +
            self.meta.size() +
            self.coins_len.size() +
            self.coins.size() +
            self.payload.size() +
            self.public_key.size() +
            self.signature.size()
    }
}

impl<D, A, P, Pk, Sig> Checkable for Input<D, A, P, Pk, Sig>
    where   D: Datable + FixedSize,
            A: Numerical,
            P: Datable,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize
{
    fn check(&self) -> Result<()> {
        self.id.check()?;
        self.id.check_size()?;
        self.meta.check()?;
        
        if self.meta.size != self.size() {
            return Err(String::from("invalid meta size"));
        }
        
        self.coins_len.check()?;
        self.coins.check()?;

        if self.coins.len() != self.coins_len as usize {
            return Err(String::from("invalid coins length"));
        }

        self.payload.check()?;
        self.public_key.check()?;
        self.public_key.check_size()?;
        self.signature.check()?;
        self.signature.check_size()?;

        Ok(())
    }
}

impl<D, A, P, Pk, Sig> Serializable for Input<D, A, P, Pk, Sig>
    where   D: Datable + FixedSize + Serializable,
            A: Numerical + Serializable,
            P: Datable + Serializable,
            Pk: Datable + FixedSize + Serializable,
            Sig: Datable + FixedSize + Serializable
{}

impl<D, A, P, Pk, Sig> Datable for Input<D, A, P, Pk, Sig>
    where   D: Datable + FixedSize,
            A: Numerical,
            P: Datable,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize
{}