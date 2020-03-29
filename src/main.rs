use std::env;
use std::path::Path;
use std::fs;
use std::io;
use std::io::Write;
pub mod token;
pub mod token_type;
pub mod scanner;

struct Interpreter {
    has_error: bool
}

impl Interpreter {
    fn new() -> Interpreter {
        Interpreter {
            has_error: false,
        }
    }

    fn get_run_type(&mut self) {
        let args: Vec<String> = env::args().collect();
        if args.len() > 2 {
            println!("Usage panther [file]");
        }
        else if args.len() == 2 {
            self.run_file(&args[1]);
        }
        else {
            self.run_prompt();
        }
    }

    fn run_file(&mut self, filename: &String) {
        let path = Path::new(&filename);
        let content = fs::read_to_string(path)
            .expect("Error reading the file");
        self.run(content);
        if self.has_error {
            std::process::exit(65);
        }
    }
    
    fn run_prompt(&mut self) {
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        loop {
            writeln!(&mut stdout, "> ").expect("Failed to get stdout");
            stdout.flush().expect("Failed to flush the stdout content");
    
            let mut input = String::new();
            stdin.read_line(&mut input).expect("Failed to read line");
            if input == "exit\n" {
                break;
            }
            self.run(input);
            //reset flag to not kill user_session in case of a mistake
            self.has_error = false; 
        }
    }
    
    fn run(&mut self, tokens: String) {
        for token in tokens.chars() {
            println!("{}", token);
        }
    }
    
    fn error(&mut self, line: i32, message: String) {
        self.report(line, String::from(""), message);
    }
    
    fn report(&mut self, line: i32, location: String, message: String) {
        println!("line: {} Error {}: {}", line, location, message);
        self.has_error = true;
    }
}

fn main() {
    let mut panther = Interpreter::new();
    panther.get_run_type();
}

