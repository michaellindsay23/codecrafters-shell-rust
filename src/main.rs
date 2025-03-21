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
                let mut input = String::new();
                input_arr.for_each(|x| input.push_str(&format!("{} ", x)));
                print!("{}\n", input);
            }
            "type" => {
                let input = input_arr.next().unwrap().trim();
                if input == "exit" || input == "echo" || input == "type" {
                    println!("{} is a shell builtin", input);
                } else {
                    println!("{}: not found", input)
                }
            }
            _ => {
                println!("{}: command not found", input.trim());
            }
        }
    }
}
