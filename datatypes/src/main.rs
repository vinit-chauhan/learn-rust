fn main() {
    let y: i32 = 11222; // immutable variable.
    let mut x: i8 = 100; // mutable variable

    println!("y: {}", y);
    println!("x: {}", x);
    x = 20; // gives error if mut is not used.
    println!("new x: {}", x);
}
