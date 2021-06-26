
/// 定义绘制行为trait
trait Draw {
    fn draw(&self) {
        println!("绘制")
    }
}

struct Button {
    width: i32,
    height: i32
}

impl Draw for Button {
    fn draw(&self) {
        println!("绘制button")
    }
}

struct SelectBox {
    width: i32,
    height: i32
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("绘制selectBox")
    }
}

mod ScreenTest0 {

    use crate::object::gui::{Draw, Button, SelectBox};

    /// 定义屏幕结构体，里面的components包含了需要绘制的对象
    struct Screen<T: Draw> {
        components: Vec<T>
    }

    impl <T> Screen<T> 
        where T: Draw {
        fn run(&self) {
            println!("屏幕开始初始化");
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    #[test]
    fn test0() {
        // Screen里面的components只能放同一种类型的，比如说只能放Button或者SelectBox
        let screen = Screen {
            components: vec![
                Button {
                    width: 20,
                    height: 20
                },
                Button {
                    width: 10,
                    height: 10
                }
            ]
        };
        screen.run();
    }
}


mod ScreenTest1 {

    use crate::object::gui::{Draw, Button, SelectBox};

    /// 定义屏幕结构体，里面的components包含了需要绘制的对象
    struct Screen {
        components: Vec<Box<dyn Draw>>
    }

    impl Screen {
        fn run(&self) {
            println!("屏幕开始初始化");
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    #[test]
    fn test0() {
        // dyn修饰之后就可以往里面添加任何Draw trait的实现，如果在有继承功能的面向对象语言找那个，只需要泛型设置为共同父类就行了
        // 编译器无法知晓所有实现了Draw trait是否都会添加到列表中，所以在编译器期间是无法确定具体类型的，需要等到运行期间才可以，所以编译期间能进行的一些优化就没法做了
        let screen = Screen {
            components: vec![
                Box::new(Button {
                    width: 20,
                    height: 20
                }),
                Box::new(Button {
                    width: 10,
                    height: 10
                }),
                Box::new(SelectBox {
                    width: 100,
                    height: 100
                }),
                // Box::new(String::from("aaa")) // String类型没有实现Draw，所以会报错
            ]
        };
        screen.run();
    }
}