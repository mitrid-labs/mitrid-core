//! # Future
//!
//! `future` is the module providing the `Future` type used throughout the library.

use futures::Future as BasicFuture;
use futures::future as base_future;

use std::ops::{Deref, DerefMut};

use base::result::Result;

/// Alias to a `futures::Future` with `String` error.
pub struct Future<T>(Box<BasicFuture<Item=T, Error=String>>);

impl<T: 'static> Future<T> {
    /// Creates a new `Future<T>` from a boxed `futures::Future`.
    pub fn new(bf: Box<BasicFuture<Item=T, Error=String>>) -> Future<T> {
        Future::<T>(bf)
    }

    /// Creates a new `Future<T>` from a `Result<T>`.
    pub fn from_result(res: Result<T>) -> Future<T> {
        Future(Box::new(base_future::result::<T, String>(res)))
    }
}

impl<T> Deref for Future<T> {
    type Target = Box<BasicFuture<Item=T, Error=String>>;

    fn deref(&self) -> &Box<BasicFuture<Item=T, Error=String>> {
        &self.0
    }
}

impl<T> DerefMut for Future<T> {
    fn deref_mut(&mut self) -> &mut Box<BasicFuture<Item=T, Error=String>> {
        &mut self.0
    }
}