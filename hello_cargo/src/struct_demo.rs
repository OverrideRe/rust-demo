fn main() {
    let user = User {
        user_name: String::from("yinghao"),
        age: 26,
        email: String::from("yinghao.re@gmail.com"),
        active: true,
    };
    println!("user name is : {}", user.user_name);
}

struct User {
    user_name: String,
    age: i32,
    email: String,
    active: bool,
}