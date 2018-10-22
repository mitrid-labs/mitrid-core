//! # Response
//!
//! `response` is the module providing the type representing network response messages.

use io::network::message::Message;

/// Type representing a network response message.
pub type Response<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C> = Message<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>;