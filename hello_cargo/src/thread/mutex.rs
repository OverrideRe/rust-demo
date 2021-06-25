use std::{sync::{Arc, Mutex}, thread};



/// lock调用返回的值是一个MutexGuard智能指针，其实现了Deref指向内部数据，也提供了一个Drop实现，当MutexGuard来作用域时会自动释放锁
#[test]
fn test0() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
        // 离开作用域会自动释放锁
    }
    println!("m: {:?}", m);
}

/// Arc和Rc类似，都是给一个对象提供多个所有者引用，不同的是Arc是线程安全的
#[test]
fn test1() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        // 创建新的引用
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // counter是不可变的，但是却可以改变其内部值，所以其具有内部可变性，跟RefCell一样
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("result : {:?}", *counter.lock().unwrap());
}