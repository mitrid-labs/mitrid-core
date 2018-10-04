//! # Eval
//!
//! `eval` is the module providing the trait implemented by types that can be evaluated (computed).

use base::result::Result;
use base::data::Datable;

/// Trait implemented by types that can be evaluated.
pub trait Evaluable
    where   Self: Datable
{
    /// Takes `Datable` params and a callback to evaluate the implementor.
    fn eval_cb<P, R>(&self, params: &P, cb: &Fn(&Self, &P) -> Result<R>) -> Result<R>
        where P: Datable,
              R: Datable
    {
        cb(self, params)
    }
}