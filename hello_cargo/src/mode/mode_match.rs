

#[test]
fn test0() {
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("match 50"),
        Some(y) => println!("match y"),     // 会匹配这一行，因为y是在match作用域新定义的，x = Some(5) = Some(?) = Some(y)
        _ => println!("nothing")                // 结尾兜底
    }
    println!("match end x : {:?}, y : {}", x, y);


    // ..= 表示匹配闭区间内的值，只适用于数字和char类型，因为只有这两个类型编译器可以推断出是否为空
    let n = 5;
    match n {
        0 | 1 => println!("any one"),       // | 表示或的意思
        2..=5 => println!("2 - 5"),         // 匹配一个闭区间的值，表示匹配 2、3、4、5这些值
        _ => println!("no match")
    }

    let n = 'a';
    match n {
        'a' => println!("a"),
        'b'..='z' => println!("b - z"),
        _ => println!("no match")
    }
}


/// 解构并分解值
mod test1 {

    struct Point {
        x: i32,
        y: i32
    }


    #[test]
    fn test0() {
        let point = Point{x: 10, y: 1};
        // 将结构体分解成结构体内的变量
        let Point {x: a, y: b} = point;
        assert_eq!(10, a);
        assert_eq!(1, b);
    }

    #[test]
    fn test1() {
        // 简写
        let point = Point {x: 4, y: 6};
        let Point {x, y} = point;
        assert_eq!(4, x);
        assert_eq!(6, y);
    }
 
    // 动态模式匹配好固定值匹配
    #[test]
    fn test2() {
        let point = Point {x: 0, y: 0};
        match point {
            Point {x: 0, y: 0} => println!("both zero"),
            Point {x, y: 0} => println!("y zero"),
            Point {x: 0, y} => println!("x zero"),
            Point {x, y} => println!("no zero")
        }
    }
}

/// 匹配枚举、匹配守卫和@绑定
mod test2 {
    enum Message {
        Quit,
        Move {x: i32, y: i32},
        Write (String),
        ChangeColor (Color)
    }

    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32)
    }

    #[test]
    fn test() {
        let message = Message::ChangeColor(Color::Hsv (2, 4, 6));
        match message {
            Message::Quit => println!("quit"),
            Message::Move { x, y } => println!("move, x: {}, y: {}", x, y),
            Message::Write(text) => println!("write : {}", text),
            Message::ChangeColor(Color::Rgb(r, g, b)) => println!("color, r: {}, g: {}, b: {}", r, g, b),
            Message::ChangeColor(Color::Hsv(h, s, v)) => println!("color, h: {}, s: {}, v: {}", h, s, v)
        }
    }

    // 匹配守卫
    #[test]
    fn test0() {
        let x = 4;
        let y = false;
        match x {
            4 | 5 | 6 if y => println!("yes"),
            7 | 8 | 9 if x > 7 => println!("great than 7"),
            _ => println!("no match")
        }
    }

    // @绑定，可以测试并保存变量值
    #[test]
    fn test1() {
        let message = Message::Move {x: 2, y: 3};
        match message {
            Message::Move {x, y: yvar@ 3..=7} => println!("match, x: {}, y: {}", x, yvar),
            Message::Move {x, y: _} => println!("aaa"),
            _ => println!("no match")
        }
    }
}