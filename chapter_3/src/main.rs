// Chapter 3; Simply repeat what the user says. Static mutable variables require 
// unsafe blocks to update, so for now I'm leaving it as a local variable inside main.
extern crate rustyline;
use std::io::{self, Write};

const PROMPT: &'static str = "lispy> ";

fn main() {
    let mut rl = rustyline::Editor::<()>::new();
    let exit: String = "exit".to_owned() ;
    
    println!("Lispy Version 0.0.0.0.1");
    println!("type 'exit' to quit");

    loop {
        match rl.readline(PROMPT) {
            Ok(ref line) if line == &exit => break,
            Ok(line) => {
                rl.add_history_entry(&line);
                println!("No you're a {}", line);
            },
            Err(_)   => panic!("Could not read line")
        }
    }
}

