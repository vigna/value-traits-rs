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
//!         self.0.get_value_unchecked(index) as usize
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

impl<S: SliceByValue + ?Sized> SliceByValue for Box<S> {
    type Value = S::Value;
    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }
}

/// Read-only slice-by-value trait.
///
/// The only method that must be implement is
/// [`get_value_unchecked`](`SliceByValueGet::get_value_unchecked`).
pub trait SliceByValueGet: SliceByValue {
    /// See [the `Index` implementation for slices](slice#impl-Index%3CI%3E-for-%5BT%5D).
    fn index_value(&self, index: usize) -> Self::Value {
        if index < self.len() {
            // SAFETY: index is without bounds
            return unsafe { self.get_value_unchecked(index) };
        }

        panic!("Index is out of range"); // TODO: equal to slices
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
            unsafe { Some(self.get_value_unchecked(index)) }
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

/// Mutable slice-by-value trait providing setting methods.
///
/// The only method that must be implement is
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
        if index < self.len() {
            // SAFETY: index is without bounds
            unsafe {
                self.set_value_unchecked(index, value);
            }
        }

        panic!("Index is out of range"); // TODO: equal to slices
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
        if index < self.len() {
            // SAFETY: index is without bounds
            return unsafe { self.replace_value_unchecked(index, value) };
        }

        panic!("Index is out of range"); // TODO: equal to slices
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

impl<S: SliceByValueRepl + ?Sized> SliceByValueRepl for Box<S> {
    fn replace_value(&mut self, index: usize, value: Self::Value) -> Self::Value {
        (**self).replace_value(index, value)
    }
    unsafe fn replace_value_unchecked(&mut self, index: usize, value: Self::Value) -> Self::Value {
        unsafe { (**self).replace_value_unchecked(index, value) }
    }
}

/// A GAT-like trait specifying the subslice type.
///
/// It implicitly restricts the lifetime `'a` used in `SliceByValueRange`
/// to be `where Self: 'a`. Moreover, it requires [`SliceByValueGet`].
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
/// `'static` (`for<'all> Self: trait`)
///
/// Please see [Sabrina's Blog][1] for more information, and how a trait
/// like this can be used to solve it by implicitly restricting HRTBs.
///
/// [1]:
///     <https://sabrinajewson.org/blog/the-better-alternative-to-lifetime-gats>
pub trait SliceByValueSubsliceGat<'a, __Implicit: ImplBound = Ref<'a, Self>>:
    SliceByValueGet
{
    type Subslice: 'a + SliceByValueGet<Value = Self::Value> + SliceByValueSubslice<usize>;
}

impl<'a, T: SliceByValueSubsliceGat<'a> + ?Sized> SliceByValueSubsliceGat<'a> for &T {
    type Subslice = <T as SliceByValueSubsliceGat<'a>>::Subslice;
}

impl<'a, T: SliceByValueSubsliceGat<'a> + ?Sized> SliceByValueSubsliceGat<'a> for &mut T {
    type Subslice = <T as SliceByValueSubsliceGat<'a>>::Subslice;
}

/// A convenience type representing the type of subslice
/// of a type implementing [`SliceByValueSubsliceGat`].
#[allow(type_alias_bounds)] // yeah the type alias bounds are not enforced, but they are useful for documentation
pub type Subslice<'a, T: SliceByValueSubsliceGat<'a>> =
    <T as SliceByValueSubsliceGat<'a>>::Subslice;

/// A trait implementing subslicing for a specific range parameter.
///
/// The user should never see this trait. [`SliceByValueSubslice`] combines all
/// instances of this trait with `R` equal to the various kind of standard
/// ranges ([`core::ops::Range`], [`core::ops::RangeFull`], etc.).
pub trait SliceByValueSubsliceRange<R>: for<'a> SliceByValueSubsliceGat<'a> {
    /// See [the `Index` implementation for slices](slice#impl-Index%3CI%3E-for-%5BT%5D).
    fn index_subslice(&self, range: R) -> Subslice<'_, Self>;

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
    fn get_subslice(&self, range: R) -> Option<Subslice<'_, Self>>;
}

