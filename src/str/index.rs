use std::slice::SliceIndex;

use super::RawStr;

/// The equivalent of `SliceIndex` for `RawStr`.
///
/// # Usage
///
/// Normally, this trait is not used directly.
/// Its functionality is exposed through `Index` and the `get` methods of `RawStr`.
///
/// # Implementors
///
/// `RawStrIndex` is automatically implemented for all that implement `SliceIndex<[u8]>`:
///
///   - a `SliceIndex<[u8], Output=[u8]>` automatically implements `RawStrIndex<Output=RawStr>`, and
///   - a `SliceIndex<[u8], Output=u8>` automatically implements `RawStrIndex<Output=u8>`.
///
/// # Examples
///
/// ```
/// # use raw_string::RawStr;
/// let s = RawStr::from_str("hello world");
/// let hello: &RawStr = &s[..5]; // This is a slice.
/// let space: u8 = s[5]; // This is a single byte
/// assert_eq!(hello, "hello");
/// assert_eq!(space, b' ');
/// ```
pub trait RawStrIndex {
	/// `RawStr` (for ranges) or `u8` (for single indexes).
	type Output: ?Sized;
	/// Get the range or byte from the given `&RawStr`.
	fn get(self, s: &RawStr) -> Option<&Self::Output>;
	/// Get the (mutable) range or byte from the given `&mut RawStr`.
	fn get_mut(self, s: &mut RawStr) -> Option<&mut Self::Output>;
	/// Like `get`, but unsafe and unchecked.
	unsafe fn get_unchecked(self, s: &RawStr) -> &Self::Output;
	/// Like `get_mut`, but unsafe and unchecked.
	unsafe fn get_unchecked_mut(self, s: &mut RawStr) -> &mut Self::Output;
	/// Like `get`, but panics on failure.
	fn index(self, s: &RawStr) -> &Self::Output;
	/// Like `get_mut`, but panics on failure.
	fn index_mut(self, s: &mut RawStr) -> &mut Self::Output;
}

#[doc(hidden)]
pub trait RawStrIndexOutput {
	type Output: ?Sized;
	fn into(&self) -> &Self::Output;
	fn into_mut(&mut self) -> &mut Self::Output;
}

impl RawStrIndexOutput for [u8] {
	type Output = RawStr;
	#[inline]
	fn into(&self) -> &RawStr {
		RawStr::from_bytes(self)
	}
	#[inline]
	fn into_mut(&mut self) -> &mut RawStr {
		RawStr::from_bytes_mut(self)
	}
}

impl RawStrIndexOutput for u8 {
	type Output = u8;
	#[inline]
	fn into(&self) -> &u8 {
		self
	}
	#[inline]
	fn into_mut(&mut self) -> &mut u8 {
		self
	}
}

impl<I> RawStrIndex for I
where
	I: SliceIndex<[u8]>,
	I::Output: RawStrIndexOutput + 'static,
{
	type Output = <<I as SliceIndex<[u8]>>::Output as RawStrIndexOutput>::Output;
	#[inline]
	fn get(self, s: &RawStr) -> Option<&Self::Output> {
		s.as_bytes().get(self).map(RawStrIndexOutput::into)
	}
	#[inline]
	fn get_mut(self, s: &mut RawStr) -> Option<&mut Self::Output> {
		s.as_bytes_mut()
			.get_mut(self)
			.map(RawStrIndexOutput::into_mut)
	}
	#[inline]
	unsafe fn get_unchecked(self, s: &RawStr) -> &Self::Output {
		RawStrIndexOutput::into(s.as_bytes().get_unchecked(self))
	}
	#[inline]
	unsafe fn get_unchecked_mut(self, s: &mut RawStr) -> &mut Self::Output {
		RawStrIndexOutput::into_mut(s.as_bytes_mut().get_unchecked_mut(self))
	}
	#[inline]
	fn index(self, s: &RawStr) -> &Self::Output {
		RawStrIndexOutput::into(&s.as_bytes()[self])
	}
	#[inline]
	fn index_mut(self, s: &mut RawStr) -> &mut Self::Output {
		RawStrIndexOutput::into_mut(&mut s.as_bytes_mut()[self])
	}
}
