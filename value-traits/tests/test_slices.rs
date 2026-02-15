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

fn test_usize(s: impl SliceByValue<Value = i32>) -> i32 {
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
    // let _ = s.index_subslice_mut(0..2); // this instead should not compile
    (s.index_subslice_mut(0..2)) as _
}

fn test_usize_range<'a, S>(s: &S) -> (i32, &[i32])
where
    S: SliceByValue<Value = i32>,
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

use value_traits::{Iterators, IteratorsMut, Subslices, SubslicesMut};

#[derive(Subslices, Iterators, SubslicesMut, IteratorsMut)]
#[value_traits_subslices_mut(bound = "T: Copy")]
#[value_traits_iterators_mut(bound = "T: Copy")]
pub struct Sbv<T: Clone = usize>(Vec<T>);

// Checks that we can derive for two different structs in the same module
#[derive(Subslices, SubslicesMut, Iterators, IteratorsMut)]
#[value_traits_subslices_mut(bound = "T: Copy")]
#[value_traits_iterators_mut(bound = "T: Copy")]
pub struct Sbv2<T: Clone>(Vec<T>);

macro_rules! impl_slice {
    ($ty:ident) => {
        impl<T: Clone> SliceByValue for $ty<T> {
            type Value = T;

            fn len(&self) -> usize {
                self.0.len()
            }

            unsafe fn get_value_unchecked(&self, index: usize) -> Self::Value {
                unsafe { self.0.as_slice().get_value_unchecked(index) }
            }
        }

        impl<T: Clone> SliceByValueMut for $ty<T>
        where
            T: Copy,
        {
            unsafe fn set_value_unchecked(&mut self, index: usize, value: Self::Value) {
                self.0.as_mut_slice().set_value(index, value)
            }

            unsafe fn replace_value_unchecked(
                &mut self,
                index: usize,
                value: Self::Value,
            ) -> Self::Value {
                self.0.as_mut_slice().replace_value(index, value)
            }

            type ChunksMut<'a>
                = core::slice::ChunksMut<'a, T>
            where
                Self: 'a;

            type ChunksMutError = core::convert::Infallible;

            fn try_chunks_mut(
                &mut self,
                chunk_size: usize,
            ) -> Result<Self::ChunksMut<'_>, Self::ChunksMutError> {
                Ok(self.0.chunks_mut(chunk_size))
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
    // test its slice (full range)
    generic_get(s.index_subslice(..), &expected);
    generic_slice(s.index_subslice(..), &expected);
    generic_derived_iter(s.index_subslice(..), &expected);
    // test its slice (partial range)
    generic_get(s.index_subslice(1..4), &expected[1..4]);
    generic_derived_iter(s.index_subslice(1..4), &expected[1..4]);
    // test its mutable slice (full range)
    generic_get(s.index_subslice_mut(..), &expected);
    generic_slice(s.index_subslice_mut(..), &expected);
    generic_mut(s.index_subslice_mut(..));
    generic_slice_mut(s.index_subslice_mut(..));
    generic_derived_iter(s.index_subslice_mut(..), &expected);
    // test its mutable slice (partial range)
    generic_get(s.index_subslice_mut(1..4), &expected[1..4]);
    generic_derived_iter(s.index_subslice_mut(1..4), &expected[1..4]);

    let mut t = s.index_subslice_mut(1..3); // should compile
    assert_eq!(t.len(), 2);
    assert_eq!(t.index_value(0), 2);
    assert_eq!(t.index_value(1), 3);
    t.set_value(1, 4);
    let u = t.index_subslice(1..);
    assert_eq!(u.len(), 1);
    assert_eq!(u.index_value(0), 4);
}

/// Test that `iter_value()` on a partial subslice only yields the subslice
/// elements, not the entire backing slice. This was a bug where
/// `Iter::new(self.slice)` was used instead of
/// `Iter::new_with_range(self.slice, self.range.clone())`.
#[test]
fn test_subslice_iter_partial_range() {
    let s = Sbv(vec![10, 20, 30, 40, 50]);

    // Immutable subslice: middle portion
    let sub = s.index_subslice(1..4);
    let values: Vec<_> = value_traits::iter::IterateByValue::iter_value(&sub).collect();
    assert_eq!(values, vec![20, 30, 40]);

    // Immutable subslice: single element
    let sub = s.index_subslice(2..3);
    let values: Vec<_> = value_traits::iter::IterateByValue::iter_value(&sub).collect();
    assert_eq!(values, vec![30]);

    // Immutable subslice: empty
    let sub = s.index_subslice(3..3);
    let values: Vec<_> = value_traits::iter::IterateByValue::iter_value(&sub).collect();
    assert!(values.is_empty());

    // Mutable subslice: middle portion
    let mut s = Sbv(vec![10, 20, 30, 40, 50]);
    let sub_mut = s.index_subslice_mut(2..5);
    let values: Vec<_> = value_traits::iter::IterateByValue::iter_value(&sub_mut).collect();
    assert_eq!(values, vec![30, 40, 50]);

    // Subslice of subslice iteration
    let s = Sbv(vec![10, 20, 30, 40, 50]);
    let sub = s.index_subslice(1..4); // [20, 30, 40]
    let sub_sub = sub.index_subslice(1..3); // [30, 40]
    let values: Vec<_> = value_traits::iter::IterateByValue::iter_value(&sub_sub).collect();
    assert_eq!(values, vec![30, 40]);
}

/// Test `iter_value_from()` on partial subslices to ensure it composes ranges
/// correctly.
#[test]
fn test_subslice_iter_value_from() {
    let s = Sbv(vec![10, 20, 30, 40, 50]);

    // iter_value_from on a subslice
    let sub = s.index_subslice(1..4); // [20, 30, 40]
    let values: Vec<_> = value_traits::iter::IterateByValueFrom::iter_value_from(&sub, 1).collect();
    assert_eq!(values, vec![30, 40]);

    // iter_value_from(0) should give the full subslice
    let values: Vec<_> = value_traits::iter::IterateByValueFrom::iter_value_from(&sub, 0).collect();
    assert_eq!(values, vec![20, 30, 40]);
}

/// Test that `nth()` on a derived iterator works correctly for subslices with
/// a non-zero start. The bug was comparing `n >= self.range.end` instead of
/// `n >= self.range.len()`.
#[test]
#[allow(clippy::iter_nth_zero)] // We intentionally test nth(0) to exercise the nth code path
fn test_derived_iter_nth() {
    let s = Sbv(vec![10, 20, 30, 40, 50]);

    // Full-range subslice nth
    let sub = s.index_subslice(..);
    let mut iter = value_traits::iter::IterateByValue::iter_value(&sub);
    assert_eq!(iter.nth(0), Some(10));
    assert_eq!(iter.nth(1), Some(30)); // skips 20
    assert_eq!(iter.nth(0), Some(40));
    assert_eq!(iter.nth(0), Some(50));
    assert_eq!(iter.nth(0), None);

    // Partial subslice with non-zero start: this is the critical case.
    // With range 2..5, the old code checked `n >= self.range.end` (i.e., n >= 5)
    // which would incorrectly allow n=3 (accessing index 2+3=5, out of bounds).
    let sub = s.index_subslice(2..5); // [30, 40, 50], range = 2..5
    let mut iter = value_traits::iter::IterateByValue::iter_value(&sub);
    assert_eq!(iter.nth(0), Some(30));
    assert_eq!(iter.nth(0), Some(40));
    assert_eq!(iter.nth(0), Some(50));
    assert_eq!(iter.nth(0), None);

    // nth that skips past the end of a partial subslice
    let sub = s.index_subslice(2..5); // [30, 40, 50], len=3
    let mut iter = value_traits::iter::IterateByValue::iter_value(&sub);
    assert_eq!(iter.nth(3), None); // exactly at boundary
    assert_eq!(iter.nth(0), None); // exhausted

    let sub = s.index_subslice(2..5);
    let mut iter = value_traits::iter::IterateByValue::iter_value(&sub);
    assert_eq!(iter.nth(100), None); // way past end

    // nth(1) on a 3-element subslice: should skip first and return second
    let sub = s.index_subslice(1..4); // [20, 30, 40]
    let mut iter = value_traits::iter::IterateByValue::iter_value(&sub);
    assert_eq!(iter.nth(1), Some(30));
    assert_eq!(iter.len(), 1);
    assert_eq!(iter.nth(0), Some(40));
    assert_eq!(iter.nth(0), None);

    // nth on mutable subslice
    let mut s = Sbv(vec![10, 20, 30, 40, 50]);
    let sub_mut = s.index_subslice_mut(2..5);
    let mut iter = value_traits::iter::IterateByValue::iter_value(&sub_mut);
    assert_eq!(iter.nth(2), Some(50));
    assert_eq!(iter.nth(0), None);
}

/// Test that derived iterators implement `FusedIterator`: once exhausted,
/// they keep returning `None`.
#[test]
fn test_derived_iter_fused() {
    let s = Sbv(vec![10, 20]);
    let sub = s.index_subslice(..);
    let mut iter = value_traits::iter::IterateByValue::iter_value(&sub);

    assert_eq!(iter.next(), Some(10));
    assert_eq!(iter.next(), Some(20));
    // Exhausted: must keep returning None (FusedIterator contract)
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);

    // Same with next_back
    let sub = s.index_subslice(..);
    let mut iter = value_traits::iter::IterateByValue::iter_value(&sub);
    assert_eq!(iter.next_back(), Some(20));
    assert_eq!(iter.next_back(), Some(10));
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next_back(), None);

    // Mixed forward/backward exhaustion
    let sub = s.index_subslice(..);
    let mut iter = value_traits::iter::IterateByValue::iter_value(&sub);
    assert_eq!(iter.next(), Some(10));
    assert_eq!(iter.next_back(), Some(20));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), None);
}

