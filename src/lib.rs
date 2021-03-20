use std::{future::Future, sync::mpsc::sync_channel};

mod task;
pub(crate) use task::Task;

mod executor;
pub use executor::Executor;

mod spawner;
pub use spawner::Spawner;

pub mod net;
pub mod util;

type BoxedFuture = std::pin::Pin<Box<dyn Future<Output = ()> + 'static + Send>>;

pub fn create_runtime() -> (Executor, Spawner) {
    const MAX_TASKS: usize = 1000;

    let (sender, queue) = sync_channel(MAX_TASKS);
    (Executor { queue }, Spawner { sender })
}
