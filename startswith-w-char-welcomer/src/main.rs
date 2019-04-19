use std::env::args;

fn main() {
    for arg in args().skip(1) {
        if let Some(c) = arg.chars().next() {
            match c {
                'w' | 'W' => println!("Hello {}!", arg),
                _ => {},
            }
        }
    }
}