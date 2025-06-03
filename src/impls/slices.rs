/*
 * SPDX-FileCopyrightText: 2025 Tommaso Fontana
 * SPDX-FileCopyrightText: 2025 Sebastiano Vigna
 * SPDX-FileCopyrightText: 2025 Inria
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use core::{
    iter::{Cloned, Skip},
    ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive},
};

use crate::{
    iter::{IterableByValue, IterableByValueFrom},
    slices::{
        SliceByValue, SliceByValueGet, SliceByValueRepl, SliceByValueSet, SliceByValueSubsliceGat,
        SliceByValueSubsliceGatMut, SliceByValueSubsliceRange, SliceByValueSubsliceRangeMut,
        Subslice, SubsliceMut,
    },
};

impl<T> SliceByValue for [T] {
    type Value = T;
    #[inline]
    fn len(&self) -> usize {
        <[T]>::len(self)
    }
}

// --- Implementations for standard slices [T] and usize index ---
impl<T: Clone> SliceByValueGet for [T] {
    #[inline]
    fn get_value(&self, index: usize) -> Option<Self::Value> {
        // slice.get returns Option<&T>, .cloned() converts to Option<T>
        (*self).get(index).cloned()
    }

    #[inline]
    fn index_value(&self, index: usize) -> Self::Value {
        // Standard indexing panics on out-of-bounds.
        // It returns &T, which we copy to return T.
        self[index].clone()
    }

    #[inline]
    unsafe fn get_value_unchecked(&self, index: usize) -> Self::Value {
        // Safety: The caller must ensure that `*self` (the index) is in bounds.
        // slice.get_unchecked returns &T, which we dereference and copy.
        unsafe { (*self).get_unchecked(index).clone() }
    }
}

impl<T: Clone> SliceByValueSet for [T] {
    #[inline]
    fn set_value(&mut self, index: usize, value: Self::Value) {
        // Standard indexing panics on out-of-bounds.
        self[index] = value;
    }

    #[inline]
    unsafe fn set_value_unchecked(&mut self, index: usize, value: Self::Value) {
        // Safety: The caller must ensure that `*self` (the index) is in bounds.
        unsafe {
            let elem = self.get_unchecked_mut(index);
            *elem = value;
        }
    }
}

impl<T: Clone> SliceByValueRepl for [T] {
    #[inline]
    fn replace_value(&mut self, index: usize, value: Self::Value) -> Self::Value {
        // Standard indexing panics on out-of-bounds.
        // We get a mutable reference `&mut T`.
        // mem::replace swaps the value at the location with the new `value`
        // and returns the old value.
        core::mem::replace(&mut self[index], value)
    }

    #[inline]
    unsafe fn replace_value_unchecked(&mut self, index: usize, value: Self::Value) -> Self::Value {
        // Safety: The caller must ensure that `*self` (the index) is in bounds.
        unsafe {
            let elem = self.get_unchecked_mut(index);
            core::mem::replace(elem, value)
        }
    }
}

impl<'a, T: Clone> SliceByValueSubsliceGat<'a> for [T] {
    type Subslice = &'a [T];
}

impl<'a, T: Clone> SliceByValueSubsliceGatMut<'a> for [T] {
    type Subslice = &'a mut [T];
}

macro_rules! impl_range_slices {
    ($range:ty) => {
        impl<T: Clone> SliceByValueSubsliceRange<$range> for [T] {
            #[inline]
            fn get_subslice(&self, index: $range) -> Option<Subslice<'_, Self>> {
                (*self).get(index)
            }

            #[inline]
            fn index_subslice(&self, index: $range) -> Subslice<'_, Self> {
                &self[index]
            }

            #[inline]
            unsafe fn get_subslice_unchecked(&self, index: $range) -> Subslice<'_, Self> {
                unsafe { (*self).get_unchecked(index) }
            }
        }

        impl<T: Clone> SliceByValueSubsliceRangeMut<$range> for [T] {
            #[inline]
            fn get_subslice_mut(&mut self, index: $range) -> Option<SubsliceMut<'_, Self>> {
                (*self).get_mut(index)
            }

            #[inline]
            fn index_subslice_mut(&mut self, index: $range) -> SubsliceMut<'_, Self> {
                &mut self[index]
            }

            #[inline]
            unsafe fn get_subslice_unchecked_mut(
                &mut self,
                index: $range,
            ) -> SubsliceMut<'_, Self> {
                unsafe { (*self).get_unchecked_mut(index) }
            }
        }
    };
}

impl_range_slices!(RangeFull);
impl_range_slices!(RangeFrom<usize>);
impl_range_slices!(RangeTo<usize>);
impl_range_slices!(Range<usize>);
impl_range_slices!(RangeInclusive<usize>);
impl_range_slices!(RangeToInclusive<usize>);

#[cfg(feature = "alloc")]
mod alloc_impl {
    use super::*;
    #[cfg(all(feature = "alloc", not(feature = "std")))]
    use alloc::boxed::Box;

    impl<T: Clone> IterableByValue for Box<[T]> {
        type Item = T;
        type Iter<'a>
            = Cloned<core::slice::Iter<'a, T>>
        where
            T: 'a;

        fn iter_value(&self) -> Self::Iter<'_> {
            self.iter().cloned()
        }
    }

    impl<T: Clone> IterableByValueFrom for Box<[T]> {
        type IterFrom<'a>
            = Cloned<Skip<core::slice::Iter<'a, T>>>
        where
            T: 'a;

        fn iter_value_from(&self, from: usize) -> Self::IterFrom<'_> {
            self.iter().skip(from).cloned()
        }
    }
}

impl<T: Clone> IterableByValue for [T] {
    type Item = T;
    type Iter<'a>
        = Cloned<core::slice::Iter<'a, T>>
    where
        T: 'a;

    fn iter_value(&self) -> Self::Iter<'_> {
        self.iter().cloned()
    }
}

impl<T: Clone> IterableByValueFrom for [T] {
    type IterFrom<'a>
        = Cloned<Skip<core::slice::Iter<'a, T>>>
    where
        T: 'a;

    fn iter_value_from(&self, from: usize) -> Self::IterFrom<'_> {
        self.iter().skip(from).cloned()
    }
}

#[macro_export]
macro_rules! impl_subslices {
    ($ty:ty) => {
        pub struct SubsliceImpl<'a> {
            slice: &'a $ty,
            range: Range<usize>,
        }

        impl<'a> SliceByValue for SubsliceImpl<'a> {
            type Value = <$ty as SliceByValue>::Value;

            #[inline]
            fn len(&self) -> usize {
                self.range.len()
            }
        }

        impl<'a> SliceByValueSubsliceGat<'a> for $ty {
            type Subslice = SubsliceImpl<'a>;
        }

        impl<'a> SliceByValueSubsliceRange<core::ops::Range<usize>> for $ty {
            unsafe fn get_subslice_unchecked(
                &self,
                range: core::ops::Range<usize>,
            ) -> Subslice<'_, Self> {
                unsafe {
                    SubsliceImpl {
                        slice: &self,
                        range: range.clone(),
                    }
                }
            }
        }
        impl<'a> SliceByValueSubsliceRange<core::ops::RangeFrom<usize>> for $ty {
            unsafe fn get_subslice_unchecked(
                &self,
                range: core::ops::RangeFrom<usize>,
            ) -> Subslice<'_, Self> {
                unsafe {
                    SubsliceImpl {
                        slice: &self,
                        range: Range {
                            start: range.start,
                            end: self.len(),
                        },
                    }
                }
            }
        }
        impl<'a> SliceByValueSubsliceRange<core::ops::RangeToInclusive<usize>> for $ty {
            unsafe fn get_subslice_unchecked(
                &self,
                range: core::ops::RangeToInclusive<usize>,
            ) -> Subslice<'_, Self> {
                unsafe {
                    SubsliceImpl {
                        slice: &self,
                        range: Range {
                            start: 0,
                            end: range.end + 1,
                        },
                    }
                }
            }
        }
        impl<'a> SliceByValueSubsliceRange<core::ops::RangeFull> for $ty {
            unsafe fn get_subslice_unchecked(
                &self,
                _range: core::ops::RangeFull,
            ) -> Subslice<'_, Self> {
                unsafe {
                    SubsliceImpl {
                        slice: &self,
                        range: Range {
                            start: 0,
                            end: self.len(),
                        },
                    }
                }
            }
        }
        impl<'a> SliceByValueSubsliceRange<core::ops::RangeInclusive<usize>> for $ty {
            unsafe fn get_subslice_unchecked(
                &self,
                range: core::ops::RangeInclusive<usize>,
            ) -> Subslice<'_, Self> {
                use core::ops::{Bound, RangeBounds};
                use std::hint::unreachable_unchecked;

                let start = match range.start_bound() {
                    Bound::Included(s) => *s,
                    // SAFETY: we cannot take this branch
                    _ => unsafe { unreachable_unchecked() },
                };
                let end = match range.end_bound() {
                    Bound::Included(s) => *s,
                    // SAFETY: we cannot take this branch
                    _ => unsafe { unreachable_unchecked() },
                };
                unsafe {
                    SubsliceImpl {
                        slice: &self,
                        range: Range {
                            start: start,
                            end: end + 1,
                        },
                    }
                }
            }
        }

        impl<'a> SliceByValueSubsliceRange<core::ops::RangeTo<usize>> for $ty {
            unsafe fn get_subslice_unchecked(
                &self,
                range: core::ops::RangeTo<usize>,
            ) -> Subslice<'_, Self> {
                unsafe {
                    SubsliceImpl {
                        slice: &self,
                        range: Range {
                            start: 0,
                            end: range.end,
                        },
                    }
                }
            }
        }

        impl<'a> SliceByValueGet for SubsliceImpl<'a> {
            unsafe fn get_value_unchecked(&self, index: usize) -> Self::Value {
                self.slice.get_value_unchecked(index + self.range.start)
            }
        }

        impl<'a, 'b> SliceByValueSubsliceGat<'b> for SubsliceImpl<'a> {
            type Subslice = SubsliceImpl<'b>;
        }

        impl<'a> SliceByValueSubsliceRange<core::ops::Range<usize>> for SubsliceImpl<'a> {
            unsafe fn get_subslice_unchecked(
                &self,
                range: core::ops::Range<usize>,
            ) -> Subslice<'_, Self> {
                unsafe {
                    SubsliceImpl {
                        slice: self.slice,
                        range: Range {
                            start: self.range.start + range.start,
                            end: self.range.start + range.end,
                        },
                    }
                }
            }
        }
        impl<'a> SliceByValueSubsliceRange<core::ops::RangeFrom<usize>> for SubsliceImpl<'a> {
            unsafe fn get_subslice_unchecked(
                &self,
                range: core::ops::RangeFrom<usize>,
            ) -> Subslice<'_, Self> {
                unsafe {
                    SubsliceImpl {
                        slice: self.slice,
                        range: Range {
                            start: self.range.start + range.start,
                            end: self.range.end,
                        },
                    }
                }
            }
        }
        impl<'a> SliceByValueSubsliceRange<core::ops::RangeToInclusive<usize>>
            for SubsliceImpl<'a>
        {
            unsafe fn get_subslice_unchecked(
                &self,
                range: core::ops::RangeToInclusive<usize>,
            ) -> Subslice<'_, Self> {
                unsafe {
                    SubsliceImpl {
                        slice: self.slice,
                        range: Range {
                            start: self.range.start,
                            end: self.range.start + range.end + 1,
                        },
                    }
                }
            }
        }
        impl<'a> SliceByValueSubsliceRange<core::ops::RangeFull> for SubsliceImpl<'a> {
            unsafe fn get_subslice_unchecked(
                &self,
                _range: core::ops::RangeFull,
            ) -> Subslice<'_, Self> {
                unsafe {
                    SubsliceImpl {
                        slice: self.slice,
                        range: self.range.clone(),
                    }
                }
            }
        }
        impl<'a> SliceByValueSubsliceRange<core::ops::RangeInclusive<usize>> for SubsliceImpl<'a> {
            unsafe fn get_subslice_unchecked(
                &self,
                range: core::ops::RangeInclusive<usize>,
            ) -> Subslice<'_, Self> {
                use core::ops::{Bound, RangeBounds};
                use std::hint::unreachable_unchecked;
                let start = match range.start_bound() {
                    Bound::Included(s) => *s,
                    // SAFETY: we cannot take this branch
                    _ => unsafe { unreachable_unchecked() },
                };
                let end = match range.end_bound() {
                    Bound::Included(s) => *s,
                    // SAFETY: we cannot take this branch
                    _ => unsafe { unreachable_unchecked() },
                };
                unsafe {
                    SubsliceImpl {
                        slice: self.slice,
                        range: Range {
                            start: self.range.start + start,
                            end: self.range.start + end + 1,
                        },
                    }
                }
            }
        }

        impl<'a> SliceByValueSubsliceRange<core::ops::RangeTo<usize>> for SubsliceImpl<'a> {
            unsafe fn get_subslice_unchecked(
                &self,
                range: core::ops::RangeTo<usize>,
            ) -> Subslice<'_, Self> {
                unsafe {
                    SubsliceImpl {
                        slice: self.slice,
                        range: Range {
                            start: self.range.start,
                            end: self.range.start + range.end,
                        },
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_subslices_mut {
    ($ty:ty) => {
        pub struct SubsliceImplMut<'a> {
            slice: &'a mut $ty,
            range: Range<usize>,
        }

        impl<'a> SliceByValue for SubsliceImplMut<'a> {
            type Value = <$ty as SliceByValue>::Value;

            #[inline]
            fn len(&self) -> usize {
                self.range.len()
            }
        }

        impl<'a> SliceByValueSubsliceGatMut<'a> for $ty {
            type Subslice = SubsliceImplMut<'a>;
        }

        impl<'a> SliceByValueSubsliceRangeMut<core::ops::Range<usize>> for $ty {
            unsafe fn get_subslice_unchecked_mut(
                &mut self,
                range: core::ops::Range<usize>,
            ) -> SubsliceMut<'_, Self> {
                unsafe {
                    SubsliceImplMut {
                        slice: self,
                        range: range.clone(),
                    }
                }
            }
        }
        impl<'a> SliceByValueSubsliceRangeMut<core::ops::RangeFrom<usize>> for $ty {
            unsafe fn get_subslice_unchecked_mut(
                &mut self,
                range: core::ops::RangeFrom<usize>,
            ) -> SubsliceMut<'_, Self> {
                unsafe {
                    let end = self.len();
                    SubsliceImplMut {
                        slice: self,
                        range: Range {
                            start: range.start,
                            end,
                        },
                    }
                }
            }
        }
        impl<'a> SliceByValueSubsliceRangeMut<core::ops::RangeToInclusive<usize>> for $ty {
            unsafe fn get_subslice_unchecked_mut(
                &mut self,
                range: core::ops::RangeToInclusive<usize>,
            ) -> SubsliceMut<'_, Self> {
                unsafe {
                    SubsliceImplMut {
                        slice: self,
                        range: Range {
                            start: 0,
                            end: range.end + 1,
                        },
                    }
                }
            }
        }
        impl<'a> SliceByValueSubsliceRangeMut<core::ops::RangeFull> for $ty {
            unsafe fn get_subslice_unchecked_mut(
                &mut self,
                _range: core::ops::RangeFull,
            ) -> SubsliceMut<'_, Self> {
                unsafe {
                    let end = self.len();
                    SubsliceImplMut {
                        slice: self,
                        range: Range { start: 0, end },
                    }
                }
            }
        }
        impl<'a> SliceByValueSubsliceRangeMut<core::ops::RangeInclusive<usize>> for $ty {
            unsafe fn get_subslice_unchecked_mut(
                &mut self,
                range: core::ops::RangeInclusive<usize>,
            ) -> SubsliceMut<'_, Self> {
                use core::ops::{Bound, RangeBounds};
                use std::hint::unreachable_unchecked;

                let start = match range.start_bound() {
                    Bound::Included(s) => *s,
                    // SAFETY: we cannot take this branch
                    _ => unsafe { unreachable_unchecked() },
                };
                let end = match range.end_bound() {
                    Bound::Included(s) => *s,
                    // SAFETY: we cannot take this branch
                    _ => unsafe { unreachable_unchecked() },
                };
                unsafe {
                    SubsliceImplMut {
                        slice: self,
                        range: Range {
                            start: start,
                            end: end + 1,
                        },
                    }
                }
            }
        }

        impl<'a> SliceByValueSubsliceRangeMut<core::ops::RangeTo<usize>> for $ty {
            unsafe fn get_subslice_unchecked_mut(
                &mut self,
                range: core::ops::RangeTo<usize>,
            ) -> SubsliceMut<'_, Self> {
                unsafe {
                    SubsliceImplMut {
                        slice: self,
                        range: Range {
                            start: 0,
                            end: range.end,
                        },
                    }
                }
            }
        }

        impl<'a> SliceByValueGet for SubsliceImplMut<'a> {
            unsafe fn get_value_unchecked(&self, index: usize) -> <$ty as SliceByValue>::Value {
                self.slice.get_value_unchecked(index + self.range.start)
            }
        }

        impl<'a, 'b> SliceByValueSubsliceGat<'b> for SubsliceImplMut<'a> {
            type Subslice = SubsliceImpl<'b>;
        }

        impl<'a> SliceByValueSubsliceRange<core::ops::Range<usize>> for SubsliceImplMut<'a> {
            unsafe fn get_subslice_unchecked(
                &self,
                range: core::ops::Range<usize>,
            ) -> Subslice<'_, Self> {
                unsafe {
                    SubsliceImpl {
                        slice: self.slice,
                        range: Range {
                            start: self.range.start + range.start,
                            end: self.range.start + range.end,
                        },
                    }
                }
            }
        }
        impl<'a> SliceByValueSubsliceRange<core::ops::RangeFrom<usize>> for SubsliceImplMut<'a> {
            unsafe fn get_subslice_unchecked(
                &self,
                range: core::ops::RangeFrom<usize>,
            ) -> Subslice<'_, Self> {
                unsafe {
                    SubsliceImpl {
                        slice: self.slice,
                        range: Range {
                            start: self.range.start + range.start,
                            end: self.range.end,
                        },
                    }
                }
            }
        }
        impl<'a> SliceByValueSubsliceRange<core::ops::RangeToInclusive<usize>>
            for SubsliceImplMut<'a>
        {
            unsafe fn get_subslice_unchecked(
                &self,
                range: core::ops::RangeToInclusive<usize>,
            ) -> Subslice<'_, Self> {
                unsafe {
                    SubsliceImpl {
                        slice: self.slice,
                        range: Range {
                            start: self.range.start,
                            end: self.range.start + range.end + 1,
                        },
                    }
                }
            }
        }
        impl<'a> SliceByValueSubsliceRange<core::ops::RangeFull> for SubsliceImplMut<'a> {
            unsafe fn get_subslice_unchecked(
                &self,
                _range: core::ops::RangeFull,
            ) -> Subslice<'_, Self> {
                unsafe {
                    SubsliceImpl {
                        slice: self.slice,
                        range: self.range.clone(),
                    }
                }
            }
        }
        impl<'a> SliceByValueSubsliceRange<core::ops::RangeInclusive<usize>>
            for SubsliceImplMut<'a>
        {
            unsafe fn get_subslice_unchecked(
                &self,
                range: core::ops::RangeInclusive<usize>,
            ) -> Subslice<'_, Self> {
                use core::ops::{Bound, RangeBounds};
                use std::hint::unreachable_unchecked;
                let start = match range.start_bound() {
                    Bound::Included(s) => *s,
                    // SAFETY: we cannot take this branch
                    _ => unsafe { unreachable_unchecked() },
                };
                let end = match range.end_bound() {
                    Bound::Included(s) => *s,
                    // SAFETY: we cannot take this branch
                    _ => unsafe { unreachable_unchecked() },
                };
                unsafe {
                    SubsliceImpl {
                        slice: self.slice,
                        range: Range {
                            start: self.range.start + start,
                            end: self.range.start + end + 1,
                        },
                    }
                }
            }
        }

        impl<'a> SliceByValueSubsliceRange<core::ops::RangeTo<usize>> for SubsliceImplMut<'a> {
            unsafe fn get_subslice_unchecked(
                &self,
                range: core::ops::RangeTo<usize>,
            ) -> Subslice<'_, Self> {
                unsafe {
                    SubsliceImpl {
                        slice: self.slice,
                        range: Range {
                            start: self.range.start,
                            end: self.range.start + range.end,
                        },
                    }
                }
            }
        }

        impl<'a> SliceByValueSet for SubsliceImplMut<'a> {
            unsafe fn set_value_unchecked(&mut self, index: usize, value: Self::Value) {
                self.slice
                    .set_value_unchecked(index + self.range.start, value)
            }
        }

        impl<'a> SliceByValueRepl for SubsliceImplMut<'a> {
            unsafe fn replace_value_unchecked(
                &mut self,
                index: usize,
                value: Self::Value,
            ) -> Self::Value {
                self.slice
                    .replace_value_unchecked(index + self.range.start, value)
            }
        }

        impl<'a, 'b> SliceByValueSubsliceGatMut<'b> for SubsliceImplMut<'a> {
            type Subslice = SubsliceImplMut<'b>;
        }

        impl<'a> SliceByValueSubsliceRangeMut<core::ops::Range<usize>> for SubsliceImplMut<'a> {
            unsafe fn get_subslice_unchecked_mut(
                &mut self,
                range: core::ops::Range<usize>,
            ) -> SubsliceMut<'_, Self> {
                unsafe {
                    SubsliceImplMut {
                        slice: self.slice,
                        range: Range {
                            start: self.range.start + range.start,
                            end: self.range.start + range.end,
                        },
                    }
                }
            }
        }
        impl<'a> SliceByValueSubsliceRangeMut<core::ops::RangeFrom<usize>> for SubsliceImplMut<'a> {
            unsafe fn get_subslice_unchecked_mut(
                &mut self,
                range: core::ops::RangeFrom<usize>,
            ) -> SubsliceMut<'_, Self> {
                unsafe {
                    SubsliceImplMut {
                        slice: self.slice,
                        range: Range {
                            start: self.range.start + range.start,
                            end: self.range.end,
                        },
                    }
                }
            }
        }
        impl<'a> SliceByValueSubsliceRangeMut<core::ops::RangeToInclusive<usize>>
            for SubsliceImplMut<'a>
        {
            unsafe fn get_subslice_unchecked_mut(
                &mut self,
                range: core::ops::RangeToInclusive<usize>,
            ) -> SubsliceMut<'_, Self> {
                unsafe {
                    SubsliceImplMut {
                        slice: self.slice,
                        range: Range {
                            start: self.range.start,
                            end: self.range.start + range.end + 1,
                        },
                    }
                }
            }
        }
        impl<'a> SliceByValueSubsliceRangeMut<core::ops::RangeFull> for SubsliceImplMut<'a> {
            unsafe fn get_subslice_unchecked_mut(
                &mut self,
                _range: core::ops::RangeFull,
            ) -> SubsliceMut<'_, Self> {
                unsafe {
                    SubsliceImplMut {
                        slice: self.slice,
                        range: self.range.clone(),
                    }
                }
            }
        }
        impl<'a> SliceByValueSubsliceRangeMut<core::ops::RangeInclusive<usize>>
            for SubsliceImplMut<'a>
        {
            unsafe fn get_subslice_unchecked_mut(
                &mut self,
                range: core::ops::RangeInclusive<usize>,
            ) -> SubsliceMut<'_, Self> {
                use core::ops::{Bound, RangeBounds};
                use std::hint::unreachable_unchecked;
                let start = match range.start_bound() {
                    Bound::Included(s) => *s,
                    // SAFETY: we cannot take this branch
                    _ => unsafe { unreachable_unchecked() },
                };
                let end = match range.end_bound() {
                    Bound::Included(s) => *s,
                    // SAFETY: we cannot take this branch
                    _ => unsafe { unreachable_unchecked() },
                };
                unsafe {
                    SubsliceImplMut {
                        slice: self.slice,
                        range: Range {
                            start: self.range.start + start,
                            end: self.range.start + end + 1,
                        },
                    }
                }
            }
        }

        impl<'a> SliceByValueSubsliceRangeMut<core::ops::RangeTo<usize>> for SubsliceImplMut<'a> {
            unsafe fn get_subslice_unchecked_mut(
                &mut self,
                range: core::ops::RangeTo<usize>,
            ) -> SubsliceMut<'_, Self> {
                unsafe {
                    SubsliceImplMut {
                        slice: self.slice,
                        range: Range {
                            start: self.range.start,
                            end: self.range.start + range.end,
                        },
                    }
                }
            }
        }
    };
}
