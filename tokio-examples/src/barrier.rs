use std::sync::Arc;

use tokio::sync::{Barrier, BarrierWaitResult, Notify};
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};

async fn barrier_example(barrier: Arc<Barrier>, notify: Arc<Notify>) -> BarrierWaitResult {
    println!("Waiting at barrier");

    let wait_result = barrier.wait().await;
    println!("Passed through  barrier");

    if wait_result.is_leader() {
        notify.notify_one();
    }

    wait_result
}

#[tokio::main]
pub async fn exec() {
    let total_cans_needed = 12;

    let barrier = Arc::new(Barrier::new(total_cans_needed));

    let notify = Arc::new(Notify::new());

    let mut task_handles: Vec<JoinHandle<BarrierWaitResult>> = Vec::new();

    // To send 1st batch of cans to barrier
    notify.notify_one();

    for can_count in 0..60 {
        if can_count % 12 == 0 {
            notify.notified().await;

            // give barrier some time to close
            sleep(Duration::from_millis(1)).await;
        }

        task_handles.push(tokio::spawn(barrier_example(
            barrier.clone(),
            notify.clone(),
        )));
    }

    let mut num_leaders = 0;

    for handle in task_handles {
        let wait_result = handle.await.unwrap();

        if wait_result.is_leader() {
            num_leaders += 1;
        }
    }

    println!("Total number of leaders: {num_leaders}");
}
