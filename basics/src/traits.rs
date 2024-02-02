use std::any;
use std::fmt;

#[derive(PartialEq)] // true -> Only if the values of all properties is same.
struct Satellite {
    name: String,
    velocity: f64,
}

struct SpaceStation {
    name: String,
    velocity: f64,
    crew_size: u8,
}

impl Satellite {
    fn new(name: String) -> Satellite {
        Satellite {
            name,
            velocity: 1001_f64,
        }
    }
}

impl SpaceStation {
    fn new(name: String) -> SpaceStation {
        SpaceStation {
            name,
            crew_size: 10,
            velocity: 100_f64,
        }
    }
}

struct SomeStruct {}

impl Description for SomeStruct {}

trait Description {
    // with no default implementation
    // fn desc(&self) -> String;

    // with default implementation
    fn desc(&self) -> String {
        String::from("This is the default description.")
    }
}

impl Description for Satellite {
    fn desc(&self) -> String {
        format!("the {} flying at {} km/s.", self.name, self.velocity)
    }
}

impl Description for SpaceStation {
    fn desc(&self) -> String {
        format!(
            "the {} flying at {} km/s with {} crew members.",
            self.name, self.velocity, self.crew_size
        )
    }
}

fn print_type<T: fmt::Display>(item: T) {
    println!("{} is {}", item, any::type_name::<T>());
}

fn cmp_type<T: fmt::Display + PartialEq + From<U>, U: fmt::Display + PartialEq + Copy>(a: T, b: U) {
    if a == T::from(b) {
        println!("{} == {}", a, b);
    } else {
        println!("{} != {}", a, b);
    }
}

fn cmp_type2<T, U>(a: T, b: U)
where
    T: fmt::Display + PartialEq + From<U>,
    U: fmt::Display + PartialEq + Copy,
{
    if a == T::from(b) {
        println!("{} == {}", a, b);
    } else {
        println!("{} != {}", a, b);
    }
}

// returns the value type which implements Display trait
fn get_displayable() -> impl fmt::Display {
    123
}

pub fn exec() {
    let s0 = Satellite::new(String::from("Hubble Telescope"));
    let s1 = Satellite::new(String::from("Hubble Telescope"));
    let s2 = Satellite::new(String::from("New Hubble"));

    let ss1 = SpaceStation::new(String::from("International Space Station"));

    println!("s1: {}", s1.desc());
    println!("s1: {}", ss1.desc());

    println!("s0 == s1: {} \t s1 == s2: {}", s0 == s1, s1 == s2);

    let temp = SomeStruct {};
    println!("default: {}", temp.desc());

    print_type(String::new());
    print_type(123_i32);
    // print_type(s1); // error: Display trait not implemented.

    cmp_type2(1.1, 2.2);
    cmp_type2(1.0_f32, 2.2);
}
