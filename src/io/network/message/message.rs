//! # Message
//!
//! `message` is the module providing the type representing network messages.

use rand::random;

use std::marker::PhantomData;

use base::Result;
use base::Numerical;
use base::{Sizable, ConstantSize, VariableSize};
use base::Checkable;
use base::Serializable;
use base::Datable;
use crypto::{Hashable, Committable, Authenticatable};
use models::meta::Meta;
use io::Session;
use io::Storable;
use io::Node;
use io::message::MessageData;

/// Type used to represent a network message.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Message<M, R, S, MDS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   M: Datable,
            R: Datable,
            S: Datable,
            MDS: Datable,
            Ad: Datable + VariableSize,
            NP: Datable,
            D: Datable + ConstantSize,
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
    /// Message method.
    method: PhantomData<M>,
    /// Message resource.
    resource: PhantomData<R>,
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
    /// Message data.
    pub data: MessageData<MDS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>,
}

impl<M, R, S, MDS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, MC>
    Message<M, R, S, MDS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, MC>
    where   M: Datable,
            R: Datable,
            S: Datable,
            MDS: Datable,
            Ad: Datable + VariableSize,
            NP: Datable,
            D: Datable + ConstantSize,
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

        // TODO: check resource against method [ping <-> none; session <-> session, !ping /\ !session <-> !none]
        
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

    /// Sets the `Message`s session.
    pub fn session(mut self, session: &Session<S>) -> Result<Self> {
        session.check()?;

        self.session = session.clone();

        // TODO: check session expiration time

        // TODO: check session given method

        self.update_size();

        Ok(self)
    }

    /// Sets the `Message`s set of receivers and its lenght.
    pub fn receivers(mut self, recvs: &Vec<Node<Ad, NP>>,) -> Result<Self>
    {
        recvs.check()?;

        self.receivers_len = recvs.len() as u64;
        self.receivers = recvs.clone();

        self.update_size();

        Ok(self)
    }

    /// Sets the `Message`s data.
    pub fn data(mut self, data: &MessageData<MDS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, MC>) -> Result<Self> {
        data.check()?;

        self.data = data.clone();

        // TODO: check data against resource

        // TODO: check data against method

        self.update_size();

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

impl<P, M, R, S, MDS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C> Hashable<P, D>
    for Message<M, R, S, MDS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   P: Datable,
            M: Datable,
            R: Datable,
            S: Datable,
            MDS: Datable,
            Ad: Datable + VariableSize,
            NP: Datable,
            D: Datable + ConstantSize,
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

impl<CP, C, M, R, S, MDS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, MC> Committable<CP, C>
    for Message<M, R, S, MDS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, MC>
    where   CP: Datable,
            C: Datable + ConstantSize,
            M: Datable,
            R: Datable,
            S: Datable,
            MDS: Datable,
            Ad: Datable + VariableSize,
            NP: Datable,
            D: Datable + ConstantSize,
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

impl<AP, T, M, R, S, MDS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C> Authenticatable<AP, T>
    for Message<M, R, S, MDS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   AP: Datable,
            T: Datable + ConstantSize,
            M: Datable,
            R: Datable,
            S: Datable,
            MDS: Datable,
            Ad: Datable + VariableSize,
            NP: Datable,
            D: Datable + ConstantSize,
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

impl<M, R, S, MDS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C> Sizable
    for Message<M, R, S, MDS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   M: Datable,
            R: Datable,
            S: Datable,
            MDS: Datable,
            Ad: Datable + VariableSize,
            NP: Datable,
            D: Datable + ConstantSize,
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
            self.data.size()
    }
}

impl<M, R, S, MDS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C> Checkable
    for Message<M, R, S, MDS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   M: Datable,
            R: Datable,
            S: Datable,
            MDS: Datable,
            Ad: Datable + VariableSize,
            NP: Datable,
            D: Datable + ConstantSize,
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

        // TODO: check resource against method [ping <-> none; session <-> session, !ping /\ !session <-> !none]
        
        if self.meta.get_size() != self.size() {
            return Err(String::from("invalid meta size"));
        }

        self.nonce.check()?;
        self.session.check()?;

        // TODO: check session expiration time

        // TODO: check session given method

        self.sender.check()?;
        self.receivers_len.check()?;
        self.receivers.check()?;

        if self.receivers_len != self.receivers.len() as u64 {
            return Err(String::from("invalid length"));
        }

        self.data.check()?;

        // TODO: check data against resource

        // TODO: check data against method

        Ok(())
    }
}

impl<M, R, S, MDS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C> Serializable
    for Message<M, R, S, MDS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   M: Datable + Serializable,
            R: Datable + Serializable,
            S: Datable + Serializable,
            MDS: Datable + Serializable,
            Ad: Datable + VariableSize + Serializable,
            NP: Datable + Serializable,
            D: Datable + ConstantSize + Serializable,
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

impl<M, R, S, MDS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C> Datable
    for Message<M, R, S, MDS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   M: Datable,
            R: Datable,
            S: Datable,
            MDS: Datable,
            Ad: Datable + VariableSize,
            NP: Datable,
            D: Datable + ConstantSize,
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

impl<S, M, R, MS, MDS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    Storable<S, D, Message<M, R, MS, MDS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>>
    for Message<M, R, MS, MDS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   S: Datable + Serializable,
            M: Datable + Serializable,
            R: Datable + Serializable,
            MS: Datable + Serializable,
            MDS: Datable + Serializable,
            Ad: Datable + VariableSize + Serializable,
            NP: Datable + Serializable,
            D: Datable + ConstantSize + Serializable,
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