//! Defines the core traits for by-value operations on slice-like and iterable structures.
//!
//! This module re-exports traits from its submodules:
//! - [`iter`]: Contains traits for by-value iteration, such as [`IterableByValue`](iter::IterableByValue).
//! - [`slices`]: Contains traits for by-value slice operations, such as [`SliceByValue`](slices::SliceByValue).
//!
//! These traits provide alternatives to Rust's standard reference-based mechanisms,
//! enabling more flexible data representations (e.g., functional, compressed, implicit).

/*
 * SPDX-FileCopyrightText: 2025 Tommaso Fontana
 * SPDX-FileCopyrightText: 2025 Sebastiano Vigna
 * SPDX-FileCopyrightText: 2025 Inria
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

pub mod iter;
pub mod slices;
