pub fn intro() {
    // can't change values as arrays are immutable by default.
    // need to add `mut` to change elements of array.
    let letters: [char; 4] = ['a', 'b', 'c', 'd'];
    let first: char = letters[0];

    println!("letters: {letters:?}");
    println!("letters: {letters:#?}");

    println!("first: {first}");

    let mut numbers: [i32; 5];
    // rust checks if the values are assigned before accessing it.
    // if not the compiler will throw an error
    // println!("numbers[0]: {}", numbers[0]); // throws an error

    // Initialize an array

    // we can use 2 ways, [0, 0, 0, 0, 0]; or [0; 5];
    // numbers = [0, 0, 0, 0, 0];
    numbers = [0; 5];
    let index: usize = 0;
    numbers[0] = 1;
    println!("numbers[0]: {}", numbers[index]); // panics at runtime if index > 4
}

pub fn multi_dimensional_array() {
    let _: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];
    let _arr: [[[i32; 3]; 2]; 5]; // 3-dimensional array [3][2][5]
    _arr = [[[0; 3]; 2]; 5]; // initialize 3d arr with 0
}

pub fn tuples() {
    let mut tuple: (i8, f32, char) = (10, 3.14, 'a');
    tuple.0 += 10;
    let first: i8 = tuple.0;

    println!("first: {first}");

    // destructuring;
    let (a, b, c) = tuple;
    println!("a: {a} b: {b} c: {c}");
}
