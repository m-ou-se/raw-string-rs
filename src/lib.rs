//! `RawString` and `RawStr` are the equivalents of `String` and
//! `str`, or `OsString` and `OsStr`, but without any guarantees
//! about the encoding.
//!
//! They are useful in all places where you would otherwise use
//! `Vec<u8>` and `[u8]` to represent your strings.

// TODO: Remove this once docs.rs supports rust stable 1.28 or later.
#![cfg_attr(feature="old-nightly",feature(slice_get_slice))]

mod str;
mod string;

pub use str::*;
pub use string::*;

#[cfg(unix)]
pub mod unix;
