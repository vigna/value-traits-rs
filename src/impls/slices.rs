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

impl<T: Clone> IterableByValue for Box<[T]> {
    type Item = T;
    type Iter<'a>
        = Cloned<std::slice::Iter<'a, T>>
    where
        T: 'a;

    fn iter_value(&self) -> Self::Iter<'_> {
        self.iter().cloned()
    }
}

impl<T: Clone> IterableByValueFrom for Box<[T]> {
    type IterFrom<'a>
        = Cloned<Skip<std::slice::Iter<'a, T>>>
    where
        T: 'a;

    fn iter_value_from(&self, from: usize) -> Self::IterFrom<'_> {
        self.iter().skip(from).cloned()
    }
}

impl<T: Clone> IterableByValue for [T] {
    type Item = T;
    type Iter<'a>
        = Cloned<std::slice::Iter<'a, T>>
    where
        T: 'a;

    fn iter_value(&self) -> Self::Iter<'_> {
        self.iter().cloned()
    }
}

impl<T: Clone> IterableByValueFrom for [T] {
    type IterFrom<'a>
        = Cloned<Skip<std::slice::Iter<'a, T>>>
    where
        T: 'a;

    fn iter_value_from(&self, from: usize) -> Self::IterFrom<'_> {
        self.iter().skip(from).cloned()
    }
}
