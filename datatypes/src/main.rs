fn main() {
    let y: i32 = 11222; // immutable variable.
    let mut x: i8 = 100; // mutable variable

    // Integers types
    let _ = 123; // default for integers is i32
    let mut overflow: u8 = 255; // u -> unsigned
    let _: i128 = 2; // i -> signed
    let _: u128 = 2; // others are i/u[8,16,32,64,128]

    println!("y: {}", y);
    println!("x: {}", x);
    x = 20; // gives error if mut is not used.
    println!("new x: {}", x);

    overflow = overflow + 1; // this line will give us overflow.
    println!("overflow: {}", overflow);
}
