use std::{sync::{Arc, Mutex, mpsc::{self, Receiver, Sender}}, thread::{self, JoinHandle}};

// 定义job类型，表示提交的任务类型
type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate
}

pub struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                // 从消费者中获取任务
                let message = receiver.lock().unwrap().recv().unwrap();
                // 判断消息类型并执行对应的逻辑
                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);
                        job()
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);
                        break;
                    }
                }
            }
        });
        Worker {
            id,
            thread: Some(thread)
        }
    }
}

pub struct ThreadPool {
    sender: Sender<Message>,
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
        self.sender.send(Message::NewJob(job)).unwrap();   
    }
}

impl Drop for ThreadPool {

    fn drop(&mut self) {
        // 为什么要遍历两次而不是在一次遍历中发送终止消息和join操作？
        // 因为线程接收消息是没有顺序的，你不知道哪个线程会接收到这个消息，你当前遍历的线程有可能没接收到消息就被join了
        // 所以遍历线程发送消息不是为了给当前线程发送（没法给指定线程发送），而是为了发送线程池里线程个数的消息
        println!("Sending terminate message to all workers.");
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        // 获取线程池中线程列表的可变借用
        let worker_list = &mut self.workers;
        // 遍历可变借用
        for worker in worker_list {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) =  worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
