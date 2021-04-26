
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn str_lifetime() {
    let str1 = String::from("str1");
    {
        let str2 = String::from("str2");
        // result的生命周期跟str2一样，所以能通过编译
        let result = longest(str1.as_str(), str2.as_str());
        println!("result : {}", result);
    }
}

fn str_lifetime1() {
    let str1 = String::from("str1");
    let result;
    {
        let str2 = String::from("str2");
        // 当函数需要返回传入的参数的时候，为了保证参数有效，函数必定是在生命周期最小的参数所处的代码块中，
        // 那么函数的返回值也必定是在最小生命周期的代码块中，所以用于接收返回值的变量，也必须是在最小生命周期的代码块中定义
        result = longest(str1.as_str(), str2.as_str());
    }
    // 会编译报错
    // println!("result : {}", result);
}

fn main() {
    str_lifetime();
    let novel = String::from("Call me Ishmael. Some years ago...");
    let split = novel.split('.').next().expect("Can not find a '.'");
    println!("next : {}", split);
}