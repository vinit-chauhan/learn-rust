use std::sync::Arc;

use tokio::{
    sync::broadcast::{self, Receiver, Sender},
    task::JoinHandle,
};

async fn read_value(mut rx: Receiver<String>) {
    while let Ok(value) = rx.recv().await {
        println!("Value is : {value}");
    }
}

#[tokio::main]
pub async fn exec() {
    let (tx, _rx) = broadcast::channel::<String>(128);
    let tx_arc: Arc<Sender<String>> = Arc::new(tx);
    let tx_1: Arc<Sender<String>> = tx_arc.clone();
    let tx_2: Arc<Sender<String>> = tx_arc.clone();

    let rx_1: Receiver<String> = tx_1.subscribe();
    let rx_2: Receiver<String> = tx_2.subscribe();
    let rx_3: Receiver<String> = tx_1.subscribe();

    tx_1.send("value 1".to_string()).unwrap();
    tx_2.send("value 2".to_string()).unwrap();
    tx_1.send("value 3".to_string()).unwrap();

    let mut handles: Vec<JoinHandle<()>> = Vec::<JoinHandle<()>>::new();

    handles.push(tokio::spawn(read_value(rx_1)));
    handles.push(tokio::spawn(read_value(rx_2)));
    handles.push(tokio::spawn(read_value(rx_3)));

    drop(tx_arc);
    drop(tx_1);
    drop(tx_2);

    for handle in handles {
        handle.await.unwrap();
    }
}
