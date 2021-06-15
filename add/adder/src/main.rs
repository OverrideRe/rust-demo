//! adder crate引入了add-one crate并调用了其中的方法
use add_one;
use rand;

fn main() {
    let num = 9;
    println!("Hello, world! {} plus one is {}", num, add_one::add_one(num));
}
