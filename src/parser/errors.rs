use crate::ast::Expression;

#[derive(Debug)]
pub struct PrecedenceNotFound {}
#[derive(Debug)]
pub struct ParseError {}
#[derive(Debug)]
pub struct NoPrefixFunctionError {}
#[derive(Debug)]
pub enum PrefixFunctionError {
    NoPrefixFunction,
    IntegerParseError,
    ParentheseError,
    PrefixExpressionError,
    IfExpressionError,
    FunctionLiteralError,
}
#[derive(Debug)]
pub enum InfixFunctionError {
    PrecedenceNotFound,
    NoInfixFunction,
    ParseError,
}
#[derive(Debug)]
pub struct NoInfixFunctionError {}
#[derive(Debug)]
pub struct IntegerParseError {}
