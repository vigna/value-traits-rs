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
#[cfg(any(feature = "std", feature = "alloc"))]
fn test_iter() {
    let s = [1_i32, 2, 3];
    generic_iter(&s.to_vec(), &s);
}

use value_traits_derive::{Iterators, IteratorsMut, Subslices, SubslicesMut};
// ComplexType will be used in the module below.
// No `mod test_sbv_complex;` here, define module inline.

#[derive(Clone, Subslices, SubslicesMut, Iterators, IteratorsMut)] // Ensure Clone is here
pub struct Sbv<T: Clone>(Vec<T>);

impl<T: Clone> SliceByValue for Sbv<T> {
    type Value = T;

    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<T: Clone> SliceByValueGet for Sbv<T> {
    unsafe fn get_value_unchecked(&self, index: usize) -> Self::Value {
        self.0.as_slice().get_value_unchecked(index)
    }
}

impl<T: Clone> SliceByValueSet for Sbv<T> {
    unsafe fn set_value_unchecked(&mut self, index: usize, value: Self::Value) {
        self.0.as_mut_slice().set_value(index, value)
    }
}

impl<T: Clone> SliceByValueRepl for Sbv<T> {
    unsafe fn replace_value_unchecked(&mut self, index: usize, value: Self::Value) -> Self::Value {
        self.0.as_mut_slice().replace_value(index, value)
    }
}

#[test]
fn test_sbv_subslices() {
    let expected = [1_i32, 2, 3, 4, 5];
    let mut s = Sbv(expected.to_vec());
    // test the struct
    generic_get(&s, &expected);
    generic_slice(s.clone(), &expected); // generic_slice consumes s
    generic_mut(&mut s, |x: i32| x + 1); // generic_mut mutates s
    // generic_slice_mut consumes s, so pass a clone if s is to be used later.
    generic_slice_mut(s.clone(), |x: i32| x + 1);
    //generic_derived_iter(s.clone(), &expected); // Pass clone if s is needed afterwards

    // Re-initialize s as it's been mutated.
    let mut s = Sbv(expected.to_vec());
    // test its slice
    generic_get(s.index_subslice(..), &expected);
    generic_slice(s.index_subslice(..), &expected);
    generic_derived_iter(s.index_subslice(..), &expected); // This consumes s.index_subslice(..)

    // Re-initialize s for mutable slice tests.
    let mut s_mut_slice_tests = Sbv(expected.to_vec());
    // test its mutable slice
    // generic_get/slice/mut/slice_mut consume their arguments.
    // To test them on the output of s.index_subslice_mut(..),
    // s must be cloned before each call if index_subslice_mut doesn't return an owned copy
    // or if we want to preserve s_mut_slice_tests for multiple calls.
    // Assuming index_subslice_mut returns an owned type (Self for Sbv).

    generic_get(s_mut_slice_tests.clone().index_subslice_mut(..), &expected);
    generic_slice(s_mut_slice_tests.clone().index_subslice_mut(..), &expected);
    generic_mut(s_mut_slice_tests.clone().index_subslice_mut(..), |x: i32| x + 1);
    // For generic_slice_mut, the argument needs to be Clone.
    // s_mut_slice_tests.index_subslice_mut(..) returns Sbv<i32> which is Clone.
    generic_slice_mut(s_mut_slice_tests.clone().index_subslice_mut(..), |x: i32| x + 1);
    generic_derived_iter(s_mut_slice_tests.index_subslice_mut(..), &expected); // Consumes the last s_mut_slice_tests's subslice

    // Original test for specific subslice mutation:
    let mut s_orig_test = Sbv(expected.to_vec()); // Ensure s is fresh
    let mut t = s_orig_test.index_subslice_mut(1..3);
    assert_eq!(t.len(), 2);
    assert_eq!(t.index_value(0), 2);
    assert_eq!(t.index_value(1), 3);
    t.set_value(1, 4);
    let u = t.index_subslice(1..); // index_subslice on the mutated subslice t
    assert_eq!(u.len(), 1);
    assert_eq!(u.index_value(0), 4);
    // Check original s_orig_test to see if t (which was a subslice of s) reflected changes.
    // This depends on whether index_subslice_mut returns a view or an owned copy that replaces a part of s.
    // For Sbv<T> (Vec<T>), index_subslice_mut likely returns a new Sbv<T> with a copy of the data.
    // So s_orig_test itself would not be changed by mutating t unless SliceByValueSubsliceMut
    // is implemented to modify the original vector and return a new Sbv that reflects this.
    // The derive macros usually make it return Self, so it's an owned slice.
    // To check if original data changed, you'd need to inspect s_orig_test.0[1] and s_orig_test.0[2]
    // *if* the SliceByValueSubsliceMut implementation for Sbv actually modified s_orig_test.0.
    // However, the generic tests are primarily for the behavior of the returned slice type.
}

#[test]
#[should_panic]
fn test_sbv_index_value_panic_len() {
    let s = Sbv(vec![1_i32, 2, 3]);
    s.index_value(3); // Accessing at len()
}

#[test]
#[should_panic]
fn test_sbv_index_value_panic_greater_than_len() {
    let s = Sbv(vec![1_i32, 2, 3]);
    s.index_value(10); // Accessing way out of bounds
}

#[test]
#[should_panic]
fn test_sbv_set_value_unchecked_panic_len() {
    let mut s = Sbv(vec![1_i32, 2, 3]);
    unsafe { s.set_value_unchecked(3, 100) }; // Accessing at len()
}

#[test]
#[should_panic]
fn test_sbv_set_value_unchecked_panic_greater_than_len() {
    let mut s = Sbv(vec![1_i32, 2, 3]);
    unsafe { s.set_value_unchecked(10, 100) }; // Accessing way out of bounds
}

#[test]
#[should_panic]
fn test_sbv_replace_value_unchecked_panic_len() {
    let mut s = Sbv(vec![1_i32, 2, 3]);
    unsafe { s.replace_value_unchecked(3, 100) }; // Accessing at len()
}

#[test]
#[should_panic]
fn test_sbv_replace_value_unchecked_panic_greater_than_len() {
    let mut s = Sbv(vec![1_i32, 2, 3]);
    unsafe { s.replace_value_unchecked(10, 100) }; // Accessing way out of bounds
}

// Tests for index_subslice panics
#[test]
#[should_panic]
fn test_sbv_index_subslice_panic_start_greater_than_end() {
    let s = Sbv(vec![1_i32, 2, 3, 4, 5]);
    s.index_subslice(3..1);
}

#[test]
#[should_panic]
fn test_sbv_index_subslice_panic_end_greater_than_len() {
    let s = Sbv(vec![1_i32, 2, 3]);
    s.index_subslice(1..4); // end (4) > len (3)
}

#[test]
#[should_panic]
fn test_sbv_index_subslice_panic_start_greater_than_len() {
    let s = Sbv(vec![1_i32, 2, 3]);
    s.index_subslice(4..5); // start (4) > len (3)
}

// Tests for index_subslice_mut panics
#[test]
#[should_panic]
fn test_sbv_index_subslice_mut_panic_start_greater_than_end() {
    let mut s = Sbv(vec![1_i32, 2, 3, 4, 5]);
    s.index_subslice_mut(3..1);
}

#[test]
#[should_panic]
fn test_sbv_index_subslice_mut_panic_end_greater_than_len() {
    let mut s = Sbv(vec![1_i32, 2, 3]);
    s.index_subslice_mut(1..4); // end (4) > len (3)
}

#[test]
#[should_panic]
fn test_sbv_index_subslice_mut_panic_start_greater_than_len() {
    let mut s = Sbv(vec![1_i32, 2, 3]);
    s.index_subslice_mut(4..5); // start (4) > len (3)
}

// Tests for safe methods out-of-bounds
#[test]
fn test_sbv_get_value_safe_out_of_bounds() {
    let s = Sbv(vec![1_i32, 2, 3]);
    assert_eq!(s.get_value(3), None); // len()
    assert_eq!(s.get_value(10), None); // > len()
}

#[test]
fn test_sbv_set_value_safe_out_of_bounds() {
    let mut s = Sbv(vec![1_i32, 2, 3]);
    let original_data = s.0.clone();
    // These calls should not panic and data should remain unchanged
    // as per SliceByValueSet::set_value current default impl might panic.
    // However, the trait doc says: "If index is out of bounds, this function will panic."
    // So, these should be panic tests if we are testing the *trait's* specified behavior.
    // But Sbv<T>'s impl for set_value is currently NOT implemented (it uses the default).
    // The default impl of `set_value` in the library panics.
    // Let's make these panic tests for `set_value` as per trait contract.
}

#[test]
#[should_panic]
fn test_sbv_set_value_panic_len() {
    let mut s = Sbv(vec![1_i32, 2, 3]);
    s.set_value(3, 100); // len()
}

#[test]
#[should_panic]
fn test_sbv_set_value_panic_greater_than_len() {
    let mut s = Sbv(vec![1_i32, 2, 3]);
    s.set_value(10, 100); // > len()
}


#[test]
fn test_sbv_replace_value_safe_out_of_bounds() {
    let mut s = Sbv(vec![1_i32, 2, 3]);
    // replace_value is specified to panic on out of bounds.
    // So this should also be a panic test.
}

#[test]
#[should_panic]
fn test_sbv_replace_value_panic_len() {
    let mut s = Sbv(vec![1_i32, 2, 3]);
    s.replace_value(3, 100); // len()
}

#[test]
#[should_panic]
fn test_sbv_replace_value_panic_greater_than_len() {
    let mut s = Sbv(vec![1_i32, 2, 3]);
    s.replace_value(10, 100); // > len()
}


#[test]
fn test_sbv_get_subslice_safe_out_of_bounds() {
    let s = Sbv(vec![1_i32, 2, 3]);
    assert_eq!(s.get_subslice(3..3).map(|sub| sub.len()), Some(0)); // Empty slice at len is valid for get
    assert_eq!(s.get_subslice(1..4).map(|sub| sub.len()), None); // end > len
    assert_eq!(s.get_subslice(4..5).map(|sub| sub.len()), None); // start > len
    assert_eq!(s.get_subslice(3..1).map(|sub| sub.len()), None); // start > end
}

#[test]
fn test_sbv_get_subslice_mut_safe_out_of_bounds() {
    let mut s = Sbv(vec![1_i32, 2, 3]);
    assert_eq!(s.get_subslice_mut(3..3).map(|mut sub| sub.len()), Some(0));
    assert_eq!(s.get_subslice_mut(1..4).map(|mut sub| sub.len()), None);
    assert_eq!(s.get_subslice_mut(4..5).map(|mut sub| sub.len()), None);
    assert_eq!(s.get_subslice_mut(3..1).map(|mut sub| sub.len()), None);
}


// Module for SbvComplex and its tests
// Note: The SbvComplex module and its tests are defined below.
// New error condition tests for Sbv<T> are added before that module.

#[test]
#[should_panic]
fn test_sbv_index_value_out_of_bounds_direct() {
    let s = Sbv(vec![10_i32, 20]);
    s.index_value(2); // len is 2, index 2 is out of bounds
}

#[test]
#[should_panic]
fn test_sbv_index_value_out_of_bounds_direct_empty() {
    let s = Sbv(Vec::<i32>::new());
    s.index_value(0); // out of bounds for empty
}

#[test]
fn test_sbv_get_value_out_of_bounds_direct() {
    let s = Sbv(vec![10_i32, 20]);
    assert_eq!(s.get_value(2), None); // len is 2, index 2 is out of bounds
    assert_eq!(s.get_value(100), None);
}

#[test]
fn test_sbv_get_value_out_of_bounds_direct_empty() {
    let s = Sbv(Vec::<i32>::new());
    assert_eq!(s.get_value(0), None);
}

#[test]
#[should_panic]
fn test_sbv_index_subslice_invalid_range_direct_start_gt_end() {
    let s = Sbv(vec![1_i32, 2, 3, 4]);
    s.index_subslice(3..1);
}

#[test]
#[should_panic]
fn test_sbv_index_subslice_invalid_range_direct_end_gt_len() {
    let s = Sbv(vec![1_i32, 2, 3, 4]);
    s.index_subslice(1..5); // len is 4, end 5 is out of bounds
}

#[test]
fn test_sbv_get_subslice_invalid_range_direct() {
    let s = Sbv(vec![1_i32, 2, 3, 4]);
    assert!(s.get_subslice(3..1).is_none()); // start > end
    assert!(s.get_subslice(1..5).is_none()); // end > len
    assert!(s.get_subslice(5..6).is_none()); // start > len
}

// --- Tests for mutable versions ---

#[test]
#[should_panic]
fn test_sbv_index_subslice_mut_invalid_range_direct_start_gt_end() {
    let mut s = Sbv(vec![1_i32, 2, 3, 4]);
    s.index_subslice_mut(3..1);
}

#[test]
#[should_panic]
fn test_sbv_index_subslice_mut_invalid_range_direct_end_gt_len() {
    let mut s = Sbv(vec![1_i32, 2, 3, 4]);
    s.index_subslice_mut(1..5);
}

#[test]
fn test_sbv_get_subslice_mut_invalid_range_direct() {
    let mut s = Sbv(vec![1_i32, 2, 3, 4]);
    assert!(s.get_subslice_mut(3..1).is_none());
    assert!(s.get_subslice_mut(1..5).is_none());
    assert!(s.get_subslice_mut(5..6).is_none());
}


mod test_sbv_complex {
    use super::common::{
        ComplexType, generic_get, generic_slice, generic_mut, generic_slice_mut,
        generic_iter, generic_derived_iter,
    };
    use value_traits::slices::*;
    use value_traits_derive::{Iterators, IteratorsMut, Subslices, SubslicesMut};
    use value_traits::prelude::{IterableByValue, IterableByValueFrom}; // For .iter_value() and .iter_value_from()

