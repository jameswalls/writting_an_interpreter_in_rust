use std::io::{self, BufRead, Read, Write};
use crate::lexer::{self, Lexer};

pub struct Repl;

impl Repl {
    pub fn new() -> Self { Repl }

    pub fn start(self) -> io::Result<()> {
        print!("Welcome to the monkey REPL!!!!!\n\n");

        let mut buffer = String::new();
        let stdin = io::stdin();

        print!("|monkey> ");
        io::stdout().flush();
        while let Ok(line) = stdin.lock().read_line(&mut buffer) {
            if buffer.trim() == "exit" { break }
            let mut lexer = Lexer::new(buffer.clone());
            
            while let Some(token) = lexer.next_token() {
                println!("{:?}", token);
            }

            buffer.clear();
            print!("|monkey> ");
            io::stdout().flush();
        }

        println!("\nBye! don't forget to eat your bananas...");

        Ok(())
    }
}
