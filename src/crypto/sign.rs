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
    fn sign_cb(&self, params: &P, sk: &Sk, cb: &Fn(&Self, &P, &Sk) -> Result<Sig>) -> Result<Sig> {
        cb(self, params, sk)
    }

    fn verify_signature_cb(&self,
                           params: &P,
                           sig: &Sig,
                           pk: &Pk,
                           cb: &Fn(&Self, &P, &Sig, &Pk) -> Result<bool>)
        -> Result<bool>
    {
        cb(self, params, sig, pk)
    }

    fn check_signature_cb(&self,
                          params: &P,
                          sig: &Sig,
                          pk: &Pk,
                          cb: &Fn(&Self, &P, &Sig, &Pk) -> Result<bool>)
        -> Result<()>
    {
        if !Self::verify_signature_cb(self, params, sig, pk, cb)? {
            return Err(String::from("invalid signature"));
        }

        Ok(())
    }
}