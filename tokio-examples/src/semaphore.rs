use std::sync::Arc;

use tokio::sync::Semaphore;
use tokio::time::{sleep, Duration};

async fn person(semaphore: Arc<Semaphore>, name: String) {
    println!("{name} is waiting.");
    teller(semaphore, name).await
}

async fn teller(semaphore: Arc<Semaphore>, customer: String) {
    let permit = semaphore.acquire().await.unwrap();

    sleep(Duration::from_secs(2)).await;

    println!("\n{customer} is being served by teller");

    sleep(Duration::from_secs(5)).await;

    println!("\n{customer} is now leaving.");

    drop(permit);
}

#[tokio::main]
pub async fn exec() {
    let num_teller = 4;
    let semaphore = Semaphore::new(num_teller);
    let semaphore_arc = Arc::new(semaphore);

    let mut people_handles = Vec::new();

    for num in 0..10 {
        people_handles.push(tokio::spawn(person(
            semaphore_arc.clone(),
            format!("person {num}"),
        )))
    }

    for handle in people_handles {
        handle.await.unwrap();
    }
}
