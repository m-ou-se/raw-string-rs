use std;
use std::convert::AsRef;
use std::ffi::OsStr;
use std::fmt::{Debug, Display, Formatter, Write};
use std::mem::transmute;
use std::ops::{
	Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};
use std::path::Path;
use std::str::{from_utf8, from_utf8_unchecked, Utf8Error};

/// A `str` with unchecked contents.
///
/// It is basically a `[u8]`, to be interpreted as string.
/// Unlike `str`, there are no guarantees about the contents being valid UTF-8.
/// Unlike `[u8]`, its Display and Debug implementations show a string, not an
/// array of numbers.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RawStr {
	inner: [u8],
}

impl RawStr {
	pub fn from_bytes(bytes: &[u8]) -> &Self {
		unsafe { transmute::<&[u8], &Self>(bytes) }
	}

	pub fn from_str(bytes: &str) -> &Self {
		Self::from_bytes(bytes.as_bytes())
	}

	pub fn as_bytes(&self) -> &[u8] {
		&self.inner
	}

	pub fn from_mut_bytes(bytes: &mut [u8]) -> &mut Self {
		unsafe { transmute::<&mut [u8], &mut Self>(bytes) }
	}

	pub fn as_mut_bytes(&mut self) -> &mut [u8] {
		&mut self.inner
	}

	pub fn len(&self) -> usize {
		self.inner.len()
	}

	pub fn is_empty(&self) -> bool {
		self.inner.is_empty()
	}

	pub fn first(&self) -> Option<&u8> {
		self.inner.first()
	}

	pub fn first_mut(&mut self) -> Option<&mut u8> {
		self.inner.first_mut()
	}

	pub fn last(&self) -> Option<&u8> {
		self.inner.last()
	}

	pub fn last_mut(&mut self) -> Option<&mut u8> {
		self.inner.last_mut()
	}

	pub fn split_first(&self) -> Option<(&u8, &RawStr)> {
		self.inner
			.split_first()
			.map(|(a, b)| (a, RawStr::from_bytes(b)))
	}

	pub fn split_first_mut(&mut self) -> Option<(&mut u8, &mut RawStr)> {
		self.inner
			.split_first_mut()
			.map(|(a, b)| (a, RawStr::from_mut_bytes(b)))
	}

	pub fn split_last(&self) -> Option<(&u8, &RawStr)> {
		self.inner
			.split_last()
			.map(|(a, b)| (a, RawStr::from_bytes(b)))
	}

	pub fn split_last_mut(&mut self) -> Option<(&mut u8, &mut RawStr)> {
		self.inner
			.split_last_mut()
			.map(|(a, b)| (a, RawStr::from_mut_bytes(b)))
	}

	pub fn iter(&self) -> std::slice::Iter<u8> {
		self.inner.iter()
	}

	pub fn split_at(&self, mid: usize) -> (&RawStr, &RawStr) {
		let (a, b) = self.inner.split_at(mid);
		(RawStr::from_bytes(a), RawStr::from_bytes(b))
	}

	pub fn split_at_mut(&mut self, mid: usize) -> (&mut RawStr, &mut RawStr) {
		let (a, b) = self.inner.split_at_mut(mid);
		(RawStr::from_mut_bytes(a), RawStr::from_mut_bytes(b))
	}

	pub fn contains(&self, x: &u8) -> bool {
		self.inner.contains(x)
	}

	pub fn starts_with<T: AsRef<RawStr>>(&self, x: T) -> bool {
		self.inner.starts_with(x.as_ref().as_bytes())
	}

	pub fn ends_with<T: AsRef<RawStr>>(&self, x: T) -> bool {
		self.inner.ends_with(x.as_ref().as_bytes())
	}

	pub fn is_ascii(&self) -> bool {
		self.inner.is_ascii()
	}

	pub fn to_str(&self) -> Result<&str, Utf8Error> {
		from_utf8(self.as_bytes())
	}

	/// Convert to an OsStr.
	///
	/// On Unix, it never fails.
	/// On other platforms, it must be encoded as UTF-8.
	///
	/// A never-failing version for Unix only is available as
	/// [`unix::RawStrExt::as_osstr`](struct.RawStr.html#method.as_osstr).
	pub fn to_osstr(&self) -> Result<&OsStr, Utf8Error> {
		self.to_osstr_()
	}

	/// Convert to a Path.
	///
	/// On Unix, it never fails.
	/// On other platforms, it must be encoded as UTF-8.
	///
	/// A never-failing version for Unix only is available as
	/// [`unix::RawStrExt::as_path`](struct.RawStr.html#method.as_path).
	pub fn to_path(&self) -> Result<&Path, Utf8Error> {
		Ok(Path::new(self.to_osstr()?))
	}

