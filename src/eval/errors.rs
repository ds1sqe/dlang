use crate::{
    ast::Expression,
    object::{Object, ObjectType},
    token::Kind,
};

#[derive(Debug, Clone)]
pub enum EvalError {
    LetStatementValueIsNone,
    EvaluationOfExpressionIsNone(Expression),
    LeftExpressionIsNone,
    RightExpressionIsNone,

    NotABoolean(Object),
    NotAFunction(Object),

    ConditionIsNone,
    FunctionIsNone,

    IdentifierNotFound,

    NotSameType,

    FunctionArgLengthNotMatched(ArgumentsLength),

    DivideWithZero,

    InvalidPrefixOperationTarget(ObjectType),
    InvalidInfixOperationTarget(ObjectType),

    InvalidStringInfixOperation(Kind),

    InvalidIntegerInfixOperation(Kind),
    InvalidIntegerPrefixOperation(Kind),

    InvalidBoolInfixOperation(Kind),
    InvalidBoolPrefixOperation(Kind),
}

#[derive(Debug, Clone)]
pub struct ArgumentsLength {
    pub function_args: usize,
    pub called_with: usize,
}
