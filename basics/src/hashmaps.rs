use std::collections::HashMap;

pub fn exec() {
    let mut map = HashMap::new();
    map.insert("Vinit", true);
    map.insert("Matt", true);
    map.insert("Linda", false);
    map.insert("Susan", false);

    println!("map: {map:?}");

    let is_matt_available: &bool = map.get("Matt").unwrap();
    println!("Is Matt available: {}", is_matt_available);

    // overwrite values.
    map.insert("Matt", false);

    // add new kv pair
    map.insert("Martin", false);

    // add new kv pair
    // if the key does not exist, create one with default value.
    map.entry("Matt").or_insert(true);
    map.entry("Charlie").or_insert(true);

    let taylor: &mut bool = map.entry("Taylor").or_insert(true);
    *taylor = true; // update the value in the map.

    println!("map: {map:?}");
}
