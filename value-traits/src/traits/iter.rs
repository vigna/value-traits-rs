/*
 * SPDX-FileCopyrightText: 2025 Tommaso Fontana
 * SPDX-FileCopyrightText: 2025 Sebastiano Vigna
 * SPDX-FileCopyrightText: 2025 Inria
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

//! Traits for value-based iterators, providing alternatives to Rust's standard reference-based iteration.
//!
//! These traits enable iteration over sequences where elements are produced by value,
//! which is useful for virtual or implicitly defined sequences (e.g., functional, compressed).
//! This contrasts with `std::iter::Iterator` which often yields references, especially when
//! iterating over collections without consuming them.

use crate::{ImplBound, Ref};

/// GAT (Generic Associated Type) trait for `IterableByValue`.
///
/// This trait defines the associated types `Item` (the type of items yielded by the iterator)
/// and `Iter` (the iterator type itself, bound by a lifetime `'a`).
/// It's primarily an implementation detail for [`IterableByValue`].
pub trait IterableByValueGat<'a, __Implicit: ImplBound = Ref<'a, Self>> {
    /// The type of items yielded by the iterator.
    type Item;
    /// The iterator type, bound by lifetime `'a` and yielding `Self::Item`.
    type Iter: 'a + Iterator<Item = Self::Item>;
}

/// Convenience type alias for the iterator type associated with `IterableByValueGat`.
pub type Iter<'a, T> = <T as IterableByValueGat<'a>>::Iter;

impl<'a, T: IterableByValueGat<'a> + ?Sized> IterableByValueGat<'a> for &T {
    type Item = T::Item;
    type Iter = T::Iter;
}

impl<'a, T: IterableByValueGat<'a> + ?Sized> IterableByValueGat<'a> for &mut T {
    type Item = T::Item;
    type Iter = T::Iter;
}

/// Provides a method to get an iterator that yields items by value.
///
/// This trait is necessary as all standard Rust containers already have
/// [`IntoIterator`]-based methods for obtaining reference-based iterators.
///
/// Note that [`iter_value`](IterableByValue::iter_value) returns a standard
/// iterator. However, the intended semantics is that the iterator will return
/// values.
///
/// If you need to iterate from a given position, and you can implement such
/// an iterator more efficiently, please consider [`IterableByValueFrom`].
pub trait IterableByValue: for<'a> IterableByValueGat<'a> {
    /// Returns an iterator on values.
    fn iter_value(&self) -> Iter<'_, Self>;
}

/// GAT (Generic Associated Type) trait for `IterableByValueFrom`.
///
/// Similar to [`IterableByValueGat`], this defines associated types `Item` and `IterFrom`
/// for iterators that can start from a specified index.
/// It's primarily an implementation detail for [`IterableByValueFrom`].
pub trait IterableByValueFromGat<'a, __Implicit: ImplBound = Ref<'a, Self>> {
    /// The type of items yielded by the iterator.
    type Item;
    /// The iterator type, starting from a specified index, bound by lifetime `'a` and yielding `Self::Item`.
    type IterFrom: 'a + Iterator<Item = Self::Item>;
}

impl<T: IterableByValue> IterableByValue for &T {
    fn iter_value(&self) -> Iter<'_, Self> {
        (**self).iter_value()
    }
}

impl<T: IterableByValue> IterableByValue for &mut T {
    fn iter_value(&self) -> Iter<'_, Self> {
        (**self).iter_value()
    }
}

pub trait IterableByValueFromGat<'a, __Implicit: ImplBound = Ref<'a, Self>> {
    type Item;
    type IterFrom: 'a + Iterator<Item = Self::Item>;
}

impl<'a, T: IterableByValueFromGat<'a> + ?Sized> IterableByValueFromGat<'a> for &T {
    type Item = T::Item;
    type IterFrom = T::IterFrom;
}

impl<'a, T: IterableByValueFromGat<'a> + ?Sized> IterableByValueFromGat<'a> for &mut T {
    type Item = T::Item;
    type IterFrom = T::IterFrom;
}

/// Convenience type alias for the iterator type associated with `IterableByValueFromGat`.
pub type IterFrom<'a, T> = <T as IterableByValueFromGat<'a>>::IterFrom;

/// Provides a method to get an iterator that yields items by value, starting from a specified position.
///
/// This is a version of [`IterableByValue::iter_value`] that is useful for
/// types in which obtaining a global iterator and skipping is expensive. Note
/// that we cannot provide a skip-based default implementation because the
/// returned type is not necessarily the same type as that returned by
/// [`IterableByValue::iter_value`], but you are free to implement
/// [`iter_value_from`](IterableByValueFrom::iter_value_from) that way.
pub trait IterableByValueFrom: for<'a> IterableByValueFromGat<'a> {
    /// Returns an iterator on values starting at the given position.
    fn iter_value_from(&self, from: usize) -> IterFrom<'_, Self>;
}

impl<T: IterableByValueFrom> IterableByValueFrom for &T {
    fn iter_value_from(&self, from: usize) -> IterFrom<'_, Self> {
        (**self).iter_value_from(from)
    }
}

impl<T: IterableByValueFrom> IterableByValueFrom for &mut T {
    fn iter_value_from(&self, from: usize) -> IterFrom<'_, Self> {
        (**self).iter_value_from(from)
    }
}
