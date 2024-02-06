use func::shout;
use tokio::task::JoinHandle;

mod func;

#[tokio::main]
async fn main() {
    let join_handler: JoinHandle<String> = tokio::spawn(shout("hey!!"));

    let val = join_handler.await.unwrap();

    dbg!(val);
}
