//! # Eval
//!
//! `eval` is the module providing the traits implemented by types that can evaluate.

use base::result::Result;
use base::check::Checkable;
use base::data::Datable;

/// Trait implemented by types that can evaluate.
pub trait Eval<T, P, R>
    where   T: 'static + Sized + Send + Sync + Checkable,
            P: Datable,
            R: Datable
{
    /// Evaluates a `Datable` T given some parameters and returning a result.
    fn eval(&self, data: &T, params: &P) -> Result<R>;

}


/// Trait implemented by types that can evaluate mutably.
pub trait EvalMut<T, P, R>
    where   T: 'static + Sized + Send + Sync + Checkable,
            P: Datable,
            R: Datable
{
    /// Evaluates mutably a `Datable` T given some parameters and returning a result.
    fn eval_mut(&mut self, data: &mut T, params: &P) -> Result<R>;
}