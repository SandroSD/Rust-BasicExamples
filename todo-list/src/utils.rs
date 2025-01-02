use std::io::{self, Write};

pub fn save_user_input() -> String {
    let mut input = String::new();

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string() // trim returns a reference, so when the fn finish it's execution, the reference is going to point to something that will not exist anymore, that is why we need to_string()
}