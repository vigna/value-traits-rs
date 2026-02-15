/*
 * SPDX-FileCopyrightText: 2025 Tommaso Fontana
 * SPDX-FileCopyrightText: 2025 Sebastiano Vigna
 * SPDX-FileCopyrightText: 2025 Inria
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

//! Implementations of by-value traits for (boxed) slices of [cloneable](Clone)
//! types.
//!
//! Implementations for boxed slices are only available if the `alloc` feature is
//! enabled.

use core::{
    iter::{Cloned, Skip},
    ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive},
};

use crate::{
    iter::{
        Iter, IterFrom, IterateByValue, IterateByValueFrom, IterateByValueFromGat,
        IterateByValueGat,
    },
    slices::{
        SliceByValue, SliceByValueMut, SliceByValueSubsliceGat, SliceByValueSubsliceGatMut,
        SliceByValueSubsliceRange, SliceByValueSubsliceRangeMut, Subslice, SubsliceMut,
    },
};

// --- Implementations for standard slices [T] and usize index ---
impl<T: Clone> SliceByValue for [T] {
    type Value = T;

    #[inline]
    fn len(&self) -> usize {
        <[T]>::len(self)
    }
    #[inline]
    fn get_value(&self, index: usize) -> Option<Self::Value> {
        (*self).get(index).cloned()
    }

    #[inline]
    fn index_value(&self, index: usize) -> Self::Value {
        self[index].clone()
    }

    #[inline]
    unsafe fn get_value_unchecked(&self, index: usize) -> Self::Value {
        // SAFETY: index is within bounds
        let value = unsafe { (*self).get_unchecked(index) };
        value.clone()
    }
}

impl<T: Clone> SliceByValueMut for [T] {
    #[inline]
    fn set_value(&mut self, index: usize, value: Self::Value) {
        self[index] = value;
    }

    #[inline]
    unsafe fn set_value_unchecked(&mut self, index: usize, value: Self::Value) {
        // SAFETY: index is within bounds
        let val_mut = unsafe { self.get_unchecked_mut(index) };
        *val_mut = value;
    }

    #[inline]
    fn replace_value(&mut self, index: usize, value: Self::Value) -> Self::Value {
        core::mem::replace(&mut self[index], value)
    }

    #[inline]
    unsafe fn replace_value_unchecked(&mut self, index: usize, value: Self::Value) -> Self::Value {
        // SAFETY: index is within bounds
        let val_mut = unsafe { self.get_unchecked_mut(index) };
        core::mem::replace(val_mut, value)
    }

    type ChunksMut<'a>
        = core::slice::ChunksMut<'a, T>
    where
        Self: 'a;

    type ChunksMutError = core::convert::Infallible;

    #[inline]
    fn try_chunks_mut(
        &mut self,
        chunk_size: usize,
    ) -> Result<Self::ChunksMut<'_>, Self::ChunksMutError> {
        Ok(self.chunks_mut(chunk_size))
    }
}

impl<'a, T: Clone> SliceByValueSubsliceGat<'a> for [T] {
    type Subslice = &'a [T];
}

impl<'a, T: Clone> SliceByValueSubsliceGatMut<'a> for [T] {
    type SubsliceMut = &'a mut [T];
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

impl<'a, T: Clone> IterateByValueGat<'a> for [T] {
    type Item = T;
    type Iter = Cloned<core::slice::Iter<'a, T>>;
}

impl<T: Clone> IterateByValue for [T] {
    fn iter_value(&self) -> Iter<'_, Self> {
        self.iter().cloned()
    }
}

impl<'a, T: Clone> IterateByValueFromGat<'a> for [T] {
    type Item = T;
    type IterFrom = Cloned<Skip<core::slice::Iter<'a, T>>>;
}

impl<T: Clone> IterateByValueFrom for [T] {
    fn iter_value_from(&self, from: usize) -> IterFrom<'_, Self> {
        self.iter().skip(from).cloned()
    }
}
