use base::Result;
use base::Datable;
use base::FixedSize;

pub trait Signable<P, Sk, Pk, Sig>
    where   P: Datable,
            Sk: Datable + FixedSize,
            Pk: Datable + FixedSize,
            Sig: Datable + FixedSize,
            Self: 'static + Sized
{
    fn sign(&self, params: &P, sk: &Sk, cb: &Fn(&Self, &P, &Sk) -> Result<Sig>) -> Result<Sig> {
        cb(self, params, sk)
    }

    fn verify_signature(&self,
                        params: &P,
                        sig: &Sig,
                        pk: &Pk,
                        cb: &Fn(&Self, &P, &Sig, &Pk) -> Result<bool>)
        -> Result<bool>
    {
        cb(self, params, sig, pk)
    }
}