use std::{sync::{mpsc, Arc, Mutex}, thread};

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker{
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker{
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker{
        let thread = thread::spawn( move || loop {
           let message = receiver.lock().unwrap().recv();
           match message {
            Ok(job) => {
                println!("Worker {} got a job; executing.", id);
                job();
            }
            Err(_) => {
                println!("Worker {} disconnected; shutting down.", id);
                break;
            }
           }
        });
        Worker{id, thread: Some(thread)}
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let mut workers = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        
        ThreadPool{workers, sender: Some(sender)}
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
       
        self.sender.as_ref().unwrap().send(job).unwrap();
        println!("Sent job to worker");
    }
}

impl Drop for ThreadPool{
    fn drop(&mut self) {
        println!("ThreadPool is being dropped - shutting down all workers...");
        
        // Close the sender to signal workers to shut down
        drop(self.sender.take());
        
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
                println!("Worker {} has been shut down", worker.id);
            }
        }
        
        println!("All workers have been shut down");
    }
}