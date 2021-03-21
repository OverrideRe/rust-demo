fn main() {
    let mut s: String = String::from("hello world !");
    let first_word_idx = first_word_index(&s);
    // 清空之后就没有第一个字符了，但是返回的值跟字符串s不对应
    s.clear();
    println!("first word index of '{}' is {}", s, first_word_idx);

    s.push_str("hello world !");
    let first_word = first_word(&s);
    // 如果使用slices则再清空的时候会报错误，因为slices是返回一个不可变引用，所以clear的时候会编译错误
    // s.clear();
    println!("first word of '{}' is {}", s, first_word);
}

fn first_word_index(s: &str) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    s
}