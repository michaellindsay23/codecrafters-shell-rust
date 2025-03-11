use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let mut input_arr = input.split(" ");

        match input_arr.next().unwrap() {
            "exit" => break,
            _ => {
                println!("{}: command not found", input.trim());
            }
        }
    }
}
