use crate::{
    ast::{Expression, Statement},
    object::Object,
};

pub fn manage_stm(stm: Statement) {
    match stm {
        Statement::LetStatement(_) => todo!(),
        Statement::ExpressionStatement(_) => todo!(),
        Statement::ReturnStatement(_) => todo!(),
        Statement::BlockStatement(_) => todo!(),
    }
}

pub fn evaluate(exp: Expression) -> Result<(), ()> {
    match exp {
        Expression::Identifier(_) => todo!(),
        Expression::IntegerLiteral(_) => todo!(),
        Expression::BooleanLiteral(_) => todo!(),
        Expression::FunctionLiteral(_) => todo!(),
        Expression::InfixExpression(_) => todo!(),
        Expression::PrefixExpression(_) => todo!(),
        Expression::IfExpression(_) => todo!(),
        Expression::CallExpression(_) => todo!(),
    }
}
