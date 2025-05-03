#![cfg_attr(not(feature = "std"), no_std)]
#![deny(unconditional_recursion)]

use core::ops::Range;
use slices::{Length, SliceByValue, SliceByValueMut};

pub mod iter;
pub mod slices;

impl<S: Length + ?Sized> Length for &S {
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
}

impl<S: Length + ?Sized> Length for &mut S {
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
}

impl<T> Length for [T] {
    #[inline]
    fn len(&self) -> usize {
        <[T]>::len(self)
    }
}

// Implement SliceByValue for &S by delegating to S
impl<I, S: SliceByValue<I> + ?Sized> SliceByValue<I> for &S {
    type Value = S::Value;

    fn get_value(&self, index: I) -> Option<Self::Value> {
        (**self).get_value(index)
    }
    fn index_value(&self, index: I) -> Self::Value {
        (**self).index_value(index)
    }
    unsafe fn get_value_unchecked(&self, index: I) -> Self::Value {
        (**self).get_value_unchecked(index)
    }
}

// Implement SliceByValue for &mut S by delegating to S (for read-only access)
impl<I, S: SliceByValue<I> + ?Sized> SliceByValue<I> for &mut S {
    type Value = S::Value;
    fn get_value(&self, index: I) -> Option<Self::Value> {
        (**self).get_value(index)
    }
    fn index_value(&self, index: I) -> Self::Value {
        (**self).index_value(index)
    }
    unsafe fn get_value_unchecked(&self, index: I) -> Self::Value {
        (**self).get_value_unchecked(index)
    }
}

impl<I, S: SliceByValueMut<I> + ?Sized> SliceByValueMut<I> for &mut S {
    fn set_value(&mut self, index: I, value: Self::Value) -> Self::Value {
        (**self).set_value(index, value)
    }
    unsafe fn set_value_unchecked(&mut self, index: I, value: Self::Value) -> Self::Value {
        (**self).set_value_unchecked(index, value)
    }
}

// --- Implementations for standard slices [T] and usize index ---
impl<T: Clone> SliceByValue<usize> for [T] {
    type Value = T;

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

impl<T: Clone> SliceByValueMut<usize> for [T] {
    #[inline]
    fn set_value(&mut self, index: usize, value: Self::Value) -> Self::Value {
        // Standard indexing panics on out-of-bounds.
        // We get a mutable reference `&mut T`.
        // mem::replace swaps the value at the location with the new `value`
        // and returns the old value.
        core::mem::replace(&mut self[index], value)
    }

    #[inline]
    unsafe fn set_value_unchecked(&mut self, index: usize, value: Self::Value) -> Self::Value {
        // Safety: The caller must ensure that `*self` (the index) is in bounds.
        unsafe {
            let elem = self.get_unchecked_mut(index);
            core::mem::replace(elem, value)
        }
    }
}

impl<'a, T> SliceByValue<Range<usize>> for &'a [T] {
    type Value = &'a [T];

    #[inline]
    fn get_value(&self, index: Range<usize>) -> Option<Self::Value> {
        (*self).get(index)
    }

    #[inline]
    fn index_value(&self, index: Range<usize>) -> Self::Value {
        &self[index]
    }

    #[inline]
    unsafe fn get_value_unchecked(&self, index: Range<usize>) -> Self::Value {
        unsafe { (*self).get_unchecked(index) }
    }
}

impl<T, const N: usize> Length for [T; N] {
    #[inline(always)]
    fn len(&self) -> usize {
        N
    }
}

impl<T: Clone, const N: usize> SliceByValue<usize> for [T; N] {
    type Value = T;

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

impl<T: Clone, const N: usize> SliceByValueMut<usize> for [T; N] {
    #[inline]
    fn set_value(&mut self, index: usize, value: Self::Value) -> Self::Value {
        // Standard indexing panics on out-of-bounds.
        // We get a mutable reference `&mut T`.
        // mem::replace swaps the value at the location with the new `value`
        // and returns the old value.
        core::mem::replace(&mut self[index], value)
    }

    #[inline]
    unsafe fn set_value_unchecked(&mut self, index: usize, value: Self::Value) -> Self::Value {
        // Safety: The caller must ensure that `*self` (the index) is in bounds.
        unsafe {
            let elem = self.get_unchecked_mut(index);
            core::mem::replace(elem, value)
        }
    }
}

impl<'a, T, const N: usize> SliceByValue<Range<usize>> for &'a [T; N] {
    type Value = &'a [T];

