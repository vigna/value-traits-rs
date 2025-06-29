/*
 * SPDX-FileCopyrightText: 2025 Tommaso Fontana
 * SPDX-FileCopyrightText: 2025 Sebastiano Vigna
 * SPDX-FileCopyrightText: 2025 Inria
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

#![warn(missing_docs)]
#![warn(clippy::missing_errors_doc)]
#![warn(clippy::missing_panics_doc)]
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(unconditional_recursion)]
#![doc = include_str!("../README.md")]

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;

#[cfg(feature = "derive")]
pub use value_traits_derive::{Iterators, IteratorsMut, Subslices, SubslicesMut};

// Impls are not re-exported
pub mod impls;

// Traits are re-exported
mod traits;
pub use traits::*;

#[doc(hidden)]
#[allow(private_bounds)]
pub trait ImplBound: ImplBoundPriv {}
#[doc(hidden)]
pub(crate) trait ImplBoundPriv {}
impl<T: ?Sized + ImplBoundPriv> ImplBound for T {}
#[doc(hidden)]
pub struct Ref<'a, T: ?Sized>(&'a T);
impl<T: ?Sized> ImplBoundPriv for Ref<'_, T> {}
