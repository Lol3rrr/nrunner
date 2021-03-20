use crate::Task;

use std::{
    future::Future,
    sync::{mpsc::SyncSender, Arc},
};

pub struct Spawner {
    pub(crate) sender: SyncSender<Arc<Task>>,
}

impl Spawner {
    pub fn spawn(&self, fut: impl Future<Output = ()> + 'static + Send) {
        let task = Arc::new(Task::new(fut, self.sender.clone()));

        self.sender.send(task).expect("Too many queued up tasks");
    }
}
