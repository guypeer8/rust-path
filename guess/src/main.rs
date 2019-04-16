extern crate rand;

use std::io;
use rand::random;

fn get_guess() -> u8 {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    match guess.trim().parse::<u8>() {
        Ok(v) => v,
        Err(e) => {
            println!("Could not read {}", e);
            return 1;
        },
    }
}

fn handle_guess(correct: u8, guess: u8) -> bool {
    if correct == guess {
        println!("Great!");
        return true;
    }

    if correct > guess {
        println!("Type a larger number");
    } else if correct < guess {
        println!("Type a smaller number");
    }

    false
}

fn main() {
    let correct = random::<u8>();
    println!("Please enter a number between {} and {}: ", std::u8::MIN, std::u8::MAX);

    loop {
        let guess = get_guess();
        if handle_guess(correct, guess) {
            break;
        }
    }
}
