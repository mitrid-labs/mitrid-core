use base::Result;
use base::FixedSize;
use base::Datable;

pub trait Hashable<P, D>
    where   P: Datable,
            D: Datable + FixedSize,
            Self: Datable
{
    fn digest(&self, params: &P, cb: &Fn(&Self, &P) -> Result<D>) -> Result<D> {
        cb(self, params)
    }

    fn verify_digest(&self, params: &P, digest: &D, cb: &Fn(&Self, &P, &D) -> Result<bool>)
        -> Result<bool>
    {
        cb(self, params, digest)
    }
}