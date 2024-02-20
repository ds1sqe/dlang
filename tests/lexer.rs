use dlang::lexer;
use dlang::token::Token;

#[test]
fn test_lexer() {
    let input = "
    let ten = 10;
    let two = 2;

    let add = fn(x,y) {
    x + y;
    };

    let result = add(ten,two);
    let flag = false;

    let test = fn(x,y) {
    if (x>y) {
    return x*y;
    }else if (x<y) {
    return x/y;
    }else {
    flag = true;
    return x%y;
    }

    ten <= two;
    ten >= two;
    ten == two;
    ten != two;

    ten & two;
    ten | two;

    true && false;
    true || false;

    ";

    let mut expects = Vec::new();

    expects.push(Token::with("LET", "let"));
    expects.push(Token::with("IDENT", "ten"));
    expects.push(Token::with("=", "="));
    expects.push(Token::with("INT", "10"));
    expects.push(Token::with(";", ";"));
    expects.push(Token::with("LET", "let"));
    expects.push(Token::with("IDENT", "two"));
    expects.push(Token::with("=", "="));
    expects.push(Token::with("INT", "2"));
    expects.push(Token::with(";", ";"));
    expects.push(Token::with("LET", "let"));
    expects.push(Token::with("IDENT", "add"));
    expects.push(Token::with("=", "="));
    expects.push(Token::with("FUNCTION", "fn"));
    expects.push(Token::with("(", "("));
    expects.push(Token::with("IDENT", "x"));
    expects.push(Token::with(",", ","));
    expects.push(Token::with("IDENT", "y"));
    expects.push(Token::with(")", ")"));
    expects.push(Token::with("{", "{"));
    expects.push(Token::with("IDENT", "x"));
    expects.push(Token::with("+", "+"));
    expects.push(Token::with("IDENT", "y"));
    expects.push(Token::with(";", ";"));
    expects.push(Token::with("}", "}"));
    expects.push(Token::with(";", ";"));
    expects.push(Token::with("LET", "let"));
    expects.push(Token::with("IDENT", "result"));
    expects.push(Token::with("=", "="));
    expects.push(Token::with("IDENT", "add"));
    expects.push(Token::with("(", "("));
    expects.push(Token::with("IDENT", "ten"));
    expects.push(Token::with(",", ","));
    expects.push(Token::with("IDENT", "two"));
    expects.push(Token::with(")", ")"));
    expects.push(Token::with(";", ";"));
    expects.push(Token::with("LET", "let"));
    expects.push(Token::with("IDENT", "flag"));
    expects.push(Token::with("=", "="));
    expects.push(Token::with("FALSE", "false"));
    expects.push(Token::with(";", ";"));
    expects.push(Token::with("LET", "let"));
    expects.push(Token::with("IDENT", "test"));
    expects.push(Token::with("=", "="));
    expects.push(Token::with("FUNCTION", "fn"));
    expects.push(Token::with("(", "("));
    expects.push(Token::with("IDENT", "x"));
    expects.push(Token::with(",", ","));
    expects.push(Token::with("IDENT", "y"));
    expects.push(Token::with(")", ")"));
    expects.push(Token::with("{", "{"));
    expects.push(Token::with("IF", "if"));
    expects.push(Token::with("(", "("));
    expects.push(Token::with("IDENT", "x"));
    expects.push(Token::with(">", ">"));
    expects.push(Token::with("IDENT", "y"));
    expects.push(Token::with(")", ")"));
    expects.push(Token::with("{", "{"));
    expects.push(Token::with("RETURN", "return"));
    expects.push(Token::with("IDENT", "x"));
    expects.push(Token::with("*", "*"));
    expects.push(Token::with("IDENT", "y"));
    expects.push(Token::with(";", ";"));
    expects.push(Token::with("}", "}"));
    expects.push(Token::with("ELSE", "else"));
    expects.push(Token::with("IF", "if"));
    expects.push(Token::with("(", "("));
    expects.push(Token::with("IDENT", "x"));
    expects.push(Token::with("<", "<"));
    expects.push(Token::with("IDENT", "y"));
    expects.push(Token::with(")", ")"));
    expects.push(Token::with("{", "{"));
    expects.push(Token::with("RETURN", "return"));
    expects.push(Token::with("IDENT", "x"));
    expects.push(Token::with("/", "/"));
    expects.push(Token::with("IDENT", "y"));
    expects.push(Token::with(";", ";"));
    expects.push(Token::with("}", "}"));
    expects.push(Token::with("ELSE", "else"));
    expects.push(Token::with("{", "{"));
    expects.push(Token::with("IDENT", "flag"));
    expects.push(Token::with("=", "="));
    expects.push(Token::with("TRUE", "true"));
    expects.push(Token::with(";", ";"));
    expects.push(Token::with("RETURN", "return"));
    expects.push(Token::with("IDENT", "x"));
    expects.push(Token::with("%", "%"));
    expects.push(Token::with("IDENT", "y"));
    expects.push(Token::with(";", ";"));
    expects.push(Token::with("}", "}"));
    expects.push(Token::with("IDENT", "ten"));
    expects.push(Token::with("<=", "<="));
    expects.push(Token::with("IDENT", "two"));
    expects.push(Token::with(";", ";"));
    expects.push(Token::with("IDENT", "ten"));
    expects.push(Token::with(">=", ">="));
    expects.push(Token::with("IDENT", "two"));
    expects.push(Token::with(";", ";"));
    expects.push(Token::with("IDENT", "ten"));
    expects.push(Token::with("==", "=="));
    expects.push(Token::with("IDENT", "two"));
    expects.push(Token::with(";", ";"));
    expects.push(Token::with("IDENT", "ten"));
    expects.push(Token::with("!=", "!="));
    expects.push(Token::with("IDENT", "two"));
    expects.push(Token::with(";", ";"));
    expects.push(Token::with("IDENT", "ten"));
    expects.push(Token::with("&", "&"));
    expects.push(Token::with("IDENT", "two"));
    expects.push(Token::with(";", ";"));
    expects.push(Token::with("IDENT", "ten"));
    expects.push(Token::with("|", "|"));
    expects.push(Token::with("IDENT", "two"));
    expects.push(Token::with(";", ";"));
    expects.push(Token::with("TRUE", "true"));
    expects.push(Token::with("&&", "&&"));
    expects.push(Token::with("FALSE", "false"));
    expects.push(Token::with(";", ";"));
    expects.push(Token::with("TRUE", "true"));
    expects.push(Token::with("||", "||"));
    expects.push(Token::with("FALSE", "false"));
    expects.push(Token::with(";", ";"));
    expects.push(Token::with("EOF", "EOF"));

    let mut lex = lexer::Lexer::new(input.to_string());

    for expect in expects {
        let cur_token = lex.next();
        assert_eq!(expect.kind, cur_token.kind);
        assert_eq!(expect.literal, cur_token.literal);
    }
}