/// Test `copy` with out-of-bounds `from`/`to` (should copy 0 elements rather
/// than panicking due to underflow).
#[test]
fn test_copy_out_of_bounds() {
    let src = vec![1_i32, 2, 3, 4, 5];
    let mut dst = vec![0_i32; 5];

    // Normal copy
    src.copy(1, &mut dst, 2, 2);
    assert_eq!(dst, vec![0, 0, 2, 3, 0]);

    // from > src.len(): should copy 0 elements
    let mut dst = vec![0_i32; 5];
    src.copy(10, &mut dst, 0, 5);
    assert_eq!(dst, vec![0, 0, 0, 0, 0]);

    // to > dst.len(): should copy 0 elements
    let mut dst = vec![0_i32; 5];
    src.copy(0, &mut dst, 10, 5);
    assert_eq!(dst, vec![0, 0, 0, 0, 0]);

    // from == src.len(): should copy 0 elements
    let mut dst = vec![0_i32; 5];
    src.copy(5, &mut dst, 0, 5);
    assert_eq!(dst, vec![0, 0, 0, 0, 0]);

    // to == dst.len(): should copy 0 elements
    let mut dst = vec![0_i32; 5];
    src.copy(0, &mut dst, 5, 5);
    assert_eq!(dst, vec![0, 0, 0, 0, 0]);

    // len = 0: should copy 0 elements
    let mut dst = vec![0_i32; 5];
    src.copy(0, &mut dst, 0, 0);
    assert_eq!(dst, vec![0, 0, 0, 0, 0]);

    // Partial copy clamped by src availability
    let mut dst = vec![0_i32; 5];
    src.copy(3, &mut dst, 0, 100);
    assert_eq!(dst, vec![4, 5, 0, 0, 0]);

    // Partial copy clamped by dst availability
    let mut dst = vec![0_i32; 3];
    src.copy(0, &mut dst, 1, 100);
    assert_eq!(dst, vec![0, 1, 2]);
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

    unsafe fn get_value_unchecked(&self, index: usize) -> Self::Value {
        index
    }
}

