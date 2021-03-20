use crate::Task;

use futures::task::waker_ref;
use std::{
    sync::{mpsc::Receiver, Arc},
    task::{Context, Poll},
};

pub struct Executor {
    pub(crate) queue: Receiver<Arc<Task>>,
}

impl Executor {
    pub fn run(&self) {
        while let Ok(task) = self.queue.recv() {
            let mut future = task.future.lock().unwrap();

            let waker = waker_ref(&task);
            let context = &mut Context::from_waker(&*waker);

            if let Poll::Pending = future.as_mut().poll(context) {}
        }
    }
}
