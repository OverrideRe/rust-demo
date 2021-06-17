
enum Link {
    Node(i32, Rc<Link>),
    Nil,
}

use std::rc::Rc;
use crate::my_box::rc1::Link::{Node, Nil};

/// 当一个对象被多个对象拥有所有权的时候，用Rc
/// 下面a就是被b和c同时拥有所有权
/// Rc是reference counting的意思，也就是引用计数，每次clone一次引用计数就会增加一个，当引用计数为0的时候，对象才会被释放
#[test]
fn test() {
    let a = Rc::new(Node(2, 
        Rc::new(Node(3, 
            Rc::new(Node(4,
                Rc::new(Node(5, 
                    Rc::new(Nil)))))))));
    // 输出1
    println!("after a count = {}", Rc::strong_count(&a));

    let b = Node(1,Rc::clone(&a));
    // 输出2
    println!("after b count = {}", Rc::strong_count(&a));
    {
        let c = Node(0, Rc::clone(&a));
        // 输出3
        println!("after c count = {}", Rc::strong_count(&a));
    }
    // 离开上面大括号中的作用域，Rc释放了一个引用，所以输出2
    println!("finish count = {}", Rc::strong_count(&a));
}
