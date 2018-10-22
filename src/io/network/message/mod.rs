//! # Message
//!
//! `message` is the module providing the types used for network messages.

/// Type used to represent a network method.
pub mod method;

/// Type used to represent a network resource.
pub mod resource;

/// Type used to represent a network message.
pub mod message;

/// Type used to represent a network request.
pub mod request;

/// Type used to represent a network response.
pub mod response;

pub use self::method::Method;
pub use self::resource::Resource;
pub use self::message::Message;
pub use self::request::Request;
pub use self::response::Response;