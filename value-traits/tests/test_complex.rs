//! Tests for `SbvComplex` and its manual trait implementations from `common.rs`.
//!
//! This file focuses on verifying the correctness of the manually implemented
//! `SliceByValue`, `SliceByValueGet`, `SliceByValueSet`, `SliceByValueRepl`,
//! and iterator traits (`IterableByValue`, `IterableByValueFrom`) for `SbvComplex`.
//! It ensures that these basic operations and iterations work as expected before
//! testing more complex scenarios or derive macros.

// value-traits/tests/test_complex.rs

// Since common.rs is in the same `tests` directory and not a lib,
// we might need to declare it as a module in the test crate's root (e.g. lib.rs or main.rs for tests)
// or if each test file is a separate crate, common.rs might need to be included.
// For now, assuming `common` module can be accessed from here.
// If common.rs is compiled as part of the same test crate, `crate::common` or `common` might work.
// Let's try with `crate::common` assuming common.rs is found by the compiler as a module.
// If `common.rs` is structured as a test utility module for `value-traits` lib,
// and `test_complex.rs` is an integration test, then `value_traits::common` might be the path
// if `common` module is made public from `value_traits/src/lib.rs` (which is not the case here).

// Assuming `common.rs` is in `tests/` and `tests/mod.rs` (or `tests/main.rs`) declares `mod common;`
// Or, if `value-traits/src/lib.rs` declares `#[cfg(test)] pub mod common_test_utils;` pointing to that file.
// Given the current setup where `common.rs` is in `tests/`, and each file in `tests/` is often a new crate root:
// The most robust way is usually to put common code in `src/` and make it `pub(crate)` or `pub` for tests.
// Or, put common test code into a module within the test file itself or a submodule.

// Let's assume common.rs is correctly declared as a module for tests.
// If common.rs is in `tests/` and `test_complex.rs` is also in `tests/`,
// they are typically separate crate roots for integration tests.
// To share code, `common.rs` should be a library or a module within the main crate (`src`).
// The instructions seem to imply `common.rs` is a shared test utility.
// If `value-traits/tests/common.rs` is a module visible to other files in `tests/`,
// it must be declared in a `mod.rs` in the `tests` directory, or each test file
// that needs it must somehow include it.

// Given the existing structure (e.g. test_slices.rs has `mod common; pub use common::*;`),
// I will assume `test_complex.rs` can do the same if `common.rs` is meant to be a general test utility module.
// However, `mod common;` would mean `common.rs` is a submodule of `test_complex.rs`, which is not right.

// The simplest way for Cargo to pick this up, if `common.rs` is just a file with test utils,
// is that `common.rs` itself might not be a module but included via `#[path = "common.rs"] mod common;`
// or its contents are directly in files that need it.

// Let's try what `test_slices.rs` does:
#[path = "common.rs"]
mod common;
use common::{ComplexType, SbvComplex}; // Import necessary items
use value_traits::slices::{SliceByValue, SliceByValueGet, SliceByValueSet, SliceByValueRepl}; // Import traits

fn sample_complex_data() -> Vec<ComplexType> {
    vec![
        ComplexType { id: 1, name: "First".to_string(), data: vec![1, 2, 3] },
        ComplexType { id: 2, name: "Second".to_string(), data: vec![4, 5, 6] },
        ComplexType { id: 3, name: "Third".to_string(), data: vec![7, 8, 9] },
    ]
}

#[test]
fn test_sbv_complex_len() {
    let data = sample_complex_data();
    let sbv = SbvComplex::from(data);
    assert_eq!(sbv.len(), 3);

    let empty_sbv = SbvComplex::from(Vec::<ComplexType>::new());
    assert_eq!(empty_sbv.len(), 0);
    assert!(empty_sbv.is_empty());
}

#[test]
fn test_sbv_complex_get() {
    let data = sample_complex_data();
    let sbv = SbvComplex::from(data.clone()); // Clone data as it's moved

    assert_eq!(sbv.get_value(0), Some(data[0].clone()));
    assert_eq!(sbv.get_value(1), Some(data[1].clone()));
    assert_eq!(sbv.get_value(2), Some(data[2].clone()));
    assert_eq!(sbv.get_value(3), None); // Out of bounds

    // Test index_value (from SliceByValueGet)
    assert_eq!(sbv.index_value(0), data[0].clone());
    assert_eq!(sbv.index_value(1), data[1].clone());
}

