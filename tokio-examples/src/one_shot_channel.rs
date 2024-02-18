use tokio::sync::oneshot::{channel, Receiver, Sender};

async fn double_agent(mut rx1: Receiver<String>, mut rx2: Receiver<String>) -> String {
    let msg: String = tokio::select! {
        msg_1 = &mut rx1 => msg_1.unwrap(),
        msg_2 = &mut rx2 => msg_2.unwrap()
    };

    println!("message: {msg}");

    msg
}

#[tokio::main]
pub async fn exec() {
    let (tx1, rx1): (Sender<String>, Receiver<String>) = channel::<String>();
    let (tx2, rx2): (Sender<String>, Receiver<String>) = channel::<String>();

    let _ = tx1.send("Hey RX1".to_string());
    let _ = tx2.send("Hey RX2".to_string());

    double_agent(rx1, rx2).await;
}
