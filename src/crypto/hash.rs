use base::Result;
use base::FixedSize;
use base::Datable;

pub trait Hashable<P, D>
    where   P: Datable,
            D: Datable + FixedSize,
            Self: Datable
{
    fn digest_cb(&self, params: &P, cb: &Fn(&Self, &P) -> Result<D>) -> Result<D> {
        cb(self, params)
    }

    fn verify_digest_cb(&self, params: &P, digest: &D, cb: &Fn(&Self, &P, &D) -> Result<bool>)
        -> Result<bool>
    {
        cb(self, params, digest)
    }

    fn check_digest_cb(&self, params: &P, digest: &D, cb: &Fn(&Self, &P, &D) -> Result<bool>)
        -> Result<()>
    {
        if !Self::verify_digest_cb(self, params, digest, cb)? {
            return Err(String::from("invalid digest"));
        }

        Ok(())
    }
}