#[test]
#[should_panic]
fn test_sbv_complex_get_panic() {
    let data = sample_complex_data();
    let sbv = SbvComplex::from(data);
    sbv.index_value(3); // Should panic
}

#[test]
fn test_sbv_complex_set() {
    let data = sample_complex_data();
    let mut sbv = SbvComplex::from(data);

    let new_item = ComplexType { id: 4, name: "Fourth".to_string(), data: vec![10] };
    sbv.set_value(1, new_item.clone());

    assert_eq!(sbv.get_value(1), Some(new_item.clone()));
    assert_eq!(sbv.len(), 3); // Length should remain the same
}

#[test]
#[should_panic]
fn test_sbv_complex_set_panic() {
    let data = sample_complex_data();
    let mut sbv = SbvComplex::from(data);
    let new_item = ComplexType { id: 4, name: "Fourth".to_string(), data: vec![10] };
    sbv.set_value(3, new_item); // Index out of bounds
}

#[test]
fn test_sbv_complex_replace() {
    let data = sample_complex_data();
    let original_item1 = data[1].clone();
    let mut sbv = SbvComplex::from(data);

    let new_item = ComplexType { id: 5, name: "Fifth".to_string(), data: vec![11] };
    let replaced_item = sbv.replace_value(1, new_item.clone());

    assert_eq!(replaced_item, original_item1);
    assert_eq!(sbv.get_value(1), Some(new_item.clone()));
    assert_eq!(sbv.len(), 3);
}

#[test]
#[should_panic]
fn test_sbv_complex_replace_panic() {
    let data = sample_complex_data();
    let mut sbv = SbvComplex::from(data);
    let new_item = ComplexType { id: 5, name: "Fifth".to_string(), data: vec![11] };
    sbv.replace_value(3, new_item); // Index out of bounds
}

// Unsafe function tests (optional for now, focus on safe versions first)
// #[test]
// fn test_sbv_complex_unsafe_get_set_repl() {
// ... (rest of existing unsafe tests if any)

// --- Tests for Manual Iterator Implementations on SbvComplex ---

#[test]
fn test_sbv_complex_iter_value() {
    let data = sample_complex_data();
    let sbv = SbvComplex::from(data.clone());

    // Test iteration
    let collected: Vec<ComplexType> = sbv.iter_value().collect();
    assert_eq!(collected, data);

    // Test ExactSizeIterator::len
    let mut iter = sbv.iter_value();
    assert_eq!(iter.len(), 3);
    assert_eq!(iter.next(), Some(data[0].clone()));
    assert_eq!(iter.len(), 2);
    assert_eq!(iter.next(), Some(data[1].clone()));
    assert_eq!(iter.len(), 1);
    assert_eq!(iter.next(), Some(data[2].clone()));
    assert_eq!(iter.len(), 0);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.len(), 0);

    // Test iterating an empty SbvComplex
    let empty_sbv = SbvComplex::from(Vec::<ComplexType>::new());
    let mut empty_iter = empty_sbv.iter_value();
    assert_eq!(empty_iter.len(), 0);
    assert_eq!(empty_iter.next(), None);
    assert_eq!(empty_iter.len(), 0);
}

