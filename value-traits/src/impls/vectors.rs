/*
 * SPDX-FileCopyrightText: 2025 Tommaso Fontana
 * SPDX-FileCopyrightText: 2025 Sebastiano Vigna
 * SPDX-FileCopyrightText: 2025 Inria
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

//! Implementations of by-value traits for vectors of [cloneable](Clone) types.
//!
//! This module is only available if the `alloc` feature is enabled.

#![cfg(feature = "alloc")]

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::vec::Vec;

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
        SliceByValue, SliceByValueGet, SliceByValueRepl, SliceByValueSet, SliceByValueSubsliceGat,
        SliceByValueSubsliceGatMut, SliceByValueSubsliceRange, SliceByValueSubsliceRangeMut,
        Subslice, SubsliceMut,
    },
};

impl<T> SliceByValue for Vec<T> {
    type Value = T;
    #[inline]
    fn len(&self) -> usize {
        <[T]>::len(self)
    }
}

impl<T: Clone> SliceByValueGet for Vec<T> {
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

impl<T: Clone> SliceByValueRepl for Vec<T> {
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
}

impl<T: Clone> SliceByValueSet for Vec<T> {
    #[inline]
    fn set_value(&mut self, index: usize, value: Self::Value) {
        self[index] = value;
    }

    #[inline]
    unsafe fn set_value_unchecked(&mut self, index: usize, value: Self::Value) {
        // SAFETY: index is within bounds
        let val_mut = { self.get_unchecked_mut(index) };
        *val_mut = value;
    }
}

impl<'a, T: Clone> SliceByValueSubsliceGat<'a> for Vec<T> {
    type Subslice = &'a [T];
}
impl<'a, T: Clone> SliceByValueSubsliceGatMut<'a> for Vec<T> {
    type SubsliceMut = &'a mut [T];
}

macro_rules! impl_range_vecs {
    ($range:ty) => {
        impl<T: Clone> SliceByValueSubsliceRange<$range> for Vec<T> {
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
        impl<T: Clone> SliceByValueSubsliceRangeMut<$range> for Vec<T> {
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

impl_range_vecs!(RangeFull);
impl_range_vecs!(RangeFrom<usize>);
impl_range_vecs!(RangeTo<usize>);
impl_range_vecs!(Range<usize>);
impl_range_vecs!(RangeInclusive<usize>);
impl_range_vecs!(RangeToInclusive<usize>);

impl<'a, T: Clone> IterateByValueGat<'a> for Vec<T> {
    type Item = T;
    type Iter = Cloned<core::slice::Iter<'a, T>>;
}

impl<T: Clone> IterateByValue for Vec<T> {
    fn iter_value(&self) -> Iter<'_, Self> {
        self.iter().cloned()
    }
}

impl<'a, T: Clone> IterateByValueFromGat<'a> for Vec<T> {
    type Item = T;
    type IterFrom = Cloned<Skip<core::slice::Iter<'a, T>>>;
}

impl<T: Clone> IterateByValueFrom for Vec<T> {
    fn iter_value_from(&self, from: usize) -> IterFrom<'_, Self> {
        self.iter().skip(from).cloned()
    }
}