// Checks that we can derive a union
#[derive(Subslices, Iterators)]
pub union Sbv4 {
    _only_this: usize,
}

impl SliceByValue for Sbv4 {
    type Value = usize;

    fn len(&self) -> usize {
        100
    }

    unsafe fn get_value_unchecked(&self, index: usize) -> Self::Value {
        index
    }
}

/// Test optimized `count()` on derived iterators.
#[test]
fn test_derived_iter_count() {
    let s = Sbv(vec![10, 20, 30, 40, 50]);

    // Full subslice
    let sub = s.index_subslice(..);
    assert_eq!(
        value_traits::iter::IterateByValue::iter_value(&sub).count(),
        5
    );

    // Partial subslice
    let sub = s.index_subslice(1..4);
    assert_eq!(
        value_traits::iter::IterateByValue::iter_value(&sub).count(),
        3
    );

    // Empty subslice
    let sub = s.index_subslice(2..2);
    assert_eq!(
        value_traits::iter::IterateByValue::iter_value(&sub).count(),
        0
    );

    // count() after partial consumption
    let sub = s.index_subslice(..);
    let mut iter = value_traits::iter::IterateByValue::iter_value(&sub);
    iter.next();
    iter.next();
    assert_eq!(iter.count(), 3);
}

/// Test optimized `last()` on derived iterators.
#[test]
fn test_derived_iter_last() {
    let s = Sbv(vec![10, 20, 30, 40, 50]);

    // Full subslice
    let sub = s.index_subslice(..);
    assert_eq!(
        value_traits::iter::IterateByValue::iter_value(&sub).last(),
        Some(50)
    );

    // Partial subslice
    let sub = s.index_subslice(1..4);
    assert_eq!(
        value_traits::iter::IterateByValue::iter_value(&sub).last(),
        Some(40)
    );

    // Single-element subslice
    let sub = s.index_subslice(3..4);
    assert_eq!(
        value_traits::iter::IterateByValue::iter_value(&sub).last(),
        Some(40)
    );

    // Empty subslice
    let sub = s.index_subslice(2..2);
    assert_eq!(
        value_traits::iter::IterateByValue::iter_value(&sub).last(),
        None
    );
}

