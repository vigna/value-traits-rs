use core::ops::RangeBounds;

use crate::helpers::range_compose;
use crate::helpers::Iter;
use crate::helpers::SubsliceImpl;
use crate::traits::iter::*;
use crate::traits::slices::*;

/// The mutable version of [`SubsliceImpl`], see its doc for more details as
/// the same caveats apply.
pub struct SubsliceImplMut<'a, T> {
    slice: &'a mut T,
    range: core::ops::Range<usize>,
}

impl<'a, T> SubsliceImplMut<'a, T> {
    /// Creates a new subslice implementation from the given slice and range.
    ///
    /// # Safety
    /// The caller must ensure that the range is valid for the slice.
    #[inline]
    pub const fn new(slice: &'a mut T, range: core::ops::Range<usize>) -> Self {
        Self { slice, range }
    }
}

impl<'a, T: SliceByValue> SliceByValue for SubsliceImplMut<'a, T> {
    type Value = T::Value;

    #[inline]
    fn len(&self) -> usize {
        self.range.len()
    }
}

impl<'a, T: SliceByValueGet> SliceByValueGet for SubsliceImplMut<'a, T> {
    #[inline]
    unsafe fn get_value_unchecked(&self, index: usize) -> Self::Value {
        self.slice.get_value_unchecked(self.range.start + index)
    }
}

/// The subslice of this subslice is a `SublsliceImpl` because this way
/// we should be able to get multiple immutable subslices from a mutable one
impl<'a, 'b, T: SliceByValueGet> SliceByValueSubsliceGat<'b> for SubsliceImplMut<'a, T> {
    type Subslice = SubsliceImpl<'b, T>;
}

impl<'a, T: SliceByValueGet, R: RangeBounds<usize> + RangeCheck> SliceByValueSubsliceRange<R>
    for SubsliceImplMut<'a, T>
{
    #[inline]
    unsafe fn get_subslice_unchecked(&self, range: R) -> Subslice<'_, Self> {
        SubsliceImpl::new(&*self.slice, range_compose(&self.range, range))
    }
}

impl<'a, T: SliceByValueGet> IterableByValue for SubsliceImplMut<'a, T> {
    type Item = T::Value;
    type Iter<'b>
        = Iter<'b, Self>
    where
        Self: 'b;

    #[inline]
    fn iter_value(&self) -> Self::Iter<'_> {
        Iter::new(self)
    }
}

impl<'a, T: SliceByValueGet> IterableByValueFrom for SubsliceImplMut<'a, T> {
    type IterFrom<'b>
        = Iter<'b, Self>
    where
        Self: 'b;

    #[inline]
    fn iter_value_from(&self, from: usize) -> Self::IterFrom<'_> {
        Iter::new_from(self, from)
    }
}

// //////////////////////////////////////////////////////////////////////////////
// The code above is like [`SubsliceImpl`], here's the additional methods for
// mutability

impl<'a, T: SliceByValueSet> SliceByValueSet for SubsliceImplMut<'a, T> {
    #[inline]
    unsafe fn set_value_unchecked(&mut self, index: usize, value: Self::Value) {
        self.slice
            .set_value_unchecked(self.range.start + index, value);
    }
}

impl<'a, T: SliceByValueRepl> SliceByValueRepl for SubsliceImplMut<'a, T> {
    #[inline]
    unsafe fn replace_value_unchecked(&mut self, index: usize, value: Self::Value) -> Self::Value {
        self.slice
            .replace_value_unchecked(self.range.start + index, value)
    }
}

impl<'a, T: SliceByValueSet + SliceByValueRepl> SliceByValueSubsliceGatMut<'a>
    for SubsliceImplMut<'_, T>
{
    type Subslice = SubsliceImplMut<'a, T>;
}

impl<'a, T: SliceByValueSet + SliceByValueRepl, R: core::ops::RangeBounds<usize> + RangeCheck>
    SliceByValueSubsliceRangeMut<R> for SubsliceImplMut<'a, T>
{
    #[inline]
    unsafe fn get_subslice_unchecked_mut(&mut self, range: R) -> SubsliceMut<'_, Self> {
        SubsliceImplMut {
            slice: self.slice,
            range: range_compose(&self.range, range),
        }
    }
}

/// This module is a type-level assertion that the `SubsliceImplMut` struct
/// implements the `SliceByValueSubslice` trait for a type that implements
/// `SliceByValueGet` and it implements the `SliceByValueSubsliceMut` trait
/// for a type that implements `SliceByValueSet`.
#[cfg(test)]
mod __type_assert {
    use crate::slices::{SliceByValueRepl, SliceByValueSubsliceRange};

    use super::{
        SliceByValueGet, SliceByValueSet, SliceByValueSubslice, SliceByValueSubsliceMut,
        SubsliceImplMut,
    };

    #[allow(dead_code)] // the code is never called, but is used to enforce the trait bounds
    fn type_assert(mut slice: impl SliceByValueGet) {
        fn assert(_: impl SliceByValueSubslice) {}
        let slice = SubsliceImplMut {
            slice: &mut slice,
            range: 1..2,
        };
        // we should be able to get multiple immutable subslices from a mutable one
        let a = slice.index_subslice(0..1);
        let b = slice.index_subslice(1..2);
        assert(a);
        assert(b);
    }

    #[allow(dead_code)] // the code is never called, but is used to enforce the trait bounds
    fn type_assert_mut(mut slice: impl SliceByValueSet + SliceByValueRepl) {
        fn assert(_: impl SliceByValueSubsliceMut) {}
        let subslice = SubsliceImplMut {
            slice: &mut slice,
            range: 1..2,
        };
        assert(subslice);
        // TODO!: also mutable subslices
    }
}
