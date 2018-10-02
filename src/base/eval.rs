use base::result::Result;
use base::data::Datable;

pub trait Evaluable<P, R>
    where   P: Datable,
            R: Datable,
            Self: Datable
{
    fn eval(&mut self, params: &P, cb: &mut FnMut(&mut Self, &P) -> Result<R>) -> Result<R> {
        cb(self, params)
    }
}