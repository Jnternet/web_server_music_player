pub mod 请求处理 {
    use super::线程池::ThreadPool;
    use crate::data_manage::config::Config;
    use crate::pause;
    // use std::io::BufReader;
    use std::fs::File;
    use std::path::Path;
    use std::sync::Arc;
    use std::{
        // fs,
        io::prelude::*,
        net::{TcpListener, TcpStream},
    };

    pub fn 监听端口等待并处理任务(path: Arc<Path>, config: &Config) {
        let addr = format!("{}:{}", config.ip(), config.proxy());
        let listener = loop {
            let listener = TcpListener::bind(&addr);
            let listener = match listener {
                Ok(lis) => lis,
                Err(e) => {
                    eprintln!("未能成功绑定: {e}");
                    eprintln!("请保证错误排除后重试");
                    pause();
                    continue;
                }
            };
            break listener;
        };
        eprintln!("启动成功");
        let pool = ThreadPool::new(4);

        match config.times() {
            None => {
                for stream in listener.incoming() {
                    let stream = stream.unwrap();
                    let p = path.clone();
                    pool.execute(move || {
                        handle_connection(stream, p);
                    });
                }
            }
            Some(n) => {
                for stream in listener.incoming().take(n) {
                    let stream = stream.unwrap();
                    let p = path.clone();
                    pool.execute(move || {
                        handle_connection(stream, p);
                    });
                }
            }
        };
    }

    fn handle_connection(mut stream: TcpStream, path: Arc<Path>) {
        // let _ = BufReader::new(&mut stream).lines().next().unwrap().unwrap();

        // let status_line = "HTTP/1.1 200 OK";
        // let contents = fs::read_to_string(path).unwrap();
        // let music = BufReader::new(File::open("2.wav").unwrap());
        let mut buf = Vec::new();
        let a = File::open(path).unwrap().read_to_end(&mut buf).unwrap();
        eprintln!("len = {a}");
        // let length = music.capacity();

        // let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{}");

        stream.write_all(&buf).unwrap();
    }
}

mod 线程池 {
    use std::sync::{mpsc, Arc, Mutex};
    use std::thread;

    pub struct ThreadPool {
        workers: Vec<Worker>,
        sender: Option<mpsc::Sender<Job>>,
    }

    impl ThreadPool {
        pub fn new(size: usize) -> ThreadPool {
            assert!(size > 0);

            let mut workers = Vec::with_capacity(size);
            let (sender, receiver) = mpsc::channel();
            let receiver = Arc::new(Mutex::new(receiver));

            for id in 0..size {
                workers.push(Worker::new(id, receiver.clone()))
            }

            ThreadPool {
                workers,
                sender: Some(sender),
            }
        }

        pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
        {
            let job = Box::new(f);

            self.sender.as_ref().unwrap().send(job).unwrap();
        }
    }

    impl Drop for ThreadPool {
        fn drop(&mut self) {
            eprintln!("关闭服务中...");

            drop(self.sender.take());

            for worker in &mut self.workers {
                eprintln!("正在关闭{}号线程工人", worker.id);

                if let Some(thread) = worker.thread.take() {
                    thread.join().unwrap();
                }
            }
            eprintln!("服务器已关闭!");
        }
    }

    struct Worker {
        id: usize,
        thread: Option<thread::JoinHandle<()>>,
    }

    impl Worker {
        fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
            let thread = thread::spawn(move || loop {
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        eprintln!("线程工人 {id} 得到任务, 执行中...");
                        job();
                    }
                    Err(e) => {
                        eprintln!("线程工人 {id} 失联; 原因: {e}");
                        break;
                    }
                }
            });

            Worker {
                id,
                thread: Some(thread),
            }
        }
    }

    type Job = Box<dyn FnOnce() + Send + 'static>;
}