/// Test optimized `nth_back()` on derived iterators.
#[test]
fn test_derived_iter_nth_back() {
    let s = Sbv(vec![10, 20, 30, 40, 50]);

    // nth_back(0) = next_back()
    let sub = s.index_subslice(1..4); // [20, 30, 40]
    let mut iter = value_traits::iter::IterateByValue::iter_value(&sub);
    assert_eq!(iter.nth_back(0), Some(40));
    assert_eq!(iter.nth_back(0), Some(30));
    assert_eq!(iter.nth_back(0), Some(20));
    assert_eq!(iter.nth_back(0), None);

    // nth_back skipping
    let sub = s.index_subslice(..); // [10, 20, 30, 40, 50]
    let mut iter = value_traits::iter::IterateByValue::iter_value(&sub);
    assert_eq!(iter.nth_back(1), Some(40)); // skip 50, return 40
    assert_eq!(iter.len(), 3); // [10, 20, 30] remain
    assert_eq!(iter.nth_back(2), Some(10)); // skip 30 and 20, return 10
    assert_eq!(iter.nth_back(0), None); // exhausted

    // nth_back at boundary
    let sub = s.index_subslice(2..5); // [30, 40, 50], len=3
    let mut iter = value_traits::iter::IterateByValue::iter_value(&sub);
    assert_eq!(iter.nth_back(3), None); // exactly at boundary
    assert_eq!(iter.len(), 0);

    // nth_back overshoot
    let sub = s.index_subslice(2..5);
    let mut iter = value_traits::iter::IterateByValue::iter_value(&sub);
    assert_eq!(iter.nth_back(100), None);
    assert_eq!(iter.len(), 0);

    // nth_back mixed with next
    let sub = s.index_subslice(..); // [10, 20, 30, 40, 50]
    let mut iter = value_traits::iter::IterateByValue::iter_value(&sub);
    assert_eq!(iter.next(), Some(10));
    assert_eq!(iter.nth_back(0), Some(50));
    assert_eq!(iter.len(), 3); // [20, 30, 40]
    assert_eq!(iter.nth_back(1), Some(30)); // skip 40, return 30
    assert_eq!(iter.len(), 1); // [20]
    assert_eq!(iter.next(), Some(20));
    assert_eq!(iter.next(), None);

    // Mutable subslice
    let mut s = Sbv(vec![10, 20, 30, 40, 50]);
    let sub_mut = s.index_subslice_mut(1..4); // [20, 30, 40]
    let mut iter = value_traits::iter::IterateByValue::iter_value(&sub_mut);
    assert_eq!(iter.nth_back(1), Some(30)); // skip 40, return 30
    assert_eq!(iter.nth_back(0), Some(20));
    assert_eq!(iter.nth_back(0), None);
}

