/*
 * SPDX-FileCopyrightText: 2025 Tommaso Fontana
 * SPDX-FileCopyrightText: 2025 Sebastiano Vigna
 * SPDX-FileCopyrightText: 2025 Inria
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

use value_traits::slices::*;

#[test]
fn test_vecs() {
    let s = vec![1_i32, 2, 3, 4, 5];
    assert_eq!(
        s.index_subslice(1..).index_subslice(..3),
        [2, 3, 4].as_ref()
    );
}
