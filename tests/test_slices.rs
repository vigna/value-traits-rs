use core::ops::Range;
use value_traits_rs::slices::*;

#[test]
fn test_slices() {
    let mut s = vec![1_i32, 2, 3];
    assert_eq!(test_usize(s.as_slice()), 1);
    let t = s.as_slice();
    assert_eq!(test_range(&t), &[1, 2]);
    assert_eq!(test_usize_range(&t), (1, [1, 2].as_ref()));
    assert_eq!(test_len(&t), 3);

    let t = s.as_mut_slice();
    assert_eq!(test_range_mut(t), &mut [1, 2]);
}

fn test_usize(s: impl SliceByValueGet<Value = i32>) -> i32 {
    s.index_value(0_usize)
}

fn test_range<'a, S>(s: &S) -> &[i32]
where
    S: SliceByValueRange<Range<usize>>,
    S: for<'b> SBVRL<'b, Range<usize>, SliceRange = &'b [i32]>,
{
    let a = s.index_range(0..2);
    let _ = s.index_range(0..3); // it can be borrowed multiple times
    a
}

fn test_range_mut<'a, S>(s: &mut S) -> &mut [i32]
where
    S: SliceByValueRangeMut<Range<usize>> + ?Sized,
    S: for<'b> SBVRML<'b, Range<usize>, SliceRangeMut = &'b mut [i32]>,
{
    let a = s.index_range_mut(0..2);
    // let _ = s.index_range_mut(0..2); // this instead should not compile
    a
}

fn test_usize_range<'a, S>(s: &S) -> (i32, &[i32])
where
    S: SliceByValueGet<Value = i32>,
    S: SliceByValueRange<Range<usize>>,
    S: for<'b> SBVRL<'b, Range<usize>, SliceRange = &'b [i32]>,
{
    (s.index_value(0_usize), s.index_range(0..2))
}

fn test_len<'a, S>(s: &S) -> usize
where
    S: SliceByValueRange<Range<usize>>,
    S: for<'b> SBVRL<'b, Range<usize>, SliceRange = &'b [i32]>,
{
    s.len()
}
