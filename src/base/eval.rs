use base::result::Result;
use base::data::Datable;

pub trait Evaluable
    where   Self: Datable
{
    fn eval_cb<P, R>(&self, params: &P, cb: &Fn(&Self, &P) -> Result<R>) -> Result<R>
        where P: Datable,
              R: Datable
    {
        cb(self, params)
    }
}