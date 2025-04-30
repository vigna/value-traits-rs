use value_traits_rs::slices::SliceBy;

#[test]
fn test_slices() {
    let s = vec![1, 2, 3];
    assert_eq!(test(s.as_ref()), 1);
}

fn test(s: impl SliceBy<Value = i32>) -> i32 {
    s.index_value(0_usize)
}
