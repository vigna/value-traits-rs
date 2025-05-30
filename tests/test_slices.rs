/*
 * SPDX-FileCopyrightText: 2025 Tommaso Fontana
 * SPDX-FileCopyrightText: 2025 Sebastiano Vigna
 * SPDX-FileCopyrightText: 2025 Inria
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use core::ops::Range;
use value_traits::{
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
    S: SliceByValueSubslice<usize>,
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
    S: SliceByValueSubslice<usize>,
    S: for<'b> SliceByValueSubsliceGat<'b, Subslice = &'b [i32]>,
{
    (s.index_value(0_usize), s.index_subslice(0..2))
}

fn test_len<'a, S>(s: &S) -> usize
where
    S: SliceByValueSubslice<usize>,
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
