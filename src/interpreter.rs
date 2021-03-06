use std::env;
use std::path::Path;
use std::fs;
use std::io;
use std::io::Write;
use std::cmp::Ordering;

pub struct Interpreter {
    has_error: bool
}

impl Default for Interpreter {
    fn default() -> Self {
        Interpreter::new()
    }
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            has_error: false,
        }
    }

    pub fn get_run_type(&mut self) {
        let args: Vec<String> = env::args().collect();
        match args.len().cmp(&2) {
            Ordering::Greater => println!("Usage panther [file]"),
            Ordering::Equal => self.run_file(&args[1]),
            Ordering::Less => self.run_prompt(),
        }
    }

    pub fn run_file(&mut self, filename: &str) {
        let path = Path::new(&filename);
        let content = fs::read_to_string(path)
            .expect("Error reading the file");
        self.run(content);
        if self.has_error {
            std::process::exit(65);
        }
    }

    pub fn run_prompt(&mut self) {
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        loop {
            write!(&mut stdout, "> ").expect("Failed to get stdout");
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
    
    pub fn run(&mut self, tokens: String) {
        for token in tokens.chars() {
            println!("{}", token);
        }
    }
    
    pub fn error(&mut self, line: usize, message: String) {
        self.report(line, String::from(""), message);
    }
    
    pub fn report(&mut self, line: usize, location: String, message: String) {
        println!("line: {} Error {}: {}", line, location, message);
        self.has_error = true;
    }
}
