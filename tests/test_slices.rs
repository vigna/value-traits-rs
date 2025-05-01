use value_traits_rs::slices::SliceByValue;

#[test]
fn test_slices() {
    let s = vec![1_i32, 2, 3];
    assert_eq!(test(s.as_slice()), 1);
}

fn test(s: impl SliceByValue<usize, Value = i32>) -> i32 {
    s.index_value(0_usize)
}
