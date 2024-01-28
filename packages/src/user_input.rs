use std::io;

pub fn get_num() {
    let mut buffer = String::new();

    println!("Enter a Message: ");

    let _res: Result<usize, io::Error> = io::stdin().read_line(&mut buffer);

    println!("buffer: {}", buffer.trim());

    // parse result
    let _number: i32 = buffer.trim().parse().unwrap();
    let _number = buffer.trim().parse::<i32>().unwrap();

    println!("number: {_number}");
}
