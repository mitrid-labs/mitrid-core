//! # Wallet
//!
//! `wallet` is the module providing the type used for wallets (accounts) in the distributed ledger.

use base::Result;
use base::Checkable;
use base::Datable;
use base::Serializable;
use base::{Sizable, ConstantSize};
use base::Evaluable;
use crypto::{Hashable, Signable, Committable, Authenticatable};
use io::Storable;
use models::Meta;

/// Type used to represent a wallet (account) in the distributed ledger.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Wallet<D, Sk, Pk, Sig, P>
    where   D: Datable + ConstantSize,
            Sk: Datable + ConstantSize,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            P: Datable
{
    /// Wallet id. It is the digest of the same wallet, but with a default `D` id.
    pub id: D,
    /// Wallet metadata.
    pub meta: Meta,
    /// Wallet secret key, if known.
    pub secret_key: Option<Sk>,
    /// Wallet public key.
    pub public_key: Pk,
    /// Custom payload.
    pub payload: P,
    /// Signature of the wallet with a default id.
    pub signature: Sig,
}

impl<D, Sk, Pk, Sig, P> Wallet<D, Sk, Pk, Sig, P>
    where   D: Datable + ConstantSize,
            Sk: Datable + ConstantSize,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            P: Datable
{
    /// Creates a new `Wallet`.
    pub fn new() -> Wallet<D, Sk, Pk, Sig, P> {
        let mut wallet = Wallet::default();
        wallet.update_size();
        wallet
    }

    /// Updates the `Wallet` size.
    pub fn update_size(&mut self) {
        let size = self.size();

        self.meta.set_size(size);
    }

    /// Sets the `Wallet`'s metadata.
    pub fn meta(mut self, meta: &Meta) -> Result<Wallet<D, Sk, Pk, Sig, P>> {
        meta.check()?;
        self.meta = meta.clone();

        self.update_size();

        Ok(self)
    }

    /// Sets the `Wallet`'s custom payload.
    pub fn payload(mut self, payload: &P) -> Result<Wallet<D, Sk, Pk, Sig, P>> {
        payload.check()?;

        self.payload = payload.clone();

        self.update_size();

        Ok(self)
    }

    /// Signs cryptographically the `Wallet`.
    pub fn sign<SP>(mut self,
                    params: &SP,
                    sk: &Sk,
                    pk: &Pk,
                    cb: &Fn(&Self, &SP, &Sk) -> Result<Sig>)
        -> Result<Wallet<D, Sk, Pk, Sig, P>>
        where   SP: Datable
    {
        params.check()?;
        sk.check()?;
        pk.check()?;

        self.public_key = pk.clone();
        
        self.signature = self.sign_cb(params, sk, cb)?;

        self.secret_key = Some(sk.clone());

        self.update_size();

        Ok(self)
    }

    /// Verifies the cryptographic signature against the `Wallet`.
    pub fn verify_signature<SP>(&self,
                                params: &SP,
                                cb: &Fn(&Self, &SP, &Pk, &Sig) -> Result<bool>)
        -> Result<bool>
        where   SP: Datable,
                Sk: Datable + ConstantSize
    {
        params.check()?;

        let pk = self.public_key.clone();
        pk.check()?;

        let sig = self.signature.clone();
        sig.check()?;

        let mut wallet = self.clone();

        wallet.signature = Sig::default();
        wallet.secret_key = None;
        wallet.id = D::default();
        wallet.update_size();

        Signable::<SP, Sk, Pk, Sig>::verify_signature_cb(&wallet, params, &pk, &sig, cb)
    }

    /// Checks the cryptographic signature against the `Wallet`.
    pub fn check_signature<SP>(&self,
                               params: &SP,
                               cb: &Fn(&Self, &SP, &Pk, &Sig) -> Result<()>)
        -> Result<()>
        where   SP: Datable,
                Sk: Datable + ConstantSize
    {
        params.check()?;

        let pk = self.public_key.clone();
        pk.check()?;

        let sig = self.signature.clone();
        sig.check()?;

        let mut wallet = self.clone();
        wallet.signature = Sig::default();
        wallet.secret_key = None;
        wallet.id = D::default();
        wallet.update_size();

        Signable::<SP, Sk, Pk, Sig>::check_signature_cb(&wallet, params, &pk, &sig, cb)
    }

    /// Finalizes the `Wallet`, building its id and returning it's complete form.
    pub fn finalize<HP: Datable>(mut self, params: &HP, cb: &Fn(&Self, &HP) -> Result<D>)
        -> Result<Wallet<D, Sk, Pk, Sig, P>>
    {
        params.check()?;

        self.update_size();

        self.id = self.digest(params, cb)?;

        self.check()?;

        Ok(self)
    }

    /// Hashes cryptographically the `Wallet`.
    pub fn digest<HP: Datable>(&self, params: &HP, cb: &Fn(&Self, &HP) -> Result<D>)
        -> Result<D>
    {
        params.check()?;

        let mut wallet = self.clone();
        wallet.id = D::default();

        wallet.digest_cb(params, cb)
    }

    /// Verifies the cryptographic digest against the `Wallet`'s digest.
    pub fn verify_digest<HP: Datable>(&self,
                                      params: &HP,
                                      cb: &Fn(&Self, &HP, &D) -> Result<bool>)
        -> Result<bool>
    {
        params.check()?;

        let digest = self.id.clone();
        digest.check()?;

        let mut wallet = self.clone();
        wallet.id = D::default();
        wallet.update_size();

        wallet.verify_digest_cb(params, &digest, cb)
    }

    /// Checks the cryptographic digest against the `Wallet`'s digest.
    pub fn check_digest<HP: Datable>(&self,
                                     params: &HP,
                                     cb: &Fn(&Self, &HP, &D) -> Result<()>)
        -> Result<()>
    {
        params.check()?;

        let digest = self.id.clone();
        digest.check()?;

        let mut wallet = self.clone();
        wallet.id = D::default();
        wallet.update_size();

        wallet.check_digest_cb(params, &digest, cb)
    }

    /// Commits cryptographically the `Wallet`.
    pub fn commit<CP, C>(&self, params: &CP, cb: &Fn(&Self, &CP) -> Result<C>)
        -> Result<C>
        where   CP: Datable,
                C: Datable + ConstantSize
    {
        params.check()?;

        self.commit_cb(params, cb)
    }

    /// Verifies the cryptographic commitment against the `Wallet`'s commitment.
    pub fn verify_commitment<CP, C>(&self,
                                    params: &CP,
                                    commitment: &C,
                                    cb: &Fn(&Self, &CP, &C) -> Result<bool>)
        -> Result<bool>
        where   CP: Datable,
                C: Datable + ConstantSize
    {
        params.check()?;
        commitment.check()?;

        self.verify_commitment_cb(params, commitment, cb)
    }

    /// Checks the cryptographic commitment against the `Wallet`'s commitment.
    pub fn check_commitment<CP, C>(&self,
                                   params: &CP,
                                   commitment: &C,
                                   cb: &Fn(&Self, &CP, &C) -> Result<()>)
        -> Result<()>
        where   CP: Datable,
                C: Datable + ConstantSize
    {
        params.check()?;
        commitment.check()?;

        self.check_commitment_cb(params, commitment, cb)
    }

    /// Authenticates cryptographically the `Wallet`.
    pub fn authenticate<AP, T>(&self, params: &AP, cb: &Fn(&Self, &AP) -> Result<T>)
        -> Result<T>
        where   AP: Datable,
                T: Datable + ConstantSize
    {
        params.check()?;

        self.authenticate_cb(params, cb)
    }

    /// Verifies the cryptographic authentication of the `Wallet` against a tag.
    pub fn verify_tag<AP, T>(&self,
                             params: &AP,
                             tag: &T,
                             cb: &Fn(&Self, &AP, &T) -> Result<bool>)
        -> Result<bool>
        where   AP: Datable,
                T: Datable + ConstantSize
    {
        params.check()?;
        tag.check()?;

        self.verify_tag_cb(params, tag, cb)
    }

    /// Checks the cryptographic authentication of the `Wallet` against a tag.
    pub fn check_tag<AP, T>(&self,
                            params: &AP,
                            tag: &T,
                            cb: &Fn(&Self, &AP, &T) -> Result<()>)
        -> Result<()>
        where   AP: Datable,
                T: Datable + ConstantSize
    {
        params.check()?;
        tag.check()?;

        self.check_tag_cb(params, tag, cb)
    }

    /// Evals the `Wallet`.
    pub fn eval<EP, R>(&self, params: &EP, cb: &Fn(&Self, &EP) -> Result<R>)
        -> Result<R>
        where   EP: Datable,
                R: Datable
    {
        params.check()?;

        self.eval_cb(params, cb)
    }
}

