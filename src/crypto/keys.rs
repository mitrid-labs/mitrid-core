use base::Result;
use base::FixedSize;
use base::Datable;

pub trait Key<P, K>
    where   P: Datable,
            K: Datable + FixedSize,
            Self: 'static + Sized
{
    fn generate_key(&self, params: &P, cb: &Fn(&Self, &P) -> Result<K>) -> Result<K> {
        cb(self, params)
    }
}

pub trait KeyPair<P, Sk, Pk>
    where   P: Datable,
            Sk: Datable + FixedSize,
            Pk: Datable + FixedSize,
            Self: 'static + Sized
{
    fn generate_keypair(&self, params: &P, cb: &Fn(&Self, &P) -> Result<(Pk, Sk)>)
        -> Result<(Pk, Sk)>
    {
        cb(self, params)
    }
}