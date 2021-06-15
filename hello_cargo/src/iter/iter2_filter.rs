//! #迭代器学习

#[derive(PartialEq, Debug)]
struct Shoe {
    size: i32,
    style: String,
}

/// # 查找指定鞋码的鞋
fn shoes_in_my_size(shoes: Vec<Shoe>, size: i32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == size)
        .collect()
}

#[test]
fn filters_by_size() {
    let shoe1 = Shoe {size: 42, style: String::from("sneaker")};
    let shoe2 = Shoe {size: 41, style: String::from("sandal")};
    let shoe3 = Shoe {size: 41, style: String::from("boot")};

    let v1 = vec![shoe1, shoe2, shoe3];

    let v2 = shoes_in_my_size(v1, 41);
    assert_eq!(v2, vec![Shoe {size: 41, style: String::from("sandal")}, Shoe {size: 41, style: String::from("boot")}])
}

// 自定义迭代器

#[derive(Debug)]
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn counter_next() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);

    // 1  2 3 4 5   
    // 2 3 4 5      1 * 2 = 2, 2 * 3 = 6, 3 * 4 = 12, 4 * 5 = 20，其中只有6和12能被3整除，所以只剩这两个相加，得18
    // 最后一对 5和None是没有的，因为zip对任一迭代器返回None就全都返回None
    let total: u32 = Counter::new().zip(Counter::new().skip(1)).
                                    map(|(a, b)| a * b)
                                    .filter(|x| x % 3 == 0)
                                    .sum();
    assert_eq!(total, 18);
}