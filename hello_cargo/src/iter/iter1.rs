
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    // v1.into_iter();  // 所有权迭代器
    // v1.iter_mut();   // 可变引用迭代器

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
}

fn main() {
    iterator_demonstration();
}