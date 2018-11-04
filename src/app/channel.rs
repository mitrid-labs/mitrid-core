//! # Channel
//!
//! `channel` is the module providing the types used to communicate with applications.

use std::sync::mpsc::{Sender, Receiver, channel};

use base::Datable;
use app::Request;
use app::Response;

pub type RequestSender<A, P> = Sender<Request<A, P>>;
pub type RequestReceiver<A, P> = Receiver<Request<A, P>>;

/// Type used to represent an application `Request` channels.
#[derive(Debug)]
pub struct RequestChannel<A, P>
    where   A: Ord + Datable,
            P: Datable,
{
    /// Request channel sender.
    pub sender: RequestSender<A, P>,
    /// Request channel receiver.
    pub receiver: RequestReceiver<A, P>,
}

impl<A, P> RequestChannel<A, P>
    where   A: Ord + Datable,
            P: Datable,
{
    /// Creates a new `RequestChannel`.
    pub fn new() -> Self {
        let (sender, receiver) = channel();

        RequestChannel {
            sender: sender,
            receiver: receiver,
        }
    }
}

pub type ResponseSender<A, R> = Sender<Response<A, R>>;
pub type ResponseReceiver<A, R> = Receiver<Response<A, R>>;

/// Type used to represent an application `Response` channel.
#[derive(Debug)]
pub struct ResponseChannel<A, R>
    where   A: Ord + Datable,
            R: Datable,
{
    /// Response channel sender.
    pub sender: ResponseSender<A, R>,
    /// Response channel receiver.
    pub receiver: ResponseReceiver<A, R>,
}

impl<A, R> ResponseChannel<A, R>
    where   A: Ord + Datable,
            R: Datable,
{
    /// Creates a new `ResponseChannel`.
    pub fn new() -> Self {
        let (sender, receiver) = channel();

        ResponseChannel {
            sender: sender,
            receiver: receiver,
        }
    }
}

/// Type used to represent an application `Request` and `Response` channels.
#[derive(Debug)]
pub struct Channels<A, P, R>
    where   A: Ord + Datable,
            P: Datable,
            R: Datable,
{
    /// Request channel.
    pub request: RequestChannel<A, P>,
    /// Response channel.
    pub response: ResponseChannel<A, R>,
}

impl<A, P, R> Channels<A, P, R>
    where   A: Ord + Datable,
            P: Datable,
            R: Datable,
{
    /// Creates a new `Channels`.
    pub fn new() -> Self {
        let request_channel = RequestChannel::new();
        let response_channel = ResponseChannel::new();

        Channels {
            request: request_channel,
            response: response_channel,
        }
    }
}