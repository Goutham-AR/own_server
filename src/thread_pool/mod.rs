use std::{thread, sync::{mpsc, Arc, Mutex}};

type Job = Box<dyn FnOnce() + Send + 'static>;
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(n: usize) -> Self {
        assert!(n > 0);
        
        let (sender, reciever) = mpsc::channel();
        let reciever = Arc::new(Mutex::new(reciever));
        let mut workers = Vec::with_capacity(n);

        for id in 0..n {
            workers.push(Worker::new(id, Arc::clone(&reciever)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let job = reciever.lock().unwrap().recv().unwrap();
            println!("Worker {id} got a job executing.");
            job();
        });
        Worker { id, thread }
    }
}
