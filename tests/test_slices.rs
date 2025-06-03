/*
 * SPDX-FileCopyrightText: 2025 Tommaso Fontana
 * SPDX-FileCopyrightText: 2025 Sebastiano Vigna
 * SPDX-FileCopyrightText: 2025 Inria
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use core::ops::Range;
use std::vec;
use value_traits::{
    impl_subslice, impl_subslice_mut,
    iter::{IterableByValue, IterableByValueFrom},
    slices::*,
};

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
    S: for<'b> SliceByValueSubsliceGatMut<'b, Subslice = &'b mut [i32]>,
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
fn test_iter() {
    let s = vec![1_i32, 2, 3];
    let mut i = s.iter_value();
    assert_eq!(i.next(), Some(1));
    assert_eq!(i.next(), Some(2));
    assert_eq!(i.next(), Some(3));
    assert_eq!(i.next(), None);
    let mut i = s.iter_value_from(1);
    assert_eq!(i.next(), Some(2));
    assert_eq!(i.next(), Some(3));
    assert_eq!(i.next(), None);
}

pub struct Sbv(vec::Vec<i32>);

impl SliceByValue for Sbv {
    type Value = i32;

    fn len(&self) -> usize {
        self.0.len()
    }
}

impl SliceByValueGet for Sbv {
    unsafe fn get_value_unchecked(&self, index: usize) -> Self::Value {
        self.0.as_slice().get_value_unchecked(index)
    }
}

impl SliceByValueSet for Sbv {
    unsafe fn set_value_unchecked(&mut self, index: usize, value: Self::Value) {
        self.0.as_mut_slice().set_value(index, value)
    }
}

impl SliceByValueRepl for Sbv {
    unsafe fn replace_value_unchecked(&mut self, index: usize, value: Self::Value) -> Self::Value {
        self.0.as_mut_slice().replace_value(index, value)
    }
}

impl_subslice![Sbv];
impl_subslice_mut![Sbv];

#[test]
fn test_sbv_subslices() {
    let mut s = Sbv(vec![1_i32, 2, 3, 4]);
    let mut t = s.index_subslice_mut(1..3); // should compile
    assert_eq!(t.len(), 2);
    assert_eq!(t.index_value(0), 2);
    assert_eq!(t.index_value(1), 3);
    t.set_value(1, 4);
    let u = t.index_subslice(1..);
    assert_eq!(u.len(), 1);
    assert_eq!(u.index_value(0), 4);
}
