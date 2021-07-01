
/// 函数与闭包
mod test0 {

    fn add_one(n: i32) -> i32 {
        n + n
    }

    fn add_twice(f: fn(i32) -> i32, n: i32) -> i32 {
        f(n) + n
    }

    fn number_2_str(n: &i32) -> String {
        n.to_string()
    }

    // 元组结构体形似函数，并且也被称为实现了闭包trait的函数指针，所以可以作为函数进行传递调用
    #[derive(Debug)]
    enum Status {
        Value(i32),
        Stop,
    }

    #[test]
    fn test() {
        println!("add num result : {}", add_twice(add_one, 4));

        let list_of_numbers = vec![0, 1, 2, 3, 4];

        // 使用闭包将数字列表转换成字符串列表
        let list_of_str0:Vec<String> = list_of_numbers.iter().map(|n| n.to_string()).collect();
        println!("list_of_str0 : {:?}", list_of_str0);

        // 使用函数参数将数字列表转换成字符串列表
        let list_of_str1: Vec<String>= list_of_numbers.iter().map(number_2_str).collect();
        println!("list_of_str1 : {:?}", list_of_str1);

        // 元组结构体初始化方法作为函数指针，0i32表明元组内部数据是i32类型的
        let status_list: Vec<Status> = (0i32..20).map(Status::Value).collect();
        println!("status_list : {:?}", status_list);
    }
}


/// 返回闭包
mod test1 {
    // 会编译报错，return type cannot have an unboxed trait object  返回类型不能有未装箱的对象
    // fn returns_closure() -> Fn(i32) ->i32 {
    //     |x| x + 1
    // }

    // 装箱之后就能编译通过了
    fn returns_closure() -> Box<Fn(i32) -> i32> {
        Box::new(|x| x + x)
    }

    #[test]
    fn test() {
        println!("运算结果 : {}", returns_closure()(3));
    }
}