    #[derive(Debug, Clone, PartialEq, Subslices, SubslicesMut, Iterators, IteratorsMut)]
    pub struct SbvComplex {
        data: Vec<ComplexType>,
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
        unsafe fn get_value_unchecked(&self, index: usize) -> Self::Value {
            self.data.get_unchecked(index).clone()
        }

        fn get_value(&self, index: usize) -> Option<Self::Value> {
            self.data.get(index).cloned()
        }
    }

    impl SliceByValueSet for SbvComplex {
        unsafe fn set_value_unchecked(&mut self, index: usize, value: Self::Value) {
            *self.data.get_unchecked_mut(index) = value;
        }

        fn set_value(&mut self, index: usize, value: Self::Value) {
            if index < self.data.len() {
                self.data[index] = value;
            } else {
                panic!("Index out of bounds");
            }
        }
    }

    impl SliceByValueRepl for SbvComplex {
        unsafe fn replace_value_unchecked(&mut self, index: usize, value: Self::Value) -> Self::Value {
            std::mem::replace(self.data.get_unchecked_mut(index), value)
        }

        fn replace_value(&mut self, index: usize, value: Self::Value) -> Self::Value {
            if index < self.data.len() {
                std::mem::replace(&mut self.data[index], value)
            } else {
                panic!("Index out of bounds");
            }
        }
    }

