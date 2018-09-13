use std;
use std::char::REPLACEMENT_CHARACTER;
use std::convert::AsRef;
use std::ffi::OsStr;
use std::fmt::{Debug, Display, Formatter, Write};
use std::mem::transmute;
use std::ops::{Index, IndexMut};
use std::path::Path;
use std::str::{from_utf8, Utf8Error};

mod index;
mod utf8chunks;

pub use self::index::{RawStrIndex, RawStrIndexOutput};
pub use self::utf8chunks::{Utf8Chunk, Utf8ChunksIter};

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

	pub fn from_bytes_mut(bytes: &mut [u8]) -> &mut Self {
		unsafe { transmute::<&mut [u8], &mut Self>(bytes) }
	}

	pub fn as_bytes_mut(&mut self) -> &mut [u8] {
		&mut self.inner
	}

	pub fn len(&self) -> usize {
		self.inner.len()
	}

	pub fn is_empty(&self) -> bool {
		self.inner.is_empty()
	}

	pub fn as_ptr(&self) -> *const u8 {
		self.inner.as_ptr()
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
			.map(|(a, b)| (a, RawStr::from_bytes_mut(b)))
	}

	pub fn split_last(&self) -> Option<(&u8, &RawStr)> {
		self.inner
			.split_last()
			.map(|(a, b)| (a, RawStr::from_bytes(b)))
	}

	pub fn split_last_mut(&mut self) -> Option<(&mut u8, &mut RawStr)> {
		self.inner
			.split_last_mut()
			.map(|(a, b)| (a, RawStr::from_bytes_mut(b)))
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
		(RawStr::from_bytes_mut(a), RawStr::from_bytes_mut(b))
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

	pub fn get<I: RawStrIndex>(&self, index: I) -> Option<&I::Output> {
		index.get(self)
	}

	pub fn get_mut<I: RawStrIndex>(&mut self, index: I) -> Option<&mut I::Output> {
		index.get_mut(self)
	}

	pub unsafe fn get_unchecked<I: RawStrIndex>(&self, index: I) -> &I::Output {
		index.get_unchecked(self)
	}

	pub unsafe fn get_unchecked_mut<I: RawStrIndex>(&mut self, index: I) -> &mut I::Output {
		index.get_unchecked_mut(self)
	}

	pub unsafe fn slice_unchecked(&self, begin: usize, end: usize) -> &RawStr {
		self.get_unchecked(begin..end)
	}

	pub unsafe fn slice_mut_unchecked(&mut self, begin: usize, end: usize) -> &mut RawStr {
		self.get_unchecked_mut(begin..end)
	}

	pub fn bytes(&self) -> std::slice::Iter<u8> {
		self.inner.iter()
	}

	pub fn bytes_mut(&mut self) -> std::slice::IterMut<u8> {
		self.inner.iter_mut()
	}

	/// Iterate over chunks of valid UTF-8.
	///
	/// The iterator iterates over the chunks of valid UTF-8 separated by any
	/// broken characters, which could be replaced by the unicode replacement
	/// character.
	pub fn utf8_chunks(&self) -> Utf8ChunksIter {
		Utf8ChunksIter { bytes: &self.inner }
	}

	// Things that could be added:
	//   pub fn split_whitespace(&self) -> SplitWhitespace
	//   pub fn lines(&self) -> Lines
	//
	//   pub fn contains<'a, P: RawPattern<'a>>(&'a self, pat: P) -> bool
	//   pub fn starts_with<'a, P: RawPattern<'a>>(&'a self, pat: P) -> bool
	//   pub fn ends_with<'a, P: RawPattern<'a>>(&'a self, pat: P) -> bool
	//   pub fn find<'a, P: RawPattern<'a>>(&'a self, pat: P) -> Option<usize>
	//   pub fn rfind<'a, P: RawPattern<'a>>(&'a self, pat: P) -> Option<usize>
	//   pub fn split<'a, P: RawPattern<'a>>(&'a self, pat: P) -> Split<'a, P>
	//   pub fn rsplit<'a, P: RawPattern<'a>>(&'a self, pat: P) -> RSplit<'a, P>
	//   pub fn split_terminator<'a, P: RawPattern<'a>>(&'a self, pat: P) -> SplitTerminator<'a, P>
	//   pub fn rsplit_terminator<'a, P: RawPattern<'a>>(&'a self, pat: P) -> RSplitTerminator<'a, P>
	//   pub fn splitn<'a, P: RawPattern<'a>>(&'a self, n: usize, pat: P) -> Split<'a, P>
	//   pub fn rsplitn<'a, P: RawPattern<'a>>(&'a self, n: usize, pat: P) -> RSplit<'a, P>
	//   pub fn matches<'a, P: RawPattern<'a>>(&'a self, pat: P) -> Matches<'a, P>
	//   pub fn rmatches<'a, P: RawPattern<'a>>(&'a self, pat: P) -> Matches<'a, P>
	//   pub fn match_indices<'a, P: RawPattern<'a>>(&'a self, pat: P) -> Matches<'a, P>
	//   pub fn rmatch_indices<'a, P: RawPattern<'a>>(&'a self, pat: P) -> Matches<'a, P>
	//   pub fn trim
	//   pub fn trim_left
	//   pub fn trim_right
	//   pub fn trim_matches <RawPattern>
	//   pub fn trim_left_matches <RawPattern>
	//   pub fn trim_right_matches <RawPattern>
	//   // (and RawPattern)
	//
	//   pub fn eq_ignore_ascii_case
	//   pub fn make_ascii_uppercase
	//   pub fn make_ascii_lowercase
	//   pub fn replace (RawPattern -> AsRef<RawStr>) -> RawString
	//   pub fn replace_n (RawPattern -> AsRef<RawStr>, n) -> RawString
	//
	//   pub fn is_utf8_char_boundary(&self, index: usize) -> bool
	//   pub fn utf8_chars() -> Utf8Chars
	//   pub fn utf8_char_indices() -> Utf8CharIndices
	//   pub fn encode_utf16(&self) -> EncodeUtf16

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
		RawStr::from_bytes_mut(&mut [])
	}
}

// }}}

// Index {{{

impl<I: RawStrIndex> Index<I> for RawStr {
	type Output = I::Output;
	fn index(&self, index: I) -> &I::Output {
		index.index(self)
	}
}

impl<I: RawStrIndex> IndexMut<I> for RawStr {
	fn index_mut(&mut self, index: I) -> &mut I::Output {
		index.index_mut(self)
	}
}

// }}}

// IntoIterator {{{

impl<'a> IntoIterator for &'a RawStr {
	type Item = &'a u8;
	type IntoIter = std::slice::Iter<'a, u8>;
	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<'a> IntoIterator for &'a mut RawStr {
	type Item = &'a mut u8;
	type IntoIter = std::slice::IterMut<'a, u8>;
	fn into_iter(self) -> Self::IntoIter {
		self.bytes_mut()
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
		for Utf8Chunk { valid, broken } in self.utf8_chunks() {
			f.write_str(valid)?;
			if !broken.is_empty() {
				f.write_char(REPLACEMENT_CHARACTER)?;
			}
		}
		Ok(())
	}
}

// }}}

// Debug {{{

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

impl Debug for RawStr {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		f.write_char('"')?;
		for Utf8Chunk { valid, broken } in self.utf8_chunks() {
			write_escaped_str(f, valid)?;
			for &b in broken {
				write!(f, "\\x{:02x}", b)?;
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
