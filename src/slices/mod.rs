//! Traits for value-based slices.
//!
//! Value-based slices are analogous to Rust's built-in slices, but they operate
//! on values rather than references. This allows for more flexibility in how
//! slices are used and manipulated.
//!
//! For example, a value-based slice can be defined functionally, implicitly, or
//! using a succinct/compressed representation.
//!
//! The fundamental trait for value-based slices is [`SliceByValue`], which
//! specifies the type of the values and the length of the slice. Additional
//! functionality is provided by the [`SliceByValueGet`], [`SliceByValueSet`],
//! and [`SliceByValueRepl`] traits, which allow for getting, setting, and
//! replacing values in the slice, respectively. Note that, contrarily to the
//! standard slices, replacement can be obtained by a pair of get/set
//! operations: [`SliceByValueRepl`] is just more efficient.
//!
//! The [`SliceByValueRange`] trait provides methods for obtaining subslices
//! given a range of indices, and the [`SliceByValueRangeMut`] trait provides
//! mutable versions of these methods.
//!
//! Both traits are a combination of underlying traits that provide more
//! specific subslicing functionality depending on the type of range used. In
//! the intended usage, these traits are interesting only for implementors, or
//! in the case an implementation does not provide the full set of ranges.
//!
//! # Examples
//!
//! This signature is for a function that takes a value-based slice of `u64`:
//! ```rust
//! use value_traits::slices::*;
//!
//! fn takes_slice_of_uint64<'a> (slice: &'a (impl SliceByValue<Value = u64> + SliceByValueGet + SliceByValueSubslice<'a>)) {
//!     // We can access values
//!     let a = slice.index_value(0);
//!     // We can get a subslice
//!     let mut s = slice.index_range(0..5);
//!     // And subslice it again with another range, getting the same type
//!     let mut t = s.index_range(1..2);
//!     let mut z = t.index_range(..);
//!     z = s;
//! }
//! ```
//!
//!

use core::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

use crate::{ImplBound, Ref};

/// Basic slice-by-value trait, specifying just the type of the values and the
/// length of the slice.
pub trait SliceByValue {
    /// The type of the values in the slice.
    type Value;
    /// See [`slice::len`].
    fn len(&self) -> usize;

    /// See [`slice::is_empty`].
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<S: SliceByValue + ?Sized> SliceByValue for &S {
    type Value = S::Value;
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
}

impl<S: SliceByValue + ?Sized> SliceByValue for &mut S {
    type Value = S::Value;
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
}

/// Read-only slice-by-value trait.
pub trait SliceByValueGet: SliceByValue {
    /// See [the `Index` implementation for slices](slice#impl-Index%3CI%3E-for-%5BT%5D).
    fn index_value(&self, index: usize) -> Self::Value;

    /// See [`slice::get_unchecked`].
    ///
    /// For a safe alternative see [`SliceByValueGet::get_value`].
    unsafe fn get_value_unchecked(&self, index: usize) -> Self::Value;

    /// See [`slice::get`].
    fn get_value(&self, index: usize) -> Option<Self::Value>;
}

impl<S: SliceByValueGet + ?Sized> SliceByValueGet for &S {
    fn get_value(&self, index: usize) -> Option<Self::Value> {
        (**self).get_value(index)
    }
    fn index_value(&self, index: usize) -> Self::Value {
        (**self).index_value(index)
    }
    unsafe fn get_value_unchecked(&self, index: usize) -> Self::Value {
        unsafe { (**self).get_value_unchecked(index) }
    }
}

impl<S: SliceByValueGet + ?Sized> SliceByValueGet for &mut S {
    fn get_value(&self, index: usize) -> Option<Self::Value> {
        (**self).get_value(index)
    }
    fn index_value(&self, index: usize) -> Self::Value {
        (**self).index_value(index)
    }
    unsafe fn get_value_unchecked(&self, index: usize) -> Self::Value {
        unsafe { (**self).get_value_unchecked(index) }
    }
}

/// Mutable slice-by-value trait, providing setting methods.
///
/// If you need to set a value and get the previous value, use
/// [`SliceByValueRepl`] instead.
pub trait SliceByValueSet: SliceByValue {
    /// Sets the value at the given index to the given value without doing
    /// bounds checking.
    ///
    /// For a safe alternative see [`SliceByValueMut::set_value`].
    unsafe fn set_value_unchecked(&mut self, index: usize, value: Self::Value);

    /// Sets the value at the given index to the given value.
    fn set_value(&mut self, index: usize, value: Self::Value);
}

impl<S: SliceByValueSet + ?Sized> SliceByValueSet for &mut S {
    fn set_value(&mut self, index: usize, value: Self::Value) {
        (**self).set_value(index, value)
    }
    unsafe fn set_value_unchecked(&mut self, index: usize, value: Self::Value) {
        unsafe { (**self).set_value_unchecked(index, value) }
    }
}

/// Mutable slice-by-value trait, providing replacement methods.
///
/// If you just need to set a value, use [`SliceByValueSet`] instead.
pub trait SliceByValueRepl: SliceByValue {
    /// Sets the value at the given index to the given value and
    /// returns the previous value, without doing bounds checking.
    ///
    /// For a safe alternative see [`SliceByValueRepl::replace_value`].
    unsafe fn replace_value_unchecked(&mut self, index: usize, value: Self::Value) -> Self::Value;

