fn main() {
    // let mut user = build_user1(String::from("yinghao.re@gmai.com"), String::from("yinghao"));
    let mut user = build_user2(String::from("yinghao.re@gmai.com"), String::from("yinghao"));
    println!("user name is : {}", user.user_name);
    user.user_name = String::from("Jiang Yinghao");
    println!("user name is : {}", user.user_name);

    // user的user_name和email所有权给了user1，所以user2里面如果用user的user_name和email会报值已经被移走的错误
    let user1 = User {
        user_name: user.user_name,
        email: user.email,
        age: 30,
        active: false
    };
    println!("user age is : {}", user1.age);

    let user2 = User {
        user_name: user1.user_name,
        email: user1.email,
        ..user
    };
    println!("user age is : {}", user2.age);
}

fn build_user() -> User {
    User {
        user_name: String::from("yinghao"),
        age: 26,
        email: String::from("yinghao.re@gmail.com"),
        active: true,
    }
}

fn build_user1(email:String, user_name:String) -> User {
    User {
        user_name: user_name,
        age: 26,
        email: email,
        active: true,
    }
}

// 如果字段与参数同名，可以省略赋值语句
fn build_user2(email:String, user_name:String) -> User {
    User {
        user_name,
        age: 26,
        email,
        active: true,
    }
}

struct User {
    user_name: String,
    age: i32,
    email: String,
    active: bool,
}