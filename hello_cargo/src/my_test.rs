#[cfg(test)]
mod tests {
    #[test]
    fn exploration1() {
        println!("hhhhhhhhhhhhhh");
        assert_eq!(2 + 2, 3);
    }

    #[test]
    fn other() {
        panic!("Make a fail");
    }
}

fn main() {
    println!("hello world!");
    println!("hello world");
    let mut a: i32 = 3;
    println!("a : {}", a);
}