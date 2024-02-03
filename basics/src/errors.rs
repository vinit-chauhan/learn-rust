use core::fmt;
use std::{
    fs,
    io::{self},
};

struct GreaterError {}

impl fmt::Display for GreaterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "a !> b")
    }
}

fn check_greater(a: i32, b: i32) -> Result<i32, GreaterError> {
    if a > b {
        return Ok(a);
    } else {
        return Err(GreaterError {});
    }
}

fn ret_err() -> Result<String, io::Error> {
    let res = fs::read_to_string("nonexisting.txt");
    let mut content = match res {
        Ok(s) => s,
        // Err(_) => String::from("Can't find file."),
        Err(error) => return Err(error),
    };

    content.push_str("hello");
    Ok(content)
}

fn ret_err_shorthand() -> Result<String, io::Error> {
    let mut content: String = fs::read_to_string("nonexisting.txt")?;
    content.push_str("hello");
    Ok(content)
}

pub fn exec() {
    // unwrap: avoid error handling, Panic when error occurs
    // let _ = fs::read_to_string("nonexisting.txt").unwrap();

    // expect: avoid error handling, Panic when error occurs with custom message
    // let _ = fs::read_to_string("nonexisting.txt").expect("Error message");

    // handle error properly
    let res = fs::read_to_string("nonexisting.txt");
    let content = match res {
        Ok(msg) => msg,
        // Err(_) => String::from("Can't find file."),
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => String::from("File not found"),
            io::ErrorKind::PermissionDenied => String::from("File permission denied"),
            _ => panic!("Other Error."),
        },
    };
    println!("contents: {content}");

    match check_greater(1, 2) {
        Ok(n) => println!("greater is: {n}"),
        Err(e) => println!("ERROR: {e}"),
    }

    match ret_err() {
        Ok(s) => println!("s: {s}"),
        Err(e) => println!("ERROR: {e}"),
    }

    match ret_err_shorthand() {
        Ok(s) => println!("s: {s}"),
        Err(e) => println!("ERROR: {e}"),
    }
}
