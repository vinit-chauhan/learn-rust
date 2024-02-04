use std::env;

pub fn exec() {
    let sum: i32 = env::args().skip(1).fold(0, |acc: i32, x: String| {
        acc + x.parse::<i32>().expect("Invalid number")
    });

    println!("Sum is {sum}")
}
