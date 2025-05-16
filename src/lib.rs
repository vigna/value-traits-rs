#![cfg_attr(not(feature = "std"), no_std)]
#![deny(unconditional_recursion)]

use core::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};
use slices::{
    SliceByValue, SliceByValueGat, SliceByValueGatMut, SliceByValueGet, SliceByValueRange,
    SliceByValueRangeMut, SliceByValueRepl, SliceByValueSet, Subslice, SubsliceMut,
};

pub mod iter;
pub mod slices;

#[doc(hidden)]
#[allow(private_bounds)]
pub trait ImplBound: ImplBoundPriv {}
#[doc(hidden)]
pub(crate) trait ImplBoundPriv {}
impl<T: ?Sized + ImplBoundPriv> ImplBound for T {}
#[doc(hidden)]
pub struct Ref<'a, T: ?Sized>(&'a T);
impl<T: ?Sized> ImplBoundPriv for Ref<'_, T> {}

impl<T> SliceByValue for [T] {
    type Value = T;
    #[inline]
    fn len(&self) -> usize {
        <[T]>::len(self)
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

impl<'a, T: Clone> SliceByValueGat<'a> for [T] {
    type Subslice = &'a [T];
}

impl<'a, T: Clone> SliceByValueGatMut<'a> for [T] {
    type Subslice = &'a mut [T];
}

macro_rules! impl_range_slices {
    ($range:ty) => {
        impl<T: Clone> SliceByValueRange<$range> for [T] {
            #[inline]
            fn get_range(&self, index: $range) -> Option<Subslice<'_, Self>> {
                (*self).get(index)
            }

            #[inline]
            fn index_range(&self, index: $range) -> Subslice<'_, Self> {
                &self[index]
            }

            #[inline]
            unsafe fn get_range_unchecked(&self, index: $range) -> Subslice<'_, Self> {
                unsafe { (*self).get_unchecked(index) }
            }
        }

        impl<T: Clone> SliceByValueRangeMut<$range> for [T] {
            #[inline]
            fn get_range_mut(&mut self, index: $range) -> Option<SubsliceMut<'_, Self>> {
                (*self).get_mut(index)
            }

            #[inline]
            fn index_range_mut(&mut self, index: $range) -> SubsliceMut<'_, Self> {
                &mut self[index]
            }

            #[inline]
            unsafe fn get_range_unchecked_mut(&mut self, index: $range) -> SubsliceMut<'_, Self> {
                unsafe { (*self).get_unchecked_mut(index) }
            }
        }
    };
}

impl_range_slices!(RangeFull);
impl_range_slices!(RangeFrom<usize>);
impl_range_slices!(RangeTo<usize>);
impl_range_slices!(Range<usize>);
impl_range_slices!(RangeInclusive<usize>);
impl_range_slices!(RangeToInclusive<usize>);

impl<T, const N: usize> SliceByValue for [T; N] {
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

impl<'a, T: Clone, const N: usize> SliceByValueGat<'a> for [T; N] {
    type Subslice = &'a [T];
}

macro_rules! impl_range_arrays {
    ($range:ty) => {
        impl<T: Clone, const N: usize> SliceByValueRange<$range> for [T; N] {
            #[inline]
            fn get_range(&self, index: $range) -> Option<Subslice<'_, Self>> {
                (*self).get(index)
            }

            #[inline]
            fn index_range(&self, index: $range) -> Subslice<'_, Self> {
                &self[index]
            }

            #[inline]
            unsafe fn get_range_unchecked(&self, index: $range) -> Subslice<'_, Self> {
                unsafe { (*self).get_unchecked(index) }
            }
        }
    };
}

impl_range_arrays!(RangeFull);
impl_range_arrays!(RangeFrom<usize>);
impl_range_arrays!(RangeTo<usize>);
impl_range_arrays!(Range<usize>);
impl_range_arrays!(RangeInclusive<usize>);
impl_range_arrays!(RangeToInclusive<usize>);

#[cfg(feature = "alloc")]
mod alloc_impls {
    use super::*;
    extern crate alloc;
    use alloc::boxed::Box;
    use alloc::vec::Vec;

    impl<S: SliceByValue + ?Sized> SliceByValue for Box<S> {
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

    impl<T> SliceByValue for Vec<T> {
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

    impl<'a, T: Clone> SliceByValueGat<'a> for Vec<T> {
        type Subslice = &'a [T];
    }

    macro_rules! impl_range_vecs {
        ($range:ty) => {
            impl<T: Clone> SliceByValueRange<$range> for Vec<T> {
                #[inline]
                fn get_range(&self, index: $range) -> Option<Subslice<'_, Self>> {
                    // slice.get returns Option<&T>, .copied() converts to Option<T>
                    (*self).get(index)
                }

                #[inline]
                fn index_range(&self, index: $range) -> Subslice<'_, Self> {
                    &self[index]
                }

                #[inline]
                unsafe fn get_range_unchecked(&self, index: $range) -> Subslice<'_, Self> {
                    unsafe { (*self).get_unchecked(index) }
                }
            }
        };
    }

    impl_range_vecs!(RangeFull);
    impl_range_vecs!(RangeFrom<usize>);
    impl_range_vecs!(RangeTo<usize>);
    impl_range_vecs!(Range<usize>);
    impl_range_vecs!(RangeInclusive<usize>);
    impl_range_vecs!(RangeToInclusive<usize>);
}

#[cfg(feature = "std")]
mod std_impls {
    use super::*;
    use std::{rc::Rc, sync::Arc};

    impl<S: SliceByValue + ?Sized> SliceByValue for Arc<S> {
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

    impl<S: SliceByValue + ?Sized> SliceByValue for Rc<S> {
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
