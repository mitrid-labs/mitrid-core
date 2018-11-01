//! # Message
//!
//! `message` is the module providing the type representing network messages.

use rand::random;

use base::Result;
use base::Numerical;
use base::{Sizable, ConstantSize, VariableSize};
use base::Checkable;
use base::Serializable;
use base::Datable;
use crypto::{Hashable, Committable, Authenticatable};
use models::meta::Meta;
use io::Session;
use io::store::{Store, Storable};
use io::Node;
use io::Method;
use io::Resource;

/// Type used to represent a network message.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Message<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   S: Datable,
            RS: Datable,
            Ad: Datable + VariableSize,
            NP: Datable,
            D: Ord + Datable + ConstantSize,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            Pr: Datable,
            Am: Numerical,
            IP: Datable,
            OP: Datable,
            TP: Datable,
            BP: Datable,
            BGP: Datable,
            C: Datable
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
    pub resource: Resource<RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>,
}

impl<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, MC>
    Message<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, MC>
    where   S: Datable,
            RS: Datable,
            Ad: Datable + VariableSize,
            NP: Datable,
            D: Ord + Datable + ConstantSize,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            Pr: Datable,
            Am: Numerical,
            IP: Datable,
            OP: Datable,
            TP: Datable,
            BP: Datable,
            BGP: Datable,
            MC: Datable
{
    /// Creates a new `Message`.
    pub fn new() -> Result<Self> {

        let mut msg = Message::default();
        
        msg.nonce = random();
        msg.update_size();

        Ok(msg)
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

    /// Sets the `Message` set of receivers and its lenght.
    pub fn receivers(mut self, recvs: &Vec<Node<Ad, NP>>,) -> Result<Self>
    {
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

        Ok(self)
    }

    /// Sets the `Message` resource.
    pub fn resource(mut self, resource: &Resource<RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, MC>) -> Result<Self> {
        resource.check()?;

        self.resource = resource.to_owned();

        Ok(self)
    }

    /// Finalizes the `Message`, building its id and returning it's complete form.
    pub fn finalize<HP: Datable>(mut self, params: &HP, cb: &Fn(&Self, &HP) -> Result<D>) -> Result<Self>
    {
        params.check()?;

        self.update_size();

        self.id = self.digest(params, cb)?;

        self.check()?;

        Ok(self)
    }

    /// Returns if the `Message` is an error message.
    pub fn is_error(&self) -> Result<bool> {
        self.check()?;

        self.resource.is_error()
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

impl<P, S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C> Hashable<P, D>
    for Message<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   P: Datable,
            S: Datable,
            RS: Datable,
            Ad: Datable + VariableSize,
            NP: Datable,
            D: Ord + Datable + ConstantSize,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            Pr: Datable,
            Am: Numerical,
            IP: Datable,
            OP: Datable,
            TP: Datable,
            BP: Datable,
            BGP: Datable,
            C: Datable
{}

impl<CP, C, S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, MC> Committable<CP, C>
    for Message<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, MC>
    where   CP: Datable,
            C: Datable + ConstantSize,
            S: Datable,
            RS: Datable,
            Ad: Datable + VariableSize,
            NP: Datable,
            D: Ord + Datable + ConstantSize,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            Pr: Datable,
            Am: Numerical,
            IP: Datable,
            OP: Datable,
            TP: Datable,
            BP: Datable,
            BGP: Datable,
            MC: Datable
{}

impl<AP, T, S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C> Authenticatable<AP, T>
    for Message<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   AP: Datable,
            T: Datable + ConstantSize,
            S: Datable,
            RS: Datable,
            Ad: Datable + VariableSize,
            NP: Datable,
            D: Ord + Datable + ConstantSize,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            Pr: Datable,
            Am: Numerical,
            IP: Datable,
            OP: Datable,
            TP: Datable,
            BP: Datable,
            BGP: Datable,
            C: Datable
{}

impl<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C> Sizable
    for Message<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   S: Datable,
            RS: Datable,
            Ad: Datable + VariableSize,
            NP: Datable,
            D: Ord + Datable + ConstantSize,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            Pr: Datable,
            Am: Numerical,
            IP: Datable,
            OP: Datable,
            TP: Datable,
            BP: Datable,
            BGP: Datable,
            C: Datable
{
    fn size(&self) -> u64 {
        self.id.size() +
            self.meta.size() +
            self.nonce.size() +
            self.session.size() +
            self.sender.size() +
            self.receivers_len.size() +
            self.receivers.size() +
            self.resource.size()
    }
}

impl<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C> Checkable
    for Message<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   S: Datable,
            RS: Datable,
            Ad: Datable + VariableSize,
            NP: Datable,
            D: Ord + Datable + ConstantSize,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            Pr: Datable,
            Am: Numerical,
            IP: Datable,
            OP: Datable,
            TP: Datable,
            BP: Datable,
            BGP: Datable,
            C: Datable
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

        Ok(())
    }
}

impl<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C> Serializable
    for Message<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   S: Datable + Serializable,
            RS: Datable + Serializable,
            Ad: Datable + VariableSize + Serializable,
            NP: Datable + Serializable,
            D: Ord + Datable + ConstantSize + Serializable,
            Pk: Datable + ConstantSize + Serializable,
            Sig: Datable + ConstantSize + Serializable,
            Pr: Datable + Serializable,
            Am: Numerical + Serializable,
            IP: Datable + Serializable,
            OP: Datable + Serializable,
            TP: Datable + Serializable,
            BP: Datable + Serializable,
            BGP: Datable + Serializable,
            C: Datable + Serializable
{}

impl<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C> Datable
    for Message<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   S: Datable,
            RS: Datable,
            Ad: Datable + VariableSize,
            NP: Datable,
            D: Ord + Datable + ConstantSize,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            Pr: Datable,
            Am: Numerical,
            IP: Datable,
            OP: Datable,
            TP: Datable,
            BP: Datable,
            BGP: Datable,
            C: Datable
{}

impl<St, S, MS, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C, StP, StPC, StRC>
    Storable<St, S, D, Message<MS, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>, StP, StPC, StRC>
    for Message<MS, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   St: Store<S, StP, StPC, StRC>,
            S: Datable + Serializable,
            MS: Datable + Serializable,
            RS: Datable + Serializable,
            Ad: Datable + VariableSize + Serializable,
            NP: Datable + Serializable,
            D: Ord + Datable + ConstantSize + Serializable,
            Pk: Datable + ConstantSize + Serializable,
            Sig: Datable + ConstantSize + Serializable,
            Pr: Datable + Serializable,
            Am: Numerical + Serializable,
            IP: Datable + Serializable,
            OP: Datable + Serializable,
            TP: Datable + Serializable,
            BP: Datable + Serializable,
            BGP: Datable + Serializable,
            C: Datable + Serializable,
            StP: Datable,
            StPC: Datable + Serializable,
            StRC: Datable + Serializable
{
    fn store_key(&self) -> Result<D> {
        self.id.check()?;

        Ok(self.id.clone())
    }

    fn store_value(&self) -> Result<Self> {
        self.check()?;

        Ok(self.clone())
    }
}