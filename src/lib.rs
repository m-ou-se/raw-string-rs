//! `RawString` and `RawStr` are the equivalents of `String` and
//! `str`, or `OsString` and `OsStr`, but without any guarantees
//! about the encoding.
//!
//! They are useful in all places where you would otherwise use
//! `Vec<u8>` and `[u8]` to represent your strings.

mod str;
mod string;

pub use str::*;
pub use string::*;

#[cfg(unix)]
pub mod unix;
