//! # Message
//!
//! `message` is the module providing the types used for network messages.

/// Type used to represent a network method.
pub mod method;

/// Type used to represent a network resource.
pub mod resource;

/// Type used to represent a network message.
pub mod message;

pub use self::method::Method;
pub use self::resource::Resource;
pub use self::message::Message;