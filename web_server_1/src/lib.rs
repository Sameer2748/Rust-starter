use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// Create a new ThreadPool with `size` worker threads.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0, "Size must be at least 1");

        // Create the channel
        let (sender, receiver) = mpsc::channel::<Job>();
        // Wrap receiver in Arc<Mutex<...>> so workers can share it safely
        let receiver = Arc::new(Mutex::new(receiver));

        // Spawn workers
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    /// Send a closure to be executed by the pool.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f) as Job;
        self.sender.send(job).unwrap();
    }
}

// Each worker owns its own thread and an ID for logging
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // Block until a job arrives, then run it
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} got a job; executing.", id);
            job();
        });

        Worker { id, thread }
    }
}
