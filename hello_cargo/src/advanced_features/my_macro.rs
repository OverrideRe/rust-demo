/// 自定义”声明“宏 macro_rules!
mod test0 {

    /// => 前面括号的内容是匹配模式，后面的大括号内容是匹配到执行的代码
    /// 前面括号中$($x: expr)表示将匹配到的任意值赋值给$x，后面的","表示匹配到的值可能以","分割，"*"表示"*"前面的匹配模式有0个或者更多个
    #[macro_export]
    macro_rules! vec {
        ($($x: expr),*) => {
            {
                let mut list = Vec::new();
                $(
                    list.push($x);
                )*
                list
            }
        };
    }

    #[test]
    fn test() {
        let list = vec![1, 2, 3];
        println!("vv : {:?}", list);
    }
}