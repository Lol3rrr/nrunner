use nrunner;

async fn hello_word() {
    println!("Hello world");
}

fn main() {
    let (exec, spawner) = nrunner::create_runtime();

    spawner.spawn(hello_word());
    drop(spawner);

    exec.run();
}
