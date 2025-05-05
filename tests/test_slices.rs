use core::ops::Range;
use value_traits_rs::slices::{SliceByValueGet, SliceByValueRange};

#[test]
fn test_slices() {
    let s = vec![1_i32, 2, 3];
    assert_eq!(test_usize(s.as_slice()), 1);
    let t = s.as_slice();
    assert_eq!(test_range(&t), &[1, 2]);
    assert_eq!(test_usize_range(&t), (1, [1, 2].as_ref()));
    assert_eq!(test_len(&t), 3);
}

fn test_usize(s: impl SliceByValueGet<Value = i32>) -> i32 {
    s.index_value(0_usize)
}

fn test_range<'a, S>(s: &S) -> &[i32]
where
    S: for<'b> SliceByValueRange<Range<usize>, SliceRange<'b> = &'b [i32]>,
{
    s.index_range(0..2)
}

fn test_usize_range<'a, S>(s: &S) -> (i32, &[i32])
where
    S: SliceByValueGet<Value = i32>
        + for<'b> SliceByValueRange<Range<usize>, SliceRange<'b> = &'b [i32]>,
{
    (s.index_value(0_usize), s.index_range(0..2))
}

fn test_len<'a, S>(s: &S) -> usize
where
    S: SliceByValueGet<Value = i32>
        + for<'b> SliceByValueRange<Range<usize>, SliceRange<'b> = &'b [i32]>,
{
    s.len()
}