    /// Sets the value at the given index to the given value and
    /// returns the previous value.
    fn replace_value(&mut self, index: usize, value: Self::Value) -> Self::Value;
}

impl<S: SliceByValueRepl + ?Sized> SliceByValueRepl for &mut S {
    fn replace_value(&mut self, index: usize, value: Self::Value) -> Self::Value {
        (**self).replace_value(index, value)
    }
    unsafe fn replace_value_unchecked(&mut self, index: usize, value: Self::Value) -> Self::Value {
        unsafe { (**self).replace_value_unchecked(index, value) }
    }
}

pub trait SliceByValueRange<'a, R>: SliceByValue + Sized + 'a {
    /// See [the `Index` implementation for slices](slice#impl-Index%3CI%3E-for-%5BT%5D).
    fn index_range(&self, range: R) -> Self;

    /// See [`slice::get_unchecked`].
    ///
    /// For a safe alternative see [`SliceByValue::get_value`].
    unsafe fn get_range_unchecked(&self, range: R) -> Self;

    /// See [`slice::get`].
    fn get_range(&self, range: R) -> Option<Self>;
}

/*
impl<'a, S: SliceByValueRange<'a, R> + ?Sized + 'a, R> SliceByValueRange<'a, R> for &'a S {
    fn get_range(&'a self, range: R) -> Option<Self> {
        (**self).get_range(range).as_ref()
    }
    fn index_range(&'a self, range: R) -> Self {
        &(**self).index_range(range)
    }
    unsafe fn get_range_unchecked(&'a self, range: R) -> Self {
        &(**self).get_range_unchecked(range)
    }
}
impl<'a, S: SliceByValueRange<'a, R> + ?Sized, R> SliceByValueRange<'a, R> for &'a mut S {
    fn get_range(&self, range: R) -> Option<Self> {
        (**self).get_range(range).as_mut()
    }
    fn index_range(&self, range: R) -> Self {
        &mut (**self).index_range(range)
    }
    unsafe fn get_range_unchecked(&self, range: R) -> Self {
        &mut (**self).get_range_unchecked(range)
    }
}
*/


/*
pub trait SliceByValueRangeMut<R>: SliceByValue + Sized {
    /// See [the `Index` implementation for slices](slice#impl-Index%3CI%3E-for-%5BT%5D).
    fn index_range_mut(&mut self, range: R) -> Self;

    /// See [`slice::get_unchecked`].
    ///
    /// For a safe alternative see [`SliceByValue::get_value`].
    unsafe fn get_range_unchecked_mut(&mut self, range: R) -> Self;

    /// See [`slice::get`].
    fn get_range_mut(&mut self, range: R) -> Option<Self>;
}

impl<S: SliceByValueRangeMut<R> + ?Sized, R> SliceByValueRangeMut<R> for &mut S {
    fn get_range_mut(&mut self, range: R) -> Option<Self> {
        (**self).get_range_mut(range).as_mut()
    }
    fn index_range_mut(&mut self, range: R) -> Self {
        &mut (**self).index_range_mut(range)
    }
    unsafe fn get_range_unchecked_mut(&mut self, range: R) -> Self {
        &mut (**self).get_range_unchecked_mut(range)
    }
}
*/

/// Helper trait for requesting all common range types, and enforce that they all
/// return the same type of slice.
pub trait SliceByValueSubslice<'a, T = usize>:
    SliceByValueRange<'a, Range<T>>
    + SliceByValueRange<'a, RangeFrom<T>>
    + SliceByValueRange<'a, RangeFull>
    + SliceByValueRange<'a, RangeInclusive<T>>
    + SliceByValueRange<'a, RangeTo<T>>
    + SliceByValueRange<'a, RangeToInclusive<T>>
{
}

impl<'a, U, T> SliceByValueSubslice<'a, T> for U
where
    U: SliceByValueRange<'a, Range<T>>,
    U: SliceByValueRange<'a, RangeFrom<T>>,
    U: SliceByValueRange<'a, RangeFull>,
    U: SliceByValueRange<'a, RangeInclusive<T>>,
    U: SliceByValueRange<'a, RangeTo<T>>,
    U: SliceByValueRange<'a, RangeToInclusive<T>>,
{
}

/*
/// Mutable version of [`SliceByValueRangeAll`].
pub trait SliceByValueSubsliceMut<T = usize>:
    SliceByValueRangeMut<Range<T>>
    + SliceByValueRangeMut<RangeFrom<T>>
    + SliceByValueRangeMut<RangeFull>
    + SliceByValueRangeMut<RangeInclusive<T>>
    + SliceByValueRangeMut<RangeTo<T>>
    + SliceByValueRangeMut<RangeToInclusive<T>>
{
}

impl<U, T> SliceByValueSubsliceMut<T> for U
where
    U: SliceByValueRangeMut<Range<T>>,
    U: SliceByValueRangeMut<RangeFrom<T>>,
    U: SliceByValueRangeMut<RangeFull>,
    U: SliceByValueRangeMut<RangeInclusive<T>>,
    U: SliceByValueRangeMut<RangeTo<T>>,
    U: SliceByValueRangeMut<RangeToInclusive<T>>,
{
}
*/
