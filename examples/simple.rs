use std::{future::Future, time::{Duration, Instant}};


struct Delay {
    start: Instant,
    wait: u8,
}

impl Future for Delay {
    type Output = String;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let current = Instant::now();
        let since = current.duration_since(self.start).as_secs() as u8;

        if since >= self.wait {
            return std::task::Poll::Ready("world".to_string())
        }

        cx.waker().wake_by_ref();

        std::task::Poll::Pending
    }
}

fn main() {
  
    let executor = tobio::executor::Executor::new();

    executor.spawn(async {
        let future = Delay {wait: 2, start: Instant::now()};
        let str = future.await;
        println!("{str}");
    });

    executor.run();
}