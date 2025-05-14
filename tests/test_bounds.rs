use core::ops::Range;
use std::ops::{RangeFrom, RangeFull, RangeTo};

use value_traits_rs::slices::*;

#[test]
fn test() {
    let s = vec![1_i32, 2, 3, 4, 5];
    assert_eq!(s.index_range(1..).index_range(..3), [2, 3, 4].as_ref());
}

// Compile-time check that all ranges can be forced to the same type
fn _test_bounds<'a, S>(s: &S)
where
    S: SliceByValueGet<Value = i32>,
    S: SliceByValueRange<Range<usize>>,
    S: SliceByValueRange<RangeFrom<usize>>,
    S: SliceByValueRange<RangeTo<usize>>,
    S: SliceByValueRange<RangeFull>,
    S: for<'b> SBVRL<'b, Range<usize>>,
    S: for<'b> SBVRL<'b, RangeFrom<usize>, SliceRange = SliceRange<'b, Range<usize>, S>>,
    S: for<'b> SBVRL<'b, RangeTo<usize>, SliceRange = SliceRange<'b, Range<usize>, S>>,
    S: for<'b> SBVRL<'b, RangeFull, SliceRange = SliceRange<'b, Range<usize>, S>>,
{
    let mut _r = s.index_range(0..2);
    _r = s.index_range(0..);
    _r = s.index_range(..2);
    _r = s.index_range(..);
}
