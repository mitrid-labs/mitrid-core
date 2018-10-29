//! # Future
//!
//! `future` is the module providing the `Future` type used throughout the library.

use futures::Future as BasicFuture;
use futures::Poll;
use futures::future as base_future;

use std::ops::{Deref, DerefMut};

use base::Result;
use base::Stream;

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

impl<T> BasicFuture for Future<T> {
    type Item=T;
    type Error=String;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.0.poll()
    }
}

impl Into<Stream<()>> for Future<()> {
    fn into(self) -> Stream<()> {
        Stream::from_future(self)
    }
}

impl Into<Stream<bool>> for Future<bool> {
    fn into(self) -> Stream<bool> {
        Stream::from_future(self)
    }
}

impl Into<Stream<u8>> for Future<u8> {
    fn into(self) -> Stream<u8> {
        Stream::from_future(self)
    }
}

impl Into<Stream<i8>> for Future<i8> {
    fn into(self) -> Stream<i8> {
        Stream::from_future(self)
    }
}

impl Into<Stream<u16>> for Future<u16> {
    fn into(self) -> Stream<u16> {
        Stream::from_future(self)
    }
}

impl Into<Stream<i16>> for Future<i16> {
    fn into(self) -> Stream<i16> {
        Stream::from_future(self)
    }
}

impl Into<Stream<u32>> for Future<u32> {
    fn into(self) -> Stream<u32> {
        Stream::from_future(self)
    }
}

impl Into<Stream<i32>> for Future<i32> {
    fn into(self) -> Stream<i32> {
        Stream::from_future(self)
    }
}

impl Into<Stream<u64>> for Future<u64> {
    fn into(self) -> Stream<u64> {
        Stream::from_future(self)
    }
}

impl Into<Stream<i64>> for Future<i64> {
    fn into(self) -> Stream<i64> {
        Stream::from_future(self)
    }
}

impl Into<Stream<String>> for Future<String> {
    fn into(self) -> Stream<String> {
        Stream::from_future(self)
    }
}