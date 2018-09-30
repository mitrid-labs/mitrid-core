use base::Result;
use base::FixedSize;
use base::Datable;

pub trait Authenticated<P, T>
    where   P: Datable,
            T: Datable + FixedSize,
            Self: Datable
{
    fn authenticate(&self, params: &P, cb: &Fn(&Self, &P) -> Result<T>) -> Result<T> {
        cb(self, params)
    }

    fn verify_token(&self, params: &P, token: &T, cb: &Fn(&Self, &P, &T) -> Result<bool>)
        -> Result<bool>
    {
        cb(self, params, token)
    }
}