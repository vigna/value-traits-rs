/*
 * SPDX-FileCopyrightText: 2025 Tommaso Fontana
 * SPDX-FileCopyrightText: 2025 Sebastiano Vigna
 * SPDX-FileCopyrightText: 2025 Inria
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

mod common;
pub use common::*;

const EXPECTED: [i32; 5] = [1, 2, 3, 4, 5];

#[test]
fn test_array() {
    generic_get(EXPECTED, &EXPECTED);
    generic_slice(EXPECTED, &EXPECTED);
    generic_mut(EXPECTED);
    generic_slice_mut(EXPECTED);
}

#[test]
fn test_slice() {
    generic_get(EXPECTED.as_slice(), &EXPECTED);
    generic_slice(EXPECTED.as_slice(), &EXPECTED);
}

#[test]
fn test_slice_mut() {
    let mut x = EXPECTED.to_vec();
    generic_get(x.as_mut_slice(), &EXPECTED);
    generic_slice(x.as_mut_slice(), &EXPECTED);
    generic_mut(x.as_mut_slice());
    generic_slice_mut(x.as_mut_slice());
}

#[test]
#[cfg(feature = "alloc")]
fn test_vecs() {
    generic_get(EXPECTED.to_vec(), &EXPECTED);
    generic_slice(EXPECTED.to_vec(), &EXPECTED);
    generic_mut(EXPECTED.to_vec());
    generic_slice_mut(EXPECTED.to_vec());
}

#[test]
#[cfg(feature = "std")]
fn test_vec_deques() {
    use std::collections::VecDeque;
    generic_get(Into::<VecDeque<_>>::into(EXPECTED.to_vec()), &EXPECTED);
    generic_mut(Into::<VecDeque<_>>::into(EXPECTED.to_vec()));
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
    generic_mut(x.clone());
    generic_slice_mut(x.clone());
}

/// Test that `Box<[T]>` iterator delegation works (via the blanket impl for
/// `Box<S>` where `S: IterateByValue`).
#[test]
#[cfg(feature = "alloc")]
fn test_boxed_slice_iter() {
    let x = EXPECTED.to_vec().into_boxed_slice();
    generic_iter(&x, &EXPECTED);
}

/// Test that `Arc<[T]>` iterator delegation works.
#[test]
#[cfg(feature = "std")]
fn test_arc_iter() {
    use std::sync::Arc;
    let x = <Arc<[i32]>>::from(EXPECTED);
    generic_iter(&x, &EXPECTED);
}

/// Test that `Rc<[T]>` iterator delegation works.
#[test]
#[cfg(feature = "std")]
fn test_rc_iter() {
    use std::rc::Rc;
    let x = <Rc<[i32]>>::from(EXPECTED);
    generic_iter(&x, &EXPECTED);
}

/// Test that `Vec<T>` iteration works.
#[test]
#[cfg(feature = "alloc")]
fn test_vec_iter() {
    let x = EXPECTED.to_vec();
    generic_iter(&x, &EXPECTED);
}

/// Test that `[T; N]` iteration works.
#[test]
fn test_array_iter() {
    generic_iter(&EXPECTED, &EXPECTED);
}

/// Test that `VecDeque<T>` iteration works.
#[test]
#[cfg(feature = "std")]
fn test_vec_deque_iter() {
    use std::collections::VecDeque;
    let x = Into::<VecDeque<_>>::into(EXPECTED.to_vec());
    generic_iter(&x, &EXPECTED);
}
