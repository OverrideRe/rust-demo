
use std::collections::HashMap;

fn vec_test() {
    let mut vector: Vec<i32> = Vec::new();
    vector.push(1);
    println!("vector length : {}", vector.len());
    let mut v = vec![1, 2, 3, 4, 5];
    let v0: i32 = v[0];
    println!("v[0] : {}", v0);

    // v.push(6); // 会报错，因为上面v0是不可变引用，push元素可能会导致扩容从而改变v0的地址

    let mut num: &i32 = &0;
    match v.get(1) {
        None => println!("没有值"),
        Some(value) => num = value
    }
    println!("v[1] : {}", num);

    for i in &mut v {
        *i += 50;
    }
    println!("new v[0] : {}", v[0]);
}

fn str_test() {
    println!("hello, {}", "world".to_string());

    let data = "hello";
    // 变成了String类型
    let mut s = data.to_string();
    println!("{}", s);

    let s0 = "世界";
    println!("s0 : {}", s0);
    println!("s0.len : {}", s0.len());

    // push_str只有String类型有&str类型没有，并且只接收&str类型参数
    s.push_str(" ");
    s.push_str(s0);
    // s0.push_str("!"); // 无法编译通过，因为&str类型没有该方法
    println!("s is '{}', s0 is '{}'", s, s0);

    let s1 = String::from("s1");
    let s2 = String::from("s2");
    let s3 = String::from("s3");

    // 从String借用返回&str类型，运算符只能作用于&str类型。+的本质是add方法，参数为(self, &str2)，所以会改变第一个值的所有权，但是后面的不会改变
    let ss = s1 + "-" + &s2 + "-" + &s3;
    // println!("ss : {}, s1 : {}, s2 : {}, s3 : {}", ss, s1, s2, s3);  // 编译错误，s1所有权已经被移动到add方法内部
    println!("ss : {}, s2 : {}, s3 : {}", ss, s2, s3);
    let s_format = format!("{}-{}", s2, s3,);
    println!("s_format : {}", s_format);

    // 编译不通过，rust字符串不支持索引获取，因为底层用的是Vec<u8>存储的，每个下标保存的是一个字节内容，而编码用的是UTF-8，一个字符并不一定是一个字节，所以不支持索引查找
    // 还有原因是因为索引操作预期总是需要常数时间 (O(1))。但是对于 String 不可能保证这样的性能，因为 Rust 必须从开头到索引位置遍历来确定有多少有效的字符
    // 所以len()方法返回的也是字符串的字节长度，而不是字符串的字符长度
    // let ix_str = s[0];

    println!("--------------字符串chars遍历---------------");
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    println!("--------------字符串bytes遍历---------------");
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}

fn map_test() {
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 20);
    println!("blue : {}", scores["blue"]);

    let teams = vec!["yellow", "blue"];
    let initial_scores = vec![100, 99];
    let mut team_scores: HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();
    println!("yellow team'score : {}", team_scores[&"yellow"]);
    team_scores.insert(&"blue", &100);      // 覆盖
    team_scores.entry(&"blue").or_insert(&0); // 只在没有key的时候插入
    team_scores.entry(&"red").or_insert(&0); // 只在没有key的时候插入

    println!("--------------遍历---------------");
    for (k, v) in &team_scores {
        println!("{} : {}", k, v);
    }

    println!("--------------单词计数------------");

    let text = "hello world wonderful world";
    let mut word_count = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0); // 返回一个可变引用
        * count += 1;
    }
    println!("word_count : {:?}", word_count);
}


fn main() {
    vec_test();
    println!("-------------------------------------------");
    str_test();
    println!("-------------------------------------------");
    map_test();
}