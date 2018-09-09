//! Conversions only available on unix.

use std::path::{Path, PathBuf};
use std::ffi::{OsStr, OsString};
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use super::{RawStr, RawString};

pub trait RawStrExt {
	fn as_osstr(&self) -> &OsStr;
	fn as_path(&self) -> &Path;
}

pub trait RawStringExt {
	fn into_osstring(self) -> OsString;
	fn into_pathbuf(self) -> PathBuf;
}

/// Conversions only available on unix.
impl RawStrExt for RawStr {
	fn as_osstr(&self) -> &OsStr {
		OsStr::from_bytes(self.as_bytes())
	}
	fn as_path(&self) -> &Path {
		Path::new(self.as_osstr())
	}
}

/// Conversions only available on unix.
impl RawStringExt for RawString {
	fn into_osstring(self) -> OsString {
		OsString::from_vec(self.into_bytes())
	}
	fn into_pathbuf(self) -> PathBuf {
		PathBuf::from(self.into_osstring())
	}
}
