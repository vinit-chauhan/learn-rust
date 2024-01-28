// use rand::random; // not optimal because we can't create our own random function.
use rand::prelude::*;

pub fn exec() {
    _generate();
}

fn _generate() {
    let number: f64 = random::<f64>();
    println!("random number: {number}");

    let number: i32 = thread_rng().gen_range(1..=10);
    println!("random number: {number}");
}
