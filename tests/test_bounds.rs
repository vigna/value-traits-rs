use value_traits::slices::*;

#[test]
fn test() {
    let s = vec![1_i32, 2, 3, 4, 5];
    test_bounds(&s[..]);
}

// Compile-time check that all ranges can be forced to the same type
fn test_bounds<'a>(s: impl SliceByValueSubslice<'a>) {
    let mut _r = s.index_range(0..2);
    _r = s.index_range(0..);
    _r = s.index_range(..2);
    _r = s.index_range(..=2);
    _r = s.index_range(0..=2);
    _r = s.index_range(..);
}