impl<S: SliceByValueSubsliceRange<R> + ?Sized, R> SliceByValueSubsliceRange<R> for &S {
    fn get_subslice(&self, range: R) -> Option<Subslice<'_, Self>> {
        (**self).get_subslice(range)
    }
    fn index_subslice(&self, range: R) -> Subslice<'_, Self> {
        (**self).index_subslice(range)
    }
    unsafe fn get_subslice_unchecked(&self, range: R) -> Subslice<'_, Self> {
        (**self).get_subslice_unchecked(range)
    }
}
impl<S: SliceByValueSubsliceRange<R> + ?Sized, R> SliceByValueSubsliceRange<R> for &mut S {
    fn get_subslice(&self, range: R) -> Option<Subslice<'_, Self>> {
        (**self).get_subslice(range)
    }
    fn index_subslice(&self, range: R) -> Subslice<'_, Self> {
        (**self).index_subslice(range)
    }
    unsafe fn get_subslice_unchecked(&self, range: R) -> Subslice<'_, Self> {
        (**self).get_subslice_unchecked(range)
    }
}

// TODO: can we implement traits conditionally on the associated type? Like,
// replace only if it present in the root slice?

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
        + SliceByValueSubsliceMut<usize>;
}

impl<'a, T: SliceByValueSubsliceGatMut<'a> + ?Sized> SliceByValueSubsliceGatMut<'a> for &mut T {
    type Subslice = <T as SliceByValueSubsliceGatMut<'a>>::Subslice;
}

/// A convenience type representing the type of subslice
/// of a type implementing [`SliceByValueSubsliceGatMut`].
#[allow(type_alias_bounds)] // yeah the type alias bounds are not enforced, but they are useful for documentation
pub type SubsliceMut<'a, T: SliceByValueSubsliceGatMut<'a>> =
    <T as SliceByValueSubsliceGatMut<'a>>::Subslice;

/// A trait implementing mutable subslicing for a specific range parameter.
///
///  The user should never see this trait. [`SliceByValueSubsliceMut`] combines
/// all instances of this trait with `R` equal to the various kind of standard
/// ranges ([`core::ops::Range`], [`core::ops::RangeFull`], etc.).
pub trait SliceByValueSubsliceRangeMut<R>: for<'a> SliceByValueSubsliceGatMut<'a> {
    /// See [the `Index` implementation for slices](slice#impl-Index%3CI%3E-for-%5BT%5D).
    fn index_subslice_mut(&mut self, range: R) -> SubsliceMut<'_, Self>;

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
    fn get_subslice_mut(&mut self, range: R) -> Option<SubsliceMut<'_, Self>>;
}

impl<S: SliceByValueSubsliceRangeMut<R> + ?Sized, R> SliceByValueSubsliceRangeMut<R> for &mut S {
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
pub trait SliceByValueSubslice<T = usize>:
    SliceByValueSubsliceRange<Range<T>>
    + SliceByValueSubsliceRange<RangeFrom<T>>
    + SliceByValueSubsliceRange<RangeFull>
    + SliceByValueSubsliceRange<RangeInclusive<T>>
    + SliceByValueSubsliceRange<RangeTo<T>>
    + SliceByValueSubsliceRange<RangeToInclusive<T>>
{
}

impl<U, T> SliceByValueSubslice<T> for U
where
    U: SliceByValueSubsliceRange<Range<T>>,
    U: SliceByValueSubsliceRange<RangeFrom<T>>,
    U: SliceByValueSubsliceRange<RangeFull>,
    U: SliceByValueSubsliceRange<RangeInclusive<T>>,
    U: SliceByValueSubsliceRange<RangeTo<T>>,
    U: SliceByValueSubsliceRange<RangeToInclusive<T>>,
{
}

/// A convenience trait combining all instances of
/// [`SliceByValueSubsliceRangeMut`] with `R` equal to the various kind of
/// standard ranges ([`core::ops::Range`], [`core::ops::RangeFull`], etc.).
///
/// A blanket implementation automatically implements the trait if all necessary
/// implementations of [`SliceByValueSubsliceMut`] are available.
pub trait SliceByValueSubsliceMut<T = usize>:
    SliceByValueSubsliceRangeMut<Range<T>>
    + SliceByValueSubsliceRangeMut<RangeFrom<T>>
    + SliceByValueSubsliceRangeMut<RangeFull>
    + SliceByValueSubsliceRangeMut<RangeInclusive<T>>
    + SliceByValueSubsliceRangeMut<RangeTo<T>>
    + SliceByValueSubsliceRangeMut<RangeToInclusive<T>>
{
}

impl<U, T> SliceByValueSubsliceMut<T> for U
where
    U: SliceByValueSubsliceRangeMut<Range<T>>,
    U: SliceByValueSubsliceRangeMut<RangeFrom<T>>,
    U: SliceByValueSubsliceRangeMut<RangeFull>,
    U: SliceByValueSubsliceRangeMut<RangeInclusive<T>>,
    U: SliceByValueSubsliceRangeMut<RangeTo<T>>,
    U: SliceByValueSubsliceRangeMut<RangeToInclusive<T>>,
{
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
