use std::io;

#[derive(Copy, Clone)]
enum MachineState {
    Normal,
    Comment,
    Lower,
    Upper,
}

fn machine_cycle(state: MachineState, c: char) -> (Option<char>, MachineState) {
    use self::MachineState::*;

    match (state, c) {
        (Normal, '#') => (None, Comment),
        (Normal, '^') => (None, Upper),
        (Normal, '_') => (None, Lower),
        (Normal, other) => (Some(other), Normal),
        (Comment, '#') => (None, Normal),
        (Comment, _) => (None, Comment),
        (Upper, '^') => (None, Normal),
        (Upper, other) => (Some(other.to_ascii_uppercase()), Upper),
        (Lower, '_') => (None, Normal), 
        (Lower, other) => (Some(other.to_ascii_lowercase()), Lower),
    }
}

fn main() {
    let mut state = MachineState::Normal;
    let mut input_string = String::new();
    let mut processed_string = String::new();

    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read input line");

    for input_char in input_string.trim().chars() {
        let (processed_char, new_state) = machine_cycle(state, input_char);
        if let Some(c) = processed_char {
            processed_string.push(c);
        }

        state = new_state;
    }

    println!("Markdown Result: {}", processed_string);
}