impl<HP, D, Sk, Pk, Sig, P> Hashable<HP, D> for Wallet<D, Sk, Pk, Sig, P>
    where   HP: Datable,
            D: Datable + ConstantSize,
            Sk: Datable + ConstantSize,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            P: Datable
{}

impl<SP, D, Sk, Pk, Sig, P> Signable<SP, Sk, Pk, Sig> for Wallet<D, Sk, Pk, Sig, P>
    where   SP: Datable,
            D: Datable + ConstantSize,
            Sk: Datable + ConstantSize,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            P: Datable
{}

impl<CP, C, D, Sk, Pk, Sig, P> Committable<CP, C> for Wallet<D, Sk, Pk, Sig, P>
    where   CP: Datable,
            C: Datable + ConstantSize,
            D: Datable + ConstantSize,
            Sk: Datable + ConstantSize,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            P: Datable
{}

impl<AP, T, D, Sk, Pk, Sig, P> Authenticatable<AP, T> for Wallet<D, Sk, Pk, Sig, P>
    where   AP: Datable,
            T: Datable + ConstantSize,
            D: Datable + ConstantSize,
            Sk: Datable + ConstantSize,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            P: Datable
{}

impl<D, Sk, Pk, Sig, P> Sizable for Wallet<D, Sk, Pk, Sig, P>
    where   D: Datable + ConstantSize,
            Sk: Datable + ConstantSize,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            P: Datable
{
    fn size(&self) -> u64 {
        self.id.size() +
            self.meta.size() +
            self.secret_key.size() +
            self.public_key.size() +
            self.payload.size() +
            self.signature.size()
    }
}

