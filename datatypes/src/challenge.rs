pub fn exec() {
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

pub fn celsius_to_fahrenheit(temprature: f64) -> f64 {
    temprature * 1.8 + 32.0
}
