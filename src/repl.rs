use std::{
    cell::RefCell,
    io::{self, BufRead, Write},
    rc::Rc,
};

use crate::{
    ast::Nodetrait,
    eval::evaluate,
    lexer::Lexer,
    object::{environment::Environment, ObjectTrait},
    parser::Parser,
    token::Kind,
};
const PROMPT: &str = "-> ";

pub fn start() {
    let mut buf = String::new();
    let mut stdin = io::stdin().lock(); // We get `Stdin` here.

    let env = Rc::new(RefCell::new(Environment::new()));

    let debug_lexer = false;
    let debug_parser = false;
    let debug_evaluator = false;
    let show_error = true;

    loop {
        io::stdout().lock().write_all(PROMPT.as_bytes()).unwrap();
        io::stdout().flush().unwrap();
        match stdin.read_line(&mut buf) {
            Ok(_) => {
                if buf == "printenv\n" {
                    dbg!(&env);
                    dbg!(Rc::weak_count(&env));
                    dbg!(Rc::strong_count(&env));
                }

                let lexer = Lexer::new(buf.clone());

                let mut cloned_lexer = lexer.clone();

                if debug_lexer {
                    loop {
                        let cur_token = cloned_lexer.next();

                        println!("Debug Output (Lexer) >> {:?}", cur_token);

                        if cur_token.kind == Kind::EOF {
                            break;
                        }
                    }
                }

                let mut parser = Parser::new(lexer);
                let program = parser.parse();

                if debug_parser {
                    println!("Debug Output (Parser) >> {:?}", program);
                }

                if program.is_ok() {
                    let program = program.unwrap();
                    let result = evaluate(program.to_node(), &env);

                    if debug_evaluator {
                        println!("Debug Output (Eval) >> {:?}", result);
                    }

                    if result.is_ok() {
                        let eval = result.unwrap();
                        if eval.is_some() {
                            let val = eval.unwrap();
                            println!("{}", val.to_str());
                        }
                    } else if show_error {
                        println!("!!!> ERROR OCCURED <!!!");
                        println!(">> ERROR DETAIL ");
                        println!("{:?}", result.err().unwrap());
                    }
                } else {
                    if show_error {
                        println!("!!!> ERROR OCCURED <!!!");
                        for errs in program.err().unwrap() {
                            println!(">> ERROR DETAIL ");
                            for err in errs {
                                println!("Pos>> {:?}", err.as_ref().position());
                                println!("Detail>> {} ", err.as_ref().detail());
                            }
                        }
                    }
                }
                buf.clear();
            }
            Err(err) => {
                println!("Error occured during reading stdin");
                println!("{:?}", err);
                return;
            }
        }
    }
}
