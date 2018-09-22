use std;
use std::borrow::{Borrow, ToOwned};
use std::cmp::Ordering;
use std::ffi::OsString;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Deref, DerefMut, RangeBounds};
use std::path::PathBuf;
use std::string::FromUtf8Error;
use std::vec::Drain;
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

	pub fn capacity(&self) -> usize {
		self.inner.capacity()
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

	pub fn truncate(&mut self, new_len: usize) {
		self.inner.truncate(new_len)
	}

	pub fn pop(&mut self) -> Option<u8> {
		self.inner.pop()
	}

	pub fn remove(&mut self, idx: usize) -> u8 {
		self.inner.remove(idx)
	}

	pub fn retain<F: FnMut(u8) -> bool>(&mut self, mut f: F) {
		self.inner.retain(|x| f(*x))
	}

	pub fn insert(&mut self, idx: usize, b: u8) {
		self.inner.insert(idx, b)
	}

	pub fn insert_str<T: AsRef<RawStr>>(&mut self, idx: usize, s: T) {
		self.inner.splice(idx..idx, s.as_ref().bytes());
	}

	pub fn split_off(&mut self, at: usize) -> RawString {
		RawString::from_bytes(self.inner.split_off(at))
	}

	pub fn drain<R: RangeBounds<usize>>(&mut self, range: R) -> Drain<u8> {
		self.inner.drain(range)
	}

	pub fn replace_range<R: RangeBounds<usize>, T: AsRef<RawStr>>(
		&mut self,
		range: R,
		replace_with: T,
	) {
		self.inner.splice(range, replace_with.as_ref().bytes());
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
		RawStr::from_bytes_mut(&mut self.inner)
	}
}

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

// {{{ IntoIterator

impl IntoIterator for RawString {
	type Item = u8;
	type IntoIter = std::vec::IntoIter<u8>;
	fn into_iter(self) -> Self::IntoIter {
		self.inner.into_iter()
	}
}

impl<'a> IntoIterator for &'a RawString {
	type Item = u8;
	type IntoIter = std::iter::Cloned<std::slice::Iter<'a, u8>>;
	fn into_iter(self) -> Self::IntoIter {
		self.bytes()
	}
}

impl<'a> IntoIterator for &'a mut RawString {
	type Item = &'a mut u8;
	type IntoIter = std::slice::IterMut<'a, u8>;
	fn into_iter(self) -> Self::IntoIter {
		self.bytes_mut()
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

// {{{ PartialEq / PartialOrd

macro_rules! impl_ord {
	($t:ty) => {
		impl<'a> PartialEq<$t> for RawString {
			fn eq(&self, other: &$t) -> bool {
				<RawStr as PartialEq>::eq(self, other.as_ref())
			}
		}
		impl<'a> PartialEq<RawString> for $t {
			fn eq(&self, other: &RawString) -> bool {
				<RawStr as PartialEq>::eq(self.as_ref(), other)
			}
		}
		impl<'a> PartialOrd<$t> for RawString {
			fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
				<RawStr as PartialOrd>::partial_cmp(self, other.as_ref())
			}
		}
		impl<'a> PartialOrd<RawString> for $t {
			fn partial_cmp(&self, other: &RawString) -> Option<Ordering> {
				<RawStr as PartialOrd>::partial_cmp(self.as_ref(), other)
			}
		}
	};
}

impl_ord!(RawStr);
impl_ord!(str);
impl_ord!([u8]);
impl_ord!(&'a RawStr);
impl_ord!(&'a str);
impl_ord!(&'a [u8]);

// }}}
