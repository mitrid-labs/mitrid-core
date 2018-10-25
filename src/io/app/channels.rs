//! # Channels
//!
//! `channels` is the module providing the types used to communicate with the I/O applications.

use futures::unsync::mpsc::{Sender, Receiver, channel};

use base::Datable;
use io::app::command::Request;
use io::app::command::Response;

pub type RequestSender<Ap, StaP, StoP, RP, EP> = Sender<Request<Ap, StaP, StoP, RP, EP>>;
pub type RequestReceiver<Ap, StaP, StoP, RP, EP> = Receiver<Request<Ap, StaP, StoP, RP, EP>>;

/// Type used to represent an I/O application `Request` channels.
#[derive(Debug)]
pub struct RequestChannel<Ap, StaP, StoP, RP, EP>
    where   Ap: Datable,
            StaP: Datable,
            StoP: Datable,
            RP: Datable,
            EP: Datable,
{
    /// Request channel sender.
    pub sender: RequestSender<Ap, StaP, StoP, RP, EP>,
    /// Request channel receiver.
    pub receiver: RequestReceiver<Ap, StaP, StoP, RP, EP>,
}

impl<Ap, StaP, StoP, RP, EP> RequestChannel<Ap, StaP, StoP, RP, EP>
    where   Ap: Datable,
            StaP: Datable,
            StoP: Datable,
            RP: Datable,
            EP: Datable,
{
    /// Creates a new `RequestChannel`.
    pub fn new(buffer: u64) -> Self {
        let (sender, receiver) = channel(buffer as usize);

        RequestChannel {
            sender: sender,
            receiver: receiver,
        }
    }
}

pub type ResponseSender<Ap, StaR, StoR, RR, ER> = Sender<Response<Ap, StaR, StoR, RR, ER>>;
pub type ResponseReceiver<Ap, StaR, StoR, RR, ER> = Receiver<Response<Ap, StaR, StoR, RR, ER>>;

/// Type used to represent an I/O application `Response` channel.
#[derive(Debug)]
pub struct ResponseChannel<Ap, StaR, StoR, RR, ER>
    where   Ap: Datable,
            StaR: Datable,
            StoR: Datable,
            RR: Datable,
            ER: Datable,
{
    /// Response channel sender.
    pub sender: ResponseSender<Ap, StaR, StoR, RR, ER>,
    /// Response channel receiver.
    pub receiver: ResponseReceiver<Ap, StaR, StoR, RR, ER>,
}

impl<Ap, StaR, StoR, RR, ER> ResponseChannel<Ap, StaR, StoR, RR, ER>
    where   Ap: Datable,
            StaR: Datable,
            StoR: Datable,
            RR: Datable,
            ER: Datable,
{
    /// Creates a new `ResponseChannel`.
    pub fn new(buffer: u64) -> Self {
        let (sender, receiver) = channel(buffer as usize);

        ResponseChannel {
            sender: sender,
            receiver: receiver,
        }
    }
}

/// Type used to represent an I/O application `Request` and `Response` channels.
#[derive(Debug)]
pub struct Channels<Ap, StaP, StaR, StoP, StoR, RP, RR, EP, ER>
    where   Ap: Datable,
            StaP: Datable,
            StaR: Datable,
            StoP: Datable,
            StoR: Datable,
            RP: Datable,
            RR: Datable,
            EP: Datable,
            ER: Datable
{
    /// Request channel.
    pub request: RequestChannel<Ap, StaP, StoP, RP, EP>,
    /// Response channel.
    pub response: ResponseChannel<Ap, StaR, StoR, RR, ER>,
}

impl<Ap, StaP, StaR, StoP, StoR, RP, RR, EP, ER> Channels <Ap, StaP, StaR, StoP, StoR, RP, RR, EP, ER>
    where   Ap: Datable,
            StaP: Datable,
            StaR: Datable,
            StoP: Datable,
            StoR: Datable,
            RP: Datable,
            RR: Datable,
            EP: Datable,
            ER: Datable
{
    /// Creates a new `Channels`.
    pub fn new(buffer: u64) -> Self {
        let request_channel = RequestChannel::new(buffer);
        let response_channel = ResponseChannel::new(buffer);

        Channels {
            request: request_channel,
            response: response_channel,
        }
    }
}