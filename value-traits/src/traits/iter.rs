/*
 * SPDX-FileCopyrightText: 2025 Tommaso Fontana
 * SPDX-FileCopyrightText: 2025 Sebastiano Vigna
 * SPDX-FileCopyrightText: 2025 Inria
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

//! Traits for value-based iterators.

use crate::{ImplBound, Ref};

/// A GAT-like trait specifying the type of a value-based iterator.
///
/// See [`SliceByValueSubsliceGat`](crate::slices::SliceByValueSubsliceGat) for
/// more information.
pub trait IterateByValueGat<'a, __Implicit: ImplBound = Ref<'a, Self>> {
    type Item;
    type Iter: 'a + Iterator<Item = Self::Item>;
}

pub type Iter<'a, T> = <T as IterateByValueGat<'a>>::Iter;

impl<'a, T: IterateByValueGat<'a> + ?Sized> IterateByValueGat<'a> for &T {
    type Item = T::Item;
    type Iter = T::Iter;
}

impl<'a, T: IterateByValueGat<'a> + ?Sized> IterateByValueGat<'a> for &mut T {
    type Item = T::Item;
    type Iter = T::Iter;
}

/// A trait for obtaining a value-based iterator.
///
/// This trait necessary as all standard Rust containers already have
/// [`IntoIterator`]-based methods for obtaining reference-based iterators.
///
/// Note that [`iter_value`](IterateByValue::iter_value) returns a standard
/// iterator. However, the intended semantics is that the iterator will return
/// values.
///
/// If you need to iterate from a given position, and you can implement such
/// an iterator more efficiently, please consider [`IterateByValueFrom`].
///
/// Note that to bound the iterator type you need to use higher-rank trait bounds:
/// ```rust
/// use value_traits::iter::*;
///
/// fn f<S>(s: S) where
///    S: IterateByValue,
///    S: for<'a> IterateByValueGat<'a, Iter: ExactSizeIterator>,
/// {
///     let _ = s.iter_value().len();
/// }
/// ```
pub trait IterateByValue: for<'a> IterateByValueGat<'a> {
    /// Returns an iterator on values.
    fn iter_value(&self) -> Iter<'_, Self>;
}

impl<T: IterateByValue> IterateByValue for &T {
    fn iter_value(&self) -> Iter<'_, Self> {
        (**self).iter_value()
    }
}

impl<T: IterateByValue> IterateByValue for &mut T {
    fn iter_value(&self) -> Iter<'_, Self> {
        (**self).iter_value()
    }
}

/// A GAT-like trait specifying the type of a value-based iterator starting from
/// a given position.
///
/// See [`SliceByValueSubsliceGat`](crate::slices::SliceByValueSubsliceGat) for
/// more information.
pub trait IterateByValueFromGat<'a, __Implicit: ImplBound = Ref<'a, Self>> {
    type Item;
    type IterFrom: 'a + Iterator<Item = Self::Item>;
}

impl<'a, T: IterateByValueFromGat<'a> + ?Sized> IterateByValueFromGat<'a> for &T {
    type Item = T::Item;
    type IterFrom = T::IterFrom;
}

impl<'a, T: IterateByValueFromGat<'a> + ?Sized> IterateByValueFromGat<'a> for &mut T {
    type Item = T::Item;
    type IterFrom = T::IterFrom;
}

pub type IterFrom<'a, T> = <T as IterateByValueFromGat<'a>>::IterFrom;

/// A trait for obtaining a value-based iterator starting from a given position.
///
/// This is a version of [`IterateByValue::iter_value`] that is useful for
/// types in which obtaining a global iterator and skipping is expensive. Note
/// that we cannot provide a skip-based default implementation because the
/// returned type is not necessarily the same type as that returned by
/// [`IterateByValue::iter_value`], but you are free to implement
/// [`iter_value_from`](IterateByValueFrom::iter_value_from) that way.
///
/// Note that to bound the iterator type you need to use higher-rank trait bounds:
/// ```rust
/// use value_traits::iter::*;
///
/// fn f<S>(s: S) where
///    S: IterateByValueFrom,
///    S: for<'a> IterateByValueFromGat<'a, IterFrom: ExactSizeIterator>,
/// {
///     let _ = s.iter_value_from(0).len();
/// }
/// ```
pub trait IterateByValueFrom: for<'a> IterateByValueFromGat<'a> {
    /// Returns an iterator on values starting at the given position.
    fn iter_value_from(&self, from: usize) -> IterFrom<'_, Self>;
}

impl<T: IterateByValueFrom> IterateByValueFrom for &T {
    fn iter_value_from(&self, from: usize) -> IterFrom<'_, Self> {
        (**self).iter_value_from(from)
    }
}

impl<T: IterateByValueFrom> IterateByValueFrom for &mut T {
    fn iter_value_from(&self, from: usize) -> IterFrom<'_, Self> {
        (**self).iter_value_from(from)
    }
}
