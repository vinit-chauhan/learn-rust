use tokio::sync::watch::{self, Receiver, Sender};
use tokio::task::JoinHandle;

#[derive(Debug)]
struct Config {
    name: String,
}

impl Clone for Config {
    fn clone(&self) -> Self {
        Config {
            name: self.name.clone(),
        }
    }
}

impl Config {
    fn change_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    fn new(name: String) -> Config {
        Config { name }
    }
}

async fn check_status(mut rx: Receiver<Config>) {
    while rx.changed().await.is_ok() {
        let new_config: Config = rx.borrow().clone();
        println!("new config is: {:#?}", new_config.name);
    }
}

#[tokio::main]
pub async fn exec() {
    // Use a mutable reference to Config
    let mut config = Config::new("db_path".to_string());

    // Create the watch channel with the reference
    let (tx, rx_1): (Sender<Config>, Receiver<Config>) = watch::channel(config.clone());

    // Subscribe multiple receivers
    let rx_2: Receiver<Config> = tx.subscribe();
    let rx_3: Receiver<Config> = tx.subscribe();

    // Spawn tasks for status checks
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    handles.push(tokio::spawn(check_status(rx_1)));
    handles.push(tokio::spawn(check_status(rx_2)));
    handles.push(tokio::spawn(check_status(rx_3)));

    // Modify the config struct directly
    config.change_name("new_db_path".to_string());

    // Send updated config to the channel (use clone to avoid move)
    tx.send(config.clone()).unwrap();

    config.change_name("new_db_path_2".to_string());
    tx.send(config.clone()).unwrap();

    // Drop the transmitter to signal end of updates
    drop(tx);

    // Wait for tasks to finish
    for handle in handles {
        handle.await.unwrap();
    }
}
