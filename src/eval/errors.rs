use crate::{
    ast::Expression,
    object::{Object, ObjectType},
    token::Kind,
};

#[derive(Debug, Clone, PartialEq)]
pub enum EvalError {
    BlankResult,

    LetStatementValueIsNone,
    EvaluationOfExpressionIsNone(Expression),
    LeftExpressionIsNone,
    RightExpressionIsNone,

    NotABoolean(Object),
    NotAFunction(Object),

    ConditionIsNone,
    FunctionIsNone,

    IdentifierNotFound(String),

    NotSameType,

    FunctionArgLengthNotMatched(ArgumentsLength),

    DivideWithZero,

    InvalidPrefixOperationTarget(ObjectType, Kind),
    InvalidInfixOperationTarget(ObjectType, Kind),

    InvalidStringInfixOperation(Kind),

    InvalidIntegerInfixOperation(Kind),
    InvalidIntegerPrefixOperation(Kind),

    InvalidBoolInfixOperation(Kind),
    InvalidBoolPrefixOperation(Kind),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArgumentsLength {
    pub function_args: usize,
    pub called_with: usize,
}
