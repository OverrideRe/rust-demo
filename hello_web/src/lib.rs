use std::{sync::{Arc, Mutex, mpsc::{self, Receiver, Sender}}, thread::{self, JoinHandle}};


// 定义job类型，表示提交的任务类型
type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                // 从消费者中获取任务
                let job = receiver.lock().unwrap().recv().unwrap();
                // 执行任务
                job()
            }
        });
        Worker {
            id,
            thread
        }
    }
}

pub struct ThreadPool {
    sender: Sender<Job>,
    workers: Vec<Worker>
}

impl ThreadPool {
    pub fn new(capacity: usize) -> ThreadPool {
        assert!(capacity > 0);
        
        // 获取多线程生产者消费者
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(capacity);

        for id in 0..capacity {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        };

        ThreadPool {
            sender,
            workers
        }
    }

    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static {
        let job = Box::new(f);
        self.sender.send(job).unwrap();   
    }
}