/// Test optimized `fold()` on derived iterators.
#[test]
#[allow(clippy::unnecessary_fold)] // We intentionally test fold, not sum
fn test_derived_iter_fold() {
    let s = Sbv(vec![10, 20, 30, 40, 50]);

    // Sum via fold on full subslice
    let sub = s.index_subslice(..);
    let sum = value_traits::iter::IterateByValue::iter_value(&sub).fold(0_i32, |acc, x| acc + x);
    assert_eq!(sum, 150);

    // Sum via fold on partial subslice
    let sub = s.index_subslice(1..4); // [20, 30, 40]
    let sum = value_traits::iter::IterateByValue::iter_value(&sub).fold(0_i32, |acc, x| acc + x);
    assert_eq!(sum, 90);

    // Collect via fold
    let sub = s.index_subslice(2..5); // [30, 40, 50]
    let collected =
        value_traits::iter::IterateByValue::iter_value(&sub).fold(Vec::new(), |mut acc, x| {
            acc.push(x);
            acc
        });
    assert_eq!(collected, vec![30, 40, 50]);

    // Fold on empty subslice
    let sub = s.index_subslice(3..3);
    let sum = value_traits::iter::IterateByValue::iter_value(&sub).fold(42_i32, |acc, x| acc + x);
    assert_eq!(sum, 42);
}

/// Test optimized `for_each()` on derived iterators.
#[test]
fn test_derived_iter_for_each() {
    let s = Sbv(vec![10, 20, 30, 40, 50]);

    // Collect via for_each on partial subslice
    let sub = s.index_subslice(1..4); // [20, 30, 40]
    let mut collected = Vec::new();
    value_traits::iter::IterateByValue::iter_value(&sub).for_each(|x| collected.push(x));
    assert_eq!(collected, vec![20, 30, 40]);

    // for_each on empty subslice
    let sub = s.index_subslice(2..2);
    let mut collected = Vec::new();
    value_traits::iter::IterateByValue::iter_value(&sub).for_each(|x| collected.push(x));
    assert!(collected.is_empty());
}

/// Test optimized `rfold()` on derived iterators.
#[test]
fn test_derived_iter_rfold() {
    let s = Sbv(vec![10, 20, 30, 40, 50]);

    // Collect in reverse via rfold
    let sub = s.index_subslice(1..4); // [20, 30, 40]
    let reversed =
        value_traits::iter::IterateByValue::iter_value(&sub).rfold(Vec::new(), |mut acc, x| {
            acc.push(x);
            acc
        });
    assert_eq!(reversed, vec![40, 30, 20]);

    // Sum via rfold (should equal fold sum)
    let sub = s.index_subslice(..);
    let sum = value_traits::iter::IterateByValue::iter_value(&sub).rfold(0_i32, |acc, x| acc + x);
    assert_eq!(sum, 150);

    // rfold on empty subslice
    let sub = s.index_subslice(3..3);
    let sum = value_traits::iter::IterateByValue::iter_value(&sub).rfold(42_i32, |acc, x| acc + x);
    assert_eq!(sum, 42);

    // rfold processes elements back-to-front; prepending each yields forward order
    let s = Sbv(vec![1, 2, 3]);
    let sub = s.index_subslice(..);
    let result = value_traits::iter::IterateByValue::iter_value(&sub)
        .rfold(String::new(), |acc, x| format!("{x}{acc}"));
    assert_eq!(result, "123");
}
