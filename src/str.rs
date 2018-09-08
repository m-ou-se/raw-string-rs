use std;
use std::convert::AsRef;
use std::fmt::{Debug, Display, Write, Formatter};
use std::mem::transmute;
use std::ops::{
	Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};
use std::str::{from_utf8, from_utf8_unchecked};

/// A `str` with unchecked contents.
///
/// It is basically a `[u8]`, to be interpreted as string.
/// Unlike `str`, there are no guarantees about the contents being valid UTF-8.
/// Unlike `[u8]`, its Display and Debug implementations show a string, not an
/// array of numbers.
///
/// Very similar to the Unix implementation of `OsStr`.
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
