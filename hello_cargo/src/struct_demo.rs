fn main() {
    let mut user = build_user();
    println!("user name is : {}", user.user_name);
    user.user_name = String::from("Jiang Yinghao");
    println!("user name is : {}", user.user_name);
}

fn build_user() -> User {
    User {
        user_name: String::from("yinghao"),
        age: 26,
        email: String::from("yinghao.re@gmail.com"),
        active: true,
    }
}

struct User {
    user_name: String,
    age: i32,
    email: String,
    active: bool,
}