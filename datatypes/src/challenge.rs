pub fn exec() {
    challenge_4();
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
