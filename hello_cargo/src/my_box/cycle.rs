/// 链表死循环测试
mod test1 {
    use std::{cell::RefCell, rc::Rc};

    #[derive(Debug)]
    enum List {
        Node(i32, RefCell<Rc<List>>),
        Nil,
    }

    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match self {
                Node(_, item) => Some(item),
                Nil => None,
            }
        }
    }

    use crate::my_box::cycle::test1::List::{Node, Nil};

    #[test]
    fn test() {
        let a = Rc::new(Node(0, RefCell::new(Rc::new(Nil))));

        println!("a初始化引用数量 : {}", Rc::strong_count(&a));

        let b = Rc::new(Node(1, RefCell::new(Rc::clone(&a))));

        println!("b初始化之后的引用数量，a : {}, b : {}", Rc::strong_count(&a), Rc::strong_count(&b));

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }
        
        println!("最后的引用数量，a : {}, b : {}", Rc::strong_count(&a), Rc::strong_count(&b));

        // println!("a.next : {:?}", a.tail()); // 会死循环
    }
}


/// Rc<T>和Weak<T>，Weak表示弱引用，当关联到对象的强引用失效时，弱引用也会失效
#[cfg(test)]
mod test2 {
    use std::{cell::RefCell, rc::{Rc, Weak}};


    #[derive(Debug)]
    struct Node {
        value: i32,
        // 因为parent可能会被修改，所以用RefCell修饰；因为希望子节点拥有父节点的引用，但不希望拥有父节点的所有权，所以使用Weak修饰。
        parent: RefCell<Weak<Node>>,
        // 因为自节点可能会被修改，所以用RefCell修饰，因为父节点对子节点是拥有所有权的强引用，所以用Rc修饰而不是Weak
        children: RefCell<Vec<Rc<Node>>>,
    }

    #[test]
    fn test1() {
        let leaf = Rc::new(Node {
            value: 1,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!("leaf初始化时引用情况：leaf.strong_count = {}, leaf.weak_count = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

        {
            let branch = Rc::new(Node {
                value: 0,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });

            // Rc::downgrade从Rc引用中创建一个指向原对象的Weak引用
            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            println!("branch初始化时引用情况：leaf.strong_count = {}, leaf.weak_count = {}, branch.strong_count = {}, branch.weak_count = {}", 
            Rc::strong_count(&leaf), Rc::weak_count(&leaf), Rc::strong_count(&branch), Rc::weak_count(&branch));
        }


        // Weak<T> 实例的 upgrade 方法，这会返回 Option<Rc<T>>。如果 Rc<T> 值还未被丢弃，则结果是 Some；如果 Rc<T> 已被丢弃，则结果是 None
        println!("leaf.parent : {:?}", leaf.parent.borrow_mut().upgrade());
        println!("最后时刻引用情况：leaf.strong_count = {}, leaf.weak_count = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    }
}

#[cfg(test)]
mod test3 {
    use std::{cell::RefCell, rc::{Rc, Weak}};


    #[derive(Debug)]
    struct Node {
        value: i32,
        // 因为parent可能会被修改，所以用RefCell修饰；因为希望子节点拥有父节点的引用，但不希望拥有父节点的所有权，所以使用Weak修饰。
        parent: RefCell<Weak<Node>>,
        // 因为自节点可能会被修改，所以用RefCell修饰，因为父节点对子节点是拥有所有权的强引用，所以用Rc修饰而不是Weak
        children: RefCell<Vec<Rc<Node>>>,
    }

    #[test]
    fn test1() {
        let leaf = Rc::new(Node {
            value: 1,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!("leaf初始化时引用情况：leaf.strong_count = {}, leaf.weak_count = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

        let branch = Rc::new(Node {
            value: 0,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        // Rc::downgrade从Rc引用中创建一个指向原对象的Weak引用
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("branch添加为leaf父节点时引用情况：leaf.strong_count = {}, leaf.weak_count = {}, branch.strong_count = {}, branch.weak_count = {}", 
        Rc::strong_count(&leaf), Rc::weak_count(&leaf), Rc::strong_count(&branch), Rc::weak_count(&branch));

        // 删除branch子节点
        *branch.children.borrow_mut() = vec![];
        println!("branch删除leaf子节点时引用情况：leaf.strong_count = {}, leaf.weak_count = {}, branch.strong_count = {}, branch.weak_count = {}", 
        Rc::strong_count(&leaf), Rc::weak_count(&leaf), Rc::strong_count(&branch), Rc::weak_count(&branch));

        // Weak<T> 实例的 upgrade 方法，这会返回 Option<Rc<T>>。如果 Rc<T> 值还未被丢弃，则结果是 Some；如果 Rc<T> 已被丢弃，则结果是 None
        println!("leaf.parent : {:?}", leaf.parent.borrow_mut().upgrade());
        println!("最后时刻引用情况：leaf.strong_count = {}, leaf.weak_count = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    }
}