	#[cfg(unix)]
	fn to_osstr_(&self) -> Result<&OsStr, Utf8Error> {
		use std::os::unix::ffi::OsStrExt;
		Ok(OsStr::from_bytes(self.as_bytes()))
	}

	#[cfg(not(unix))]
	fn to_osstr_(&self) -> Result<&OsStr, Utf8Error> {
		Ok(OsStr::new(self.to_str()?))
	}
}

// AsRef {{{

impl AsRef<RawStr> for RawStr {
	fn as_ref(&self) -> &RawStr {
		self
	}
}

impl AsRef<RawStr> for [u8] {
	fn as_ref(&self) -> &RawStr {
		RawStr::from_bytes(self)
	}
}

impl AsRef<RawStr> for str {
	fn as_ref(&self) -> &RawStr {
		RawStr::from_bytes(self.as_bytes())
	}
}

impl AsRef<[u8]> for RawStr {
	fn as_ref(&self) -> &[u8] {
		&self.inner
	}
}

// }}}

// Default {{{

impl<'a> Default for &'a RawStr {
	fn default() -> Self {
		RawStr::from_bytes(&[])
	}
}

impl<'a> Default for &'a mut RawStr {
	fn default() -> Self {
		RawStr::from_mut_bytes(&mut [])
	}
}

// }}}

// Index {{{

macro_rules! impl_index {
	($range:ty) => {
		impl Index<$range> for RawStr {
			type Output = RawStr;
			fn index(&self, index: $range) -> &RawStr {
				RawStr::from_bytes(&self.as_bytes()[index])
			}
		}
		impl IndexMut<$range> for RawStr {
			fn index_mut(&mut self, index: $range) -> &mut RawStr {
				RawStr::from_mut_bytes(&mut self.as_mut_bytes()[index])
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

// IntoIterator {{{

impl<'a> IntoIterator for &'a RawStr {
	type Item = &'a u8;
	type IntoIter = std::slice::Iter<'a, u8>;
	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

// }}}

// From {{{

impl<'a> From<&'a str> for &'a RawStr {
	fn from(src: &'a str) -> &'a RawStr {
		RawStr::from_str(src)
	}
}

impl<'a> From<&'a [u8]> for &'a RawStr {
	fn from(src: &'a [u8]) -> &'a RawStr {
		RawStr::from_bytes(src)
	}
}

// }}}

// Display {{{

impl Display for RawStr {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		let mut bytes = self.as_bytes();
		while !bytes.is_empty() {
			match from_utf8(bytes) {
				Ok(s) => {
					f.write_str(s)?;
					break;
				}
				Err(e) => {
					let (valid, rest) = bytes.split_at(e.valid_up_to());
					f.write_str(unsafe { from_utf8_unchecked(valid) })?;
					f.write_char('\u{FFFD}')?;
					match e.error_len() {
						Some(n) => bytes = &rest[n..],
						None => break,
					}
				}
			}
		}
		Ok(())
	}
}

// }}}

// Debug {{{

impl Debug for RawStr {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		fn write_escaped_str(f: &mut std::fmt::Formatter, s: &str) -> std::fmt::Result {
			let mut written = 0;
			for (i, c) in s.char_indices() {
				let e = c.escape_debug();
				if e.len() != 1 {
					f.write_str(&s[written..i])?;
					for c in e {
						f.write_char(c)?;
					}
					written = i + c.len_utf8();
				}
			}
			f.write_str(&s[written..])
		}
		f.write_char('"')?;
		let mut bytes = self.as_bytes();
		while !bytes.is_empty() {
			match from_utf8(bytes) {
				Ok(s) => {
					write_escaped_str(f, s)?;
					break;
				}
				Err(e) => {
					let (valid, rest) = bytes.split_at(e.valid_up_to());
					write_escaped_str(f, unsafe { from_utf8_unchecked(valid) })?;
					let n = e.error_len().unwrap_or(rest.len());
					for i in 0..n {
						write!(f, "\\x{:02x}", rest[i])?;
					}
					bytes = &rest[n..];
				}
			}
		}
		f.write_char('"')
	}
}

// }}}

// Tests {{{

#[test]
fn test_display() {
	let a: &RawStr = "1\" μs / °C".as_ref();
	assert_eq!(&format!("{}", a), "1\" μs / °C");

	let b: &RawStr = b"1 \xFF \xce\xbcs / \xc2\xb0C"[..].as_ref();
	assert_eq!(&format!("{}", b), "1 \u{FFFD} μs / °C");
}

#[test]
fn test_debug() {
	let a: &RawStr = "1\" μs / °C".as_ref();
	assert_eq!(&format!("{:?}", a), "\"1\\\" μs / °C\"");

	let b: &RawStr = b"1 \xFF \xce\xbcs / \xc2\xb0C"[..].as_ref();
	assert_eq!(&format!("{:?}", b), "\"1 \\xff μs / °C\"");
}

// }}}
