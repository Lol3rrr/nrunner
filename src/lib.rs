use futures::task::{waker_ref, ArcWake};
use std::{
    future::Future,
    sync::{
        mpsc::{sync_channel, Receiver, SyncSender},
        Arc, Mutex,
    },
    task::Context,
    task::Poll,
};

type BoxedFuture = std::pin::Pin<Box<dyn Future<Output = ()> + 'static + Send>>;

struct Task {
    future: Mutex<BoxedFuture>,
    task_sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self
            .task_sender
            .send(cloned)
            .expect("Too many queued up tasks");
    }
}

struct Spawner {
    sender: SyncSender<Arc<Task>>,
}

impl Spawner {
    pub fn spawn(&self, fut: impl Future<Output = ()> + 'static + Send) {
        let future = Box::pin(fut);
        let task = Arc::new(Task {
            future: Mutex::new(future),
            task_sender: self.sender.clone(),
        });
        self.sender.send(task).expect("Too many queued up tasks");
    }
}

struct Executor {
    queue: Receiver<Arc<Task>>,
}

impl Executor {
    pub fn run(&self) {
        while let Ok(task) = self.queue.recv() {
            let mut future = task.future.lock().unwrap();

            let waker = waker_ref(&task);
            let context = &mut Context::from_waker(&*waker);

            if let Poll::Pending = future.as_mut().poll(context) {
                println!("Pending Future");
            }
        }
    }
}

fn create_runtime() -> (Executor, Spawner) {
    const MAX_TASKS: usize = 1000;

    let (sender, queue) = sync_channel(MAX_TASKS);
    (Executor { queue }, Spawner { sender })
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
