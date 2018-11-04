//! # Message
//!
//! `message` is the module providing the type representing network messages.

use rand::random;

use base::Result;
use base::{Sizable, ConstantSize, VariableSize};
use base::Checkable;
use base::Serializable;
use base::Datable;
use crypto::{Hashable, Committable, Authenticatable};
use models::meta::Meta;
use io::Session;
use io::Node;
use io::Method;
use io::Resource;

/// Type used to represent a network message.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Message<S, Ad, NP, D, P>
    where   S: Datable,
            Ad: Ord + Datable + VariableSize,
            NP: Datable,
            D: Ord + Datable + ConstantSize,
            P: Datable,
{
    /// Message id (hash digest)
    pub id: D,
    /// Message metadata.
    pub meta: Meta,
    /// Message nonce.
    pub nonce: u64,
    /// Message session.
    pub session: Session<S>,
    /// Message sending node.
    pub sender: Node<Ad, NP>,
    /// Length of the message receivers.
    pub receivers_len: u64,
    /// Message receiving nodes.
    pub receivers: Vec<Node<Ad, NP>>,
    /// Message method.
    pub method: Method,
    /// Message resource.
    pub resource: Resource,
    /// Message payload.
    pub payload: P,
}

impl<S, Ad, NP, D, P> Message<S, Ad, NP, D, P>
    where   S: Datable,
            Ad: Ord + Datable + VariableSize,
            NP: Datable,
            D: Ord + Datable + ConstantSize,
            P: Datable
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

    /// Sets the `Message` sender.
    pub fn sender(mut self, sender: &Node<Ad, NP>) -> Result<Self> {
        sender.check()?;

        self.sender = sender.clone();

        self.update_size();

        Ok(self)
    }

    /// Sets the `Message` set of receivers and its lenght.
    pub fn receivers(mut self, recvs: &Vec<Node<Ad, NP>>,) -> Result<Self> {
        recvs.check()?;

        self.receivers_len = recvs.len() as u64;
        self.receivers = recvs.clone();

        self.update_size();

        Ok(self)
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
    pub fn finalize<HP: Datable>(mut self, params: &HP, cb: &Fn(&Self, &HP) -> Result<D>) -> Result<Self>
    {
        params.check()?;

        self.id = self.digest(params, cb)?;

        self.update_size();

        self.check()?;

        Ok(self)
    }

    /// Hashes cryptographically the `Message`.
    pub fn digest<HP: Datable>(&self, params: &HP, cb: &Fn(&Self, &HP) -> Result<D>) -> Result<D>
    {
        params.check()?;

        let mut coin = self.clone();
        coin.id = D::default();

        coin.digest_cb(params, cb)
    }

    /// Verifies the cryptographic digest against the `Message`'s digest.
    pub fn verify_digest<HP: Datable>(&self,
                                      params: &HP,
                                      cb: &Fn(&Self, &HP, &D) -> Result<bool>)
        -> Result<bool>
    {
        params.check()?;

        let digest = self.id.clone();
        digest.check()?;

        let mut coin = self.clone();
        coin.id = D::default();
        coin.update_size();

        coin.verify_digest_cb(params, &digest, cb)
    }

    /// Checks the cryptographic digest against the `Message`'s digest.
    pub fn check_digest<HP: Datable>(&self,
                                     params: &HP,
                                     cb: &Fn(&Self, &HP, &D) -> Result<()>)
        -> Result<()>
    {
        params.check()?;

        let digest = self.id.clone();
        digest.check()?;

        let mut coin = self.clone();
        coin.id = D::default();
        coin.update_size();

        coin.check_digest_cb(params, &digest, cb)
    }

    /// Commits cryptographically the `Message`.
    pub fn commit<CP, C>(&self, params: &CP, cb: &Fn(&Self, &CP) -> Result<C>)
        -> Result<C>
        where   CP: Datable,
                C: Datable + ConstantSize
    {
        params.check()?;

        self.commit_cb(params, cb)
    }

    /// Verifies the cryptographic commitment against the `Message`'s commitment.
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

    /// Checks the cryptographic commitment against the `Message`'s commitment.
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

    /// Authenticates cryptographically the `Message`.
    pub fn authenticate<AP, T>(&self, params: &AP, cb: &Fn(&Self, &AP) -> Result<T>)
        -> Result<T>
        where   AP: Datable,
                T: Datable + ConstantSize
    {
        params.check()?;

        self.authenticate_cb(params, cb)
    }

    /// Verifies the cryptographic authentication of the `Message` against a tag.
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

    /// Checks the cryptographic authentication of the `Message` against a tag.
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
}

impl<HP, S, Ad, NP, D, P> Hashable<HP, D> for Message<S, Ad, NP, D, P>
    where   HP: Datable,
            S: Datable,
            Ad: Ord + Datable + VariableSize,
            NP: Datable,
            D: Ord + Datable + ConstantSize,
            P: Datable
{}

impl<CP, C, S, Ad, NP, D, P> Committable<CP, C> for Message<S, Ad, NP, D, P>
    where   CP: Datable,
            C: Datable + ConstantSize,
            S: Datable,
            Ad: Ord + Datable + VariableSize,
            NP: Datable,
            D: Ord + Datable + ConstantSize,
            P: Datable
{}

impl<AP, T, S, Ad, NP, D, P> Authenticatable<AP, T> for Message<S, Ad, NP, D, P>
    where   AP: Datable,
            T: Datable + ConstantSize,
            S: Datable,
            Ad: Ord + Datable + VariableSize,
            NP: Datable,
            D: Ord + Datable + ConstantSize,
            P: Datable
{}

impl<S, Ad, NP, D, P> Sizable for Message<S, Ad, NP, D, P>
    where   S: Datable,
            Ad: Ord + Datable + VariableSize,
            NP: Datable,
            D: Ord + Datable + ConstantSize,
            P: Datable
{
    fn size(&self) -> u64 {
        self.id.size() +
            self.meta.size() +
            self.nonce.size() +
            self.session.size() +
            self.sender.size() +
            self.receivers_len.size() +
            self.receivers.size() +
            self.method.size() +
            self.resource.size() +
            self.payload.size()
    }
}

impl<S, Ad, NP, D, P> Checkable for Message<S, Ad, NP, D, P>
    where   S: Datable,
            Ad: Ord + Datable + VariableSize,
            NP: Datable,
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

        self.sender.check()?;
        self.receivers_len.check()?;
        self.receivers.check()?;

        if self.receivers_len != self.receivers.len() as u64 {
            return Err(String::from("invalid length"));
        }

        self.method.check()?;
        self.method.check_permission(&self.session.permission)?;

        self.resource.check()?;
        self.resource.check_method(&self.method)?;

        self.payload.check()?;

        Ok(())
    }
}

impl<S, Ad, NP, D, P> Serializable for Message<S, Ad, NP, D, P>
    where   S: Datable + Serializable,
            Ad: Ord + Datable + VariableSize + Serializable,
            NP: Datable + Serializable,
            D: Ord + Datable + ConstantSize + Serializable,
            P: Datable + Serializable
{}

impl<S, Ad, NP, D, P> Datable for Message<S, Ad, NP, D, P>
    where   S: Datable,
            Ad: Ord + Datable + VariableSize,
            NP: Datable,
            D: Ord + Datable + ConstantSize,
            P: Datable
{}