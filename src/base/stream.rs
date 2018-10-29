//! # Stream
//!
//! `stream` is the module providing the `Stream` type used throughout the library.

use futures::Future as BasicFuture;
use futures::Stream as BasicStream;
use futures::Poll;

use std::ops::{Deref, DerefMut};

use base::Result;
use base::Future;

/// Alias to a `futures::Stream` with `String` error.
pub struct Stream<T>(Box<BasicStream<Item=T, Error=String>>);

impl<T: 'static> Stream<T> {
    /// Creates a new `Stream<T>` from a boxed `futures::Stream`.
    pub fn new(bf: Box<BasicStream<Item=T, Error=String>>) -> Stream<T> {
        Stream::<T>(bf)
    }

    /// Creates a new `Stream<T>` from a `Future<T>`.
    pub fn from_future(res: Future<T>) -> Stream<T> {
        Stream(Box::new(res.into_stream()))
    }

    /// Creates a new `Stream<T>` from a `Result<T>`.
    pub fn from_result(res: Result<T>) -> Stream<T> {
        Stream::from_future(Future::from_result(res))
    }
}

impl<T> Deref for Stream<T> {
    type Target = Box<BasicStream<Item=T, Error=String>>;

    fn deref(&self) -> &Box<BasicStream<Item=T, Error=String>> {
        &self.0
    }
}

impl<T> DerefMut for Stream<T> {
    fn deref_mut(&mut self) -> &mut Box<BasicStream<Item=T, Error=String>> {
        &mut self.0
    }
}

impl<T> BasicStream for Stream<T> {
    type Item=T;
    type Error=String;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.0.poll()
    }
}