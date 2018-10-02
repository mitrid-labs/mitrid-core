use base::Result;
use base::FixedSize;
use base::Datable;

pub trait Committable<P, C>
    where   P: Datable,
            C: Datable + FixedSize,
            Self: 'static + Sized
{
    fn commit_cb(&self, params: &P, cb: &Fn(&Self, &P) -> Result<C>) -> Result<C> {
        cb(self, params)
    }

    fn verify_commit_cb(&self, params: &P, commit: &C, cb: &Fn(&Self, &P, &C) -> Result<bool>)
        -> Result<bool>
    {
        cb(self, params, commit)
    }

    fn check_commit_cb(&self, params: &P, commit: &C, cb: &Fn(&Self, &P, &C) -> Result<bool>)
        -> Result<()>
    {
        if !Self::verify_commit_cb(self, params, commit, cb)? {
            return Err(String::from("invalid commit"));
        }

        Ok(())
    }
}