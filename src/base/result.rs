//! # Result
//!
//! `result` is the module providing the `Result` type used throughout the library.

use std::result::Result as StdResult;
use std::convert::Into;

use base::Future;
use base::Stream;

/// Alias type to a `std::result::Result` with `String` error.
pub type Result<T> = StdResult<T, String>;

impl Into<Future<()>> for Result<()> {
    fn into(self) -> Future<()> {
        Future::from_result(self)
    }
}

impl Into<Future<bool>> for Result<bool> {
    fn into(self) -> Future<bool> {
        Future::from_result(self)
    }
}

impl Into<Future<u8>> for Result<u8> {
    fn into(self) -> Future<u8> {
        Future::from_result(self)
    }
}

impl Into<Future<i8>> for Result<i8> {
    fn into(self) -> Future<i8> {
        Future::from_result(self)
    }
}

impl Into<Future<u16>> for Result<u16> {
    fn into(self) -> Future<u16> {
        Future::from_result(self)
    }
}

impl Into<Future<i16>> for Result<i16> {
    fn into(self) -> Future<i16> {
        Future::from_result(self)
    }
}

impl Into<Future<u32>> for Result<u32> {
    fn into(self) -> Future<u32> {
        Future::from_result(self)
    }
}

impl Into<Future<i32>> for Result<i32> {
    fn into(self) -> Future<i32> {
        Future::from_result(self)
    }
}

impl Into<Future<u64>> for Result<u64> {
    fn into(self) -> Future<u64> {
        Future::from_result(self)
    }
}

impl Into<Future<i64>> for Result<i64> {
    fn into(self) -> Future<i64> {
        Future::from_result(self)
    }
}

impl Into<Future<String>> for Result<String> {
    fn into(self) -> Future<String> {
        Future::from_result(self)
    }
}

impl Into<Stream<()>> for Result<()> {
    fn into(self) -> Stream<()> {
        Stream::from_result(self)
    }
}

impl Into<Stream<bool>> for Result<bool> {
    fn into(self) -> Stream<bool> {
        Stream::from_result(self)
    }
}

impl Into<Stream<u8>> for Result<u8> {
    fn into(self) -> Stream<u8> {
        Stream::from_result(self)
    }
}

impl Into<Stream<i8>> for Result<i8> {
    fn into(self) -> Stream<i8> {
        Stream::from_result(self)
    }
}

impl Into<Stream<u16>> for Result<u16> {
    fn into(self) -> Stream<u16> {
        Stream::from_result(self)
    }
}

impl Into<Stream<i16>> for Result<i16> {
    fn into(self) -> Stream<i16> {
        Stream::from_result(self)
    }
}

impl Into<Stream<u32>> for Result<u32> {
    fn into(self) -> Stream<u32> {
        Stream::from_result(self)
    }
}

impl Into<Stream<i32>> for Result<i32> {
    fn into(self) -> Stream<i32> {
        Stream::from_result(self)
    }
}

impl Into<Stream<u64>> for Result<u64> {
    fn into(self) -> Stream<u64> {
        Stream::from_result(self)
    }
}

impl Into<Stream<i64>> for Result<i64> {
    fn into(self) -> Stream<i64> {
        Stream::from_result(self)
    }
}

impl Into<Stream<String>> for Result<String> {
    fn into(self) -> Stream<String> {
        Stream::from_result(self)
    }
}