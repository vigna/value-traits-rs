/*
 * SPDX-FileCopyrightText: 2025 Tommaso Fontana
 * SPDX-FileCopyrightText: 2025 Sebastiano Vigna
 * SPDX-FileCopyrightText: 2025 Inria
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use value_traits::slices::*;

#[test]
fn test() {
    let s = vec![1_i32, 2, 3, 4, 5];
    test_bounds(&s);
}

// Compile-time check that all ranges can be forced to the same type
fn test_bounds(s: &impl SliceByValueSubslice) {
    let mut _r = s.index_subslice(0..2);
    _r = s.index_subslice(0..);
    _r = s.index_subslice(..2);
    _r = s.index_subslice(..=2);
    _r = s.index_subslice(0..=2);
    _r = s.index_subslice(..);
}
