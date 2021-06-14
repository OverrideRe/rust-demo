
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    // v1.into_iter();  // 所有权迭代器
    // v1.iter_mut();   // 可变引用迭代器

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
}

#[test]
fn mut_iterator_sum() {
    let mut v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter_mut();
    // let total: i32 = v1_iter.sum();
    assert_eq!(v1_iter.next(), Some(&mut 1));  // 会报错，因为sum方法会获取iter的所有权
    // assert_eq!(6, total);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    // assert_eq!(v1_iter.next(), Some(&1));  // 会报错，因为sum方法会获取iter的所有权
    assert_eq!(6, total);
}

#[test]
fn iterator_map() {
    let v1 = vec![1, 2, 3];
    let v2: Vec<i32> = v1.iter().map(|num| num + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}



