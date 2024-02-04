use quality_of_life::Bag;

pub fn exec() {
    let mut my_bag = Bag::new();

    let items = vec![
        "Pen".to_string(),
        "NoteBook".to_string(),
        "Charger".to_string(),
    ];

    for item in items {
        my_bag.put(item);
    }
}
