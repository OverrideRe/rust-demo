fn main() {
    let r = Rectangle::square(18);
    println!("result is {}", area(&r));
    println!("r is {:?}", r);
    println!("r is {:#?}", r);
    println!("方法输出 : {}", r.area());
    println!("是否能包裹 : {}", r.can_hold(& Rectangle {width:20, height: 20}));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// debug注解，表示派生debug trait
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.width >= rectangle.width && self.height >= rectangle.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}