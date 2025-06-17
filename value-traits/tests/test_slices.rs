/*
 * SPDX-FileCopyrightText: 2025 Tommaso Fontana
 * SPDX-FileCopyrightText: 2025 Sebastiano Vigna
 * SPDX-FileCopyrightText: 2025 Inria
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use core::ops::Range;
use std::vec;
use value_traits::slices::*;

mod common;
pub use common::*;

#[test]
fn test_slices() {
    let mut s = vec![1_i32, 2, 3];
    assert_eq!(test_usize(s.as_slice()), 1);
    let t = s.as_slice();
    assert_eq!(test_range(&t), &[1, 2]);
    assert_eq!(test_usize_range(&t), (1, [1, 2].as_ref()));
    assert_eq!(test_len(&t), 3);

    let t = s.as_mut_slice();
    assert_eq!(test_range_mut(t), &mut [1, 2]);
}

fn test_usize(s: impl SliceByValueGet<Value = i32>) -> i32 {
    s.index_value(0_usize)
}

fn test_range<'a, S>(s: &S) -> &[i32]
where
    S: SliceByValueSubslice,
    S: for<'b> SliceByValueSubsliceGat<'b, Subslice = &'b [i32]>,
{
    let a = &s.index_subslice(0..2);
    let _ = s.index_subslice(0..3); // it can be borrowed multiple times
    a
}

fn test_range_mut<'a, S>(s: &mut S) -> &mut [i32]
where
    S: SliceByValueSubsliceRangeMut<Range<usize>> + ?Sized,
    S: for<'b> SliceByValueSubsliceGatMut<'b, SubsliceMut = &'b mut [i32]>,
{
    let a = s.index_subslice_mut(0..2);
    // let _ = s.index_subslice_mut(0..2); // this instead should not compile
    a
}

fn test_usize_range<'a, S>(s: &S) -> (i32, &[i32])
where
    S: SliceByValueGet<Value = i32>,
    S: SliceByValueSubslice,
    S: for<'b> SliceByValueSubsliceGat<'b, Subslice = &'b [i32]>,
{
    (s.index_value(0_usize), s.index_subslice(0..2))
}

fn test_len<'a, S>(s: &S) -> usize
where
    S: SliceByValueSubslice,
    S: for<'b> SliceByValueSubsliceGat<'b, Subslice = &'b [i32]>,
{
    s.len()
}

#[test]
#[cfg(any(feature = "std", feature = "alloc"))]
fn test_iter() {
    let s = [1_i32, 2, 3];
    generic_iter(&s.to_vec(), &s);
}

use value_traits_derive::{Iterators, IteratorsMut, Subslices, SubslicesMut};

#[derive(Subslices, SubslicesMut, Iterators, IteratorsMut)]
pub struct Sbv<T: Clone>(Vec<T>);

// Checks that we can derive for two different structs in the same module
#[derive(Subslices, SubslicesMut, Iterators, IteratorsMut)]
pub struct Sbv2<T: Clone>(Vec<T>);

macro_rules! impl_slice {
    ($ty:ident) => {
        impl<T: Clone> SliceByValue for $ty<T> {
            type Value = T;

            fn len(&self) -> usize {
                self.0.len()
            }
        }

        impl<T: Clone> SliceByValueGet for $ty<T> {
            unsafe fn get_value_unchecked(&self, index: usize) -> Self::Value {
                self.0.as_slice().get_value_unchecked(index)
            }
        }

        impl<T: Clone> SliceByValueSet for $ty<T> {
            unsafe fn set_value_unchecked(&mut self, index: usize, value: Self::Value) {
                self.0.as_mut_slice().set_value(index, value)
            }
        }

        impl<T: Clone> SliceByValueRepl for $ty<T> {
            unsafe fn replace_value_unchecked(
                &mut self,
                index: usize,
                value: Self::Value,
            ) -> Self::Value {
                self.0.as_mut_slice().replace_value(index, value)
            }
        }
    };
}

impl_slice!(Sbv);
impl_slice!(Sbv2);

#[test]
fn test_sbv_subslices() {
    let expected = [1_i32, 2, 3, 4, 5];
    let mut s = Sbv(expected.to_vec());
    // test the struct
    generic_get(&s, &expected);
    generic_slice(&s, &expected);
    generic_mut(&mut s);
    generic_slice_mut(&mut s);
    //generic_derived_iter(s, &expected);
    // test its slice
    generic_get(s.index_subslice(..), &expected);
    generic_slice(s.index_subslice(..), &expected);
    generic_derived_iter(s.index_subslice(..), &expected);
    // test its mutable slice
    generic_get(s.index_subslice_mut(..), &expected);
    generic_slice(s.index_subslice_mut(..), &expected);
    generic_mut(s.index_subslice_mut(..));
    generic_slice_mut(s.index_subslice_mut(..));
    generic_derived_iter(s.index_subslice_mut(..), &expected);

    let mut t = s.index_subslice_mut(1..3); // should compile
    assert_eq!(t.len(), 2);
    assert_eq!(t.index_value(0), 2);
    assert_eq!(t.index_value(1), 3);
    t.set_value(1, 4);
    let u = t.index_subslice(1..);
    assert_eq!(u.len(), 1);
    assert_eq!(u.index_value(0), 4);
}

// Checks that we can derive an enum.
#[derive(Subslices, Iterators)]
pub enum Sbv3 {
    OnlyThis,
}

impl SliceByValue for Sbv3 {
    type Value = usize;

    fn len(&self) -> usize {
        100
    }
}

impl SliceByValueGet for Sbv3 {
    unsafe fn get_value_unchecked(&self, index: usize) -> Self::Value {
        index
    }
}

// Checks that we can derive a union.
#[derive(Subslices, Iterators)]
pub union Sbv4 {
    _only_this: usize,
}

impl SliceByValue for Sbv4 {
    type Value = usize;

    fn len(&self) -> usize {
        100
    }
}

impl SliceByValueGet for Sbv4 {
    unsafe fn get_value_unchecked(&self, index: usize) -> Self::Value {
        index
    }
}
