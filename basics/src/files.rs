use std::fs;
use std::io::Write;

pub const PATH: &str = "/home/vinit-chauhan/Downloads/test.txt";

pub fn exec() {
    _write_file();

    let contents = fs::read_to_string(PATH).unwrap();
    _read_line(&contents);
    // _read_bin();
}

fn _read_full(contents: &String) {
    println!("content is:\n {}", contents);
}

fn _read_line(contents: &String) {
    for lines in contents.lines() {
        match lines.chars().nth(0) {
            Some(c) => {
                if c == '#' || c == '\n' {
                    continue;
                }
            }
            None => (),
        }
        println!("{lines}");
    }
}

fn _read_bin() {
    let blob = fs::read("/home/vinit-chauhan/Downloads/Resume202312300340.pdf").unwrap();
    println!("content: {blob:?}");
}

fn _write_file() {
    // Write a file with wrapper method.
    let text_blob: String = String::from("Hello, from file.");
    let _ = fs::write(PATH, text_blob);

    let mut file: fs::File = fs::OpenOptions::new().append(true).open(PATH).unwrap();
    // low-level system call.
    let _ = file.write(b"\nnew message..");
}
