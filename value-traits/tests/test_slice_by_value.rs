/*
 * SPDX-FileCopyrightText: 2025 Tommaso Fontana
 * SPDX-FileCopyrightText: 2025 Sebastiano Vigna
 * SPDX-FileCopyrightText: 2025 Inria
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

mod common;
pub use common::*;
// Add necessary imports for slice traits to be in scope
// Removed `Slice` from this list as it might be causing resolution issues or is not needed directly.
use value_traits::slices::{SliceByValueGet, SliceByValueSet, SliceByValueRepl, SliceByValueSubslice, SliceByValueSubsliceRange, SliceByValueSubsliceRangeMut};


const EXPECTED: [i32; 5] = [1, 2, 3, 4, 5];

#[test]
fn test_array() {
    generic_get(EXPECTED, &EXPECTED);
    generic_slice(EXPECTED, &EXPECTED);
    generic_mut(EXPECTED, |x| x + 1);
    generic_slice_mut(EXPECTED, |x| x + 1);
}

#[test]
fn test_slice() {
    generic_get(EXPECTED.as_slice(), &EXPECTED);
    generic_slice(EXPECTED.as_slice(), &EXPECTED);
}

#[test]
fn test_slice_mut() {
    let mut x_vec = EXPECTED.to_vec(); // Use a different name to avoid confusion with x in other tests
    generic_get(x_vec.as_mut_slice(), &EXPECTED);
    generic_slice(x_vec.as_mut_slice(), &EXPECTED);
    generic_mut(x_vec.as_mut_slice(), |x| x + 1);
    // The following call is problematic because x_vec.as_mut_slice() is &mut [i32] which is not Clone.
    // generic_slice_mut requires S: Clone.
    // To test mutable slices with generic_slice_mut, one would typically wrap them in an owned, Cloneable type.
    // For now, this specific test line for &mut [i32] with generic_slice_mut will be commented out.
    // generic_slice_mut(x_vec.as_mut_slice(), |x| x + 1);
}

#[test]
#[cfg(feature = "alloc")]
fn test_vecs() {
    generic_get(EXPECTED.to_vec(), &EXPECTED);
    generic_slice(EXPECTED.to_vec(), &EXPECTED);
    generic_mut(EXPECTED.to_vec(), |x| x + 1);
    generic_slice_mut(EXPECTED.to_vec(), |x| x + 1);
}

#[test]
#[cfg(feature = "std")]
fn test_rc() {
    use std::rc::Rc;
    let x = <Rc<[i32]>>::from(EXPECTED);
    generic_get(x.clone(), &EXPECTED);
    generic_slice(x.clone(), &EXPECTED);
    // no muts
}

#[test]
#[cfg(feature = "std")]
fn test_arc() {
    use std::sync::Arc;
    let x = <Arc<[i32]>>::from(EXPECTED);
    generic_get(x.clone(), &EXPECTED);
    generic_slice(x.clone(), &EXPECTED);
    // no muts
}

#[test]
#[cfg(feature = "alloc")]
fn test_boxed_slice() {
    let x = EXPECTED.to_vec().into_boxed_slice();
    generic_get(x.clone(), &EXPECTED);
    generic_slice(x.clone(), &EXPECTED);
    generic_mut(x.clone(), |x| x + 1);
    generic_slice_mut(x.clone(), |x| x + 1);
}

// --- Additional Panic/Safety Tests for Standard Types ---

// Test index_value panics for standard slices
#[test]
#[should_panic]
fn test_std_slice_index_value_panic() {
    let arr = [1_i32, 2, 3];
    let s = arr.as_slice();
    s.index_value(3); // len
}

#[test]
#[should_panic]
fn test_std_mut_slice_index_value_panic() {
    let mut arr = [1_i32, 2, 3];
    let s = arr.as_mut_slice();
    s.index_value(3); // len
}

// Test get_value safety for standard slices
#[test]
fn test_std_slice_get_value_safe() {
    let arr = [1_i32, 2, 3];
    let s = arr.as_slice();
    assert_eq!(s.get_value(3), None); // len
    assert_eq!(s.get_value(10), None); // > len
}

#[test]
fn test_std_mut_slice_get_value_safe() {
    let mut arr = [1_i32, 2, 3];
    let s = arr.as_mut_slice();
    assert_eq!(s.get_value(3), None); // len
    assert_eq!(s.get_value(10), None); // > len
}

// Test set_value panics for standard slices (as per trait default)
#[test]
// This test was ill-posed as &[T] does not implement SliceByValueSet. Removing.
// #[test]
// #[should_panic]
// fn test_std_slice_set_value_panic() {
//     let arr = [1_i32, 2, 3];
//     let s = arr.as_slice(); // s is &[i32]
//     // s.set_value(0, 100); // This would not compile as &[i32] is not &mut self
// }

#[test]
#[should_panic]
fn test_std_mut_slice_set_value_panic() {
    let mut arr = [1_i32, 2, 3];
    let s = arr.as_mut_slice();
    s.set_value(3, 100); // len
}

// Test replace_value panics for standard slices (as per trait default)
#[test]
// This test was ill-posed as &[T] does not implement SliceByValueRepl. Removing.
// #[test]
// #[should_panic]
// fn test_std_slice_replace_value_panic() {
//     // let arr = [1_i32, 2, 3];
//     // let s = arr.as_slice();
//     // s.replace_value(0, 100); // This would not compile
// }

#[test]
#[should_panic]
fn test_std_mut_slice_replace_value_panic() {
    let mut arr = [1_i32, 2, 3];
    let s = arr.as_mut_slice();
    s.replace_value(3, 100); // len
}

// Unchecked methods are inherently unsafe and callers must uphold invariants.
// Testing them for panics is about the wrapper's behavior if it adds checks,
// or direct UB/panic from underlying slice if it doesn't.
// Standard library unchecked access does not guarantee panics; it's UB.
// Our trait's default unsafe methods for `&[T]` and `&mut [T]` directly use `get_unchecked / get_unchecked_mut`.
// So, these tests might be UB rather than guaranteed panics.
// For the purpose of this exercise, we'll assume that if the index is bad,
// it *might* panic due to debug assertions or specific platform behavior,
// but strictly speaking, it's UB.
// We will skip adding should_panic for get_value_unchecked on standard slices
// as the behavior of UB is not reliably testable with should_panic.

// Tests for get_subslice and get_subslice_mut safety for standard slices
#[test]
fn test_std_slice_get_subslice_safe() {
    let arr = [1_i32, 2, 3];
    let s = arr.as_slice();
    assert_eq!(s.get_subslice(3..3).map(|sub| sub.len()), Some(0)); // Changed Slice::len to sub.len()
    assert_eq!(s.get_subslice(1..4).map(|sub| sub.len()), None); // end > len
    assert_eq!(s.get_subslice(4..5).map(|sub| sub.len()), None); // start > len
    assert_eq!(s.get_subslice(3..1).map(|sub| sub.len()), None); // start > end
}

#[test]
fn test_std_mut_slice_get_subslice_mut_safe() {
    let mut arr = [1_i32, 2, 3];
    let s = arr.as_mut_slice();
    assert_eq!(s.get_subslice_mut(3..3).map(|sub| sub.len()), Some(0)); // Changed Slice::len to sub.len()
    assert_eq!(s.get_subslice_mut(1..4).map(|sub| sub.len()), None);
    assert_eq!(s.get_subslice_mut(4..5).map(|sub| sub.len()), None);
    assert_eq!(s.get_subslice_mut(3..1).map(|sub| sub.len()), None);
}

// index_subslice for standard slices will panic due to Rust's built-in checks
#[test]
#[should_panic]
fn test_std_slice_index_subslice_panic_end_out_of_bounds() {
    let arr = [1_i32, 2, 3];
    let s = arr.as_slice();
    s.index_subslice(1..4);
}

#[test]
#[should_panic]
fn test_std_mut_slice_index_subslice_mut_panic_end_out_of_bounds() {
    let mut arr = [1_i32, 2, 3];
    let s = arr.as_mut_slice();
    s.index_subslice_mut(1..4);
}