    #[inline]
    fn get_value(&self, index: Range<usize>) -> Option<Self::Value> {
        (*self).get(index)
    }

    #[inline]
    fn index_value(&self, index: Range<usize>) -> Self::Value {
        &self[index]
    }

    #[inline]
    unsafe fn get_value_unchecked(&self, index: Range<usize>) -> Self::Value {
        unsafe { (*self).get_unchecked(index) }
    }
}

#[cfg(feature = "alloc")]
mod alloc_impls {
    use super::*;
    extern crate alloc;
    use alloc::boxed::Box;
    use alloc::vec::Vec;

    impl<S: Length + ?Sized> Length for Box<S> {
        #[inline]
        fn len(&self) -> usize {
            (**self).len()
        }
    }

    impl<I, S: SliceByValue<I> + ?Sized> SliceByValue<I> for Box<S> {
        type Value = S::Value;

        fn get_value(&self, index: I) -> Option<Self::Value> {
            (**self).get_value(index)
        }
        fn index_value(&self, index: I) -> Self::Value {
            (**self).index_value(index)
        }
        unsafe fn get_value_unchecked(&self, index: I) -> Self::Value {
            (**self).get_value_unchecked(index)
        }
    }

    impl<I, S: SliceByValueMut<I> + ?Sized> SliceByValueMut<I> for Box<S> {
        fn set_value(&mut self, index: I, value: Self::Value) -> Self::Value {
            (**self).set_value(index, value)
        }
        unsafe fn set_value_unchecked(&mut self, index: I, value: Self::Value) -> Self::Value {
            (**self).set_value_unchecked(index, value)
        }
    }

    impl<T> Length for Vec<T> {
        #[inline]
        fn len(&self) -> usize {
            <[T]>::len(self)
        }
    }

    impl<T: Clone> SliceByValue<usize> for Vec<T> {
        type Value = T;

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

    impl<T: Clone> SliceByValueMut<usize> for Vec<T> {
        #[inline]
        fn set_value(&mut self, index: usize, value: Self::Value) -> Self::Value {
            // Standard indexing panics on out-of-bounds.
            // We get a mutable reference `&mut T`.
            // mem::replace swaps the value at the location with the new `value`
            // and returns the old value.
            core::mem::replace(&mut self[index], value)
        }

        #[inline]
        unsafe fn set_value_unchecked(&mut self, index: usize, value: Self::Value) -> Self::Value {
            // Safety: The caller must ensure that `*self` (the index) is in bounds.
            unsafe {
                let elem = self.get_unchecked_mut(index);
                core::mem::replace(elem, value)
            }
        }
    }

    impl<'a, T> SliceByValue<Range<usize>> for &'a Vec<T> {
        type Value = &'a [T];

        #[inline]
        fn get_value(&self, index: Range<usize>) -> Option<Self::Value> {
            // slice.get returns Option<&T>, .copied() converts to Option<T>
            (*self).get(index)
        }

        #[inline]
        fn index_value(&self, index: Range<usize>) -> Self::Value {
            &self[index]
        }

        #[inline]
        unsafe fn get_value_unchecked(&self, index: Range<usize>) -> Self::Value {
            unsafe { (*self).get_unchecked(index) }
        }
    }
}

#[cfg(feature = "std")]
mod std_impls {
    use super::*;
    use std::{rc::Rc, sync::Arc};

    impl<S: Length + ?Sized> Length for Arc<S> {
        #[inline]
        fn len(&self) -> usize {
            (**self).len()
        }
    }

    impl<I, S: SliceByValue<I> + ?Sized> SliceByValue<I> for Arc<S> {
        type Value = S::Value;

        fn get_value(&self, index: I) -> Option<Self::Value> {
            (**self).get_value(index)
        }
        fn index_value(&self, index: I) -> Self::Value {
            (**self).index_value(index)
        }
        unsafe fn get_value_unchecked(&self, index: I) -> Self::Value {
            (**self).get_value_unchecked(index)
        }
    }

    impl<S: Length + ?Sized> Length for Rc<S> {
        #[inline]
        fn len(&self) -> usize {
            (**self).len()
        }
    }

    impl<I, S: SliceByValue<I> + ?Sized> SliceByValue<I> for Rc<S> {
        type Value = S::Value;

        fn get_value(&self, index: I) -> Option<Self::Value> {
            (**self).get_value(index)
        }
        fn index_value(&self, index: I) -> Self::Value {
            (**self).index_value(index)
        }
        unsafe fn get_value_unchecked(&self, index: I) -> Self::Value {
            (**self).get_value_unchecked(index)
        }
    }
}
