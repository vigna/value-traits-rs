use core::ops::Range;
use value_traits_rs::slices::SliceByValue;

#[test]
fn test_slices() {
    let s = vec![1_i32, 2, 3];
    assert_eq!(test_usize(s.as_slice()), 1);
    assert_eq!(test_range(s.as_slice()), &[1, 2]);
    assert_eq!(test_usize_range(s.as_slice()), (1, [1, 2].as_ref()));
}

fn test_usize(s: impl SliceByValue<usize, Value = i32>) -> i32 {
    s.index_value(0_usize)
}

fn test_range<'a>(s: impl SliceByValue<Range<usize>, Value = &'a [i32]>) -> &'a [i32] {
    s.index_value(0..2)
}

fn test_usize_range<'a>(
    s: impl SliceByValue<usize, Value = i32> + SliceByValue<Range<usize>, Value = &'a [i32]>,
) -> (i32, &'a [i32]) {
    (s.index_value(0_usize), s.index_value(0..2))
}

fn test_len<'a>(
    s: impl SliceByValue<usize, Value = i32> + SliceByValue<Range<usize>, Value = &'a [i32]>,
) -> usize {
    s.len()
}
