//! This module provides implementations of the `value-traits` for various standard Rust types.
//!
//! It includes implementations for:
//! - Fixed-size arrays (`[T; N]`) in the [`arrays`] module.
//! - Dynamically sized slices (`[T]`) and `Box<[T]>` in the [`slices`] module.
//! - Vectors (`Vec<T>`) in the [`vectors`] module (requires the `alloc` feature).
//!
//! These implementations allow standard Rust collection types to be used seamlessly
//! with the by-value traits defined in this crate.

/*
 * SPDX-FileCopyrightText: 2025 Tommaso Fontana
 * SPDX-FileCopyrightText: 2025 Sebastiano Vigna
 * SPDX-FileCopyrightText: 2025 Inria
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

pub mod arrays;
pub mod slices;
pub mod vectors;
