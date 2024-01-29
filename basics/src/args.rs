use std::env;

// arg[0] -> executable name
// arg[x] -> xth argument

pub fn exec() {
    if env::args().len() <= 3 {
        println!("Code will work..");
        return;
    }

    for (index, arg) in env::args().enumerate() {
        println!("arg {index}: {arg}");
    }

    let first_arg = env::args().nth(1).expect("Can't find 1st arg");
    println!("First arg: {first_arg}");

    let second_arg: String;

    match env::args().nth(2) {
        None => {
            panic!("Can't find 2nd arg");
        }
        Some(s) => second_arg = s,
    }

    println!("Second arg: {second_arg}");
}
