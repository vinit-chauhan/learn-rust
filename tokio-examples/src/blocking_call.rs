use std::thread;
use tokio::time::{sleep, Duration};

fn blocking_call() -> String {
    thread::sleep(Duration::from_secs(5));

    "Done".to_string()
}

async fn async_call(id: i32) {
    sleep(Duration::from_secs(1)).await;
    println!("Async call ID: {id}")
}

#[tokio::main]
pub async fn exec() {
    let blocking_call_handle = tokio::task::spawn_blocking(blocking_call);

    let mut async_handles = Vec::new();

    for id in 0..=10 {
        async_handles.push(tokio::spawn(async_call(id)));
    }

    for handle in async_handles {
        handle.await.unwrap();
    }

    let result = blocking_call_handle.await.unwrap();

    println!("Blocking call: {result}");
}