impl<D, Sk, Pk, Sig, P> Checkable for Wallet<D, Sk, Pk, Sig, P>
    where   D: Datable + ConstantSize,
            Sk: Datable + ConstantSize,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            P: Datable
{
    fn check(&self) -> Result<()> {
        self.id.check()?;
        self.id.check_size()?;
        self.meta.check()?;
        
        if self.meta.get_size() != self.size() {
            return Err(String::from("invalid meta size"));
        }
        
        if let Some(ref sk) = self.secret_key {
            sk.check()?;
            sk.check_size()?;
        }

        self.public_key.check()?;
        self.public_key.check_size()?;
        self.payload.check()?;
        self.signature.check()?;
        self.signature.check_size()?;

        Ok(())
    }
}

impl<D, Sk, Pk, Sig, P> Serializable for Wallet<D, Sk, Pk, Sig, P>
    where   D: Datable + ConstantSize + Serializable,
            Sk: Datable + ConstantSize + Serializable,
            Pk: Datable + ConstantSize + Serializable,
            Sig: Datable + ConstantSize + Serializable,
            P: Datable + Serializable
{}

impl<D, Sk, Pk, Sig, P> Datable for Wallet<D, Sk, Pk, Sig, P>
    where   D: Datable + ConstantSize,
            Sk: Datable + ConstantSize,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            P: Datable
{}

impl<D, Sk, Pk, Sig, P> Evaluable for Wallet<D, Sk, Pk, Sig, P>
    where   D: Datable + ConstantSize,
            Sk: Datable + ConstantSize,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            P: Datable
{}

impl<S, D, Sk, Pk, Sig, P> Storable<S, D, Wallet<D, Sk, Pk, Sig, P>> for Wallet<D, Sk, Pk, Sig, P>
    where   S: Datable + Serializable,
            D: Datable + ConstantSize + Serializable,
            Sk: Datable + ConstantSize + Serializable,
            Pk: Datable + ConstantSize + Serializable,
            Sig: Datable + ConstantSize + Serializable,
            P: Datable + Serializable
{
    fn store_key(&self) -> Result<D> {
        self.id.check()?;

        Ok(self.id.clone())
    }

    fn store_value(&self) -> Result<Wallet<D, Sk, Pk, Sig, P>> {
        self.check()?;

        Ok(self.clone())
    }
}