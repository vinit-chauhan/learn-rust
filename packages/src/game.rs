use rand::prelude::*;
use std::io;

pub fn run() {
    let answer: i8 = thread_rng().gen_range(1..=100);

    loop {
        println!("Enter your guess: ");
        let mut buff: String = String::new();
        match io::stdin().read_line(&mut buff) {
            Ok(_) => {}
            Err(err) => {
                println!("Error reading input: {}", err);
            }
        }

        let guess: i8;
        match buff.trim().parse::<i8>() {
            Ok(num) => guess = num,
            Err(_) => {
                println!("Error parsing the input to Int.");
                continue;
            }
        }

        if guess == answer {
            println!("Congratulations!! You Won!!");
            break;
        } else if guess < answer {
            println!("Try a bigger number");
        } else {
            println!("Try a smaller number");
        }
    }
}
