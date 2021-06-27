//! # 第一次rust测试项目
//! 根目录

mod closure;
mod collections;
mod iter;
mod my_match;
mod my_box;
mod thread;
mod object;
mod mode;
mod advanced_features;

/// # 第一个测试方法
/// assert_eq!(2 + 2, 4);
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