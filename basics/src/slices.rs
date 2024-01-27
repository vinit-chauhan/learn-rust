/*
    ************ SLICES *****************

Stack:
    &str: [ptr', len] :: borrower of referred memory space
    &String: [ptr*] -> String: [ptr', len, cap] :: Owner of the referred memory space.

    ptr'  are reference in Heap,

    prt* is ref to stack variable.

Visualize:

            0x1300: &String                 0x1200: &str
            [ptr: 0x1000]            [ptr: 0x12300012, len: 3]
                    |
                    V
            0x1000: String
        [ptr: 0x12300000, len: 15, cap: 16]


Concept:
    Deref Coercion:
        We can use &String in place of &str: &Str has ptr and len of string in Stack
        However, revers is not possible

Takeaway:
    When we are working with String and slice in a function slice,
    Use `&str` for input and return type, as we can pass both String and slice values.

*/

pub fn exec() {
    let mut message: String = String::from("Some String Literal.");

    message.push_str(" new str.");

    // _slices_1(&message);

    // _slices_2();

    let word = _slice_3(&message);
    println!("first: {word}");

    /*
        won't work, because &String != &str
        &String points to a String which owns actual heap memory address space
        &str borrows the reference in the heap.
    */

    // let word = _slice_3(&message[word.len()..]);
    // println!("first: {word}");
}

fn _slices_1(message: &String) {
    // length in byte not char.
    // char can be multiple byte in UTF-8
    let my_slice = &message[10..];
    println!("my_slice: {my_slice}");

    // char 10 (included) to 15 (excluded)
    let my_slice = &message[10..15];
    println!("my_slice: {my_slice}");

    // chars before 10
    let my_slice = &message[..10];
    println!("my_slice: {my_slice}");
}

fn _slices_2() {
    let nums: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];

    let first_4_nums = &nums[..4];

    println!("First 4 nums are: {first_4_nums:?}");
}

fn _slice_3(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (index, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..index];
        }
    }

    /*
        Deref Coercion:
        We can use &String in place of &str: &Str has ptr and len of string in Stack
        However, revers is not possible

    */
    &s // if no space is found
}
