#[derive(PartialEq, Debug)]
struct Shoe {
    size: i32,
    style: String,
}

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