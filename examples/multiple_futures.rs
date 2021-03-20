use std::time::Duration;

use nrunner;

async fn first() {
    println!("First - 1");

    nrunner::util::TimerFuture::new(Duration::new(3, 0)).await;

    println!("Frist - 2");
}

async fn second() {
    println!("Second");
}

fn main() {
    let (exec, spawner) = nrunner::create_runtime();

    spawner.spawn(first());
    spawner.spawn(second());
    drop(spawner);

    exec.run();
}
