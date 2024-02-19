use std::sync::Arc;

use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::task::JoinHandle;

async fn submit_homework(producer: Arc<Sender<String>>, id: u8) {
    producer
        .send(format!("Homework, {id}"))
        .await
        .expect("Error Submitting Homework");
}

async fn collect_homework(
    mut consumer: Receiver<String>,
    mut homeworks: Vec<String>,
) -> Vec<String> {
    while let Some(homework) = consumer.recv().await {
        println!("Collected homework: {homework}");
        homeworks.push(homework);
    }

    homeworks
}

#[tokio::main]
pub async fn exec() {
    let (producer, consumer): (Sender<String>, Receiver<String>) = mpsc::channel::<String>(10);

    let producer_arc: Arc<Sender<String>> = Arc::new(producer);
    let homeworks: Vec<String> = Vec::new();

    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    let res_handle: JoinHandle<Vec<String>> = tokio::spawn(collect_homework(consumer, homeworks));

    for id in 0u8..=10u8 {
        let handle: JoinHandle<()> = tokio::spawn(submit_homework(producer_arc.clone(), id));
        handles.push(handle);
    }

    /*
        - When you drop the producer_arc (an Arc<Sender<String>>),
            all remaining copies of the Sender lose their access to the internal channel state.
        - This triggers the underlying implementation of mpsc to shut down the channel gracefully.
        - Any further send attempts on Sender instances,
            including the original one, will result in errors.
    */
    drop(producer_arc);

    let homeworks = res_handle.await.unwrap();
    println!("Homeworks: \n {:?}", homeworks);
}
