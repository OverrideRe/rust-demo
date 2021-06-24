use std::{sync::mpsc, thread, time::Duration};

#[test]
fn test1() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi from main");
        tx.send(val).unwrap();
        // println!("val is {}", val); // 会报错，因为send函数会将所有权转移给接受者
    });

    // recv方法会阻塞线程，try_recv方法不会
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

/// 发送多条消息以观察接受者等待
#[test]
fn test2() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("1"),
            String::from("2"),
            String::from("3"),
            String::from("4"),
            String::from("5"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 可以迭代器便利接受者
    for received in rx {
        println!("received : {}", received);
    }
}

/// 多个线程发送消息给同一个接受者
#[test]
fn test3() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    let handle0 = thread::Builder::new().name(String::from("aaaaa")).spawn(move || {
        let handle = thread::current();
        let thread_name = match handle.name() {
            Some(name) => name,
            None => "none"
        };
        let vals = vec![
            String::from("1 from ") + thread_name,
            String::from("2 from ") + thread_name,
            String::from("3 from ") + thread_name
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let handle1 = thread::Builder::new().name(String::from("bbbbb")).spawn(move || {
        let handle = thread::current();
        let thread_name = match handle.name() {
            Some(name) => name,
            None => "none"
        };
        let vals = vec![
            String::from("1 from ") + thread_name,
            String::from("2 from ") + thread_name,
            String::from("3 from ") + thread_name
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("received : {}", received);
    }

}