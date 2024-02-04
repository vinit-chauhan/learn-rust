pub struct Bag {
    items: Vec<String>,
}

impl Bag {
    pub fn new() -> Self {
        Bag { items: Vec::new() }
    }

    pub fn put(&mut self, item: String) {
        println!("Added {item} in the bag");
        self.items.push(item);
    }

    pub fn remove(&mut self) -> Option<String> {
        let item = self.items.pop();
        println!(
            "Removed {} from bag",
            item.clone().unwrap_or("No Item".to_string())
        );

        item
    }
}
