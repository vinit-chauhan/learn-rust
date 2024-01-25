fn main() {
    let y = 11222.0; // immutable variable.
    let mut x = 100; // mutable variable

    println!("y: {}", y);
    println!("x: {}", x);
    x = 20; // gives error if mut is not used.
    println!("new x: {}", x);

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

    // let mut overflow: u8 = 255;
    // overflow = overflow + 1; // this line will give us overflow.
    // println!("overflow: {}", overflow);
}
