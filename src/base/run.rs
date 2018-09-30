use base::result::Result;
use base::data::Datable;

pub trait Runnable<P, R>
    where   P: Datable,
            R: Datable,
            Self: Datable
{
    fn run(&mut self, params: &P, cb: &mut FnMut(&mut Self, &P) -> Result<R>) -> Result<R> {
        cb(self, params)
    }
}