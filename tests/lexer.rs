use dlang::lexer;

#[test]
fn test() {
    let input = "=+-*%(){}";

    let lex = lexer::Lexer::new(input.to_string());
}
