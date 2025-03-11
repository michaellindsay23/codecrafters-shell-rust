use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let mut input;
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        input = String::new();
        stdin.read_line(&mut input).unwrap();

        let mut input_arr = input.split(" ");

        match input_arr.next().unwrap() {
            "exit" => break,
            "echo" => {
                for ele in input_arr {
                    print!("{} ", ele);
                }
            }
            _ => {
                println!("{}: command not found", input.trim());
            }
        }
    }
}
