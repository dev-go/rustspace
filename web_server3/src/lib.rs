use std::borrow::Borrow;

pub struct ThreadPool {
    workers: Vec<(
        usize,
        std::thread::ThreadId,
        Option<std::thread::JoinHandle<()>>,
        Option<std::sync::mpsc::Sender<()>>,
    )>,
    sender: std::sync::mpsc::Sender<Box<dyn FnOnce() + Send + 'static>>,
    receiver: std::sync::Arc<
        std::sync::Mutex<std::sync::mpsc::Receiver<Box<dyn FnOnce() + Send + 'static>>>,
    >,
}

impl ThreadPool {
    pub fn new(mut n: usize) -> ThreadPool {
        if n <= 0 || n > 1024 * 1024 {
            let cpu_cores = num_cpus::get();
            n = cpu_cores;
        }
        let (sender, receiver) = std::sync::mpsc::channel::<Box<dyn FnOnce() + Send + 'static>>();
        let receiver = std::sync::Arc::new(std::sync::Mutex::new(receiver));
        let mut workers = vec![];
        for id in 1..=n {
            let receiver = receiver.clone();
            let (quit_sender, quit_receiver) = std::sync::mpsc::channel::<()>();
            let handle = std::thread::spawn(move || {
                let thread_id = std::thread::current().id();
                let mut job_id = 0;
                loop {
                    println!("[THREAD_POOL] worker_id={id}, thread_id={thread_id:?} loop 1");
                    let receiver = receiver.try_lock();
                    if let Result::Ok(receiver) = receiver {
                        let job = receiver.try_recv();
                        if let Result::Ok(job) = job {
                            job_id += 1;
                            println!(
                                    "[THREAD_POOL] worker_id={id}, thread_id={thread_id:?} get new job: job_id={job_id}"
                                );
                            job();
                            println!(
                                    "[THREAD_POOL] worker_id={id}, thread_id={thread_id:?} run job finished: job_id={job_id}"
                                );
                        }
                    }

                    println!("[THREAD_POOL] worker_id={id}, thread_id={thread_id:?} loop 2");
                    if let Result::Ok(_) = quit_receiver.try_recv() {
                        println!(
                            "[THREAD_POOL] worker_id={id}, thread_id={thread_id:?} thread quit: job_count={job_id}"
                        );
                        return;
                    }

                    println!("[THREAD_POOL] worker_id={id}, thread_id={thread_id:?} loop 3");
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
            });
            let thread_id = handle.thread().id();
            println!("[THREAD_POOL] create new worker: worker_id={id}, thread_id={thread_id:?}");

            workers.push((
                id,
                thread_id,
                Option::Some(handle),
                Option::Some(quit_sender),
            ));
        }
        ThreadPool {
            workers: workers,
            sender: sender,
            receiver: receiver,
        }
    }

    pub fn run<F: FnOnce() + Send + 'static>(&mut self, job: F) {
        self.sender.send(Box::new(job));
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("ThreadPool: drop >>>");
        for work in &mut self.workers {
            println!("ThreadPool: drop: id={}", work.0);
            let quit_sender = (work.3).take().unwrap();
            quit_sender.send(());
            (work.2).take().unwrap().join();
            drop(quit_sender);
        }
        drop(&self.sender);
        println!("ThreadPool: drop <<<");
    }
}
