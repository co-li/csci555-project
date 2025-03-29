/*
Baseline Rust Web Server

Reference:
S. Klabnik and C. Nichols, The Rust Programming Language, 2nd Edition, 2nd Edition. New York: No Starch Press, 2023.
*/

use std::{
    sync::{Arc, Mutex},
    sync::mpsc::{Sender, Receiver, channel},
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    fn new(_id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    // println!("Request handled by thread #{id}");
                    job();
                }
                Err(_) => {
                    // println!("Thread {id} encounters an error");
                    break;
                }
            }
        });

        Worker {
            thread: Some(thread)
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<Sender<Job>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) : (Sender<Job>, Receiver<Job>) = channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 1..(size+1) {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
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
        drop(self.sender.take());

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}


