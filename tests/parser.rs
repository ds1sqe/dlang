use dlang::{lexer::Lexer, parser::Parser};

#[test]
fn test_let_statement() {
    let mut inputs = Vec::new();

    inputs.push("let x = 50;".to_string());
    inputs.push("let y = x;".to_string());
    inputs.push("let result;".to_string());

    for input in inputs {
        let lexer = Lexer::new(input.clone());

        let mut parser = Parser::new(lexer);
        let res = parser.parse();
        assert!(res.statements[0].to_str() == input);
    }
}

#[test]
fn test_return_statement() {
    let mut inputs = Vec::new();

    inputs.push("return;".to_string());
    inputs.push("return x;".to_string());
    inputs.push("return 10;".to_string());

    for input in inputs {
        let lexer = Lexer::new(input.clone());

        let mut parser = Parser::new(lexer);
        let res = parser.parse();
        assert!(res.statements[0].to_str() == input);
    }
}
#[test]
fn test_bool_literal() {
    let mut inputs = Vec::new();

    inputs.push("let x = false;".to_string());
    inputs.push("let y = true;".to_string());

    for input in inputs {
        let lexer = Lexer::new(input.clone());

        let mut parser = Parser::new(lexer);
        let res = parser.parse();
        assert!(res.statements[0].to_str() == input);
    }
}

#[test]
fn test_parentheses() {
    let mut inputs = Vec::new();

    inputs.push("((true || (false || true)))".to_string());

    for input in inputs {
        let lexer = Lexer::new(input.clone());

        let mut parser = Parser::new(lexer);
        let res = parser.parse();
        assert!(res.statements[0].to_str() == input);
    }
}

#[test]
fn test_infix() {
    let input = "10 * 10 + 20 * 20 + 100".to_string();
    let expect = "(((10 * 10) + (20 * 20)) + 100)".to_string();

    let lexer = Lexer::new(input.clone());

    let mut parser = Parser::new(lexer);
    let res = parser.parse();
    assert!(res.statements[0].to_str() == expect);
}
