use std::sync::Arc;

use tokio::sync::Notify;
use tokio::time::{sleep, Duration};

async fn order_package(package_delivered: Arc<Notify>) {
    sleep(Duration::from_secs(2)).await;
    println!("Find package in warehouse");

    sleep(Duration::from_secs(2)).await;
    println!("Package shipped.");

    sleep(Duration::from_secs(2)).await;
    println!("Package Delivered.");
    package_delivered.notify_one();
}

async fn grab_package(package_delivered: Arc<Notify>) {
    println!("Ordered package...");

    package_delivered.notified().await;
    println!("Checking outside for package");
    sleep(Duration::from_secs(2)).await;

    println!("Grab package");
}

#[tokio::main]
pub async fn exec() {
    let package_delivered = Notify::new();
    let package_delivered_arc = Arc::new(package_delivered);

    let order_package_handle = tokio::spawn(order_package(package_delivered_arc.clone()));

    let grab_package_handle = tokio::spawn(grab_package(package_delivered_arc.clone()));

    grab_package_handle.await.unwrap();
    order_package_handle.await.unwrap();
}
