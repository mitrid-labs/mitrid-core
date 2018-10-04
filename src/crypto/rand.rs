//! # Rand
//!
//! `rand` is the module providing the trait used to implement random generation.

use base::Result;
use base::Datable;

/// Trait implemented by types that can be generated randomly.
pub trait Random
    where   Self: Datable
{
    /// Generates a random value.
    fn generate_cb<P: Datable>(&self, params: &P, cb: &Fn(&Self, &P) -> Result<Self>) -> Result<Self> {
        cb(self, params)
    }

    /// Generates a random value between a range (`from` is included, `to` is excluded).
    fn range_cb<P: Datable>(&self,
                            params: &P,
                            from: &Self,
                            to: &Self,
                            cb: &Fn(&Self, &P, &Self, &Self) -> Result<Self>)
        -> Result<Self>
    {
        cb(self, params, from, to)
    }

    /// Samples a vector of random values picked between a range (`from` is included, `to` is excluded).
    fn sample_cb<P: Datable>(&self,
                             params: &P,
                             from: &Self,
                             to: &Self,
                             cb: &Fn(&Self, &P, &Self, &Self) -> Result<Vec<Self>>)
        -> Result<Vec<Self>>
    {
        cb(self, params, from, to)
    }
}