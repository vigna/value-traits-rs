/*
 * SPDX-FileCopyrightText: 2025 Tommaso Fontana
 * SPDX-FileCopyrightText: 2025 Sebastiano Vigna
 * SPDX-FileCopyrightText: 2025 Inria
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

//! Implementations of by-value traits for arrays of [cloneable](Clone) types.

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
        SliceByValue, SliceByValueMut, SliceByValueSubsliceGat,
        SliceByValueSubsliceGatMut, SliceByValueSubsliceRange, SliceByValueSubsliceRangeMut,
        Subslice, SubsliceMut,
    },
};

impl<T: Clone, const N: usize> SliceByValue for [T; N] {
    type Value = T;

    #[inline(always)]
    fn len(&self) -> usize {
        N
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
        let val_ref = unsafe { (*self).get_unchecked(index) };
        val_ref.clone()
    }
}

impl<T: Clone, const N: usize> SliceByValueMut for [T; N] {
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

    type ChunksMut<'a> = core::slice::ChunksMut<'a, T>
    where
        Self: 'a;

    type ChunksMutError = core::convert::Infallible;

    #[inline]
    fn try_chunks_mut(&mut self, chunk_size: usize) -> Result<Self::ChunksMut<'_>, Self::ChunksMutError> {
        Ok(self.chunks_mut(chunk_size))
    }
}

impl<'a, T: Clone, const N: usize> SliceByValueSubsliceGat<'a> for [T; N] {
    type Subslice = &'a [T];
}

impl<'a, T: Clone, const N: usize> SliceByValueSubsliceGatMut<'a> for [T; N] {
    type SubsliceMut = &'a mut [T];
}

macro_rules! impl_range_arrays {
    ($range:ty) => {
        impl<T: Clone, const N: usize> SliceByValueSubsliceRange<$range> for [T; N] {
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

        impl<T: Clone, const N: usize> SliceByValueSubsliceRangeMut<$range> for [T; N] {
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

impl_range_arrays!(RangeFull);
impl_range_arrays!(RangeFrom<usize>);
impl_range_arrays!(RangeTo<usize>);
impl_range_arrays!(Range<usize>);
impl_range_arrays!(RangeInclusive<usize>);
impl_range_arrays!(RangeToInclusive<usize>);

impl<'a, T: Clone, const N: usize> IterateByValueGat<'a> for [T; N] {
    type Item = T;
    type Iter = Cloned<core::slice::Iter<'a, T>>;
}

impl<T: Clone, const N: usize> IterateByValue for [T; N] {
    fn iter_value(&self) -> Iter<'_, Self> {
        self.iter().cloned()
    }
}

impl<'a, T: Clone, const N: usize> IterateByValueFromGat<'a> for [T; N] {
    type Item = T;
    type IterFrom = Cloned<Skip<core::slice::Iter<'a, T>>>;
}

impl<T: Clone, const N: usize> IterateByValueFrom for [T; N] {
    fn iter_value_from(&self, from: usize) -> IterFrom<'_, Self> {
        self.iter().skip(from).cloned()
    }
}
