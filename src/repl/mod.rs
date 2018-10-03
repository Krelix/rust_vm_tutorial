use std;
use std::io;
use std::io::Write;
use vm::VM;

pub struct REPL {
    command_buffer: Vec<String>,
    vm: VM
}

impl REPL {
    pub fn new() -> REPL {
        REPL {
            command_buffer: vec![],
            vm : VM::new()
        }
    }

    pub fn run(&mut self) {
        println!("This is an iridium look alike. And this is it's REPL. Enter commands below :");
        loop {
            let mut buffer = String::new();

            // Print ">>> " in front of the command
            print!(">>> ");
            // Force flush, otherwise print acts as a buffer until a flush is ordered
            // and then content is shown on screen (terminal)
            io::stdout().flush().expect("Unable to flush stdout");

            io::stdin().read_line(&mut buffer).expect("Unable to read line from user");
            let buffer = buffer.trim();
            self.command_buffer.push(buffer.to_string());

            match buffer {
                ".quit" => {
                    println!("Goodbye then.");
                    std::process::exit(0);
                },
                ".history" => {
                    for command in &self.command_buffer {
                        println!("{}", command);
                    }
                }
                _ => {
                    println!("Invalid input");
                }
            }
        }
    }
}