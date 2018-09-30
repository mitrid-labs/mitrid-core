use base::Result;
use base::FixedSize;
use base::Datable;

pub trait Committable<P, C>
    where   P: Datable,
            C: Datable + FixedSize,
            Self: 'static + Sized
{
    fn commit(&self, params: &P, cb: &Fn(&Self, &P) -> Result<C>) -> Result<C> {
        cb(self, params)
    }

    fn verify_commit(&self, params: &P, commit: &C, cb: &Fn(&Self, &P, &C) -> Result<bool>)
        -> Result<bool>
    {
        cb(self, params, commit)
    }
}