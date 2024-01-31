/*
Generics are derived at compile time and not run time.
for example, if we are using a generic struct Abc with type u8,
    then the compiler will create

*/

use std::u8;

#[derive(Debug)]
struct Rectangle<T> {
    height: T,
    width: T,
}

impl<T> Rectangle<T> {
    // methods
    fn get_height(&self) -> &T {
        // we are returning reference of T, because we don't know what T is.
        &self.height
    }

    fn get_width(&self) -> &T {
        &self.width
    }

    // functions
    fn new(height: T, width: T) -> Rectangle<T> {
        // short hand initialization syntax.
        Rectangle { height, width }
    }
}

// Following syntax means, that the given implementation is
// only for Rectangle with <u8> Generic type.
// Note: There's no Generic type next to `impl` and a specific type next to struct name.
impl Rectangle<u8> {
    fn get_perimeter(&self) -> u16 {
        self.height as u16 + self.width as u16
    }
}

pub fn exec() {
    _example_1();
}

fn _example_1() {
    let rect_u8: Rectangle<u8> = Rectangle {
        height: 123,
        width: 34,
    };

    let rect_f64: Rectangle<f64> = Rectangle::new(20.01, 10.00);

    println!("rect_u8 : {}x{}", rect_u8.get_height(), rect_u8.get_width());
    println!(
        "rect_f64: {}x{}",
        rect_f64.get_height(),
        rect_f64.get_width()
    );

    println!(
        "perimeters: u8: {}, f64: {}",
        rect_u8.get_perimeter(),
        // rect_f64.get_perimeter() // doesn't exist
        0
    );

    println!("Amongst {} and {}, {} is bigger", 5, 10, get_biggest(5, 10));
}

fn get_biggest<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}
