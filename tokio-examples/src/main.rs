mod func;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    func::exec().await;
}
