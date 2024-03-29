use colored::Colorize;
use std::env;
use std::fmt;
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;

pub fn exec() {
    challenge_9();
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

fn sum_boxes<T: std::ops::Add<Output = T>>(item1: Box<T>, item2: Box<T>) -> Box<T> {
    Box::new(*item1 + *item2)
}

pub fn challenge_7() {
    let box1: Box<i32> = Box::new(1);
    let box2: Box<i32> = Box::new(2);
    assert_eq!(*sum_boxes(box1, box2), 3);

    let box1: Box<f64> = Box::new(12_f64);
    let box2: Box<f64> = Box::new(2_f64);
    assert_eq!(*sum_boxes(box1, box2), 14_f64);

    let box1: Box<i8> = Box::new(1);
    let box2: Box<i8> = Box::new(2);
    assert_eq!(*sum_boxes(box1, box2), 3_i8);

    println!("{}", "Test Passed!".green());
}

struct Satellite {
    name: String,
    velocity: f64,
}

impl fmt::Display for Satellite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is flying at {} miles/hr", self.name, self.velocity)
    }
}

pub fn challenge_8() {
    let hubble: Satellite = Satellite {
        name: String::from("Hubble Space Telescope"),
        velocity: 4.72,
    };

    println!("Satellite: {}", hubble)
}

pub fn challenge_9() {
    enum Location {
        Unknown,
        Anonymous,
        Known(f64, f64),
    }

    impl Location {
        fn display(&self) {
            match *self {
                Location::Unknown => println!("Unknown location."),
                Location::Anonymous => println!("Anonymous location."),
                Location::Known(lat, lon) => println!("location is: ({}, {})", lat, lon),
            }
        }
    }

    impl Display for Location {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match *self {
                Location::Unknown => write!(f, "Unknown location."),
                Location::Anonymous => write!(f, "Anonymous location."),
                Location::Known(lat, lon) => write!(f, "location is: ({}, {})", lat, lon),
            }
        }
    }

    let mut address: Location = Location::Unknown;
    address.display();

    address = Location::Anonymous;
    println!("{address}");

    address = Location::Known(3.12312, 3.12312);
    println!("{address}");
}
