
use std::cmp::PartialOrd;

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &i in list.iter() {
        if (i > largest) {
            largest = i;
        }
    }
    largest
}

fn largest1<T: PartialOrd>(list: &[&'static T]) -> &'static T {
    // 获取的是T类型不是&T类型
    let mut largest = list[0];
    for &i in list.iter() {
        if (i > largest) {
            largest = i;
        }
    }
    largest
}

trait Print {
    fn print(&self) {
        println!("hello");
    }
}

struct Student {
    age: usize,
    name: String,
    class: String,
}

impl Print for Student {
    fn print(&self) {
        println!("name : {}", self.name);
    }
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T: std::fmt::Display> Print for Point<T> {
    fn print(&self) {
        println!("x : {}", self.x);
    }
}

fn main() {
    let i32_list = vec![&1, &2, &2, &0, &8, &9, &3, &5, &6, &1];
    println!("i32_list largest : {}", largest1(&i32_list));
    
    let char_list = vec![&'a', &'i', &'w', &'o', &'c'];
    println!("char_list largest : {}", largest1(&char_list));

    // let char_list = vec!['a', 'i', 'w', 'o', 'c'];
    // println!("char_list largest : {}", largest(&char_list));


    let student = Student {name: String::from("yinghao"), age: 26, class: String::from("502")};
    student.print();

    let point = Point { x: 20, y: 20};
    point.print();
}