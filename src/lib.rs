#![cfg_attr(not(feature = "std"), no_std)]
#![deny(unconditional_recursion)]

use core::ops::Range;
use slices::{
    LengthValue, SliceByValueGet, SliceByValueRange, SliceByValueRangeMut, SliceByValueRepl,
    SliceByValueSet,
};

pub mod iter;
pub mod slices;

impl<S: LengthValue + ?Sized> LengthValue for &S {
    type Value = S::Value;
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
}

impl<S: LengthValue + ?Sized> LengthValue for &mut S {
    type Value = S::Value;
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
}

impl<T> LengthValue for [T] {
    type Value = T;
    #[inline]
    fn len(&self) -> usize {
        <[T]>::len(self)
    }
}

// Implement SliceByValue for &S by delegating to S
impl<S: SliceByValueGet + ?Sized> SliceByValueGet for &S {
    fn get_value(&self, index: usize) -> Option<Self::Value> {
        (**self).get_value(index)
    }
    fn index_value(&self, index: usize) -> Self::Value {
        (**self).index_value(index)
    }
    unsafe fn get_value_unchecked(&self, index: usize) -> Self::Value {
        (**self).get_value_unchecked(index)
    }
}

// Implement SliceByValue for &mut S by delegating to S (for read-only access)
impl<S: SliceByValueGet + ?Sized> SliceByValueGet for &mut S {
    fn get_value(&self, index: usize) -> Option<Self::Value> {
        (**self).get_value(index)
    }
    fn index_value(&self, index: usize) -> Self::Value {
        (**self).index_value(index)
    }
    unsafe fn get_value_unchecked(&self, index: usize) -> Self::Value {
        (**self).get_value_unchecked(index)
    }
}

impl<S: SliceByValueSet + ?Sized> SliceByValueSet for &mut S {
    fn set_value(&mut self, index: usize, value: Self::Value) {
        (**self).set_value(index, value)
    }
    unsafe fn set_value_unchecked(&mut self, index: usize, value: Self::Value) {
        (**self).set_value_unchecked(index, value)
    }
}

impl<S: SliceByValueRepl + ?Sized> SliceByValueRepl for &mut S {
    fn replace_value(&mut self, index: usize, value: Self::Value) -> Self::Value {
        (**self).replace_value(index, value)
    }
    unsafe fn replace_value_unchecked(&mut self, index: usize, value: Self::Value) -> Self::Value {
        (**self).replace_value_unchecked(index, value)
    }
}

// --- Implementations for standard slices [T] and usize index ---
impl<T: Clone> SliceByValueGet for [T] {
    #[inline]
    fn get_value(&self, index: usize) -> Option<Self::Value> {
        // slice.get returns Option<&T>, .copied() converts to Option<T>
        (*self).get(index).cloned()
    }

    #[inline]
    fn index_value(&self, index: usize) -> Self::Value {
        // Standard indexing panics on out-of-bounds.
        // It returns &T, which we copy to return T.
        self[index].clone()
    }

    #[inline]
    unsafe fn get_value_unchecked(&self, index: usize) -> Self::Value {
        // Safety: The caller must ensure that `*self` (the index) is in bounds.
        // slice.get_unchecked returns &T, which we dereference and copy.
        unsafe { (*self).get_unchecked(index).clone() }
    }
}

impl<T: Clone> SliceByValueSet for [T] {
    #[inline]
    fn set_value(&mut self, index: usize, value: Self::Value) {
        // Standard indexing panics on out-of-bounds.
        self[index] = value;
    }

    #[inline]
    unsafe fn set_value_unchecked(&mut self, index: usize, value: Self::Value) {
        // Safety: The caller must ensure that `*self` (the index) is in bounds.
        unsafe {
            let elem = self.get_unchecked_mut(index);
            *elem = value;
        }
    }
}

impl<T: Clone> SliceByValueRepl for [T] {
    #[inline]
    fn replace_value(&mut self, index: usize, value: Self::Value) -> Self::Value {
        // Standard indexing panics on out-of-bounds.
        // We get a mutable reference `&mut T`.
        // mem::replace swaps the value at the location with the new `value`
        // and returns the old value.
        core::mem::replace(&mut self[index], value)
    }

    #[inline]
    unsafe fn replace_value_unchecked(&mut self, index: usize, value: Self::Value) -> Self::Value {
        // Safety: The caller must ensure that `*self` (the index) is in bounds.
        unsafe {
            let elem = self.get_unchecked_mut(index);
            core::mem::replace(elem, value)
        }
    }
}

impl<'a, T: Clone> SliceByValueRange<Range<usize>> for &'a [T] {
    type SliceRange<'b>
        = &'b [T]
    where
        Self: 'b;
    #[inline]
    fn get_range(&self, index: Range<usize>) -> Option<Self::SliceRange<'_>> {
        (*self).get(index)
    }

    #[inline]
    fn index_range(&self, index: Range<usize>) -> Self::SliceRange<'_> {
        &self[index]
    }

    #[inline]
    unsafe fn get_range_unchecked(&self, index: Range<usize>) -> Self::SliceRange<'_> {
        unsafe { (*self).get_unchecked(index) }
    }
}

