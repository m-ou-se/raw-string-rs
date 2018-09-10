use std;
use std::borrow::{Borrow, ToOwned};
use std::ffi::OsString;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{
	Deref, DerefMut, Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo,
	RangeToInclusive,
};
use std::path::PathBuf;
use std::string::FromUtf8Error;
use str::RawStr;

/// A `String` with unchecked contents.
///
/// It is basically a `Vec<u8>`, to be interpreted as string.
/// Unlike `String`, there are no guarantees about the contents being valid UTF-8.
/// Unlike `Vec<u8>`, its Display and Debug implementations show a string, not
/// an array of numbers.
#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RawString {
	inner: Vec<u8>,
}

impl RawString {
	pub fn new() -> Self {
		RawString { inner: Vec::new() }
	}

	pub fn with_capacity(capacity: usize) -> Self {
		RawString {
			inner: Vec::with_capacity(capacity),
		}
	}

	pub fn from_bytes(bytes: Vec<u8>) -> Self {
		RawString { inner: bytes }
	}

	pub fn from_string(bytes: String) -> Self {
		Self::from_bytes(bytes.into_bytes())
	}

	pub fn into_bytes(self) -> Vec<u8> {
		self.inner
	}

	pub fn reserve(&mut self, additional: usize) {
		self.inner.reserve(additional)
	}

	pub fn reserve_exact(&mut self, additional: usize) {
		self.inner.reserve_exact(additional)
	}

	pub fn shrink_to_fit(&mut self) {
		self.inner.shrink_to_fit()
	}

	/* Unstable feature:
	pub fn shrink_to(&mut self, min_capacity: usize) {
		self.inner.shrink_to(min_capacity)
	}
	*/

	pub fn clear(&mut self) {
		self.inner.clear()
	}

	pub fn into_boxed_raw_str(self) -> Box<RawStr> {
		let raw = Box::into_raw(self.inner.into_boxed_slice()) as *mut RawStr;
		unsafe { Box::from_raw(raw) }
	}

	pub fn push(&mut self, b: u8) {
		self.inner.push(b)
	}

	pub fn push_str<T: AsRef<RawStr>>(&mut self, s: T) {
		self.inner.extend_from_slice(s.as_ref().as_bytes())
	}

	pub fn as_mut_bytes(&mut self) -> &mut Vec<u8> {
		&mut self.inner
	}

	pub fn to_string(self) -> Result<String, FromUtf8Error> {
		String::from_utf8(self.into_bytes())
	}

	/// Convert to an OsString.
	///
	/// On Unix, it never fails.
	/// On other platforms, it must be encoded as UTF-8.
	///
	/// A never-failing version for Unix only is available as
	/// [`unix::RawStringExt::into_osstring`](struct.RawString.html#method.into_osstring).
	pub fn to_osstring(self) -> Result<OsString, FromUtf8Error> {
		self.to_osstring_()
	}

	/// Convert to a PathBuf.
	///
	/// On Unix, it never fails.
	/// On other platforms, it must be encoded as UTF-8.
	///
	/// A never-failing version for Unix only is available as
	/// [`unix::RawStringExt::into_pathbuf`](struct.RawString.html#method.into_pathbuf).
	pub fn to_pathbuf(self) -> Result<PathBuf, FromUtf8Error> {
		Ok(PathBuf::from(self.to_osstring()?))
	}

	#[cfg(unix)]
	fn to_osstring_(self) -> Result<OsString, FromUtf8Error> {
		use std::os::unix::ffi::OsStringExt;
		Ok(OsString::from_vec(self.into_bytes()))
	}

	#[cfg(not(unix))]
	fn to_osstring_(self) -> Result<OsString, FromUtf8Error> {
		Ok(OsString::from(self.to_string()?))
	}
}

// Deref / DerefMut {{{

impl Deref for RawString {
	type Target = RawStr;
	fn deref(&self) -> &RawStr {
		RawStr::from_bytes(&self.inner)
	}
}

impl DerefMut for RawString {
	fn deref_mut(&mut self) -> &mut RawStr {
		RawStr::from_mut_bytes(&mut self.inner)
	}
}

// }}}

// Index {{{

macro_rules! impl_index {
	($range:ty) => {
		impl Index<$range> for RawString {
			type Output = RawStr;
			fn index(&self, index: $range) -> &RawStr {
				&self.deref()[index]
			}
		}
		impl IndexMut<$range> for RawString {
			fn index_mut(&mut self, index: $range) -> &mut RawStr {
				&mut self.deref_mut()[index]
			}
		}
	};
}

impl_index!(Range<usize>);
impl_index!(RangeFrom<usize>);
impl_index!(RangeFull);
impl_index!(RangeInclusive<usize>);
impl_index!(RangeTo<usize>);
impl_index!(RangeToInclusive<usize>);

// }}}

// Borrow / ToOwned {{{

impl Borrow<RawStr> for RawString {
	fn borrow(&self) -> &RawStr {
		RawStr::from_bytes(&self.inner)
	}
}

impl ToOwned for RawStr {
	type Owned = RawString;
	fn to_owned(&self) -> RawString {
		RawString::from_bytes(self.as_bytes().to_owned())
	}
}

// }}}

// AsRef {{{

impl AsRef<RawStr> for RawString {
	fn as_ref(&self) -> &RawStr {
		RawStr::from_bytes(&self.inner)
	}
}

impl AsRef<[u8]> for RawString {
	fn as_ref(&self) -> &[u8] {
		&self.inner
	}
}

// }}}

// From {{{

impl<'a> From<&'a RawStr> for RawString {
	fn from(src: &'a RawStr) -> RawString {
		RawString::from_bytes(src.as_bytes().to_owned())
	}
}

impl<'a> From<&'a str> for RawString {
	fn from(src: &'a str) -> RawString {
		RawString::from_bytes(src.as_bytes().to_owned())
	}
}

impl<'a> From<&'a [u8]> for RawString {
	fn from(src: &'a [u8]) -> RawString {
		RawString::from_bytes(src.to_owned())
	}
}

impl From<String> for RawString {
	fn from(src: String) -> RawString {
		RawString::from_bytes(src.into_bytes())
	}
}

impl From<Vec<u8>> for RawString {
	fn from(src: Vec<u8>) -> RawString {
		RawString::from_bytes(src)
	}
}

// }}}

// Display / Debug {{{

impl Display for RawString {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		Display::fmt(self.deref(), f)
	}
}

impl Debug for RawString {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		Debug::fmt(self.deref(), f)
	}
}

// }}}
