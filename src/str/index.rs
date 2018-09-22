use std::slice::SliceIndex;

use super::RawStr;

pub trait RawStrIndex {
	type Output: ?Sized;
	fn get(self, s: &RawStr) -> Option<&Self::Output>;
	fn get_mut(self, s: &mut RawStr) -> Option<&mut Self::Output>;
	unsafe fn get_unchecked(self, s: &RawStr) -> &Self::Output;
	unsafe fn get_unchecked_mut(self, s: &mut RawStr) -> &mut Self::Output;
	fn index(self, s: &RawStr) -> &Self::Output;
	fn index_mut(self, s: &mut RawStr) -> &mut Self::Output;
}

pub trait RawStrIndexOutput {
	type Output: ?Sized;
	fn into(&self) -> &Self::Output;
	fn into_mut(&mut self) -> &mut Self::Output;
}

impl RawStrIndexOutput for [u8] {
	type Output = RawStr;
	fn into(&self) -> &RawStr {
		RawStr::from_bytes(self)
	}
	fn into_mut(&mut self) -> &mut RawStr {
		RawStr::from_bytes_mut(self)
	}
}

impl RawStrIndexOutput for u8 {
	type Output = u8;
	fn into(&self) -> &u8 {
		self
	}
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
	fn get(self, s: &RawStr) -> Option<&Self::Output> {
		s.as_bytes().get(self).map(RawStrIndexOutput::into)
	}
	fn get_mut(self, s: &mut RawStr) -> Option<&mut Self::Output> {
		s.as_bytes_mut()
			.get_mut(self)
			.map(RawStrIndexOutput::into_mut)
	}
	unsafe fn get_unchecked(self, s: &RawStr) -> &Self::Output {
		RawStrIndexOutput::into(s.as_bytes().get_unchecked(self))
	}
	unsafe fn get_unchecked_mut(self, s: &mut RawStr) -> &mut Self::Output {
		RawStrIndexOutput::into_mut(s.as_bytes_mut().get_unchecked_mut(self))
	}
	fn index(self, s: &RawStr) -> &Self::Output {
		RawStrIndexOutput::into(&s.as_bytes()[self])
	}
	fn index_mut(self, s: &mut RawStr) -> &mut Self::Output {
		RawStrIndexOutput::into_mut(&mut s.as_bytes_mut()[self])
	}
}
