#![cfg_attr(not(feature = "std"), no_std)]
#![deny(unconditional_recursion)]

use core::ops::{Deref, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};
use slices::{
    SliceByValue, SliceByValueGet, SliceByValueRange,
    /*SliceByValueRangeMut, */SliceByValueRepl, SliceByValueSet,
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

impl<U: Clone, T: Deref<Target=[U]>> SliceByValue for T {
    type Value = U;
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
}

// --- Implementations for standard slices [T] and usize index ---
impl<U: Clone, T: AsRef<[U]>> SliceByValueGet for T where T: SliceByValue<Value=U> {
    #[inline]
    fn get_value(&self, index: usize) -> Option<Self::Value> {
        // slice.get returns Option<&T>, .copied() converts to Option<T>
        self.as_ref().get(index).cloned()
    }

    #[inline]
    fn index_value(&self, index: usize) -> Self::Value {
        // Standard indexing panics on out-of-bounds.
        // It returns &T, which we copy to return T.
        self.as_ref()[index].clone()
    }

    #[inline]
    unsafe fn get_value_unchecked(&self, index: usize) -> Self::Value {
        // Safety: The caller must ensure that `*self` (the index) is in bounds.
        // slice.get_unchecked returns &T, which we dereference and copy.
        unsafe { self.as_ref().get_unchecked(index).clone() }
    }
}

impl<U: Clone, T: AsMut<[U]>> SliceByValueSet for T where T: SliceByValue<Value=U> {
    #[inline]
    fn set_value(&mut self, index: usize, value: Self::Value) {
        // Standard indexing panics on out-of-bounds.
        self.as_mut()[index] = value;
    }

    #[inline]
    unsafe fn set_value_unchecked(&mut self, index: usize, value: Self::Value) {
        // Safety: The caller must ensure that `*self` (the index) is in bounds.
        unsafe {
            let elem = self.as_mut().get_unchecked_mut(index);
            *elem = value;
        }
    }
}

impl<U: Clone, T: AsMut<[U]>> SliceByValueRepl for T where T: SliceByValue<Value=U> {
    #[inline]
    fn replace_value(&mut self, index: usize, value: Self::Value) -> Self::Value {
        // Standard indexing panics on out-of-bounds.
        // We get a mutable reference `&mut T`.
        // mem::replace swaps the value at the location with the new `value`
        // and returns the old value.
        core::mem::replace(&mut self.as_mut()[index], value)
    }

    #[inline]
    unsafe fn replace_value_unchecked(&mut self, index: usize, value: Self::Value) -> Self::Value {
        // Safety: The caller must ensure that `*self` (the index) is in bounds.
        unsafe {
            let elem = self.as_mut().get_unchecked_mut(index);
            core::mem::replace(elem, value)
        }
    }
}

macro_rules! impl_range_slices {
    ($range:ty) => {
        impl<'a, U: Clone> SliceByValueRange<'a, $range> for &'a [U] where &'a [U]: SliceByValue<Value=U> {
            #[inline]
            fn get_range(&self, index: $range) -> Option<Self> {
                self.get(index)
            }

            #[inline]
            fn index_range(&self, index: $range) -> Self {
                &self[index]
            }

            #[inline]
            unsafe fn get_range_unchecked(&self, index: $range) -> Self {
                unsafe { self.get_unchecked(index) }
            }
        }
    }
}

impl_range_slices!(RangeFull);
impl_range_slices!(RangeFrom<usize>);
impl_range_slices!(RangeTo<usize>);
impl_range_slices!(Range<usize>);
impl_range_slices!(RangeInclusive<usize>);
impl_range_slices!(RangeToInclusive<usize>);

