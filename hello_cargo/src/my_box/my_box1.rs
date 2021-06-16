#[test]
fn test1() {
    let x = 5;
    let y = &5;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

#[test]
fn test2() {
    let x = 5;
    let y = Box::new(5);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}



struct MyBox<T> (T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox (x)
    }
}

struct MyStr<T,E> {
    t: T,
    e: E,
}

impl<T,E> MyStr<T,E> {
    fn new(t: T, e: E) -> MyStr<T,E> {
        MyStr{t, e}
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T, E> Deref for MyStr<T,E> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.t
    }
}


#[test]
fn test3() {
    let x = 5;
    let y = MyStr::new(5, 9);

    assert_eq!(5, x);
    assert_eq!(5, *y); // *y = *(y.deref())
}
