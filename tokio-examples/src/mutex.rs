use std::sync::Arc;

use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};

async fn person(remote_arc: Arc<Mutex<i32>>, name: String, new_channel: i32) {
    // request access to remote_arc
    let mut real_remote = remote_arc.lock().await;

    *real_remote = new_channel;
    sleep(Duration::from_secs(5)).await;
    println!("{name} changed the channel to {new_channel}");
    // change value of channel
}

#[tokio::main]
pub async fn exec() {
    let tv_channel = 10;
    let remote = Mutex::new(tv_channel);
    let remote_arc = Arc::new(remote);

    let mut task_handles: Vec<JoinHandle<()>> = Vec::new();

    for (name, new_channel) in [("Matt", 11), ("Jeffery", 25), ("Matt", 123)] {
        task_handles.push(tokio::spawn(person(
            remote_arc.clone(),
            name.to_string(),
            new_channel,
        )));
    }

    for handle in task_handles {
        handle.await.unwrap();
    }
}
