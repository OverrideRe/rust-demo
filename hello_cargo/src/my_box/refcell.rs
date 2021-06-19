

mod refcell_test {
    use std::cell::RefCell;

    trait Storage {
        fn send(&self, str: &str);
    }


    struct Storage0 {
        list: Vec<String>
    }
    
    /// 当send中的self是不可变引用时，下面的实现会报错，因为调用了self的list更改值
    /// 当send中的self是可变引用时，UserStorage中的use_storage会编译报错，因为函数中的self也是不可变引用
    // impl Storage for Storage0 {
    //     fn send(&self, str: &str) {
    //         self.list.push(str.to_string());
    //     }
    // }


    struct Storage1 {
        list: RefCell<Vec<String>>
    }

    /// 第二种方式使用了RefCell，RefCell可以实现内部可变，borrow_mut就是获取RefCell修饰对象的可变借用
    /// RefCell只允许多个不可变借用和一个可变借用，所以如果连续调用多次borrow_mut虽然不会报编译错误，但是运行时会报错
    impl Storage for Storage1 {
        fn send(&self, str: &str) {
            self.list.borrow_mut().push(str.to_string());
        }
    }

    /// 生命周期是约束借用的泛型使用，借用需要使用生命周期，如果直接赋予所有权，那么无需生命周期也能使用泛型
    struct UserStorage<'a, T: Storage> {
        storage: &'a T,
    }

    impl<'a,T> UserStorage<'a,T> 
        where T: Storage {

        fn new(storage: &T) -> UserStorage<T> {
            UserStorage {storage: storage}
        }

        fn use_storage(&self, str: &str) {
            self.storage.send(str);
        }
    }

    // #[test]
    // fn test_0() {
    //     let s = Storage0{list: vec![String::from("0")]};

    //     let mut use_storage = UserStorage::new(&s);
    //     use_storage.use_storage("str0");
    //     use_storage.use_storage("str1");

    //     assert_eq!(s.list.len(), 3);
    // }


    #[test]
    fn test_1() {
        let s = Storage1{list: RefCell::new(vec![String::from("0")])};

        let mut use_storage = UserStorage::new(&s);
        use_storage.use_storage("str0");
        use_storage.use_storage("str1");

        assert_eq!(s.list.borrow().len(), 3);
    }
}


/// RefCell和Rc结合使用，创造可以修改并且能拥有多个所有者的变量
#[cfg(test)]
mod refcell_rc {
    use std::{cell::RefCell, rc::Rc};

    #[derive(Debug)]
    enum Node {
        list(Rc<RefCell<i32>>, Rc<Node>),
        Nil,
    }

    use crate::my_box::refcell::refcell_rc::Node::{list, Nil};

    #[test]
    fn test() {
        let value = Rc::new(RefCell::new(3));

        let a = Rc::new(list(Rc::clone(&value), Rc::new(Nil)));

        let b = list(Rc::new(RefCell::new(2)), Rc::clone(&a));
        let c = list(Rc::new(RefCell::new(1)), Rc::clone(&a));

        *value.borrow_mut() += 10;

        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
    }
}