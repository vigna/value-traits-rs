use value_traits_rs::slices::*;

#[test]
fn test_vecs() {
    let s = vec![1_i32, 2, 3, 4, 5];
    assert_eq!(s.index_range(1..).index_range(..3), [2, 3, 4].as_ref());
}
