use std;
use std::io;
use std::io::Write;
use super::vm::VM;

pub struct REPL{
    command_buffer : Vec<String>,
    vm : VM
}

impl REPL {
    pub fn new() -> REPL {
        REPL {
            vm: VM::new(),
            command_buffer: vec![]
        }
    }

    pub fn run(&mut self){
        println!("Welcome to Chernobyl! Let's get reactive!");
        loop {
            let mut buffer = String::new();
            let stdin = io::stdin();
            print!(">>> ");
            io::stdout().flush().expect("Unable to flush stdout");
            stdin.read_line(&mut buffer).expect("Unable to read line from user");
            let buffer = buffer.trim();
            self.command_buffer.push(buffer.to_string());
            match buffer {
                ".history" | ".h" => {
                    for command in &self.command_buffer {
                        println!("{}", command);
                    }
                },
                ".quit" | ".q" => {
                    println!("Farewell! Have a great day!");
                    std::process::exit(0);
                },
                ".help" => {
                    println!( " '.history' or '.h' shows history of commands");
                    println!( " '.quit' or '.q' quits the program");
                }
                _ => {
                    println!("Invalid input");
                }
            }
        }
    }
}