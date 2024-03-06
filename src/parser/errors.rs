use std::fmt::Debug;

use crate::lexer::Position;

pub trait ParserError: Debug {
    fn detail(&self) -> String;
    fn position(&self) -> Position;
    // Do we need
    // fn next(&self) -> Option<&dyn ParserError>;
    // to show error stack?
}

#[derive(Debug)]
pub struct ParseError {
    pub detail: String,
    pub position: Position,
}

impl ParserError for ParseError {
    fn detail(&self) -> String {
        self.detail.clone()
    }
    fn position(&self) -> Position {
        self.position.clone()
    }
}

#[derive(Debug)]
pub struct PrefixFunctionError {
    pub detail: String,
    pub position: Position,
    pub kind: PrefixFunctionErrorKind,
}

impl ParserError for PrefixFunctionError {
    fn detail(&self) -> String {
        self.detail.clone()
    }

    fn position(&self) -> Position {
        self.position.clone()
    }
}

#[derive(Debug)]
pub enum PrefixFunctionErrorKind {
    NoPrefixFunction,
    IntegerParseError,
    ParentheseError,
    PrefixExpressionError,
    IfExpressionError,
    GroupExpressionError,
    FunctionLiteralError,
}

#[derive(Debug)]
pub struct InfixFunctionError {
    pub detail: String,
    pub position: Position,
    pub kind: InfixFunctionErrorKind,
}

impl ParserError for InfixFunctionError {
    fn detail(&self) -> String {
        self.detail.clone()
    }

    fn position(&self) -> Position {
        self.position.clone()
    }
}

#[derive(Debug)]
pub enum InfixFunctionErrorKind {
    PrecedenceNotFound,
    NoInfixFunction,
    ParseError,
}
