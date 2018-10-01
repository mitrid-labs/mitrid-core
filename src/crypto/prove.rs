use base::Result;
use base::Datable;

pub trait Provable<P, Pr>
    where   P: Datable,
            Pr: Datable,
            Self: 'static + Sized
{
    fn prove_cb(&self, params: &P, cb: &Fn(&Self, &P) -> Result<Pr>) -> Result<Pr> {
        cb(self, params)
    }

    fn verify_proof_cb(&self, params: &P, proof: &Pr, cb: &Fn(&Self, &P, &Pr) -> Result<bool>)
        -> Result<bool>
    {
        cb(self, params, proof)
    }
}