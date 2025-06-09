use crate::slices::SliceByValueGet;
use core::ops::Range;

/// This is a generic implementation of an iterator over anything that implements
/// [`SliceByValueGet`].
///
/// The purpose of this is mostly for convenience when implementing
/// [`IterableByValue`](crate::traits::iter::IterableByValue) and
/// [`IterableByValueFrom`](crate::traits::iter::IterableByValueFrom) for
/// a type that implements [`SliceByValueGet`].
///
/// # Example
///
/// ```rust
/// use value_traits::slices::*;
/// use value_traits::iter::*;
/// use value_traits::helpers::Iter;
///
/// /// An implicit vector of the first `len` squares.
/// pub struct ImplicitVector{
///     len: usize
/// };
///
/// impl SliceByValue for ImplicitVector {
///     type Value = usize;
///     fn len(&self) -> usize {
///         self.len
///     }
/// }
///
/// impl SliceByValueGet for ImplicitVector {
///    unsafe fn get_value_unchecked(&self, index: usize) -> usize {
///        index * index
///    }
/// }
///
/// impl IterableByValue for ImplicitVector {
///    type Item = usize;
///    type Iter<'a> = Iter<'a, Self>;
///    fn iter_value(&self) -> Self::Iter<'_> {
///        Iter::new(self)
///    }
/// }
///
/// impl IterableByValueFrom for ImplicitVector {
///     type IterFrom<'a> = Iter<'a, Self>;
///     fn iter_value_from(&self, from: usize) -> Self::IterFrom<'_> {
///         Iter::new_from(self, from)
///     }
/// }
/// ```
pub struct Iter<'a, T: SliceByValueGet> {
    subslice: &'a T,
    /// The range of indices we are iterating over.
    range: Range<usize>,
}

impl<'a, T: SliceByValueGet> Iter<'a, T> {
    pub fn new(subslice: &'a T) -> Self {
        Self {
            subslice,
            range: 0..subslice.len(),
        }
    }
    pub fn new_from(subslice: &'a T, from: usize) -> Self {
        let len = subslice.len();
        if from > len {
            panic!("index out of bounds: the len is {len} but the starting index is {from}");
        }
        Self {
            subslice,
            range: from..len,
        }
    }
}

/// Ideally we would like to also implement [`Iterator::advance_by`], but it is
/// nightly, and [`Iterator::skip`], [`Iterator::take`], [`Iterator::step_by`],
/// as we can do it more efficiently, but the [`Iterator`] trait definition
/// doesn't allow to return an arbitrary type.
impl<'a, T: SliceByValueGet> Iterator for Iter<'a, T> {
    type Item = T::Value;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.range.is_empty() {
            return None;
        }
        let value = unsafe { self.subslice.get_value_unchecked(self.range.start) };
        self.range.start += 1;
        Some(value)
    }

    /// Since we are indexing into a subslice, we can implement
    /// [`Iterator::nth`] without needing to consume the first `n` elements.
    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if n >= self.range.end {
            return None;
        }
        let value = unsafe { self.subslice.get_value_unchecked(self.range.start + n) };
        self.range.start += n + 1;
        Some(value)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.range.len();
        (len, Some(len))
    }
}

impl<'a, T: SliceByValueGet> DoubleEndedIterator for Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.range.is_empty() {
            return None;
        }
        self.range.end -= 1;
        let value = unsafe { self.subslice.get_value_unchecked(self.range.end) };
        Some(value)
    }
}

impl<'a, T: SliceByValueGet> ExactSizeIterator for Iter<'a, T> {
    #[inline]
    fn len(&self) -> usize {
        self.range.len()
    }
}
