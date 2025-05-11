use std::{
    env::{self, split_paths},
    io::{self, Write},
    path::PathBuf,
    //os::linux,
};

enum Builtin {
    Exit,
    Echo(Vec<String>),
    TypeCMD(Vec<String>),
    TypePATH(Vec<String>, Vec<PathBuf>),
    Invalid(String, Vec<String>),
}

impl Builtin {
    fn call(&self) {
        match &self {
            Builtin::Exit => return,
            Builtin::Echo(tail) => println!("{}", Builtin::to_valid_str(tail.clone())),
            Builtin::TypeCMD(tail) => match Builtin::find_type(tail[0].clone(), tail.clone()) {
                Builtin::Invalid(_head, tail) => {
                    println!("{}: not found", Builtin::to_valid_str(tail))
                }
                _ => println!("{} is a shell builtin", tail[0]),
            },
            Builtin::TypePATH(tail, paths) => {
                for path in paths {
                    if path.ends_with(&tail[0]) {
                        println!("type is {}", tail[0]);
                    }
                }
            }
            Builtin::Invalid(head, _tail) => println!("{}: command not found", head),
        }
    }

    // Find the Builtin type associated with the given String
    fn find_type(head: String, tail: Vec<String>) -> Builtin {
        match head.as_ref() {
            "exit" => Builtin::Exit,
            "echo" => Builtin::Echo(tail),
            "type" => match env::var_os("PATH") {
                Some(paths) => Builtin::TypePATH(tail, split_paths(&paths).collect()),
                None => Builtin::TypeCMD(tail),
            },
            _ => Builtin::Invalid(head.to_string(), tail),
        }
    }

    // Converts a Vec<String> to a valid String for printing
    //   - consists of all vec elements seperated by " "
    fn to_valid_str(vec: Vec<String>) -> String {
        let mut str = String::new();
        for s in vec {
            str.push_str(s.as_ref());
            str.push_str(" ");
        }
        return str.trim().to_string();
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

        let mut input_arr = input.split_whitespace().map(str::to_string);

        let head = input_arr.next().unwrap();
        let tail = input_arr.collect();

        if head.eq("exit") {
            break;
        }

        Builtin::find_type(head, tail).call();
    }
}