impl<'a, T: Clone> SliceByValueRangeMut<Range<usize>> for &'a mut [T] {
    type SliceRangeMut<'b>
        = &'b mut [T]
    where
        Self: 'b;
    #[inline]
    fn get_range_mut(&mut self, index: Range<usize>) -> Option<Self::SliceRangeMut<'_>> {
        (*self).get_mut(index)
    }

    #[inline]
    fn index_range_mut(&mut self, index: Range<usize>) -> Self::SliceRangeMut<'_> {
        &mut self[index]
    }

    #[inline]
    unsafe fn get_range_unchecked_mut(&mut self, index: Range<usize>) -> Self::SliceRangeMut<'_> {
        unsafe { (*self).get_unchecked_mut(index) }
    }
}

impl<T, const N: usize> LengthValue for [T; N] {
    type Value = T;
    #[inline(always)]
    fn len(&self) -> usize {
        N
    }
}

impl<T: Clone, const N: usize> SliceByValueGet for [T; N] {
    #[inline]
    fn get_value(&self, index: usize) -> Option<Self::Value> {
        // slice.get returns Option<&T>, .copied() converts to Option<T>
        (*self).get(index).cloned()
    }

    #[inline]
    fn index_value(&self, index: usize) -> Self::Value {
        // Standard indexing panics on out-of-bounds.
        // It returns &T, which we copy to return T.
        self[index].clone()
    }

    #[inline]
    unsafe fn get_value_unchecked(&self, index: usize) -> Self::Value {
        // Safety: The caller must ensure that `*self` (the index) is in bounds.
        // slice.get_unchecked returns &T, which we dereference and copy.
        unsafe { (*self).get_unchecked(index).clone() }
    }
}

impl<T: Clone, const N: usize> SliceByValueSet for [T; N] {
    #[inline]
    fn set_value(&mut self, index: usize, value: Self::Value) {
        // Standard indexing panics on out-of-bounds.
        self[index] = value;
    }

    #[inline]
    unsafe fn set_value_unchecked(&mut self, index: usize, value: Self::Value) {
        // Safety: The caller must ensure that `*self` (the index) is in bounds.
        unsafe {
            let elem = self.get_unchecked_mut(index);
            *elem = value;
        }
    }
}

impl<T: Clone, const N: usize> SliceByValueRepl for [T; N] {
    #[inline]
    fn replace_value(&mut self, index: usize, value: Self::Value) -> Self::Value {
        // Standard indexing panics on out-of-bounds.
        // We get a mutable reference `&mut T`.
        // mem::replace swaps the value at the location with the new `value`
        // and returns the old value.
        core::mem::replace(&mut self[index], value)
    }

    #[inline]
    unsafe fn replace_value_unchecked(&mut self, index: usize, value: Self::Value) -> Self::Value {
        // Safety: The caller must ensure that `*self` (the index) is in bounds.
        unsafe {
            let elem = self.get_unchecked_mut(index);
            core::mem::replace(elem, value)
        }
    }
}

impl<'a, T: Clone, const N: usize> SliceByValueRange<Range<usize>> for &'a [T; N] {
    type SliceRange<'b>
        = &'b [T]
    where
        Self: 'b;

    #[inline]
    fn get_range(&self, index: Range<usize>) -> Option<Self::SliceRange<'_>> {
        (*self).get(index)
    }

    #[inline]
    fn index_range(&self, index: Range<usize>) -> Self::SliceRange<'_> {
        &self[index]
    }

    #[inline]
    unsafe fn get_range_unchecked(&self, index: Range<usize>) -> Self::SliceRange<'_> {
        unsafe { (*self).get_unchecked(index) }
    }
}

#[cfg(feature = "alloc")]
mod alloc_impls {
    use super::*;
    extern crate alloc;
    use alloc::boxed::Box;
    use alloc::vec::Vec;

    impl<S: LengthValue + ?Sized> LengthValue for Box<S> {
        type Value = S::Value;
        #[inline]
        fn len(&self) -> usize {
            (**self).len()
        }
    }

    impl<S: SliceByValueGet + ?Sized> SliceByValueGet for Box<S> {
        fn get_value(&self, index: usize) -> Option<Self::Value> {
            (**self).get_value(index)
        }
        fn index_value(&self, index: usize) -> Self::Value {
            (**self).index_value(index)
        }
        unsafe fn get_value_unchecked(&self, index: usize) -> Self::Value {
            (**self).get_value_unchecked(index)
        }
    }

    impl<S: SliceByValueRepl + ?Sized> SliceByValueRepl for Box<S> {
        fn replace_value(&mut self, index: usize, value: Self::Value) -> Self::Value {
            (**self).replace_value(index, value)
        }
        unsafe fn replace_value_unchecked(
            &mut self,
            index: usize,
            value: Self::Value,
        ) -> Self::Value {
            (**self).replace_value_unchecked(index, value)
        }
    }

