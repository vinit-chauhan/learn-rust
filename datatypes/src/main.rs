fn main() {
    // _basics_and_print_formatting();
    // _numbers_and_arithmetic();
    // _binary_operations();
    _booleans();
}

fn _basics_and_print_formatting() {
    let y = 11222.0; // immutable variable.
    let mut x = 100; // mutable variable

    println!("y: {}", y);
    println!("x: {}", x);
    x = 20; // gives error if mut is not used.
    println!("new x: {}", x);

    // let mut overflow: u8 = 255;
    // overflow = overflow + 1; // this line will give us overflow.
    // println!("overflow: {}", overflow);

    // Print Formatting
    // format: {INDEX: [0 - Pad with 0][Length].[decimal precision]}
    let id1: i32 = 110123359;
    println!("------------------------------------------");
    println!("|     Name     |        Student ID       |");
    println!("------------------------------------------");
    println!("| {:12} | {:23} |", "Matt", id1);
    println!("| {:12} | {:023} |", "Edward", id1 + 1);
    println!("------------------------------------------");

    println!("a: {0}, b: {1}, a: {0}", 10, 20);
    println!("x: {x:10}, y: {y}");
}

fn _numbers_and_arithmetic() {
    let y = 11222.0; // immutable variable.
    let x = 100; // mutable variable

    // Integers types
    let _ = 123; // default for integers is i32
    let _: u8 = 255; // u -> unsigned
    let _: i128 = 2; // i -> signed
    let _: u128 = 2; // others are i/u[8,16,32,64,128]

    // Float types
    let _ = 1.0; // default is f64
    let _: f32 = 1.1; // there are only 2 float types
    let _: f64 = 1.2; // f32 and f64

    /*
       here ans = x + y would give us error
       because rust doesn't allow inter-type arithmetic
       Both variables need to be of same type.

       typecasting can be done using `as` keyword.
    */
    let _: f32 = 3 as f32;
    let i_ans: i32 = x + y as i32;
    let f_ans: f32 = x as f32 / y;
    println!("ans i: {}, f: {}", i_ans, f_ans);

    let u_int: u8 = 254;
    println!("u: {}, i: {}", u_int, u_int as i8); // u: 254, i:-2
}

fn _binary_operations() {
    // Storing binary representation of an Integer
    // 0b(BIN Representation with optional _ to break it in smaller chunks)[Optional - TYPE to cast to]
    let b: u8 = 0b1111_0000u8;

    // You can't directly use negative bit rep. so will have to explicitly cast to i8
    let _negative_b: i8 = 0b1111_0000u8 as i8;

    println!("bin8_b:{b:0b}, bin16_b:{b:016b}, octal_b: {b:0o}, hex_b: {b:0x}");

    let base: u8 = 0b1111_1111u8;
    let not_b: u8 = !b;
    let and_b: u8 = base & b;
    let or_b: u8 = base | b;
    let xor_b: u8 = base ^ b;

    println!("\n--------- Bitwise Operations ---------");
    println!("{base:08b} & {b:08b}  = {and_b:08b}");
    println!("{base:08b} | {b:08b}  = {or_b:08b}");
    println!("{base:08b} ^ {b:08b}  = {xor_b:08b}");
    println!("! {b:08b}           = {not_b:08b}");
    println!("{b:08b} >> 4        = {0:08b}", b >> 4); // bit-shift right by 4
    println!("{b:08b} << 4        = {0:08b}", b << 4); // bit-shift left  by 4
}

fn _booleans() {}