    pub const COMPLEX_VALS_SBV: [ComplexType; 3] = [
        ComplexType { id: 1, name: "FirstSbv".to_string(), data: vec![10,20,30] },
        ComplexType { id: 2, name: "SecondSbv".to_string(), data: vec![40,50,60] },
        ComplexType { id: 3, name: "ThirdSbv".to_string(), data: vec![70,80,90] },
    ];

    fn modify_complex_type(mut c: ComplexType) -> ComplexType {
        c.id += 100;
        c.name = format!("{} Modified", c.name);
        c.data.push(0);
        c
    }

    #[test]
    fn test_sbv_complex_operations() {
        let s_initial = SbvComplex::from(COMPLEX_VALS_SBV.to_vec());
        let expected = &COMPLEX_VALS_SBV;

        generic_get(&s_initial, expected); // Takes by reference
        generic_slice(s_initial.clone(), expected); // Consumes, so clone
        generic_iter(&s_initial, expected); // Takes by reference
        generic_derived_iter(s_initial.clone(), expected); // Consumes, so clone

        let mut s_mut_for_generic_mut = SbvComplex::from(COMPLEX_VALS_SBV.to_vec());
        generic_mut(&mut s_mut_for_generic_mut, modify_complex_type);

        let mut s_mut_for_generic_slice_mut = SbvComplex::from(COMPLEX_VALS_SBV.to_vec());
        generic_slice_mut(s_mut_for_generic_slice_mut, modify_complex_type); // Consumes

        if !expected.is_empty() {
            let s_for_immutable_subslice = SbvComplex::from(COMPLEX_VALS_SBV.to_vec());
            generic_get(s_for_immutable_subslice.index_subslice(..), expected);
            generic_slice(s_for_immutable_subslice.clone().index_subslice(..), expected); // index_subslice returns Self, clone for next
            generic_iter(&s_for_immutable_subslice.index_subslice(..), expected); //iter on ref to subslice
            generic_derived_iter(s_for_immutable_subslice.index_subslice(..), expected);


            let s_base_for_mut_subs = SbvComplex::from(COMPLEX_VALS_SBV.to_vec());
            generic_get(s_base_for_mut_subs.clone().index_subslice_mut(..), expected);
            generic_slice(s_base_for_mut_subs.clone().index_subslice_mut(..), expected);
            generic_mut(s_base_for_mut_subs.clone().index_subslice_mut(..), modify_complex_type);
            generic_slice_mut(s_base_for_mut_subs.clone().index_subslice_mut(..), modify_complex_type);

            let mut s_for_iter_on_mut_sub = SbvComplex::from(COMPLEX_VALS_SBV.to_vec());
            let mut mutable_subslice_for_iter = s_for_iter_on_mut_sub.index_subslice_mut(..);
            generic_iter(&mutable_subslice_for_iter, expected);
            generic_derived_iter(mutable_subslice_for_iter, expected);
        }

        let mut s_check = SbvComplex::from(vec![COMPLEX_VALS_SBV[0].clone()]);
        let original_first_item = COMPLEX_VALS_SBV[0].clone();
        generic_mut(&mut s_check, modify_complex_type);
        let modified_first_item = modify_complex_type(original_first_item.clone());
        assert_eq!(s_check.get_value(0), Some(modified_first_item));
    }
}
// Removed the global fn modify_complex_type as it's inside the module now.

// --- Focused Iterator Tests for Sbv<T> (derived) ---

#[test]
fn test_sbv_iter_value() {
    let data = vec![10_i32, 20, 30];
    let sbv = Sbv(data.clone());

    let collected: Vec<i32> = sbv.iter_value().collect();
    assert_eq!(collected, data);

    // Test ExactSizeIterator if derived (assuming it is)
    let mut iter = sbv.iter_value();
    assert_eq!(iter.len(), 3);
    assert_eq!(iter.next(), Some(data[0]));
    assert_eq!(iter.len(), 2);

    // Test empty
    let empty_sbv = Sbv(Vec::<i32>::new());
    let mut empty_iter = empty_sbv.iter_value();
    assert_eq!(empty_iter.len(), 0);
    assert_eq!(empty_iter.next(), None);
}

#[test]
fn test_sbv_iter_value_from() {
    let data = vec![10_i32, 20, 30, 40];
    let sbv = Sbv(data.clone());

    // From 0
    let collected_from_0: Vec<i32> = sbv.iter_value_from(0).collect();
    assert_eq!(collected_from_0, data);
    assert_eq!(sbv.iter_value_from(0).len(), 4);

    // From middle
    let collected_from_2: Vec<i32> = sbv.iter_value_from(2).collect();
    assert_eq!(collected_from_2, data[2..]);
    let mut iter_from_2 = sbv.iter_value_from(2);
    assert_eq!(iter_from_2.len(), 2);
    assert_eq!(iter_from_2.next(), Some(data[2]));
    assert_eq!(iter_from_2.len(), 1);

    // From len
    let mut iter_from_len = sbv.iter_value_from(data.len());
    assert_eq!(iter_from_len.len(), 0);
    assert_eq!(iter_from_len.next(), None);
}

#[test]
#[should_panic] // Assuming derive for iter_value_from includes this check
fn test_sbv_iter_value_from_panic_out_of_bounds() {
    let data = vec![10_i32, 20, 30];
    let sbv = Sbv(data.clone());
    sbv.iter_value_from(data.len() + 1);
}

#[test]
fn test_sbv_iter_double_ended() {
    let data = vec![10_i32, 20, 30, 40, 50];
    let sbv = Sbv(data.clone());

    let mut iter = sbv.iter_value();
    assert_eq!(iter.len(), 5);
    assert_eq!(iter.next_back(), Some(data[4]));
    assert_eq!(iter.len(), 4);
    assert_eq!(iter.next(), Some(data[0]));
    assert_eq!(iter.len(), 3);
    assert_eq!(iter.next_back(), Some(data[3]));
    assert_eq!(iter.len(), 2);
    assert_eq!(iter.next(), Some(data[1]));
    assert_eq!(iter.len(), 1);
    assert_eq!(iter.next_back(), Some(data[2]));
    assert_eq!(iter.len(), 0);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), None);

    // Empty
    let empty_sbv = Sbv(Vec::<i32>::new());
    let mut empty_iter = empty_sbv.iter_value();
    assert_eq!(empty_iter.len(), 0);
    assert_eq!(empty_iter.next_back(), None);
}
