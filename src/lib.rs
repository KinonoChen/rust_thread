//test
/*明确需求和功能:
我们需要一个线程池，可以执行并发任务。
线程池需要管理一组工作线程，这些线程可以从一个共享的任务队列中获取任务并执行。
提供一个简单的接口来提交任务到线程池中执行。
确定核心组件:
ThreadPool: 管理工作线程和任务队列。
Worker: 工作线程，从任务队列中获取并执行任务。
Job: 代表需要执行的任务。
选择并发原语:
使用 Rust 的 std::sync 模块提供的原语来实现并发控制，包括 mpsc 通道、Arc 和 Mutex。 */
use std::{
    sync::{mpsc,Arc,Mutex},
    thread,
};

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Worker{
    id: usize,
    thread: thread::JoinHandle<()>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool{
    pub fn new(size: usize) -> ThreadPool{
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size{
            workers.push(Worker::new(id,Arc::clone(&receiver)));
        }
        ThreadPool{workers,sender}
    }

    pub fn execute<F>(&self,f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

impl Worker{
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker{
        let thread = thread::spawn(move || loop{
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} got a job; executing.",id);
            job();
        });
        Worker {id, thread}
    }
}


