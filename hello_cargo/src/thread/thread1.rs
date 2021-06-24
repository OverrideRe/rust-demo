use std::{thread, time::Duration};


#[test]
fn test1() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread.", i);
            thread::sleep(Duration::from_secs(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread.", i);
        thread::sleep(Duration::from_secs(1));
    }

    // 调用join方法会阻塞当前线程直到handle所代表的线程执行结束
    handle.join().unwrap();
}

/// 使用move强制闭包获取其所使用值的所有权，如果不适用move将会报错，因为vec的所有权是在主线程中的，其它线程使用它可能会因为主线程给drop掉从而产生错误引用
#[test]
fn test2() {
    let vec= vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", vec);
    });

    // drop(vec);

    handle.join().unwrap();
}