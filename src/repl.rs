use std::io::{self, BufRead, Write};

use crate::{lexer::Lexer, parser::Parser};

const PROMPT: &str = "-> ";

pub fn start() {
    let mut buf = String::new();
    let mut stdin = io::stdin().lock(); // We get `Stdin` here.
    loop {
        io::stdout().lock().write_all(PROMPT.as_bytes()).unwrap();
        io::stdout().flush().unwrap();
        match stdin.read_line(&mut buf) {
            Ok(n) => {
                let lexer = Lexer::new(buf.clone());
                let mut parser = Parser::new(lexer);
                let program = parser.parse();

                for stm in program.statements {
                    println!("Debug output>> {:?}", stm);
                    println!("To String >> {}", stm.to_str());
                }
            }
            Err(err) => {
                println!("Error occured during reading stdin");
                return;
            }
        }
    }
}
