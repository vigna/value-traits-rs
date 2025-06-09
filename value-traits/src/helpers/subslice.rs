/*
 * SPDX-FileCopyrightText: 2025 Tommaso Fontana
 * SPDX-FileCopyrightText: 2025 Sebastiano Vigna
 * SPDX-FileCopyrightText: 2025 Inria
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use crate::helpers::iter::Iter;
use crate::helpers::range_compose;
use crate::traits::iter::*;
use crate::traits::slices::*;

/// This is a generic subslice implementation that can be used on anything that
/// implements [`SliceByValue`] and [`SliceByValueGet`].
///
/// This is the struct used by the procedural macros [`Subslices`][value_traits_derive::Subslices]
/// which generates the implementations for [`SliceByValueSubsliceGat`]
/// and [`SliceByValueSubsliceRange`] for all ranges.
///
/// Additionally, this struct also implements [`IterableByValue`] and
/// [`IterableByValueFrom`] so that you can iterate over the subslice
/// without needing to implement those traits separately, it does so by
/// using the [`Iter`](`crate::impls::iter::Iter`) helper.
///
/// # Caveats
/// The range `..=usize::MAX` cannot be represented and it will panic if you
/// try to use it.
///
/// # Example
///
/// ```rust
/// use value_traits::slices::*;
/// use value_traits::iter::*;
/// use value_traits::helpers::{Iter, SubsliceImpl, range_compose};
///
/// /// An implicit vector of the first 1000 even numbers.
/// pub struct ImplicitVec;
///
/// impl SliceByValue for ImplicitVec {
///     type Value = u32;
///     fn len(&self) -> usize {
///        1000 // Let's say we have 1000 even numbers
///     }
/// }
///
/// impl SliceByValueGet for ImplicitVec {
///     unsafe fn get_value_unchecked(&self, index: usize) -> Self::Value {
///         (index as u32) * 2
///     }
/// }
///
/// /// Register this subslice implementation as the Subslice type for `ImplicitVec`.
/// impl<'a> SliceByValueSubsliceGat<'a> for ImplicitVec  {
///     type Subslice = SubsliceImpl<'a, Self>;
/// }
///
/// /// Only one range implementation in this example, but you would need to
/// /// implement it for all the ranges you want to support.
/// impl<'a> SliceByValueSubsliceRange<core::ops::Range<usize>> for ImplicitVec {
///    unsafe fn get_subslice_unchecked(
///        &self,
///        range: core::ops::Range<usize>,
///    ) -> Subslice<'_, Self> {
///        SubsliceImpl::new(
///            self,
///            range_compose(0..self.len(), range),
///        )
///    }
/// }
/// ```
pub struct SubsliceImpl<'a, T> {
    slice: &'a T,
    range: core::ops::Range<usize>,
}

impl<'a, T> SubsliceImpl<'a, T> {
    /// Creates a new subslice implementation from the given slice and range.
    ///
    /// # Safety
    /// The caller must ensure that the range is valid for the slice.
    #[inline]
    pub const fn new(slice: &'a T, range: core::ops::Range<usize>) -> Self {
        Self { slice, range }
    }
}

impl<'a, T: SliceByValue> SliceByValue for SubsliceImpl<'a, T> {
    type Value = T::Value;

    #[inline]
    fn len(&self) -> usize {
        self.range.len()
    }
}

impl<'a, T: SliceByValueGet> SliceByValueGet for SubsliceImpl<'a, T> {
    #[inline]
    unsafe fn get_value_unchecked(&self, index: usize) -> Self::Value {
        self.slice.get_value_unchecked(self.range.start + index)
    }
}

/// The subslice of this subslice implementation is itself as subslices get flatten
impl<'a, 'b, T: SliceByValueGet> SliceByValueSubsliceGat<'b> for SubsliceImpl<'a, T> {
    type Subslice = SubsliceImpl<'b, T>;
}

impl<'a, T: SliceByValueGet, R: core::ops::RangeBounds<usize> + core::fmt::Debug>
    SliceByValueSubsliceRange<R> for SubsliceImpl<'a, T>
{
    #[inline]
    unsafe fn get_subslice_unchecked(&self, range: R) -> Subslice<'_, Self> {
        SubsliceImpl {
            slice: self.slice,
            range: range_compose(&self.range, range),
        }
    }
}

impl<'a, T: SliceByValueGet> IterableByValue for SubsliceImpl<'a, T> {
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

impl<'a, T: SliceByValueGet> IterableByValueFrom for SubsliceImpl<'a, T> {
    type IterFrom<'b>
        = Iter<'b, Self>
    where
        Self: 'b;

    #[inline]
    fn iter_value_from(&self, from: usize) -> Self::IterFrom<'_> {
        Iter::new_from(self, from)
    }
}

/// This module is a type-level assertion that the `SubsliceImpl` struct
/// implements the `SliceByValueSubslice` trait for a type that implements
/// `SliceByValueGet`.
mod __type_assert {
    use super::{SliceByValueGet, SliceByValueSubslice, SubsliceImpl};
    #[allow(dead_code)] // the code is never called, but is used to enforce the trait bounds
    fn type_assert(slice: impl SliceByValueGet) {
        fn assert(_: impl SliceByValueSubslice) {}
        assert(SubsliceImpl {
            slice: &slice,
            range: 1..2,
        });
    }
}
