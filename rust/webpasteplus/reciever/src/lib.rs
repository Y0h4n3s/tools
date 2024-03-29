#![recursion_limit = "256"]
#![allow(dead_code)]

#[macro_use]
extern crate diesel;

use std::sync::Arc;
use std::sync::mpsc;
use std::sync::Mutex;
use std::thread;

pub mod separator;
pub mod paster;
pub mod argparser;
pub mod dbwriter;
pub mod schema;
mod dbmodels;

pub struct ThreadPool {
    threads: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, reciever) = mpsc::channel();
        let reciever = Arc::new(Mutex::new(reciever));

        let mut threads = Vec::with_capacity(size);

        for i in 0..size {
            threads.push(Worker::new(i, Arc::clone(&reciever)));
        }
        ThreadPool { threads, sender }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

struct Worker {
    work: Option<thread::JoinHandle<()>>,
    id: usize,
}

impl Worker {
    fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let work = thread::spawn(move || loop {
            let job = reciever.lock().unwrap().recv().unwrap();
            println!("executing in {}", id);
            job();
        });

        Worker {
            work: Some(work),
            id: id,
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.threads {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.work.take() {
                thread.join().unwrap();
            }
        }
    }
}


