use std::io::{self, BufRead, Write};

use crate::{
    ast::Node, ast::Nodetrait, eval::evaluate, lexer::Lexer,
    object::environment::Environment, parser::Parser,
};
const PROMPT: &str = "-> ";

pub fn start() {
    let mut buf = String::new();
    let mut stdin = io::stdin().lock(); // We get `Stdin` here.
    let mut env: Environment<String> = Environment::new();
    loop {
        io::stdout().lock().write_all(PROMPT.as_bytes()).unwrap();
        io::stdout().flush().unwrap();
        match stdin.read_line(&mut buf) {
            Ok(n) => {
                let lexer = Lexer::new(buf.clone());
                let mut parser = Parser::new(lexer);
                let program = parser.parse();

                if program.is_ok() {
                    let program = program.unwrap();
                    for stm in program.statements {
                        let node = stm.to_node();
                        let result = evaluate(node, &mut env);
                        println!("Debug output>> {:?}", result);

                        if result.is_ok() {
                            let eval = result.unwrap();
                            println!(">>{:?}", eval);
                        }
                    }
                } else {
                    println!("!!!> ERROR OCCURED <!!!");
                    for errs in program.err().unwrap() {
                        println!(">> ERROR DETAIL ");
                        for err in errs {
                            println!("Pos>> {:?}", err.as_ref().position());
                            println!("Detail>> {} ", err.as_ref().detail());
                        }
                    }
                }
                buf.clear();
            }
            Err(err) => {
                println!("Error occured during reading stdin");
                return;
            }
        }
    }
}
