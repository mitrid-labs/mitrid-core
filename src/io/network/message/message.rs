//! # Message
//!
//! `message` is the module providing the type representing network messages.

use rand::random;

use base::Result;
use base::{Sizable, ConstantSize};
use base::Checkable;
use base::Serializable;
use base::Datable;
use base::{Eval, EvalMut};
use base::Meta;
use crypto::Hash;
use io::Session;
use io::Method;
use io::Resource;

/// Type used to represent a network message.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Message<S, D, P>
    where   S: Datable,
            D: Ord + Datable + ConstantSize,
            P: Datable
{
    /// Message id (hash digest)
    pub id: D,
    /// Message metadata.
    pub meta: Meta,
    /// Message nonce.
    pub nonce: u64,
    /// Message session.
    pub session: Session<S>,
    /// Message method.
    pub method: Method,
    /// Message resource.
    pub resource: Resource,
    /// Message payload.
    pub payload: P
}

impl<S, D, P> Message<S, D, P>
    where   S: Datable,
            D: Ord + Datable + ConstantSize,
            P: Datable,
            Self: Serializable
{
    /// Creates a new `Message`.
    pub fn new() -> Self {

        let mut msg = Message::default();
        
        msg.nonce = random();
        msg.update_size();

        msg
    }

    /// Updates the `Message` size.
    pub fn update_size(&mut self) {
        let size = self.size();

        self.meta.set_size(size);
    }

    /// Sets the `Message`'s metadata.
    pub fn meta(mut self, meta: &Meta) -> Result<Self> {
        meta.check()?;
        self.meta = meta.clone();

        self.update_size();

        Ok(self)
    }

    /// Sets the `Message` session.
    pub fn session(mut self, session: &Session<S>) -> Result<Self> {
        session.check()?;

        self.session = session.clone();

        self.update_size();

        Ok(self)
    }

    /// Returns if the `Message` is expired.
    pub fn is_expired(&self) -> Result<bool> {
        self.session.is_expired()
    }

    /// Sets the `Message` method.
    pub fn method(mut self, method: &Method) -> Result<Self> {
        method.check()?;

        self.method = method.to_owned();

        self.update_size();

        Ok(self)
    }

    /// Sets the `Message` resource.
    pub fn resource(mut self, resource: &Resource) -> Result<Self> {
        resource.check()?;

        self.resource = resource.to_owned();

        self.update_size();

        Ok(self)
    }

    /// Returns if the `Message` is an error message.
    pub fn is_error(&self) -> bool {
        match self.resource {
            Resource::Error => true,
            _ => false,
        }
    }

    /// Sets the `Message` payload.
    pub fn payload(mut self, payload: &P) -> Result<Self> {
        payload.check()?;

        self.payload = payload.to_owned();

        self.update_size();

        Ok(self)
    }

    /// Finalizes the `Message`, building its id and returning it's complete form.
    pub fn finalize<H: Hash<D>>(mut self, hasher: &mut H) -> Result<Self> {
        let msg = self.to_bytes()?;
        self.id = hasher.digest(&msg)?;

        self.update_size();

        self.check()?;

        Ok(self)
    }

    /// Hashes cryptographically the `Message`.
    pub fn digest<H: Hash<D>>(&self, hasher: &mut H) -> Result<D> {
        let mut message = self.clone();
        message.id = D::default();
        message.update_size();

        let msg = message.to_bytes()?;
        hasher.digest(&msg)
    }

    /// Verifies the cryptographic digest against the `Message`'s digest.
    pub fn verify_digest<H: Hash<D>>(&self, hasher: &mut H) -> Result<bool> {
        let digest = self.id.clone();
        digest.check()?;

        let mut message = self.clone();
        message.id = D::default();
        message.update_size();

        let msg = message.to_bytes()?;
        hasher.verify(&msg, &digest)
    }

    /// Checks the cryptographic digest against the `Message`'s digest.
    pub fn check_digest<H: Hash<D>>(&self, hasher: &mut H) -> Result<()> {
        let digest = self.id.clone();
        digest.check()?;

        let mut message = self.clone();
        message.id = D::default();
        message.update_size();

        let msg = message.to_bytes()?;
        hasher.check(&msg, &digest)
    }

    /// Evals the `Message`.
    pub fn eval<Ev, EP, ER>(&self, params: &EP, evaluator: &Ev)
        -> Result<ER>
        where   Ev: Eval<Self, EP, ER>,
                EP: Datable,
                ER: Datable
    {
        self.check()?;
        params.check()?;

        evaluator.eval(self, params)
    }

    /// Evals mutably the `Message`.
    pub fn eval_mut<EvM, EP, ER>(&mut self, params: &EP, evaluator: &mut EvM)
        -> Result<ER>
        where   EvM: EvalMut<Self, EP, ER>,
                EP: Datable,
                ER: Datable
    {
        self.check()?;
        params.check()?;

        let result = evaluator.eval_mut(self, params)?;
        self.update_size();

        self.check()?;

        Ok(result)
    }
}

impl<S, D, P> Sizable for Message<S, D, P>
    where   S: Datable,
            D: Ord + Datable + ConstantSize,
            P: Datable
{
    fn size(&self) -> u64 {
        self.id.size() +
            self.meta.size() +
            self.nonce.size() +
            self.session.size() +
            self.method.size() +
            self.resource.size() +
            self.payload.size()
    }
}

impl<S, D, P> Checkable for Message<S, D, P>
    where   S: Datable,
            D: Ord + Datable + ConstantSize,
            P: Datable
{
    fn check(&self) -> Result<()> {
        self.id.check()?;
        self.id.check_size()?;
        self.meta.check()?;
        
        if self.meta.get_size() != self.size() {
            return Err(String::from("invalid meta size"));
        }

        self.nonce.check()?;
        self.session.check()?;

        self.method.check()?;
        self.method.check_permission(&self.session.permission)?;

        self.resource.check()?;
        self.resource.check_method(&self.method)?;

        self.payload.check()?;

        Ok(())
    }
}

impl<S, D, P> Serializable for Message<S, D, P>
    where   S: Datable + Serializable,
            D: Ord + Datable + ConstantSize + Serializable,
            P: Datable + Serializable
{}

impl<S, D, P> Datable for Message<S, D, P>
    where   S: Datable,
            D: Ord + Datable + ConstantSize,
            P: Datable
{}