use std::{
    clone,
    future::Future,
    pin::Pin,
    sync::{mpsc::Sender, Arc},
    task::{Context, Poll},
};

use futures::{lock::Mutex, task::ArcWake};

pub struct Tasks {
    sender: Sender<Arc<Tasks>>,
    future: Mutex<ToBePolled>,
}

struct ToBePolled {
    future: Pin<Box<dyn Future<Output = ()> + Send>>,
    poll: Poll<()>,
}

impl ToBePolled {
    pub fn new(future: impl Future<Output = ()> + Send + 'static) -> Self {
        let pinned_future = Box::pin(future);

        Self {
            future: pinned_future,
            poll: Poll::Pending,
        }
    }

    pub fn poll(&mut self, cx: &mut Context<'_>) {
        if self.poll.is_pending() {
            self.poll = self.future.as_mut().poll(cx);
        }
    }
}

impl Tasks {
    pub fn spawn<F>(future: F, sender: &Sender<Arc<Tasks>>)
    where F: Future<Output = ()> + Send + 'static,
    {
        let tasks = Arc::new(Tasks {
            sender: sender.clone(),
            future: Mutex::new(ToBePolled::new(future)),
        });

        let _ = sender.send(tasks);
    }

    pub fn shedule(self: &Arc<Self>) {
        self.sender.send(self.clone()).unwrap();
    }

    pub fn poll(self: Arc<Self>) {
        let waker = futures::task::waker(self.clone());
        let mut context = Context::from_waker(&waker);

        let mut task = self.future.try_lock().unwrap();
        let _ = task.poll(&mut context);
    }
}

impl ArcWake for Tasks {
    fn wake_by_ref(arc_self: &std::sync::Arc<Self>) {
        arc_self.shedule();
    }
}
