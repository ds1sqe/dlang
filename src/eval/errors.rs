use crate::{
    ast::Expression,
    object::{Object, ObjectType},
    token::Kind,
};

#[derive(Debug, Clone, PartialEq)]
pub enum EvalError {
    BlankResult,
    EnvironmentHasDropped,

    LetStatementValueIsNone,
    EvaluationOfExpressionIsNone(Expression),
    LeftExpressionIsNone,
    RightExpressionIsNone,

    NotABoolean(Object),
    NotAFunction(Object),

    ConditionIsNone,
    FunctionIsNone,
    ElementIsNone,
    ArrayIsNone,

    IdentifierNotFound(String),

    NotSameType,
    NotArray,

    IndexIsNotAInt(Object),
    IndexIsNegative(Object),
    IndexOutOfRange(IndexErrorDetail),

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

#[derive(Debug, Clone, PartialEq)]
pub struct IndexErrorDetail {
    pub array_length: usize,
    pub called_with: usize,
}