    impl<S: SliceByValueSet + ?Sized> SliceByValueSet for Box<S> {
        fn set_value(&mut self, index: usize, value: Self::Value) {
            (**self).set_value(index, value)
        }
        unsafe fn set_value_unchecked(&mut self, index: usize, value: Self::Value) {
            (**self).set_value_unchecked(index, value)
        }
    }

    impl<T> LengthValue for Vec<T> {
        type Value = T;
        #[inline]
        fn len(&self) -> usize {
            <[T]>::len(self)
        }
    }

    impl<T: Clone> SliceByValueGet for Vec<T> {
        #[inline]
        fn get_value(&self, index: usize) -> Option<Self::Value> {
            // slice.get returns Option<&T>, .copied() converts to Option<T>
            (*self).get(index).cloned()
        }

        #[inline]
        fn index_value(&self, index: usize) -> Self::Value {
            // Standard indexing panics on out-of-bounds.
            // It returns &T, which we copy to return T.
            self[index].clone()
        }

        #[inline]
        unsafe fn get_value_unchecked(&self, index: usize) -> Self::Value {
            // Safety: The caller must ensure that `*self` (the index) is in bounds.
            // slice.get_unchecked returns &T, which we dereference and copy.
            unsafe { (*self).get_unchecked(index).clone() }
        }
    }

    impl<T: Clone> SliceByValueRepl for Vec<T> {
        #[inline]
        fn replace_value(&mut self, index: usize, value: Self::Value) -> Self::Value {
            // Standard indexing panics on out-of-bounds.
            // We get a mutable reference `&mut T`.
            // mem::replace swaps the value at the location with the new `value`
            // and returns the old value.
            core::mem::replace(&mut self[index], value)
        }

        #[inline]
        unsafe fn replace_value_unchecked(
            &mut self,
            index: usize,
            value: Self::Value,
        ) -> Self::Value {
            // Safety: The caller must ensure that `*self` (the index) is in bounds.
            unsafe {
                let elem = self.get_unchecked_mut(index);
                core::mem::replace(elem, value)
            }
        }
    }

    impl<T: Clone> SliceByValueSet for Vec<T> {
        #[inline]
        fn set_value(&mut self, index: usize, value: Self::Value) {
            // Standard indexing panics on out-of-bounds
            self[index] = value;
        }

        #[inline]
        unsafe fn set_value_unchecked(&mut self, index: usize, value: Self::Value) {
            // Safety: The caller must ensure that `*self` (the index) is in bounds.
            unsafe {
                let elem = self.get_unchecked_mut(index);
                *elem = value;
            }
        }
    }

    impl<'a, T: Clone> SliceByValueRange<Range<usize>> for &'a Vec<T> {
        type SliceRange<'b>
            = &'b [T]
        where
            Self: 'b;

        #[inline]
        fn get_range(&self, index: Range<usize>) -> Option<Self::SliceRange<'_>> {
            // slice.get returns Option<&T>, .copied() converts to Option<T>
            (*self).get(index)
        }

        #[inline]
        fn index_range(&self, index: Range<usize>) -> Self::SliceRange<'_> {
            &self[index]
        }

        #[inline]
        unsafe fn get_range_unchecked(&self, index: Range<usize>) -> Self::SliceRange<'_> {
            unsafe { (*self).get_unchecked(index) }
        }
    }
}

#[cfg(feature = "std")]
mod std_impls {
    use super::*;
    use std::{rc::Rc, sync::Arc};

    impl<S: LengthValue + ?Sized> LengthValue for Arc<S> {
        type Value = S::Value;
        #[inline]
        fn len(&self) -> usize {
            (**self).len()
        }
    }

    impl<S: SliceByValueGet + ?Sized> SliceByValueGet for Arc<S> {
        fn get_value(&self, index: usize) -> Option<Self::Value> {
            (**self).get_value(index)
        }
        fn index_value(&self, index: usize) -> Self::Value {
            (**self).index_value(index)
        }
        unsafe fn get_value_unchecked(&self, index: usize) -> Self::Value {
            (**self).get_value_unchecked(index)
        }
    }

    impl<S: LengthValue + ?Sized> LengthValue for Rc<S> {
        type Value = S::Value;
        #[inline]
        fn len(&self) -> usize {
            (**self).len()
        }
    }

    impl<S: SliceByValueGet + ?Sized> SliceByValueGet for Rc<S> {
        fn get_value(&self, index: usize) -> Option<Self::Value> {
            (**self).get_value(index)
        }
        fn index_value(&self, index: usize) -> Self::Value {
            (**self).index_value(index)
        }
        unsafe fn get_value_unchecked(&self, index: usize) -> Self::Value {
            (**self).get_value_unchecked(index)
        }
    }
}
