use crate::BoxedFuture;

use futures::{task::ArcWake, Future};
use std::sync::{mpsc::SyncSender, Arc, Mutex};

pub struct Task {
    pub(crate) future: Mutex<BoxedFuture>,
    task_sender: SyncSender<Arc<Task>>,
}

impl Task {
    pub fn new(
        fut: impl Future<Output = ()> + 'static + Send,
        channel: SyncSender<Arc<Task>>,
    ) -> Self {
        Self {
            future: Mutex::new(Box::pin(fut)),
            task_sender: channel,
        }
    }
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
