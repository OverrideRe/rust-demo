
/// 默认泛型类型和运算符重载
mod test0 {
    use std::ops::Add;

    // 给Point实现Add，使用默认类型
    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32, 
        y: i32
    }

    // 给point实现Add trait，让Point能够使用"+"。点进Add可以发现该trait是有泛型的，不过默认泛型类型是实现者本身
    impl Add for Point {
        type Output = Point;

        fn add(self, rhs: Self) -> Self::Output {
            Point { x: self.x + rhs.x, y: self.y + rhs.y }
        }
    }


    // 更改Add实现，让add相加不同类型
    #[derive(Debug)]
    struct Millimeters(i32);
    #[derive(Debug)]
    struct Meters(i32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, rhs: Meters) -> Self::Output {
            Millimeters(self.0 + rhs.0 * 1000)
        }
    }

    #[test]
    fn test() {
        let point0 = Point {x: 3, y: 4};
        let point1 = Point {x: 4, y: 3};
        println!("point after add : {:?}", point0 + point1);

        // 毫米 + 米
        let millimeters = Millimeters(30);
        let meters = Meters(2);
        println!("millimeters after add meters : {:?}", millimeters + meters);
    }
}


/// 相同名称的方法调用如何消除歧义，分为trait实现方法（可以理解为实例方法）和关联函数（静态方法）
mod test1 {
    // 实例方法同名如何指定
    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Human {
        fn fly(&self) {
            println!("fly human");
        }
    }

    impl Pilot for Human {
        fn fly(&self) {
            println!("fly pilot");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("fly wizard");
        }
    }


    #[test]
    fn test0() {
        // 由于可以看作是类实例方法，所以方法第一个参数都是实例本身，那么调用的时候传实例就能指定
        let human = Human;
        Pilot::fly(&human);
        Wizard::fly(&human);
        human.fly();
    }


    // 关联函数同名如何指定
    trait Animal {
        fn by_name();
    }

    struct Dog;


    impl Dog {
        fn by_name() {
            println!("dog by_name");
        }
    }

    impl Animal for Dog {
        fn by_name() {
            println!("animal by_name");
        }
    }

    #[test]
    fn test1() {
        Dog::by_name();
        // 完全限定语法调用，意思是调用 Animal中by_name方法在Dog中的实现
        <Dog as Animal>::by_name();
    }
}

/// 如果定义一个trait里面某个方法需要使用其它trait方法
mod test2 {
    use std::fmt::Display;

    // 表明如果需要实现OutlinePrint，则必须实现Display，因为OutlinePrint需要Display
    trait OutlinePrint: Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    struct Point {
        x: i32, 
        y: i32
    }

    impl Display for Point {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    // 如果没有实现Display则会编译报错，提示没有实现Display
    impl OutlinePrint for Point {
        
    }

    #[test]
    fn test() {
        let point = Point {x:50, y: 50};
        point.outline_print();
    }
}


/// 使用newtype模式避开孤儿规则（使用元祖结构体封装代理）。孤儿规则指的是trait和trait必须有一个是当前crate里的
mod test3 {
    use std::{fmt::Display, ops::{Deref, DerefMut}};

    // 定义一个Vec的封装类型结构体
    struct VecWrapper(Vec<String>);

    // 然后在这个封装类型上实现Display
    impl Display for VecWrapper {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "[{}]", self.0.join(","))
        }
    }

    // 为VecWrapper实现Deref以返回VecWrapper封装的类型
    impl Deref for VecWrapper {
        type Target = Vec<String>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    // 为VecWrapper实现DerefMut以返回VecWrapper封装的类型，并能够调用vec更改内部数据的方法
    impl DerefMut for VecWrapper {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    #[test]
    fn test() {
        let mut v = VecWrapper(vec![String::from("0"), String::from("1"), String::from("2")]);
        println!("v0 : {}", v);
        v.pop();
        println!("v1 : {}", v);
    }
}