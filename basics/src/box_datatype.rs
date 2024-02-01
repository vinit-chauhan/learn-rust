/*
Box<T>:
    Box data type is used to explicitly store data in heap,
    though given T is normally stored in stack.

    They are smart pointers
    - They have ownership of the referred object
    - When they go out of scope, it deallocates the heap memory

*/

/*
Memory Map:

Stack: 8 byte
     Box<Shuttle>
    (ptr: 0x1000)

Heap: 40 + 4 byte

    0x1000 (Shuttle):
    Here align = 8, which means memory will grow my 8 bytes,
    Therefore, to allocate 33 bytes, struct will reserve 40 bytes.
    -----------------------------------------
    | name:  : 24 bytes                     |
    |   ---------------                     |
    |   | ptr: 0x1200 | : 8 bytes           |
    |   | len: 4      | : 8 bytes           |
    |   | cap: 4      | : 8 bytes           |
    |   ---------------                     |
    |                                       |
    | crew_size: u8     : 1 byte            |
    | propellent: f64   : 8 bytes           |
    -----------------------------------------

    0x1200-0x1204 (String): 4 bytes
    -------------------------
    | 'H' | 'i' | 'i' | '!' |
    -------------------------

*/

use std::mem;

#[allow(dead_code)]
struct Shuttle {
    // takes 40 bytes on stack
    name: String,
    crew_size: u8,
    propellent: f64,
}

impl Shuttle {
    fn new(name: &str) -> Shuttle {
        Shuttle {
            name: String::from(name),
            crew_size: 10,
            propellent: 100.0,
        }
    }
}

pub fn exec() {
    let vehicle: Shuttle = Shuttle::new("V1");

    println!("size in stack: {}", mem::size_of_val(&vehicle));

    let boxed_v: Box<Shuttle> = Box::new(Shuttle::new("V2"));
    println!("size in stack: {}", mem::size_of_val(&boxed_v));
    // * dereference operator: gives reference pointed by variable.
    println!("size in stack: {}", mem::size_of_val(&*boxed_v));

    let unboxed_v: Shuttle = *boxed_v;
    println!("size in stack: {}", mem::size_of_val(&unboxed_v));
    let size = 1.0;
    println!("size in stack: {}", mem::size_of_val(&size));
}
