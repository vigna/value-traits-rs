/*
 * SPDX-FileCopyrightText: 2025 Tommaso Fontana
 * SPDX-FileCopyrightText: 2025 Sebastiano Vigna
 * SPDX-FileCopyrightText: 2025 Inria
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

//! Helpers structs used by the procedural macro, and possibly by users, to
//! simplify the implementation of the traits.

use core::borrow::Borrow;
use core::ops::RangeBounds;

mod iter;
pub use iter::Iter;

mod subslice;
pub use subslice::SubsliceImpl;

mod subslice_mut;
pub use subslice_mut::SubsliceImplMut;

/// Given a range, apply a subrange to it, producing a new range.
///
/// # Caveats
/// The range `..=usize::MAX` cannot be represented and it will panic if you
/// try to use it.
pub fn range_compose(
    base: impl Borrow<core::ops::Range<usize>>,
    subrange: impl RangeBounds<usize>,
) -> core::ops::Range<usize> {
    let base = base.borrow();
    let start = match subrange.start_bound() {
        core::ops::Bound::Included(s) => base.start + *s,
        core::ops::Bound::Excluded(s) => base.start + *s + 1,
        core::ops::Bound::Unbounded => base.start,
    };
    let end = match subrange.end_bound() {
        core::ops::Bound::Included(s) => base.start + *s + 1,
        core::ops::Bound::Excluded(s) => base.start + *s,
        core::ops::Bound::Unbounded => base.end,
    };
    start..end
}
