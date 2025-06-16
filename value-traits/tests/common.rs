//! Common utilities, structs, and generic test functions for `value-traits` tests.
//!
//! This module provides:
//! - `ComplexType`: A sample struct used for testing traits with more complex, non-`Copy` data.
//! - `SbvComplex`: A wrapper around `Vec<ComplexType>` that manually implements basic by-value slice
//!   and iterator traits. This is used to test these traits independently of procedural macros.
//! - `SbvComplexIter`: The iterator type for `SbvComplex`.
//! - Generic test functions (`generic_get`, `generic_slice`, `generic_mut`, `generic_slice_mut`,
//!   `generic_iter`, `generic_derived_iter`): These functions are designed to test any type
//!   that implements the corresponding by-value traits. They are used across different test files
//!   (e.g., `test_slices.rs`, `test_slice_by_value.rs`) to ensure consistent behavior.
//!   Currently, some of these generic functions may expose limitations or bugs when used with
//!   types that derive traits via `value-traits-derive` (especially concerning GATs and
//!   iterator/subslice helper types not meeting all expected trait bounds).

/*
 * SPDX-FileCopyrightText: 2025 Tommaso Fontana
 * SPDX-FileCopyrightText: 2025 Sebastiano Vigna
 * SPDX-FileCopyrightText: 2025 Inria
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

/// A sample struct for testing with non-`Copy` types.
///
/// It contains a few fields of different types, including heap-allocated `String` and `Vec<u8>`.
/// It derives `Clone`, `Debug`, and `PartialEq` for use in assertions.
#[derive(Clone, Debug, PartialEq)]
pub struct ComplexType {
    /// An identifier.
    pub id: u32,
    /// A name.
    pub name: String,
    pub data: Vec<u8>,
}

// Removed derive macros for Subslices, Iterators etc. from SbvComplex as per subtask instructions
/// A wrapper around `Vec<ComplexType>` for manually testing basic by-value slice and iterator traits.
///
/// This struct is used to verify the fundamental correctness of the by-value traits
/// (`SliceByValue*`, `IterableByValue*`) without involving procedural macros, which helps
/// isolate issues between manual implementations and derived ones.
#[derive(Clone, Debug, PartialEq)]
pub struct SbvComplex {
    /// The underlying data store.
    pub data: Vec<ComplexType>,
}

impl From<Vec<ComplexType>> for SbvComplex {
    fn from(data: Vec<ComplexType>) -> Self {
        Self { data }
    }
}

impl SliceByValue for SbvComplex {
    type Value = ComplexType;

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

impl SliceByValueGet for SbvComplex {
    // unsafe fn get_value_unchecked(&self, index: usize) -> Self::Value {
    //     self.data.get_unchecked(index).clone()
    // }
    unsafe fn get_value_unchecked(&self, index: usize) -> Self::Value {
        // Ensuring `assume` is available, typically via `std::intrinsics::assume` or similar
        // For now, direct unchecked access, assuming caller guarantees validity.
        // In a real scenario, one might use `debug_assert!` or `cfg!` for safety levels.
        let value = self.data.get_unchecked(index);
        // Call `clone` outside of the unsafe block if possible, or ensure it doesn't panic.
        // Here, ComplexType::clone might not panic unless there are strange impls.
        value.clone()
    }


    fn get_value(&self, index: usize) -> Option<Self::Value> {
        self.data.get(index).cloned()
    }
}

impl SliceByValueSet for SbvComplex {
    // unsafe fn set_value_unchecked(&mut self, index: usize, value: Self::Value) {
    //     *self.data.get_unchecked_mut(index) = value;
    // }
    unsafe fn set_value_unchecked(&mut self, index: usize, value: Self::Value) {
        // Similar to get_value_unchecked, assuming caller upholds safety.
        let slot = self.data.get_unchecked_mut(index);
        *slot = value;
    }

    fn set_value(&mut self, index: usize, value: Self::Value) {
        if index < self.data.len() {
            self.data[index] = value;
        } else {
            // As per trait docs, this should panic for out-of-bounds.
            panic!("set_value: index out of bounds");
        }
    }
}

impl SliceByValueRepl for SbvComplex {
    // unsafe fn replace_value_unchecked(&mut self, index: usize, value: Self::Value) -> Self::Value {
    //     std::mem::replace(self.data.get_unchecked_mut(index), value)
    // }
    unsafe fn replace_value_unchecked(&mut self, index: usize, value: Self::Value) -> Self::Value {
        let slot = self.data.get_unchecked_mut(index);
        std::mem::replace(slot, value)
    }

    fn replace_value(&mut self, index: usize, value: Self::Value) -> Self::Value {
        if index < self.data.len() {
            std::mem::replace(&mut self.data[index], value)
        } else {
            panic!("replace_value: index out of bounds");
        }
    }
}

/// An iterator for `SbvComplex` that yields items by value (cloning them).
///
/// This is manually implemented to test the `IterableByValue` and `IterableByValueFrom` traits
/// for `SbvComplex`. It supports forward and backward iteration, and tracks its exact size.
#[derive(Debug)]
pub struct SbvComplexIter<'a> {
    /// A reference to the `SbvComplex` being iterated over.
    slice: &'a SbvComplex,
    /// The current start position for `next()`.
    current_pos: usize,
    /// The current end position (exclusive) for `next_back()`.
    current_back_pos: usize,
}

impl<'a> Iterator for SbvComplexIter<'a> {
    type Item = ComplexType;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_pos >= self.current_back_pos { // Check against back pos for DEIter
            None
        } else {
            // Safety: Bounds are checked by the condition above and initial current_back_pos = len.
            // get_value_unchecked is used because SbvComplex::get_value returns Option<ComplexType>,
            // but Iterator::next() itself should return Option<ComplexType>.
            // Here, we want to clone the item. Direct access for clone.
            // The manual impl of SbvComplex::get_value_unchecked already clones.
            let item = unsafe { self.slice.get_value_unchecked(self.current_pos) };
            self.current_pos += 1;
            Some(item)
        }
    }
}

impl<'a> ExactSizeIterator for SbvComplexIter<'a> {
    fn len(&self) -> usize {
        self.current_back_pos.saturating_sub(self.current_pos)
    }
}

impl<'a> DoubleEndedIterator for SbvComplexIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.current_pos >= self.current_back_pos {
            None
        } else {
            self.current_back_pos -= 1;
            let item = unsafe { self.slice.get_value_unchecked(self.current_back_pos) };
            Some(item)
        }
    }
}


// Manual IterableByValue implementations for SbvComplex
impl<'a> IterableByValueGat<'a> for SbvComplex {
    type Item = ComplexType;
    type Iter = SbvComplexIter<'a>;
}

impl IterableByValue for SbvComplex {
    fn iter_value(&self) -> <Self as IterableByValueGat<'_>>::Iter {
        SbvComplexIter {
            slice: self,
            current_pos: 0,
            current_back_pos: self.len(),
        }
    }
}

impl<'a> IterableByValueFromGat<'a> for SbvComplex {
    type Item = ComplexType;
    type IterFrom = SbvComplexIter<'a>; // Reusing SbvComplexIter
}

impl IterableByValueFrom for SbvComplex {
    fn iter_value_from(&self, from: usize) -> <Self as IterableByValueFromGat<'_>>::IterFrom {
        assert!(from <= self.len(), "iter_value_from: from index out of bounds");
        SbvComplexIter {
            slice: self,
            current_pos: from,
            current_back_pos: self.len(),
        }
    }
}


use core::borrow::Borrow;

use value_traits::{
    iter::{Iter as TraitIter, IterFrom, IterableByValue, IterableByValueFrom, IterableByValueGat, IterableByValueFromGat}, // Added GAT traits
    slices::*,
};

/// Generic function to test `SliceByValue` and `SliceByValueGet` implementations.
///
/// It checks `len()`, `is_empty()`, `get_value()`, `index_value()`, and `get_value_unchecked()`.
/// Note: Current limitations with derive macros might cause E0277 errors when `S`'s GATs
/// (Generic Associated Types) from derive macros do not meet all trait bounds.
pub fn generic_get<T, S>(s: S, expected: &[T])
where
    T: Clone + PartialEq + std::fmt::Debug,
    S: SliceByValue<Value = T> + SliceByValueGet,
{
    assert_eq!(SliceByValue::len(&s), expected.len());
    for i in 0..expected.len() {
        assert_eq!(SliceByValueGet::get_value(&s, i).as_ref(), Some(&expected[i]));
        assert_eq!(SliceByValueGet::get_value(&s, i + expected.len()), None); // Check OOB
        assert_eq!(&SliceByValueGet::index_value(&s, i), &expected[i]);
        assert_eq!(
            unsafe { SliceByValueGet::get_value_unchecked(&s, i) },
            expected[i]
        );
    }
}

/// Generic function to test `SliceByValueSubslice` implementations.
///
/// It checks various subslice ranges using `index_subslice`, `get_subslice`, and `get_subslice_unchecked`,
/// then calls `generic_get` on the resulting subslices.
/// Note: Current limitations with derive macros might cause E0277 errors.
pub fn generic_slice<T, S>(s: S, expected: &[T])
where
    T: Clone + PartialEq + std::fmt::Debug,
    S: SliceByValue<Value = T> + SliceByValueGet + SliceByValueSubslice,
    // This bound is critical: the result of `s.index_subslice` must also be usable by `generic_get`.
    // Typically, this means `S::Subslice` (if GATs are used) or the direct return type of `index_subslice`
    // must also implement `SliceByValue<Value = T> + SliceByValueGet`.
    // If `index_subslice` returns `Self`, this is implicitly satisfied.
    // This might need to be more specific, like:
    // for<'a> <S as SliceByValueSubsliceGat<'a>>::Subslice: SliceByValue<Value = T> + SliceByValueGet,
    // However, often `index_subslice` returns `Self` for by-value slice types.
{
    assert_eq!(SliceByValue::len(&s), expected.len());

    // Test full slice
    let r = ..;
    generic_get(s.index_subslice(r), expected);
    if let Some(sub) = s.get_subslice(r) {
        generic_get(sub, expected);
    } else if !expected.is_empty() {
        panic!("get_subslice(..) returned None for non-empty expected slice");
    }
    generic_get(unsafe { s.get_subslice_unchecked(r) }, expected);

    if expected.is_empty() { // No further slicing tests if expected is empty
        return;
    }

    // Test slicing from an index to the end
    let r = 1..;
    if expected.len() >= 1 { // Ensure the slice is valid
        generic_get(s.index_subslice(r.clone()), &expected[r.clone()]);
        if let Some(sub) = s.get_subslice(r.clone()) {
            generic_get(sub, &expected[r.clone()]);
        } else {
            panic!("get_subslice(1..) returned None");
        }
        generic_get(unsafe { s.get_subslice_unchecked(r.clone()) }, &expected[r.clone()]);
    }

    // Test slicing with a bounded range
    let r = 1..std::cmp::min(4, expected.len());
     if expected.len() >= 1 && r.start < r.end { // Ensure the slice is valid and non-empty
        generic_get(s.index_subslice(r.clone()), &expected[r.clone()]);
        if let Some(sub) = s.get_subslice(r.clone()) {
            generic_get(sub, &expected[r.clone()]);
        } else {
            panic!("get_subslice(1..4) returned None");
        }
        generic_get(unsafe { s.get_subslice_unchecked(r.clone()) }, &expected[r.clone()]);
    }

    // Test slicing from the beginning to an index
    let end_index = std::cmp::min(3, expected.len());
    let r = ..end_index;
    // A RangeTo (..end_index) is considered "empty" for slicing if end_index is 0.
    // The resulting slice expected[r.clone()] will be empty if end_index is 0.
    if end_index != 0 || expected.is_empty() {
        generic_get(s.index_subslice(r.clone()), &expected[r.clone()]);
        if let Some(sub) = s.get_subslice(r.clone()) {
            generic_get(sub, &expected[r.clone()]);
        } else if !expected[r.clone()].is_empty() { // If the expected subslice isn't empty, then getting None is a problem.
             panic!("get_subslice(..{}) returned None for non-empty subslice", end_index);
        }
        generic_get(unsafe { s.get_subslice_unchecked(r.clone()) }, &expected[r.clone()]);
    }


    // Test slicing with an inclusive end index
    if expected.len() > 3 { // Ensure ..=3 is a valid range
        let r = ..=3;
        generic_get(s.index_subslice(r.clone()), &expected[r.clone()]);
        if let Some(sub) = s.get_subslice(r.clone()) {
            generic_get(sub, &expected[r.clone()]);
        } else {
            panic!("get_subslice(..=3) returned None");
        }
        generic_get(unsafe { s.get_subslice_unchecked(r.clone()) }, &expected[r.clone()]);
    }

    // Test slicing with a bounded range with inclusive end
     if expected.len() > 4 { // Ensure 1..=4 is a valid range
        let r = 1..=4;
        generic_get(s.index_subslice(r.clone()), &expected[r.clone()]);
        if let Some(sub) = s.get_subslice(r.clone()) {
            generic_get(sub, &expected[r.clone()]);
        } else {
            panic!("get_subslice(1..=4) returned None");
        }
        generic_get(unsafe { s.get_subslice_unchecked(r.clone()) }, &expected[r.clone()]);
    }
}

/// Generic function to test `SliceByValueSet` and `SliceByValueRepl` implementations.
///
/// It iterates through the slice, testing `set_value`, `replace_value`, and their `_unchecked`
/// counterparts. Requires a function `new_val_fn` to generate modified values for testing.
/// Note: Current limitations with derive macros might cause E0277 errors.
pub fn generic_mut<T, S>(mut s: S, new_val_fn: fn(T) -> T)
where
    T: Clone + PartialEq + std::fmt::Debug,
    S: SliceByValue<Value = T> + SliceByValueGet + SliceByValueSet + SliceByValueRepl,
{
    if s.is_empty() {
        return;
    }
    for i in 0..s.len() {
        let old_value = SliceByValueGet::index_value(&s, i);
        let new_value = new_val_fn(old_value.clone()); // Use the provided function

        SliceByValueSet::set_value(&mut s, i, new_value.clone());
        assert_eq!(&SliceByValueGet::index_value(&s, i), &new_value);

        let replaced_value = SliceByValueRepl::replace_value(&mut s, i, old_value.clone());
        assert_eq!(&replaced_value, &new_value);
        assert_eq!(&SliceByValueGet::index_value(&s, i), &old_value);

        let replaced_value_unchecked =
            unsafe { SliceByValueRepl::replace_value_unchecked(&mut s, i, new_value.clone()) };
        assert_eq!(&replaced_value_unchecked, &old_value);
        assert_eq!(&SliceByValueGet::index_value(&s, i), &new_value);

        unsafe {
            SliceByValueSet::set_value_unchecked(&mut s, i, old_value.clone()); // Restore original
        }
        assert_eq!(&SliceByValueGet::index_value(&s, i), &old_value);
    }
}

/// Generic function to test `SliceByValueSubsliceMut` implementations.
///
/// It obtains mutable subslices using various ranges and calls `generic_mut` on them.
/// Requires `S: Clone` to test different ranges on a consistent state, as `generic_mut` consumes its argument.
/// Note: Current limitations with derive macros (especially `SubsliceImplMut` not being `Clone` or
/// not satisfying `SliceByValueGet` etc.) might cause E0277 errors.
pub fn generic_slice_mut<T, S>(mut s: S, new_val_fn: fn(T) -> T)
where
    T: Clone + PartialEq + std::fmt::Debug,
    S: SliceByValue<Value = T>
        + SliceByValueGet
        + SliceByValueSet
        + SliceByValueRepl
        + SliceByValueSubsliceMut
        + Clone, // Adding S: Clone bound back
    for<'a> <S as SliceByValueSubsliceGatMut<'a>>::Subslice:
        SliceByValue<Value = T> + SliceByValueGet + SliceByValueSet + SliceByValueRepl,
{
    // If s is empty, get_subslice_mut(..) might return Some(empty_slice) or None.
    // generic_mut should handle an empty slice gracefully.
    if s.is_empty() {
        if let Some(empty_sub) = s.get_subslice_mut(..) {
            generic_mut(empty_sub, new_val_fn);
        }
        // After this, s is still empty. The rest of the function might not run or might panic.
        // It's better to return if s is empty, as most range tests below assume non-empty.
        return;
    }

    // Test full range `..`
    // `s` is modified here by `generic_mut` if `index_subslice_mut` allows direct mutation
    // or if it returns an owned slice that replaces the original `s` (less likely for by-value).
    // Assuming `index_subslice_mut` returns an owned slice (Self) and `generic_mut` consumes it.
    // The original `s` is not directly changed by `generic_mut` unless `index_subslice_mut` has side effects on `s`'s internal data.
    // For `Sbv<T>` and `SbvComplex`, `index_subslice_mut` (if derived) should return `Self`.
    // The goal of `generic_slice_mut` is to test that such subslices are valid and can be mutated.
    // The state of `s` is not easily reset without `S: Clone` here. Tests are sequential.

    let r = ..;
    generic_mut(s.index_subslice_mut(r), new_val_fn);
    // s might be "partially moved" if index_subslice_mut takes &mut self and returns an owned part.
    // Or, if index_subslice_mut takes self, then s is gone.
    // The current SliceByValueSubsliceMut trait implies index_subslice_mut(&mut self, ...) -> Self::SubsliceMut
    // And SubsliceMut is often Self for these wrappers. So it takes &mut self and returns Self.
    // This means the original `s` is modified by the operation that creates the subslice,
    // and then `generic_mut` consumes that subslice.

    // To test the next operation, we need `s` again.
    // This structure implies `s` must be a type where `index_subslice_mut` doesn't consume `s` entirely,
    // allowing further operations. Or, `generic_slice_mut` should only test one subslice op.
    // Given the test structure, let's assume `s` is still valid after `s.index_subslice_mut(r)` for some `S`.
    // This is complex with by-value owned slices.
    // A simpler `generic_slice_mut` might just pick ONE range, get subslice, and call `generic_mut`.
    // The current extensive tests for all ranges are more suited if s is cloned for each.
    // Let's simplify for now: test a few distinct operations, assuming `s` remains usable.

    // Test `get_subslice_mut` for full range
    // This requires `s` to be usable after the previous `generic_mut` call.
    // This can only work if `index_subslice_mut` returned a subslice that, when mutated by `generic_mut`,
    // reflected those changes back into the original `s`'s data store without consuming `s`.
    // This is typical if `SubsliceMut` is `&mut [T]`. But for `Sbv`, `SubsliceMut` is `Sbv<T>`.
    // So, `generic_mut` consumes the returned `Sbv<T>`. The original `s` IS NOT MODIFIED by `generic_mut`.
    // The `index_subslice_mut` method itself would be what modifies `s`.

    // The design of these tests needs `S: Clone` to test each operation independently.
    // If `S: Clone` is removed, then `generic_slice_mut` can only test a sequence of operations
    // that destructively modify `s`.
    // Given the errors with `&mut T: Clone`, let's assume `S` IS Cloneable and passed by value.
    // The calls from `test_slice_by_value.rs` for `&mut [i32]` must be changed.
    // For now, keep `S: Clone` and fix the calls in `test_slice_by_value.rs`.
    // The previous diff for `generic_slice_mut` (the one that failed to apply) was closer.
    // I need to re-read the original `generic_slice_mut` and apply fixes carefully.

    // Re-instating S: Clone and keeping internal clones for now to resolve E0277 first.
    // The issue with `&mut [i32]: Clone` will be handled at call sites.
    // The `T: 'static` was removed.

    // This is the version from before the last attempt, with T: 'static removed and S: Clone kept.
    // The E0502 errors were the main target.

    // Handle empty case first.
    let is_s_empty = s.is_empty(); // Check before mutable borrow
    if is_s_empty {
        if let Some(empty_sub) = s.get_subslice_mut(..) {
            generic_mut(empty_sub, new_val_fn);
        }
        return;
    }

    // Test full range `..`
    let r = ..;
    generic_mut(s.clone().index_subslice_mut(r), new_val_fn); // Use s.clone()

    let mut s_clone_for_get_full = s.clone();
    let sub_opt_full = s_clone_for_get_full.get_subslice_mut(r); // Borrows s_clone_for_get_full
    if let Some(sub_mut) = sub_opt_full {
        generic_mut(sub_mut, new_val_fn);
    } else {
        panic!("get_subslice_mut(..) returned None for non-empty slice (was len {})", s.len());
    }
    // s_clone_for_get_full is fine here.
    generic_mut(unsafe { s.clone().get_subslice_unchecked_mut(r) }, new_val_fn);


    // Test range `1..`
    if s.len() > 1 {
        let r = 1..;
        generic_mut(s.clone().index_subslice_mut(r.clone()), new_val_fn);
        let mut s_clone_for_get_1 = s.clone();
        let sub_opt_1 = s_clone_for_get_1.get_subslice_mut(r.clone());
        if let Some(sub_mut) = sub_opt_1 {
            generic_mut(sub_mut, new_val_fn);
        } else {
            panic!("get_subslice_mut(1..) returned None");
        }
        generic_mut(unsafe { s.clone().get_subslice_unchecked_mut(r.clone()) }, new_val_fn);
    }

    // Test range `1..X` (e.g., `1..4`)
    let end_1_x = std::cmp::min(4, s.len());
    if s.len() >= 1 && end_1_x > 1 {
        let r = 1..end_1_x;
        if r.start < r.end {
            generic_mut(s.clone().index_subslice_mut(r.clone()), new_val_fn);
            let mut s_clone_for_get_1x = s.clone();
            if let Some(sub_mut) = s_clone_for_get_1x.get_subslice_mut(r.clone()) {
                generic_mut(sub_mut, new_val_fn);
            } else {
                panic!("get_subslice_mut(1..{}) returned None", end_1_x);
            }
            generic_mut(unsafe { s.clone().get_subslice_unchecked_mut(r.clone()) }, new_val_fn);
        }
    }

    // Test range `..X` (e.g., `..3`)
    let end_x = std::cmp::min(3, s.len());
    if end_x > 0 {
        let r = ..end_x;
        generic_mut(s.clone().index_subslice_mut(r.clone()), new_val_fn);
        let mut s_clone_for_get_x = s.clone();
        if let Some(sub_mut) = s_clone_for_get_x.get_subslice_mut(r.clone()) {
            generic_mut(sub_mut, new_val_fn);
        } else {
             panic!("get_subslice_mut(..{}) returned None", end_x);
        }
        generic_mut(unsafe { s.clone().get_subslice_unchecked_mut(r.clone()) }, new_val_fn);
    }

    // Test range `..=X` (e.g., `..=3`)
    if s.len() > 3 {
        let r = ..=3;
        generic_mut(s.clone().index_subslice_mut(r.clone()), new_val_fn);
        let mut s_clone_for_get_eqx = s.clone();
        if let Some(sub_mut) = s_clone_for_get_eqx.get_subslice_mut(r.clone()) {
            generic_mut(sub_mut, new_val_fn);
        } else {
            panic!("get_subslice_mut(..=3) returned None");
        }
        generic_mut(unsafe { s.clone().get_subslice_unchecked_mut(r.clone()) }, new_val_fn);
    }

    // Test range `Y..=X` (e.g., `1..=4`)
    if s.len() > 4 {
        let r = 1..=4;
        if *r.start() <= *r.end() && *r.end() < s.len() {
            generic_mut(s.clone().index_subslice_mut(r.clone()), new_val_fn);
            let mut s_clone_for_get_yx = s.clone();
            if let Some(sub_mut) = s_clone_for_get_yx.get_subslice_mut(r.clone()) {
                generic_mut(sub_mut, new_val_fn);
            } else {
                panic!("get_subslice_mut({:?}) returned None", r);
            }
            generic_mut(unsafe { s.clone().get_subslice_unchecked_mut(r.clone()) }, new_val_fn);
        }
    }

    // Boundary condition checks for get_subslice_mut
    assert!(s.get_subslice_mut(s.len()..s.len()).is_some()); // Empty slice at the end
    assert!(s.get_subslice_mut(0..0).is_some()); // Empty slice at the beginning
    if !s.is_empty() {
        assert!(s.get_subslice_mut(1..usize::MAX).is_none());
        assert!(s.get_subslice_mut(1..=usize::MAX).is_none());
    }
    assert!(s.get_subslice_mut(..=usize::MAX).is_none());
    assert!(s.get_subslice_mut(..usize::MAX).is_none());
    assert!(s.get_subslice_mut(usize::MAX..).is_none());
}
/// Generic function to test `IterableByValue` and `IterableByValueFrom` implementations.
///
/// It checks forward iteration using `iter_value()` and `iter_value_from()`.
/// Note: Current limitations with derive macros (e.g., `SbvComplex` not deriving `IterableByValue` correctly)
/// might cause E0277 errors.
pub fn generic_iter<T, S>(s: &S, expected: &[T])
where
    T: Clone + PartialEq + std::fmt::Debug,
    S: ?Sized + IterableByValue<Item = T> + IterableByValueFrom<Item = T>, // Added ?Sized
{
    let s_borrow = s.borrow(); // s is already a reference, borrowing it should be fine.

    let mut iter = IterableByValue::iter_value(s_borrow);
    let mut truth = expected.iter();
    for _ in 0..=expected.len() { // Iterate one past to check None case
        assert_eq!(iter.next().as_ref(), truth.next());
    }

    for start in 0..expected.len() {
        let mut iter = IterableByValueFrom::iter_value_from(s_borrow, start);
        let mut truth = expected[start..].iter();
        for _ in 0..=(expected.len() - start) { // Iterate one past
            assert_eq!(iter.next().as_ref(), truth.next());
        }
    }
     // Test iterating beyond bounds for iter_value_from
    if !expected.is_empty() {
        let mut iter_oob = IterableByValueFrom::iter_value_from(s_borrow, expected.len());
        assert_eq!(iter_oob.next(), None);
    }
    let mut iter_empty_oob = IterableByValueFrom::iter_value_from(s_borrow, 0);
    if expected.is_empty() {
        assert_eq!(iter_empty_oob.next(), None);
    } else {
        assert_eq!(iter_empty_oob.next().as_ref(), Some(&expected[0]));
    }

}

/// Generic function to test iterators that also implement `ExactSizeIterator` and `DoubleEndedIterator`.
///
/// It checks `len()`, `next()`, and `next_back()` on iterators from `iter_value()` and `iter_value_from()`.
/// Note: Current limitations with derive macros (iterators not deriving these extended traits correctly)
/// might cause E0277 errors.
pub fn generic_derived_iter<T, S>(s: S, expected: &[T])
where
    T: Clone + PartialEq + std::fmt::Debug,
    S: Borrow<S>,
    S: IterableByValue<Item = T> + IterableByValueFrom<Item = T>,
    // Use the GAT directly to avoid ambiguity with derived Iter/IterFrom types
    for<'a> <S as IterableByValueGat<'a>>::Iter: Iterator<Item = T> + ExactSizeIterator + DoubleEndedIterator,
    for<'a> <S as IterableByValueFromGat<'a>>::IterFrom: Iterator<Item = T> + ExactSizeIterator + DoubleEndedIterator,
{
    let s_borrow = s.borrow();

    // Test iter_value
    let mut iter = IterableByValue::iter_value(s_borrow);
    let mut truth = expected.iter();
    assert_eq!(iter.len(), truth.len());
    for _ in 0..=expected.len() {
        assert_eq!(iter.next().as_ref(), truth.next());
        assert_eq!(iter.len(), truth.len());
    }

    // Test iter_value with next_back
    let mut iter_rev = IterableByValue::iter_value(s_borrow);
    let mut truth_rev = expected.iter();
    assert_eq!(iter_rev.len(), truth_rev.len());
    for i in 0..expected.len() {
        if i % 2 == 0 {
            assert_eq!(iter_rev.next().as_ref(), truth_rev.next());
        } else {
            assert_eq!(iter_rev.next_back().as_ref(), truth_rev.next_back());
        }
        assert_eq!(iter_rev.len(), truth_rev.len());
    }
     assert_eq!(iter_rev.next(), None);
     assert_eq!(iter_rev.next_back(), None);


    // Test iter_value_from
    for start in 0..expected.len() {
        let mut iter_from = IterableByValueFrom::iter_value_from(s_borrow, start);
        let mut truth_from = expected[start..].iter();
        assert_eq!(iter_from.len(), truth_from.len());
        for _ in 0..=(expected.len() - start) {
            assert_eq!(iter_from.next().as_ref(), truth_from.next());
            assert_eq!(iter_from.len(), truth_from.len());
        }

        // Test iter_value_from with next_back
        let mut iter_from_rev = IterableByValueFrom::iter_value_from(s_borrow, start);
        let mut truth_from_rev = expected[start..].iter();
        assert_eq!(iter_from_rev.len(), truth_from_rev.len());
        for i in 0..(expected.len() - start) {
            if i % 2 == 0 {
                assert_eq!(iter_from_rev.next().as_ref(), truth_from_rev.next());
            } else {
                assert_eq!(iter_from_rev.next_back().as_ref(), truth_from_rev.next_back());
            }
            assert_eq!(iter_from_rev.len(), truth_from_rev.len());
        }
        assert_eq!(iter_from_rev.next(), None);
        assert_eq!(iter_from_rev.next_back(), None);
    }
     // Test iter_value_from with start == len
    let mut iter_from_end = IterableByValueFrom::iter_value_from(s_borrow, expected.len());
    assert_eq!(iter_from_end.len(), 0);
    assert_eq!(iter_from_end.next(), None);
    assert_eq!(iter_from_end.next_back(), None);
}
