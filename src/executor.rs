use crate::tasks::Tasks;
use std::{
    future::Future,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc,
    },
};

pub struct Executor {
    sheduler: Receiver<Arc<Tasks>>,
    sender: Sender<Arc<Tasks>>,
}

impl Executor {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        Self {
            sender: tx,
            sheduler: rx,
        }
    }

    pub fn run(&self) {
        while let Ok(task) = self.sheduler.recv() {
            task.poll();
        }
    }

    pub fn spawn<F>(&self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        Tasks::spawn(future, &self.sender);
    }
}


