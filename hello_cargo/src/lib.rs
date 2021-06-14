mod collections;
mod iter;

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        println!("555555555555");
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn panic_test() {
        panic!("Make a fail");
    }
}