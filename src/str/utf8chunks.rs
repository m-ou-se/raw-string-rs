use std::iter::FusedIterator;
use std::str::{from_utf8, from_utf8_unchecked};

/// An iterator over chunks of valid UTF-8 in a RawStr.
///
/// See [`RawStr::utf8_chunks`](struct.RawStr.html#method.utf8_chunks).
pub struct Utf8ChunksIter<'a> {
	pub(super) bytes: &'a [u8],
}

/// A chunk of valid UTF-8, possibly followed by a broken character encoding.
pub struct Utf8Chunk<'a> {
	/// A valid UTF-8 piece, at the start, end, or between broken chars.
	///
	/// Empty between adjacent broken chars.
	pub valid: &'a str,

	/// A broken char.
	///
	/// Can only be empty in the last chunk.
	///
	/// Should be replaced by a single unicode replacement character, if not empty.
	pub broken: &'a [u8],
}

impl<'a> Iterator for Utf8ChunksIter<'a> {
	type Item = Utf8Chunk<'a>;

	fn next(&mut self) -> Option<Utf8Chunk<'a>> {
		if self.bytes.is_empty() {
			return None;
		}
		match from_utf8(self.bytes) {
			Ok(s) => {
				self.bytes = &self.bytes[s.len()..];
				Some(Utf8Chunk {
					valid: s,
					broken: &self.bytes[..0],
				})
			}
			Err(e) => {
				let (valid, rest) = self.bytes.split_at(e.valid_up_to());
				let valid = unsafe { from_utf8_unchecked(valid) };
				let (broken, rest) = rest.split_at(e.error_len().unwrap_or(rest.len()));
				self.bytes = rest;
				Some(Utf8Chunk { valid, broken })
			}
		}
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		if self.bytes.is_empty() {
			(0, Some(0))
		} else {
			(1, None)
		}
	}
}

impl<'a> FusedIterator for Utf8ChunksIter<'a> {}
