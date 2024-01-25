pub fn run() {
    // here type of num is i8 because we are passing it to
    // `say_number` function and it requires i8 as argument.
    // so compiler casts the num as i8 if we don't specify explicitly.
    let num = 10;
    say_number(num); // passed by value.

    println!("square of num is: {}", ret_square_number(num));

    let (num1, num2) = (1, 9);
    let res = is_sum_10(num1, num2);
    println!("{} + {} == 10 ? {}", res.0, res.1, res.2);
}

// here default return type is `()` : Unit Data-type
fn say_number(number: i8) {
    println!("number: {number}");
    // number = 23;
}

fn ret_square_number(number: i8) -> i16 {
    number as i16 * number as i16
    // return number as i16 * number as i16;
}

fn is_sum_10(num1: i32, num2: i32) -> (i32, i32, char) {
    if num1 + num2 != 10 {
        return (num1, num2, 'n');
    } else {
        return (num1, num2, 'y');
    }
}
