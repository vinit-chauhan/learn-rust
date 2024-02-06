pub async fn exec() {
    for i in 0..=100_000_000 {
        if i % 100_000 == 0 {
            println!("task {} completed", i / 100_000);
        }
    }
    println!("whole task done");
}

pub async fn shout(msg: &str) -> String {
    msg.to_uppercase()
}
