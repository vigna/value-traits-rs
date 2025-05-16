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
//! fn takes_slice_of_uint64(slice: &(impl SliceByValue<Value = u64> + SliceByValueGet + SliceByValueSubslice)) {
//!     // We can access values
//!     let a = slice.index_value(0);
//!     // We can get a subslice
//!     let mut s = slice.index_range(0..5);
//!     // And subslice it again with another range, getting the same type
//!     let t = s.index_range(1..2);
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
        (**self).get_value_unchecked(index)
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
        (**self).get_value_unchecked(index)
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
        (**self).set_value_unchecked(index, value)
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
        (**self).replace_value_unchecked(index, value)
    }
}

/// A GAT-like trait specifying the subslice type for a given range.
///
/// It implicitly restricts the lifetime `'a` used in `SliceByValueRange`
/// to be `where Self: 'a`.
///
/// As in other theoretical applications of GATs, like
/// [lenders](https://crates.io/crates/lender), using a GAT to express the type
/// of a subslice is problematic because when bounding the type itself in a
/// where clause using Higher-Ranked Trait Bounds (HRTBs) the bound must be true
/// for all lifetimes, including `'static`, resulting in the sliced type having
/// to be `'static` as well.
///
/// This is a result of HRTBs not having a way to express qualifiers (`for<'any
/// where Self: 'any> Self: Trait`) and effectively making HRTBs only useful
/// when you want to express a trait constraint on ALL lifetimes, including
/// 'static (`for<'all> Self: trait`)
///
/// Please see [Sabrina's Blog][1] for more information, and how a trait
/// like this can be used to solve it by implicitly restricting HRTBs.
///
/// # I
///
/// [1]:
///     <https://sabrinajewson.org/blog/the-better-alternative-to-lifetime-gats>
pub trait SliceByValueGat<'a, __Implicit: ImplBound = Ref<'a, Self>>: SliceByValue {
    type Subslice: 'a
        + SliceByValueGet<Value = Self::Value>
        + SliceByValueGat<'a, Subslice = Self::Subslice>; // recursion
}

impl<'a, T: SliceByValue + SliceByValueGat<'a> + ?Sized> SliceByValueGat<'a> for &T {
    type Subslice = <T as SliceByValueGat<'a>>::Subslice;
}

impl<'a, T: SliceByValue + SliceByValueGat<'a> + ?Sized> SliceByValueGat<'a> for &mut T {
    type Subslice = <T as SliceByValueGat<'a>>::Subslice;
}

#[allow(type_alias_bounds)] // yeah the type alias bounds are not enforced, but they are useful for documentation
pub type Subslice<'a, T: SliceByValue + SliceByValueGat<'a>> = <T as SliceByValueGat<'a>>::Subslice;

pub trait SliceByValueRange<R>: SliceByValue + for<'a> SliceByValueGat<'a> {
    /// See [the `Index` implementation for slices](slice#impl-Index%3CI%3E-for-%5BT%5D).
    fn index_range(&self, range: R) -> Subslice<'_, Self>;

    /// See [`slice::get_unchecked`].
    ///
    /// For a safe alternative see [`SliceByValue::get_value`].
    unsafe fn get_range_unchecked(&self, range: R) -> Subslice<'_, Self>;

    /// See [`slice::get`].
    fn get_range(&self, range: R) -> Option<Subslice<'_, Self>>;
}

impl<S: SliceByValueRange<R> + ?Sized, R> SliceByValueRange<R> for &S {
    fn get_range(&self, range: R) -> Option<Subslice<'_, Self>> {
        (**self).get_range(range)
    }
    fn index_range(&self, range: R) -> Subslice<'_, Self> {
        (**self).index_range(range)
    }
    unsafe fn get_range_unchecked(&self, range: R) -> Subslice<'_, Self> {
        (**self).get_range_unchecked(range)
    }
}
impl<S: SliceByValueRange<R> + ?Sized, R> SliceByValueRange<R> for &mut S {
    fn get_range(&self, range: R) -> Option<Subslice<'_, Self>> {
        (**self).get_range(range)
    }
    fn index_range(&self, range: R) -> Subslice<'_, Self> {
        (**self).index_range(range)
    }
    unsafe fn get_range_unchecked(&self, range: R) -> Subslice<'_, Self> {
        (**self).get_range_unchecked(range)
    }
}

pub trait SliceByValueGatMut<'a, __Implicit = &'a Self>: SliceByValue {
    type Subslice: 'a
        + SliceByValueSet<Value = Self::Value>
        + SliceByValueRepl<Value = Self::Value>
        + SliceByValueGatMut<'a, Subslice = Self::Subslice>; // recursion
}

impl<'a, T: SliceByValue + SliceByValueGatMut<'a> + ?Sized> SliceByValueGatMut<'a> for &mut T {
    type Subslice = <T as SliceByValueGatMut<'a>>::Subslice;
}

#[allow(type_alias_bounds)] // yeah the type alias bounds are not enforced, but they are useful for documentation
pub type SubsliceMut<'a, T: SliceByValue + SliceByValueGatMut<'a>> =
    <T as SliceByValueGatMut<'a>>::Subslice;

pub trait SliceByValueRangeMut<R>: SliceByValue + for<'a> SliceByValueGatMut<'a> {
    /// See [the `Index` implementation for slices](slice#impl-Index%3CI%3E-for-%5BT%5D).
    fn index_range_mut(&mut self, range: R) -> SubsliceMut<'_, Self>;

    /// See [`slice::get_unchecked`].
    ///
    /// For a safe alternative see [`SliceByValue::get_value`].
    unsafe fn get_range_unchecked_mut(&mut self, range: R) -> SubsliceMut<'_, Self>;

    /// See [`slice::get`].
    fn get_range_mut(&mut self, range: R) -> Option<SubsliceMut<'_, Self>>;
}

impl<S: SliceByValueRangeMut<R> + ?Sized, R> SliceByValueRangeMut<R> for &mut S {
    fn get_range_mut(&mut self, range: R) -> Option<SubsliceMut<'_, Self>> {
        (**self).get_range_mut(range)
    }
    fn index_range_mut(&mut self, range: R) -> SubsliceMut<'_, Self> {
        (**self).index_range_mut(range)
    }
    unsafe fn get_range_unchecked_mut(&mut self, range: R) -> SubsliceMut<'_, Self> {
        (**self).get_range_unchecked_mut(range)
    }
}

/// Helper trait for requesting all common range types, and enforce that they all
/// return the same type of slice.
pub trait SliceByValueSubslice<T = usize>:
    SliceByValueRange<Range<T>>
    + SliceByValueRange<RangeFrom<T>>
    + SliceByValueRange<RangeFull>
    + SliceByValueRange<RangeInclusive<T>>
    + SliceByValueRange<RangeTo<T>>
    + SliceByValueRange<RangeToInclusive<T>>
{
}

impl<U, T> SliceByValueSubslice<T> for U
where
    U: SliceByValueRange<Range<T>>,
    U: SliceByValueRange<RangeFrom<T>>,
    U: SliceByValueRange<RangeFull>,
    U: SliceByValueRange<RangeInclusive<T>>,
    U: SliceByValueRange<RangeTo<T>>,
    U: SliceByValueRange<RangeToInclusive<T>>,
{
}

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
    for<'a> <Self as SliceByValueGatMut<'a>>::Subslice: SliceByValueSubsliceMut<T>,
{
}