#[test]
fn test_sbv_complex_iter_value_from() {
    let data = sample_complex_data();
    let sbv = SbvComplex::from(data.clone());

    // Test from index 0
    let collected_from_0: Vec<ComplexType> = sbv.iter_value_from(0).collect();
    assert_eq!(collected_from_0, data);
    assert_eq!(sbv.iter_value_from(0).len(), 3);

    // Test from middle index
    let collected_from_1: Vec<ComplexType> = sbv.iter_value_from(1).collect();
    assert_eq!(collected_from_1, data[1..]);
    let mut iter_from_1 = sbv.iter_value_from(1);
    assert_eq!(iter_from_1.len(), 2);
    assert_eq!(iter_from_1.next(), Some(data[1].clone()));
    assert_eq!(iter_from_1.len(), 1);
    assert_eq!(iter_from_1.next(), Some(data[2].clone()));
    assert_eq!(iter_from_1.len(), 0);
    assert_eq!(iter_from_1.next(), None);


    // Test from len() (should be empty iterator)
    let mut iter_from_len = sbv.iter_value_from(sbv.len());
    assert_eq!(iter_from_len.len(), 0);
    assert_eq!(iter_from_len.next(), None);
    assert_eq!(iter_from_len.len(), 0);

    // Test from len() on empty slice
    let empty_sbv = SbvComplex::from(Vec::<ComplexType>::new());
    let mut empty_iter_from_len = empty_sbv.iter_value_from(0);
    assert_eq!(empty_iter_from_len.len(), 0);
    assert_eq!(empty_iter_from_len.next(), None);
}

#[test]
#[should_panic(expected = "iter_value_from: from index out of bounds")]
fn test_sbv_complex_iter_value_from_panic() {
    let data = sample_complex_data();
    let sbv = SbvComplex::from(data);
    sbv.iter_value_from(sbv.len() + 1); // from > len()
}

#[test]
fn test_sbv_complex_iter_double_ended() {
    let data = sample_complex_data();
    let sbv = SbvComplex::from(data.clone());

    // Full iteration
    let mut iter = sbv.iter_value();
    assert_eq!(iter.len(), 3);
    assert_eq!(iter.next_back(), Some(data[2].clone()));
    assert_eq!(iter.len(), 2);
    assert_eq!(iter.next(), Some(data[0].clone()));
    assert_eq!(iter.len(), 1);
    assert_eq!(iter.next_back(), Some(data[1].clone()));
    assert_eq!(iter.len(), 0);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.len(), 0);

    // Mixed
    let mut iter_mixed = sbv.iter_value();
    assert_eq!(iter_mixed.len(), 3);
    assert_eq!(iter_mixed.next(), Some(data[0].clone())); // pos = 1, back_pos = 3, len = 2
    assert_eq!(iter_mixed.len(), 2);
    assert_eq!(iter_mixed.next_back(), Some(data[2].clone())); // pos = 1, back_pos = 2, len = 1
    assert_eq!(iter_mixed.len(), 1);
    assert_eq!(iter_mixed.next(), Some(data[1].clone())); // pos = 2, back_pos = 2, len = 0
    assert_eq!(iter_mixed.len(), 0);
    assert_eq!(iter_mixed.next(), None);
    assert_eq!(iter_mixed.next_back(), None);

    // Iter from
    let mut iter_from = sbv.iter_value_from(1);
    assert_eq!(iter_from.len(), 2);
    assert_eq!(iter_from.next_back(), Some(data[2].clone()));
    assert_eq!(iter_from.len(), 1);
    assert_eq!(iter_from.next(), Some(data[1].clone()));
    assert_eq!(iter_from.len(), 0);
    assert_eq!(iter_from.next(), None);

    // Empty
    let empty_sbv = SbvComplex::from(Vec::<ComplexType>::new());
    let mut empty_iter = empty_sbv.iter_value();
    assert_eq!(empty_iter.len(), 0);
    assert_eq!(empty_iter.next_back(), None);
    assert_eq!(empty_iter.next(), None);
}
//     let data = sample_complex_data();
//     let mut sbv = SbvComplex::from(data.clone());

//     unsafe {
//         // Get
//         assert_eq!(sbv.get_value_unchecked(0), data[0].clone());

//         // Set
//         let new_item_set = ComplexType { id: 10, name: "SetUnchecked".to_string(), data: vec![100] };
//         sbv.set_value_unchecked(0, new_item_set.clone());
//         assert_eq!(sbv.get_value_unchecked(0), new_item_set.clone());

//         // Replace
//         let new_item_repl = ComplexType { id: 11, name: "ReplUnchecked".to_string(), data: vec![101] };
//         let replaced = sbv.replace_value_unchecked(0, new_item_repl.clone());
//         assert_eq!(replaced, new_item_set.clone());
//         assert_eq!(sbv.get_value_unchecked(0), new_item_repl.clone());
//     }
// }
