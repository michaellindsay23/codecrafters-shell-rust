use std::io::{self, Write};

enum Builtin {
    Exit,
    Echo(String),
    Type(String),
    Invalid(String, String),
}

impl Builtin {
    fn call(&self) {
        match &self {
            Builtin::Exit => todo!(),
            Builtin::Echo(tail) => println!("{}", tail),
            Builtin::Type(tail) => match Builtin::find_type(tail, tail.clone()) {
                Builtin::Invalid(_head, tail) => println!("{}: not found", tail),
                _ => println!("{} is a shell builtin", tail),
            },
            Builtin::Invalid(head, _tail) => println!("{}: command not found", head),
        }
    }

    fn find_type(head: &str, tail: String) -> Builtin {
        match head {
            "exit" => Builtin::Exit,
            "echo" => Builtin::Echo(tail),
            "type" => Builtin::Type(tail),
            _ => Builtin::Invalid(head.to_string(), tail),
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut input;
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        input = String::new();
        stdin.read_line(&mut input).unwrap();

        let mut input_arr = input.split_whitespace();

        let head = input_arr.next().unwrap();
        let tail: String = input_arr.collect();

        if head.eq("exit") {
            break;
        }

        Builtin::find_type(head, tail).call();
    }
}
