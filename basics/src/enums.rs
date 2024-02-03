#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

impl Shape {
    fn get_perimeter(&self) -> f64 {
        match *self {
            Shape::Circle(r) => r * 2_f64 * std::f64::consts::PI,
            Shape::Rectangle(h, w) => 2_f64 * (h + w),
            Shape::Triangle(l1, l2, l3) => l1 + l2 + l3,
        }
    }

    fn get_type(&self) -> &str {
        match *self {
            Shape::Circle(_) => "Circle",
            Shape::Rectangle(_, _) => "Rectangle",
            Shape::Triangle(_, _, _) => "Triangle",
        }
    }
}

pub fn exec() {
    example_5();
}

pub fn example_1() {
    let my_shape: Shape = Shape::Circle(2.3);
    let my_shape: Shape = Shape::Triangle(1.2, 1.1, 2.3);
    let my_shape: Shape = Shape::Rectangle(1.2, 2.3);

    println!("my_shape: {my_shape:?}");

    match my_shape {
        Shape::Circle(r) => println!("This is a circle"),
        Shape::Rectangle(h, w) => println!("This is a Rectangle"),
        Shape::Triangle(l1, l2, l3) => println!("This is a Triangle"),
    }
}

pub fn example_2() {
    /*
        We need to add wildcard `_` as one of the match arms,
        Because, match statement will throw an compile error,
        if all the possible branches of passed variable is not
        handled.

        For the following example,
        my_number can be 0 - 255, so we would need to have a
        handler for all 256 numbers.

        It runs from top to bottom sequentially, just like switch-case
        So we need to put our wildcard at very bottom.
    */

    let my_number: u8 = 7_u8;
    let res: &str = match my_number {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 | 4 => "three",
        _ => {
            println!("{} is something else", my_number);
            "wildcard"
        }
    };

    println!("res: {res}");
}

pub fn example_3() {
    let my_shape: Shape = Shape::Circle(2.3);

    println!(
        "perimeter of {} is: {:.2}",
        my_shape.get_type(),
        my_shape.get_perimeter()
    );

    let my_shape: Shape = Shape::Triangle(1.2, 1.1, 2.3);
    println!(
        "perimeter of {} is: {:.2}",
        my_shape.get_type(),
        my_shape.get_perimeter()
    );

    let my_shape: Shape = Shape::Rectangle(1.2, 2.3);
    println!(
        "perimeter of {} is: {:.2}",
        my_shape.get_type(),
        my_shape.get_perimeter()
    );
}

pub fn example_4() {
    let _something: Option<i32> = Some(12);
    let _nothing: Option<i32> = None;

    let arr: [i32; 5] = [5, 4, 3, 2, 1];

    let num = arr.get(3);
    let num: i32 = match num {
        Some(n) => *n,
        None => -1,
    };
    println!("number is : {num}")
}

// if-let statement
pub fn example_5() {
    let num = Some(13);

    match num {
        Some(13) => println!("thirteen"),
        _ => (),
    }

    // simpler way
    if let Some(13) = num {
        println!("thirteen")
    }
}
