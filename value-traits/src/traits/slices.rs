/*
 * SPDX-FileCopyrightText: 2025 Tommaso Fontana
 * SPDX-FileCopyrightText: 2025 Sebastiano Vigna
 * SPDX-FileCopyrightText: 2025 Inria
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

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
//! The [`SliceByValueSubslice`] trait provides methods for obtaining subslices
//! given a range of indices, and the [`SliceByValueSubsliceMut`] trait provides
//! mutable versions of these methods.
//!
//! Both traits are a combination of underlying traits that provide more
//! specific subslicing functionality depending on the type of range used. In
//! the intended usage, these traits are interesting only for implementors, or
//! in the case an implementation does not provide the full set of ranges.
//!
//! ## Examples
//!
//! As a very simple worked-out example, let us a by-value read-only slice of
//! `usize` using a vector of `u8` as a basic form of compression:
//!
//! ```rust
//! use value_traits::slices::*;
//!
//! struct CompSlice<'a>(&'a [u8]);
//!
//! impl<'a> SliceByValue for CompSlice<'a> {
//!     type Value = usize;
//!     fn len(&self) -> usize {
//!         self.0.len()
//!     }
//! }
//!
//! impl<'a> SliceByValueGet for CompSlice<'a> {
//!     unsafe fn get_value_unchecked(&self, index: usize) -> usize {
//!         unsafe { self.0.get_value_unchecked(index) as usize }
//!     }
//! }
//!
//! fn f(slice_by_value: impl SliceByValueGet<Value = usize>, index: usize) -> usize {
//!     slice_by_value.index_value(index)
//! }
//!
//! fn main() {
//!     let vec = vec![0_u8, 1, 2, 3];
//!     let slice_by_value = CompSlice(&vec);
//!     // Note that we can pass a reference
//!     assert_eq!(f(&slice_by_value, 0), 0);
//!     assert_eq!(f(&slice_by_value, 1), 1);
//!     assert_eq!(f(&slice_by_value, 2), 2);
//!     assert_eq!(f(&slice_by_value, 3), 3);
//! }
//!
//! ```
//! In this example, instead, we define functionally a slice containing the
//! first 100 squares:
//!
//! ```rust
//! use value_traits::slices::*;
//!
//! struct Squares();
//!
//! impl<'a> SliceByValue for Squares {
//!     type Value = usize;
//!     fn len(&self) -> usize {
//!         100
//!     }
//! }
//!
//! impl<'a> SliceByValueGet for Squares {
//!     unsafe fn get_value_unchecked(&self, index: usize) -> usize {
//!         index * index
//!     }
//! }
//!
//! fn f(slice_by_value: &impl SliceByValueGet<Value = usize>, index: usize) -> usize {
//!     slice_by_value.index_value(index)
//! }
//!
//! fn main() {
//!     let squares = Squares();
//!     for i in 0..100 {
//!         assert_eq!(squares.index_value(i), i * i);
//!     }
//! }
//! ```

use core::ops::{
    Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};

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

#[inline(always)]
fn assert_index(index: usize, len: usize) {
    assert!(
        index < len,
        "index out of bounds: the len is {len} but the index is {index}",
    );
}

#[inline(always)]
fn assert_range(range: &impl ComposeRange, len: usize) {
    assert!(
        range.is_valid(len),
        "range {range:?} out of range for slice of length {len}: ",
    );
}

/// Read-only slice-by-value trait.
///
/// The only method that must be implemented is
/// [`get_value_unchecked`](`SliceByValueGet::get_value_unchecked`).
pub trait SliceByValueGet: SliceByValue {
    /// See [the `Index` implementation for slices](slice#impl-Index%3CI%3E-for-%5BT%5D).
    fn index_value(&self, index: usize) -> Self::Value {
        assert_index(index, self.len());
        // SAFETY: index is without bounds
        unsafe { self.get_value_unchecked(index) }
    }

    /// See [`slice::get_unchecked`].
    ///
    /// For a safe alternative see [`get_value`](SliceByValueGet::get_value)
    /// or [`index_value`](SliceByValueGet::index_value).
    ///
    /// # Safety
    ///
    /// The index must be within bounds.
    unsafe fn get_value_unchecked(&self, index: usize) -> Self::Value;

    /// See [`slice::get`].
    fn get_value(&self, index: usize) -> Option<Self::Value> {
        if index < self.len() {
            // SAFETY: index is without bounds
            let value = unsafe { self.get_value_unchecked(index) };
            Some(value)
        } else {
            None
        }
    }
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

/// Mutable slice-by-value trait providing setting methods.
///
/// The only method that must be implemented is
/// [`set_value_unchecked`](`SliceByValueSet::set_value_unchecked`).
///
/// If you need to set a value and get the previous value, use
/// [`SliceByValueRepl`] instead.
pub trait SliceByValueSet: SliceByValue {
    /// Sets the value at the given index to the given value without doing
    /// bounds checking.
    ///
    /// For a safe alternative see [`set_value`](SliceByValueSet::set_value).
    ///
    /// # Safety
    ///
    /// The index must be within bounds.
    unsafe fn set_value_unchecked(&mut self, index: usize, value: Self::Value);

    /// Sets the value at the given index to the given value.
    ///
    /// # Panics
    ///
    /// This method will panic is the index is not within bounds.
    fn set_value(&mut self, index: usize, value: Self::Value) {
        assert_index(index, self.len());
        // SAFETY: index is without bounds
        unsafe {
            self.set_value_unchecked(index, value);
        }
    }
}

impl<S: SliceByValueSet + ?Sized> SliceByValueSet for &mut S {
    fn set_value(&mut self, index: usize, value: Self::Value) {
        (**self).set_value(index, value);
    }
    unsafe fn set_value_unchecked(&mut self, index: usize, value: Self::Value) {
        (**self).set_value_unchecked(index, value);
    }
}

/// Mutable slice-by-value trait providing replacement methods.
///
/// If you just need to set a value, use [`SliceByValueSet`] instead.
pub trait SliceByValueRepl: SliceByValue {
    /// Sets the value at the given index to the given value and
    /// returns the previous value, without doing bounds checking.
    ///
    /// For a safe alternative see [`SliceByValueRepl::replace_value`].
    ///
    /// # Safety
    ///
    /// The index must be within bounds.
    unsafe fn replace_value_unchecked(&mut self, index: usize, value: Self::Value) -> Self::Value;

    /// Sets the value at the given index to the given value and
    /// returns the previous value.
    ///
    /// # Panics
    ///
    /// This method will panic is the index is not within bounds.
    fn replace_value(&mut self, index: usize, value: Self::Value) -> Self::Value {
        assert_index(index, self.len());
        // SAFETY: index is without bounds
        unsafe { self.replace_value_unchecked(index, value) }
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

/// A range that can check whether it is within the bounds of a slice, and
/// intersect itself with another range.
///
/// This trait is implemented for the six Rust range types in [`core::ops`],
/// making it possible to treat them uniformly in implementations, and in
/// particular in procedural macros.
pub trait ComposeRange: RangeBounds<usize> + core::fmt::Debug {
    /// Returns `true` if the range is within the bounds of a slice of given
    /// length
    fn is_valid(&self, len: usize) -> bool;

    /// Returns a new range that is the composition of `base` with the range.
    ///
    /// The resulting range is guaranteed to be contained in `base` if `self` [is
    /// valid](ComposeRange::is_valid) for `base.len()`.
    ///
    /// ```rust
    /// use value_traits::slices::ComposeRange;
    ///
    /// assert_eq!((2..5).compose(10..20),  12..15);
    /// assert_eq!((2..=5).compose(10..20), 12..16);
    /// assert_eq!((..5).compose(10..20),   10..15);
    /// assert_eq!((..=5).compose(10..20),  10..16);
    /// assert_eq!((2..).compose(10..20),   12..20);
    /// assert_eq!((..).compose(10..20),    10..20);
    /// ```
    fn compose(&self, base: Range<usize>) -> Range<usize>;
}

impl ComposeRange for Range<usize> {
    fn is_valid(&self, len: usize) -> bool {
        self.start <= len && self.end <= len && self.start <= self.end
    }

    fn compose(&self, base: Range<usize>) -> Range<usize> {
        (base.start + self.start)..(base.start + self.end)
    }
}

impl ComposeRange for RangeFrom<usize> {
    fn is_valid(&self, len: usize) -> bool {
        self.start <= len
    }

    fn compose(&self, base: Range<usize>) -> Range<usize> {
        (base.start + self.start)..base.end
    }
}

impl ComposeRange for RangeFull {
    fn is_valid(&self, _len: usize) -> bool {
        true
    }

    fn compose(&self, base: Range<usize>) -> Range<usize> {
        base
    }
}

impl ComposeRange for RangeInclusive<usize> {
    fn is_valid(&self, len: usize) -> bool {
        *self.start() < len && *self.end() < len && self.start() <= self.end()
    }

    fn compose(&self, base: Range<usize>) -> Range<usize> {
        (base.start + self.start())..(base.start + self.end() + 1)
    }
}

impl ComposeRange for RangeTo<usize> {
    fn is_valid(&self, len: usize) -> bool {
        self.end <= len
    }

    fn compose(&self, base: Range<usize>) -> Range<usize> {
        base.start..(base.start + self.end)
    }
}

impl ComposeRange for RangeToInclusive<usize> {
    fn is_valid(&self, len: usize) -> bool {
        self.end < len
    }

    fn compose(&self, base: Range<usize>) -> Range<usize> {
        base.start..(base.start + self.end + 1)
    }
}

/// A GAT-like trait specifying the subslice type.
///
/// It implicitly restricts the lifetime `'a` used in `SliceByValueRange` to be
/// `where Self: 'a`. Moreover, it requires [`SliceByValueGet`].
///
/// As in other theoretical applications of GATs (Generic Associated Types),
/// like [lenders](https://crates.io/crates/lender), using a GAT to express the
/// type of a subslice is problematic because when bounding the type itself in a
/// `where` clause using Higher-Rank Trait Bounds (HRTBs) the bound must be true
/// for all lifetimes, including `'static`, resulting in the sliced type having
/// to be `'static` as well.
///
/// This is a result of HRTBs not having a way to express qualifiers (`for<'any
/// where Self: 'any> Self: Trait`) and effectively making HRTBs only useful
/// when you want to express a trait constraint on ALL lifetimes, including
/// `'static` (`for<'all> Self: trait`)
///
/// Please see [Sabrina's Blog][1] for more information, and how a trait like
/// this can be used to solve it by implicitly restricting HRTBs.
///
/// [1]:
///     <https://sabrinajewson.org/blog/the-better-alternative-to-lifetime-gats>
pub trait SliceByValueSubsliceGat<'a, __Implicit: ImplBound = Ref<'a, Self>>:
    SliceByValueGet
{
    type Subslice: 'a + SliceByValueGet<Value = Self::Value> + SliceByValueSubslice;
}

/// A convenience type representing the type of subslice
/// of a type implementing [`SliceByValueSubsliceGat`].
#[allow(type_alias_bounds)] // yeah the type alias bounds are not enforced, but they are useful for documentation
pub type Subslice<'a, T: SliceByValueSubsliceGat<'a>> =
    <T as SliceByValueSubsliceGat<'a>>::Subslice;

impl<'a, T: SliceByValueSubsliceGat<'a> + ?Sized> SliceByValueSubsliceGat<'a> for &T {
    type Subslice = T::Subslice;
}

impl<'a, T: SliceByValueSubsliceGat<'a> + ?Sized> SliceByValueSubsliceGat<'a> for &mut T {
    type Subslice = T::Subslice;
}

/// A trait implementing subslicing for a specific range parameter.
///
/// The user should never see this trait. [`SliceByValueSubslice`] combines all
/// instances of this trait with `R` equal to the various kind of standard
/// ranges ([`core::ops::Range`], [`core::ops::RangeFull`], etc.).
///
/// The only method that must be implemented is
/// [`get_subslice_unchecked`](`SliceByValueSubsliceRange::get_subslice_unchecked`).
///
/// Note that to bind the subslice type you need to use higher-rank trait bounds:
/// ```rust
/// use value_traits::slices::*;
/// use core::ops::Range;
///
/// fn f<S>(s: S) where
///    S: SliceByValueSubsliceRange<Range<usize>>,
///    S: for<'a> SliceByValueSubsliceGat<'a, Subslice = &'a [u8]>,
/// {
///     let _: &[u8] = s.index_subslice(0..10);
/// }
/// ```
/// However, such a bound is usually applied to the [`SliceByValueSubslice`]
/// trait.

pub trait SliceByValueSubsliceRange<R: ComposeRange>: for<'a> SliceByValueSubsliceGat<'a> {
    /// See [the `Index` implementation for slices](slice#impl-Index%3CI%3E-for-%5BT%5D).
    fn index_subslice(&self, range: R) -> Subslice<'_, Self> {
        assert_range(&range, self.len());
        unsafe {
            // SAFETY: range is within bounds
            self.get_subslice_unchecked(range)
        }
    }

    /// See [`slice::get_unchecked`].
    ///
    /// For a safe alternative see
    /// [`get_subslice`](SliceByValueSubsliceRange::get_subslice) or
    /// [`index_subslice`](SliceByValueSubsliceRange::index_subslice).
    ///
    /// # Safety
    ///
    /// The range must be within bounds.
    unsafe fn get_subslice_unchecked(&self, range: R) -> Subslice<'_, Self>;

    /// See [`slice::get`].
    fn get_subslice(&self, range: R) -> Option<Subslice<'_, Self>> {
        if range.is_valid(self.len()) {
            let subslice = unsafe { self.get_subslice_unchecked(range) };
            Some(subslice)
        } else {
            None
        }
    }
}

impl<R: ComposeRange, S: SliceByValueSubsliceRange<R> + ?Sized> SliceByValueSubsliceRange<R>
    for &S
{
    fn get_subslice(&self, range: R) -> Option<Subslice<'_, Self>> {
        (**self).get_subslice(range)
    }
    fn index_subslice(&self, range: R) -> Subslice<'_, Self> {
        (**self).index_subslice(range)
    }
    unsafe fn get_subslice_unchecked(&self, range: R) -> Subslice<'_, Self> {
        unsafe { (**self).get_subslice_unchecked(range) }
    }
}
impl<R: ComposeRange, S: SliceByValueSubsliceRange<R> + ?Sized> SliceByValueSubsliceRange<R>
    for &mut S
{
    fn get_subslice(&self, range: R) -> Option<Subslice<'_, Self>> {
        (**self).get_subslice(range)
    }
    fn index_subslice(&self, range: R) -> Subslice<'_, Self> {
        (**self).index_subslice(range)
    }
    unsafe fn get_subslice_unchecked(&self, range: R) -> Subslice<'_, Self> {
        unsafe { (**self).get_subslice_unchecked(range) }
    }
}

/// A GAT-like trait specifying the mutable subslice type.
///
/// See [`SliceByValueSubsliceGat`].
pub trait SliceByValueSubsliceGatMut<'a, __Implicit = &'a Self>:
    SliceByValueSet + SliceByValueRepl
{
    type Subslice: 'a
        + SliceByValueSet<Value = Self::Value>
        + SliceByValueRepl<Value = Self::Value>
        + SliceByValueSubsliceGatMut<'a, Subslice = Self::Subslice> // recursion
        + SliceByValueSubsliceMut;
}

/// A convenience type representing the type of subslice
/// of a type implementing [`SliceByValueSubsliceGatMut`].
#[allow(type_alias_bounds)] // yeah the type alias bounds are not enforced, but they are useful for documentation
pub type SubsliceMut<'a, T: SliceByValueSubsliceGatMut<'a>> =
    <T as SliceByValueSubsliceGatMut<'a>>::Subslice;

impl<'a, T: SliceByValueSubsliceGatMut<'a> + ?Sized> SliceByValueSubsliceGatMut<'a> for &mut T {
    type Subslice = T::Subslice;
}

/// A trait implementing mutable subslicing for a specific range parameter.
///
///  The user should never see this trait. [`SliceByValueSubsliceMut`] combines
/// all instances of this trait with `R` equal to the various kind of standard
/// ranges ([`core::ops::Range`], [`core::ops::RangeFull`], etc.).
///
/// The only method that must be implemented is
/// [`get_subslice_unchecked_mut`](`SliceByValueSubsliceRangeMut::get_subslice_unchecked_mut`).
///
///
/// Note that to bind the subslice type you need to use higher-rank trait bounds:
/// ```rust
/// use value_traits::slices::*;
/// use core::ops::Range;
///
/// fn f<S>(mut s: S) where
///    S: SliceByValueSubsliceRangeMut<Range<usize>>,
///    S: for<'a> SliceByValueSubsliceGatMut<'a, Subslice = &'a mut [u8]>,
/// {
///     let _: &mut [u8] = s.index_subslice_mut(0..10);
/// }
/// ```
/// However, such a bound is usually applied to the [`SliceByValueSubsliceMut`]
/// trait.
pub trait SliceByValueSubsliceRangeMut<R: ComposeRange>:
    for<'a> SliceByValueSubsliceGatMut<'a>
{
    /// See [the `Index` implementation for slices](slice#impl-Index%3CI%3E-for-%5BT%5D).
    fn index_subslice_mut(&mut self, range: R) -> SubsliceMut<'_, Self> {
        assert_range(&range, self.len());
        unsafe {
            // SAFETY: range is within bounds
            self.get_subslice_unchecked_mut(range)
        }
    }

    /// See [`slice::get_unchecked`].
    ///
    /// For a safe alternative see
    /// [`get_subslice_mut`](SliceByValueSubsliceRangeMut::get_subslice_mut) or
    /// [`index_subslice_mut`](SliceByValueSubsliceRangeMut::index_subslice_mut).
    ///
    /// # Safety
    ///
    /// The range must be within bounds.
    unsafe fn get_subslice_unchecked_mut(&mut self, range: R) -> SubsliceMut<'_, Self>;

    /// See [`slice::get`].
    fn get_subslice_mut(&mut self, range: R) -> Option<SubsliceMut<'_, Self>> {
        if range.is_valid(self.len()) {
            // SAFETY: range is within bounds
            let subslice_mut = unsafe { self.get_subslice_unchecked_mut(range) };
            Some(subslice_mut)
        } else {
            None
        }
    }
}

impl<R: ComposeRange, S: SliceByValueSubsliceRangeMut<R> + ?Sized> SliceByValueSubsliceRangeMut<R>
    for &mut S
{
    fn get_subslice_mut(&mut self, range: R) -> Option<SubsliceMut<'_, Self>> {
        (**self).get_subslice_mut(range)
    }
    fn index_subslice_mut(&mut self, range: R) -> SubsliceMut<'_, Self> {
        (**self).index_subslice_mut(range)
    }
    unsafe fn get_subslice_unchecked_mut(&mut self, range: R) -> SubsliceMut<'_, Self> {
        (**self).get_subslice_unchecked_mut(range)
    }
}

/// A convenience trait combining all instances of [`SliceByValueSubsliceRange`]
/// with `R` equal to the various kind of standard ranges ([`core::ops::Range`],
/// [`core::ops::RangeFull`], etc.).
///
/// A blanket implementation automatically implements the trait if all necessary
/// implementations of [`SliceByValueSubsliceRange`] are available.
///
/// Note that to bind the subslice type you need to use higher-rank trait bounds:
/// ```rust
/// use value_traits::slices::*;
///
/// fn f<S>(s: S) where
///    S: SliceByValueSubslice,
///    S: for<'a> SliceByValueSubsliceGat<'a, Subslice = &'a [u8]>,
/// {
///     let _: &[u8] = s.index_subslice(0..10);
/// }
/// ```
/// The bound applies uniformly to all type of ranges.
pub trait SliceByValueSubslice:
    SliceByValueSubsliceRange<Range<usize>>
    + SliceByValueSubsliceRange<RangeFrom<usize>>
    + SliceByValueSubsliceRange<RangeFull>
    + SliceByValueSubsliceRange<RangeInclusive<usize>>
    + SliceByValueSubsliceRange<RangeTo<usize>>
    + SliceByValueSubsliceRange<RangeToInclusive<usize>>
{
}

impl<U> SliceByValueSubslice for U
where
    U: SliceByValueSubsliceRange<Range<usize>>,
    U: SliceByValueSubsliceRange<RangeFrom<usize>>,
    U: SliceByValueSubsliceRange<RangeFull>,
    U: SliceByValueSubsliceRange<RangeInclusive<usize>>,
    U: SliceByValueSubsliceRange<RangeTo<usize>>,
    U: SliceByValueSubsliceRange<RangeToInclusive<usize>>,
{
}

/// A convenience trait combining all instances of
/// [`SliceByValueSubsliceRangeMut`] with `R` equal to the various kind of
/// standard ranges ([`core::ops::Range`], [`core::ops::RangeFull`], etc.).
///
/// A blanket implementation automatically implements the trait if all necessary
/// implementations of [`SliceByValueSubsliceMut`] are available.
///
/// Note that to bind the subslice type you need to use higher-rank trait bounds:
/// ```rust
/// use value_traits::slices::*;
///
/// fn f<S>(mut s: S) where
///    S: SliceByValueSubsliceMut,
///    S: for<'a> SliceByValueSubsliceGatMut<'a, Subslice = &'a mut [u8]>,
/// {
///     let _: &mut [u8] = s.index_subslice_mut(0..10);
/// }
/// ```
/// The bound applies uniformly to all type of ranges.
pub trait SliceByValueSubsliceMut:
    SliceByValueSubsliceRangeMut<Range<usize>>
    + SliceByValueSubsliceRangeMut<RangeFrom<usize>>
    + SliceByValueSubsliceRangeMut<RangeFull>
    + SliceByValueSubsliceRangeMut<RangeInclusive<usize>>
    + SliceByValueSubsliceRangeMut<RangeTo<usize>>
    + SliceByValueSubsliceRangeMut<RangeToInclusive<usize>>
{
}

impl<U> SliceByValueSubsliceMut for U
where
    U: SliceByValueSubsliceRangeMut<Range<usize>>,
    U: SliceByValueSubsliceRangeMut<RangeFrom<usize>>,
    U: SliceByValueSubsliceRangeMut<RangeFull>,
    U: SliceByValueSubsliceRangeMut<RangeInclusive<usize>>,
    U: SliceByValueSubsliceRangeMut<RangeTo<usize>>,
    U: SliceByValueSubsliceRangeMut<RangeToInclusive<usize>>,
{
}

#[cfg(feature = "alloc")]
mod alloc_impls {
    use super::*;
    #[cfg(all(feature = "alloc", not(feature = "std")))]
    use alloc::boxed::Box;

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
            unsafe { (**self).get_value_unchecked(index) }
        }
    }

    impl<S: SliceByValueSet + ?Sized> SliceByValueSet for Box<S> {
        fn set_value(&mut self, index: usize, value: Self::Value) {
            (**self).set_value(index, value);
        }
        unsafe fn set_value_unchecked(&mut self, index: usize, value: Self::Value) {
            unsafe {
                (**self).set_value_unchecked(index, value);
            }
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
            unsafe { (**self).replace_value_unchecked(index, value) }
        }
    }

    impl<'a, S: SliceByValueSubsliceGat<'a> + ?Sized> SliceByValueSubsliceGat<'a> for Box<S> {
        type Subslice = S::Subslice;
    }
    impl<'a, S: SliceByValueSubsliceGatMut<'a> + ?Sized> SliceByValueSubsliceGatMut<'a> for Box<S> {
        type Subslice = S::Subslice;
    }

    macro_rules! impl_range_alloc {
        ($range:ty) => {
            impl<S: SliceByValueSubsliceRange<$range> + ?Sized> SliceByValueSubsliceRange<$range>
                for Box<S>
            {
                #[inline]
                fn get_subslice(&self, index: $range) -> Option<Subslice<'_, Self>> {
                    (**self).get_subslice(index)
                }

                #[inline]
                fn index_subslice(&self, index: $range) -> Subslice<'_, Self> {
                    (**self).index_subslice(index)
                }

                #[inline]
                unsafe fn get_subslice_unchecked(&self, index: $range) -> Subslice<'_, Self> {
                    unsafe { (**self).get_subslice_unchecked(index) }
                }
            }
            impl<S: SliceByValueSubsliceRangeMut<$range> + ?Sized>
                SliceByValueSubsliceRangeMut<$range> for Box<S>
            {
                #[inline]
                fn get_subslice_mut(&mut self, index: $range) -> Option<SubsliceMut<'_, Self>> {
                    (**self).get_subslice_mut(index)
                }

                #[inline]
                fn index_subslice_mut(&mut self, index: $range) -> SubsliceMut<'_, Self> {
                    (**self).index_subslice_mut(index)
                }

                #[inline]
                unsafe fn get_subslice_unchecked_mut(
                    &mut self,
                    index: $range,
                ) -> SubsliceMut<'_, Self> {
                    unsafe { (**self).get_subslice_unchecked_mut(index) }
                }
            }
        };
    }

    impl_range_alloc!(RangeFull);
    impl_range_alloc!(RangeFrom<usize>);
    impl_range_alloc!(RangeTo<usize>);
    impl_range_alloc!(Range<usize>);
    impl_range_alloc!(RangeInclusive<usize>);
    impl_range_alloc!(RangeToInclusive<usize>);
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
    impl<'a, S: SliceByValueSubsliceGat<'a> + ?Sized> SliceByValueSubsliceGat<'a> for Arc<S> {
        type Subslice = S::Subslice;
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

    impl<'a, S: SliceByValueSubsliceGat<'a> + ?Sized> SliceByValueSubsliceGat<'a> for Rc<S> {
        type Subslice = S::Subslice;
    }

    macro_rules! impl_range_arc_and_rc {
        ($range:ty) => {
            impl<S: SliceByValueSubsliceRange<$range> + ?Sized> SliceByValueSubsliceRange<$range>
                for Rc<S>
            {
                #[inline]
                fn get_subslice(&self, index: $range) -> Option<Subslice<'_, Self>> {
                    (**self).get_subslice(index)
                }

                #[inline]
                fn index_subslice(&self, index: $range) -> Subslice<'_, Self> {
                    (**self).index_subslice(index)
                }

                #[inline]
                unsafe fn get_subslice_unchecked(&self, index: $range) -> Subslice<'_, Self> {
                    unsafe { (**self).get_subslice_unchecked(index) }
                }
            }
            impl<S: SliceByValueSubsliceRange<$range> + ?Sized> SliceByValueSubsliceRange<$range>
                for Arc<S>
            {
                #[inline]
                fn get_subslice(&self, index: $range) -> Option<Subslice<'_, Self>> {
                    (**self).get_subslice(index)
                }

                #[inline]
                fn index_subslice(&self, index: $range) -> Subslice<'_, Self> {
                    (**self).index_subslice(index)
                }

                #[inline]
                unsafe fn get_subslice_unchecked(&self, index: $range) -> Subslice<'_, Self> {
                    unsafe { (**self).get_subslice_unchecked(index) }
                }
            }
        };
    }

    impl_range_arc_and_rc!(RangeFull);
    impl_range_arc_and_rc!(RangeFrom<usize>);
    impl_range_arc_and_rc!(RangeTo<usize>);
    impl_range_arc_and_rc!(Range<usize>);
    impl_range_arc_and_rc!(RangeInclusive<usize>);
    impl_range_arc_and_rc!(RangeToInclusive<usize>);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[allow(clippy::reversed_empty_ranges)]
    fn test_good_ranges() {
        // Range
        assert!((0..1).is_valid(1));
        assert!(!(1..0).is_valid(1));
        assert!(!(0..1).is_valid(0));

        // RangeFrom
        assert!((0..).is_valid(1));
        assert!((1..).is_valid(1));
        assert!(!(2..).is_valid(1));

        // RangeFull
        assert!((..).is_valid(0));
        assert!((..).is_valid(1));

        // RangeInclusive
        assert!((0..=1).is_valid(2));
        assert!(!(1..=0).is_valid(2));
        assert!(!(0..=1).is_valid(1));

        // RangeTo
        assert!((..0).is_valid(1));
        assert!((..1).is_valid(1));
        assert!(!(..2).is_valid(1));

        // RangeToInclusive
        assert!((..=0).is_valid(2));
        assert!((..=1).is_valid(2));
        assert!(!(..=2).is_valid(2));
    }
}
