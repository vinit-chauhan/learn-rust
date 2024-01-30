use colored::Colorize;
use std::env;
use std::fs::File;
use std::io::prelude::*;

pub fn exec() {
    challenge_6();
}

pub fn challenge_2() {
    let temp: f64 = 23.0;
    assert_eq!(celsius_to_fahrenheit(temp), 73.4);
    println!("test passed")
}

fn _calculate_avg() {
    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;
    let average: f64;
    let average_f32: f32;

    average = (a as f64 + b + c as f64) / 3.0; // 45.1
    average_f32 = (a as f32 + b as f32 + c) / 3.0; // 45.100002

    // This difference is due to inaccuracy in floating point arithmetic
    // to avoid such issues, use approximate equality
    // use `approx` crate

    assert_eq!(average, 45.1);
    assert_eq!(average_f32, 45.100002);
    println!("test passed!")
}

pub fn celsius_to_fahrenheit(temperature: f64) -> f64 {
    temperature * 1.8 + 32.0
}

pub fn challenge_3() {
    let numbers: [i32; 14] = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32 = numbers[0];
    let mut min: i32 = numbers[0];
    let mut mean: f64 = 0.0;

    for num in numbers {
        max = if num > max { num } else { max };
        min = if num < min { num } else { min };
        mean += num as f64;
    }

    mean /= numbers.len() as f64;

    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);

    println!("test passed!");
}

pub fn challenge_4() {
    fn trim_spaces(s: &str) -> &str {
        let mut front: usize = 0;
        let mut back: usize = s.len();

        for c in s.chars() {
            if c == ' ' {
                front += 1;
            } else {
                break;
            }
        }

        for c in s.chars().rev() {
            if c == ' ' {
                back -= 1;
            } else {
                break;
            }
        }

        &s[front..back]
    }

    let string: String = String::from("  Hello World.");

    assert_eq!(trim_spaces(&string), "Hello World.");
    println!("test passed!");
}

pub fn challenge_5() {
    if env::args().len() != 3 {
        panic!("Invalid Argument count")
    }

    let file_path: String = env::args().nth(1).unwrap();
    let name: String = env::args().nth(2).unwrap();

    let mut fd: File = File::open(file_path).expect("Unable to open file.");
    let mut buff: String = String::new();

    let _ = fd.read_to_string(&mut buff).expect("Unable to read file.");

    for line in buff.lines() {
        if name == line {
            println!("{} {}", name.green(), "is Invited.".green());
            return;
        }
    }
    println!("{} {}", name.red(), "is not Invited.".red());
}

pub fn challenge_6() {
    struct Rectangle {
        width: f64,
        height: f64,
    }

    impl Rectangle {
        fn area(&self) -> f64 {
            self.width * self.height
        }

        fn scale(&mut self, factor: f64) {
            self.width *= factor;
            self.height *= factor;
        }

        fn new(height: f64, width: f64) -> Rectangle {
            Rectangle {
                height: height,
                width: width,
            }
        }
    }

    let mut rect: Rectangle = Rectangle::new(1.2, 3.4);
    assert_eq!(rect.area(), 4.08);
    rect.scale(0.5);
    assert_eq!(rect.area(), 1.02);
    println!("{}", "test passed".green())
}
