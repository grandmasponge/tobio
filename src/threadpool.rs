use std::sync::Arc;

use futures::lock::Mutex;

use crate::{executor::Executor, tasks::Tasks};

pub struct ThreadPool {
    pool: Vec<Arc<Mutex<Worker>>>,
    global_tasks: Vec<Arc<Tasks>>,

}

impl ThreadPool {
    pub fn create_new(queue_size: u32, worker_size: u32) -> Self {
        assert!(queue_size > 0);
        assert!(worker_size > 0);
        let mut pool = Vec::with_capacity(worker_size as usize);
        let _ = (0..worker_size)
        .map(|_x| {
            let worker = Arc::new(Mutex::new(Worker::new(queue_size)));
            pool.push(worker);
        });

        Self {
            pool,
            global_tasks: Vec::with_capacity(queue_size as usize)
        }
    }

    pub fn allocate_task() {

    }

    pub fn spawn() {

    }
}

pub struct Worker {
    //a each worker needs its own Executor and its own grouping of tasks
    pub local_executor: Executor,
    pub local_tasks: Vec<Arc<Tasks>>,
}

impl Worker {
    pub fn new(capacity: u32) -> Self {
        let executor = Executor::new();
        Self {
            local_executor: executor,
            local_tasks: Vec::with_capacity(capacity as usize)
        }
    }

    pub fn add_task() {

    }

    pub fn run() {
         
    }
}