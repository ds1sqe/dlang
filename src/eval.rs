pub mod errors;

use std::rc::Rc;

use std::cell::RefCell;

use crate::{
    ast::{
        CallExpression, Expression, IfExpression, IndexExpression, InfixExpression,
        Node, Nodetrait, PrefixExpression, Program, Statement,
    },
    object::{
        environment::{Environ, Environment},
        is_same_type, Array, Bool, Function, Int, Object, ObjectTrait, ObjectType,
        Return, StringObject,
    },
    token::Kind,
};

use self::errors::{ArgumentsLength, EvalError, IndexErrorDetail};

pub fn evaluate(
    node: Node,
    env: &Environ<String>,
) -> Result<Option<Object>, EvalError> {
    match node {
        Node::Program(pro) => eval_program(pro, env),
        Node::Statement(stm) => eval_stm(stm, env),
        Node::Expression(exp) => eval_exp(exp, env),
    }
}

fn eval_program(
    pro: Program,
    env: &Environ<String>,
) -> Result<Option<Object>, EvalError> {
    let mut result: Result<Option<Object>, EvalError> = Err(EvalError::BlankResult);

    for stm in pro.statements {
        result = eval_stm(stm, env);

        match result.clone() {
            Ok(opt) => match opt {
                Some(obj) => match obj {
                    Object::Return(rtv) => match rtv.value {
                        Some(val) => {
                            return Ok(Some(*val));
                        }
                        None => {
                            return Ok(None);
                        }
                    },
                    __ => {}
                },
                None => {
                    result = Ok(opt);
                }
            },
            Err(err) => return Err(err),
        }
    }

    result
}

fn eval_stm(
    stm: Statement,
    env: &Environ<String>,
) -> Result<Option<Object>, EvalError> {
    match stm {
        Statement::LetStatement(stm) => {
            let ident = stm.identifier;

            if stm.value.is_none() {
                return Err(EvalError::LetStatementValueIsNone);
            }

            let result = eval_exp(stm.value.clone().unwrap(), env);
            if result.is_ok() {
                let value = result.unwrap();
                if value.is_some() {
                    let obj = value.unwrap();

                    // if obj is a function,
                    if obj.get_type() == ObjectType::Function {
                        let Object::Function(mut fun) = obj else {unreachable!()};
                        fun.identifier = Some(ident.clone().value);
                        env.borrow_mut()
                            .set(ident.clone().value, Object::Function(fun));
                    // if obj is not a function,
                    } else {
                        env.borrow_mut().set(ident.clone().value, obj);
                    }

                    return Ok(None);
                }
                Err(EvalError::EvaluationOfExpressionIsNone(stm.value.unwrap()))
            } else {
                Err(result.err().unwrap())
            }
        }

        Statement::ExpressionStatement(stm) => {
            let exp = stm.expression.unwrap();

            eval_exp(exp, env)
        }

        Statement::BlockStatement(stm) => {
            let stms = stm.statements;
            // result of evaluation of statement block
            let mut result: Result<Option<Object>, EvalError>;

            // clone outer-context here
            let env = Rc::new(RefCell::new(Environment::new_inner(env)));

            // initialize result to prepare case of blank block
            result = Ok(None);

            for stm in stms {
                result = eval_stm(stm, &env);
                // this is clone for prenventing falty error of move
                match result.clone() {
                    Err(_) => {
                        // if there was error, stop evaluation
                        return result;
                    }
                    Ok(rst) => {
                        match rst {
                            Some(obj) => match obj {
                                // catch unwrap return and skip block
                                Object::Return(rt) => {
                                    if rt.value.is_some() {
                                        let val: Object = *rt.value.unwrap();
                                        return Ok(Some(val));
                                    }
                                    return Ok(None);
                                }
                                // else continue evaluation of block statement
                                _ => {}
                            },
                            None => {}
                        }
                    }
                }
            }
            result
        }

        Statement::ReturnStatement(stm) => {
            if stm.value.is_none() {
                return Ok(Some(Object::Return(Return { value: None })));
            }
            let val = eval_exp(stm.value.unwrap(), env);
            if val.is_err() {
                return val;
            }

            let return_val = val.unwrap();
            if return_val.is_none() {
                return Ok(Some(Object::Return(Return { value: None })));
            }
            // unwrap and rewrap with box
            let value = Some(Box::new(return_val.unwrap()));

            Ok(Some(Object::Return(Return { value })))
        }
    }
}

fn eval_exp(
    exp: Expression,
    env: &Environ<String>,
) -> Result<Option<Object>, EvalError> {
    match exp {
        Expression::Identifier(id_exp) => {
            let key = id_exp.value;
            let obj = env.borrow().get_clone(&key);
            if obj.is_some() {
                Ok(obj)
            } else {
                // identifier not found
                Err(EvalError::IdentifierNotFound(key.clone()))
            }
        }
        Expression::IntegerLiteral(lit) => {
            Ok(Some(Object::Int(Int { value: lit.value })))
        }
        Expression::BooleanLiteral(lit) => {
            Ok(Some(Object::Bool(Bool { value: lit.value })))
        }
        Expression::StringLiteral(lit) => {
            Ok(Some(Object::String(StringObject { value: lit.value })))
        }

        Expression::FunctionLiteral(func) => {
            let mut fun = Function {
                identifier: None,
                args: func.parameters,
                block: func.body,
                // have to clone to catch the current lexical environment
                env: Rc::downgrade(&env),
            };

            // if this function have identifier, bind to environment
            if func.ident.is_some() {
                fun.identifier = Some(func.ident.as_ref().unwrap().to_str());
                env.borrow_mut().set(
                    func.ident.unwrap().to_str(),
                    Object::Function(fun.clone()),
                );
            }
            Ok(Some(Object::Function(fun)))
        }

        Expression::ArrayLiteral(arr) => {
            let mut elements = Vec::new();

            for exp in arr.elements {
                let obj = eval_exp(exp, env);
                if obj.is_err() {
                    return obj;
                }
                if obj.as_ref().unwrap().is_none() {
                    return Err(EvalError::ElementIsNone);
                }
                elements.push(obj.unwrap().unwrap())
            }
            Ok(Some(Object::Array(Array { elements })))
        }

        Expression::InfixExpression(exp) => eval_infix_exp(exp, env),
        Expression::PrefixExpression(exp) => eval_prefix_exp(exp, env),
        Expression::IfExpression(exp) => eval_if_exp(exp, env),
        Expression::CallExpression(exp) => eval_call_exp(exp, env),
        Expression::IndexExpression(exp) => eval_index_exp(exp, env),
    }
}

fn eval_infix_exp(
    exp: InfixExpression,
    env: &Environ<String>,
) -> Result<Option<Object>, EvalError> {
    // check left, right is valid
    let left = eval_exp(*exp.left, env);
    if left.is_err() {
        return left;
    }
    let left = left.unwrap();
    if left.is_none() {
        return Err(EvalError::LeftExpressionIsNone);
    }
    let left = left.unwrap();

    let right = eval_exp(*exp.right, env);
    if right.is_err() {
        return right;
    }
    let right = right.unwrap();
    if right.is_none() {
        return Err(EvalError::RightExpressionIsNone);
    }
    let right = right.unwrap();

    if !is_same_type(&left, &right) {
        return Err(EvalError::NotSameType);
    }

    match left.get_type() {
        ObjectType::Int => {
            let Object::Int(left) = left else {unreachable!()};
            let Object::Int(right) = right else {unreachable!()};
            let result = eval_infix_int_exp(left, exp.operator.kind, right);
            if result.is_err() {
                return Err(result.err().unwrap());
            }
            let result = Some(result.unwrap());
            Ok(result)
        }
        ObjectType::Bool => {
            let Object::Bool(left) = left else {unreachable!()};
            let Object::Bool(right) = right else {unreachable!()};

            let result = eval_infix_bool_exp(left, exp.operator.kind, right);
            if result.is_err() {
                return Err(result.err().unwrap());
            }
            let result = Some(result.unwrap());
            Ok(result)
        }
        ObjectType::String => {
            let Object::String(left) = left else {unreachable!()};
            let Object::String(right) = right else {unreachable!()};
            let result = eval_infix_string_exp(left, exp.operator.kind, right);
            if result.is_err() {
                return Err(result.err().unwrap());
            }
            let result = Some(result.unwrap());
            Ok(result)
        }
        ObjectType::Array | ObjectType::Function | ObjectType::Return => {
            Err(EvalError::InvalidInfixOperationTarget(
                left.get_type(),
                exp.operator.kind,
            ))
        }
    }
}

fn eval_infix_int_exp(
    left: Int,
    operator: Kind,
    right: Int,
) -> Result<Object, EvalError> {
    match operator {
        Kind::Plus => {
            let value = left.value + right.value;
            Ok(Object::Int(Int { value }))
        }
        Kind::Minus => {
            let value = left.value - right.value;
            Ok(Object::Int(Int { value }))
        }
        Kind::Product => {
            let value = left.value * right.value;
            Ok(Object::Int(Int { value }))
        }
        Kind::Divide => {
            if right.value == 0 {
                return Err(EvalError::DivideWithZero);
            }
            let value = left.value / right.value;
            Ok(Object::Int(Int { value }))
        }
        Kind::Mod => {
            let value = left.value % right.value;
            Ok(Object::Int(Int { value }))
        }
        Kind::LT => Ok(Object::Bool(Bool {
            value: left.value < right.value,
        })),
        Kind::LT_OR_EQ => Ok(Object::Bool(Bool {
            value: left.value <= right.value,
        })),
        Kind::GT => Ok(Object::Bool(Bool {
            value: left.value > right.value,
        })),
        Kind::GT_OR_EQ => Ok(Object::Bool(Bool {
            value: left.value >= right.value,
        })),
        Kind::EQ => Ok(Object::Bool(Bool {
            value: left.value == right.value,
        })),
        Kind::NOT_EQ => Ok(Object::Bool(Bool {
            value: left.value != right.value,
        })),
        Kind::Bit_And => Ok(Object::Int(Int {
            value: left.value & right.value,
        })),
        Kind::Bit_Or => Ok(Object::Int(Int {
            value: left.value | right.value,
        })),
        oper => Err(EvalError::InvalidIntegerInfixOperation(oper)),
    }
}

fn eval_infix_bool_exp(
    left: Bool,
    operator: Kind,
    right: Bool,
) -> Result<Object, EvalError> {
    match operator {
        Kind::And | Kind::Bit_And => Ok(Object::Bool(Bool {
            value: left.value && right.value,
        })),
        Kind::Or | Kind::Bit_Or => Ok(Object::Bool(Bool {
            value: left.value || right.value,
        })),
        Kind::EQ => Ok(Object::Bool(Bool {
            value: left.value == right.value,
        })),
        Kind::NOT_EQ => Ok(Object::Bool(Bool {
            value: left.value != right.value,
        })),

        oper => {
            // add err
            Err(EvalError::InvalidBoolInfixOperation(oper))
        }
    }
}
fn eval_infix_string_exp(
    left: StringObject,
    operator: Kind,
    right: StringObject,
) -> Result<Object, EvalError> {
    match operator {
        Kind::EQ => Ok(Object::Bool(Bool {
            value: left.value == right.value,
        })),
        Kind::NOT_EQ => Ok(Object::Bool(Bool {
            value: left.value != right.value,
        })),
        Kind::Plus => {
            let value = left.value + &right.value;
            Ok(Object::String(StringObject { value }))
        }

        oper => {
            // add err
            Err(EvalError::InvalidStringInfixOperation(oper))
        }
    }
}

fn eval_prefix_exp(
    exp: PrefixExpression,
    env: &Environ<String>,
) -> Result<Option<Object>, EvalError> {
    let operator = exp.token.kind;

    // evaluate first
    let result = eval_exp(*exp.right.clone(), env);

    if result.is_err() {
        return result;
    }

    let result = result.unwrap();
    if result.is_none() {
        return Err(EvalError::EvaluationOfExpressionIsNone(*exp.right));
    }
    let obj = result.unwrap();

    match obj.get_type() {
        ObjectType::Int => {
            let Object::Int(obj) = obj else {unreachable!()};

            let result = eval_prefix_int_exp(operator, obj);
            if result.is_err() {
                return Err(result.err().unwrap());
            }
            Ok(Some(result.unwrap()))
        }
        ObjectType::Bool => {
            let Object::Bool(obj) = obj else {unreachable!()};

            let result = eval_prefix_bool_exp(operator, obj);
            if result.is_err() {
                return Err(result.err().unwrap());
            }
            Ok(Some(result.unwrap()))
        }
        ObjectType::Array
        | ObjectType::Function
        | ObjectType::Return
        | ObjectType::String => Err(EvalError::InvalidPrefixOperationTarget(
            obj.get_type(),
            operator,
        )),
    }
}

fn eval_prefix_int_exp(operator: Kind, right: Int) -> Result<Object, EvalError> {
    match operator {
        Kind::Bang => Ok(Object::Int(Int {
            value: !right.value,
        })),
        Kind::Minus => Ok(Object::Int(Int {
            value: -right.value,
        })),
        oper => Err(EvalError::InvalidIntegerPrefixOperation(oper)),
    }
}

fn eval_prefix_bool_exp(operator: Kind, right: Bool) -> Result<Object, EvalError> {
    match operator {
        Kind::Bang => Ok(Object::Bool(Bool {
            value: !right.value,
        })),
        oper => Err(EvalError::InvalidBoolPrefixOperation(oper)),
    }
}

fn eval_if_exp(
    exp: IfExpression,
    env: &Environ<String>,
) -> Result<Option<Object>, EvalError> {
    let condition_val = eval_exp(*exp.condition, env);
    if condition_val.is_err() {
        return condition_val;
    };

    let obj = condition_val.unwrap();
    if obj.is_none() {
        return Err(EvalError::ConditionIsNone);
    }

    let object = obj.unwrap();
    let Object::Bool(flag) = object else { return Err(EvalError::NotABoolean(object))};

    if flag.value {
        return eval_stm(Statement::BlockStatement(exp.consequence), env);
    }
    if exp.alternative.is_some() {
        return eval_stm(Statement::BlockStatement(exp.alternative.unwrap()), env);
    }

    Ok(None)
}

fn eval_call_exp(
    exp: CallExpression,
    env: &Environ<String>,
) -> Result<Option<Object>, EvalError> {
    let func = eval_exp(*exp.function, env);

    if func.is_err() {
        return Err(func.err().unwrap());
    }
    if func.clone().unwrap().is_none() {
        return Err(EvalError::FunctionIsNone);
    }
    let func = func.unwrap().unwrap();
    match func {
        Object::Function(func) => {
            let args = eval_function_parameters(exp.arguments, env);
            if args.is_err() {
                return Err(args.err().unwrap());
            }
            let args = args.unwrap();
            apply_function(func, args)
        }
        // func is not a function
        obj => Err(EvalError::NotAFunction(obj)),
    }
}

fn eval_function_parameters(
    args: Vec<Expression>,
    env: &Environ<String>,
) -> Result<Vec<Object>, EvalError> {
    let mut result: Vec<Object> = Vec::new();

    for (_, arg) in args.iter().enumerate() {
        let evaluated = eval_exp(arg.clone(), env);

        if evaluated.is_err() {
            return Err(evaluated.err().unwrap());
        }
        if evaluated.clone().unwrap().is_none() {
            return Err(EvalError::EvaluationOfExpressionIsNone(arg.clone()));
        }

        result.push(evaluated.unwrap().unwrap())
    }

    Ok(result)
}

fn apply_function(
    fun: Function,
    args: Vec<Object>,
) -> Result<Option<Object>, EvalError> {
    if args.len() != fun.args.len() {
        return Err(EvalError::FunctionArgLengthNotMatched(ArgumentsLength {
            function_args: fun.args.len(),
            called_with: args.len(),
        }));
    }

    let extended_env = extend_function_env(fun.clone(), args);

    if extended_env.is_err() {
        return Err(extended_env.unwrap_err());
    }

    let evaluated =
        eval_stm(Statement::BlockStatement(fun.block), &extended_env.unwrap());

    if evaluated.is_err() {
        return evaluated;
    }
    let evaluated = evaluated.unwrap();

    Ok(unwrap_return_value(evaluated))
}

fn extend_function_env(
    fun: Function,
    args: Vec<Object>,
) -> Result<Environ<String>, EvalError> {
    let opt_rc = &fun.env.upgrade();

    if opt_rc.is_none() {
        return Err(EvalError::EnvironmentHasDropped);
    }

    let mut env = Environment::new_inner(&opt_rc.clone().unwrap());

    // bind given args(object) to fun's parameters(ident)
    for (idx, arg) in fun.args.iter().enumerate() {
        env.set(arg.value.clone(), args[idx].clone());
    }

    Ok(Rc::new(RefCell::new(env)))
}
/// unwrap return value to object.
/// if given obj is not a return value, don't do anything
fn unwrap_return_value(obj: Option<Object>) -> Option<Object> {
    if obj.is_none() {
        return obj;
    }
    let obj = obj.unwrap();

    match obj.clone() {
        Object::Return(rtv) => match rtv.value {
            Some(val) => Some(*val),
            None => None,
        },
        __ => Some(obj),
    }
}

fn eval_index_exp(
    exp: IndexExpression,
    env: &Environ<String>,
) -> Result<Option<Object>, EvalError> {
    let left_rst = eval_exp(*exp.left, env);

    if left_rst.is_err() {
        return left_rst;
    }
    if left_rst.as_ref().unwrap().is_none() {
        return Err(EvalError::ArrayIsNone);
    }
    let left = left_rst.unwrap().unwrap();

    if left.get_type() != ObjectType::Array {
        return Err(EvalError::NotArray);
    }

    let index_rst = eval_exp(*exp.index, env);
    if index_rst.is_err() {
        return index_rst;
    }
    if index_rst.as_ref().unwrap().is_none() {
        return Err(EvalError::ArrayIsNone);
    }
    let index = index_rst.unwrap().unwrap();
    if index.get_type() != ObjectType::Int {
        return Err(EvalError::IndexIsNotAInt(index));
    }

    let Object::Array(arr) = left else {unreachable!()};
    let Object::Int(idx) = index else {unreachable!()};

    if idx.value < 0 {
        return Err(EvalError::IndexIsNegative(index));
    }

    let idx = idx.value as usize;

    if arr.elements.len() < idx {
        return Err(EvalError::IndexOutOfRange(IndexErrorDetail {
            array_length: arr.elements.len(),
            called_with: idx as usize,
        }));
    }

    return Ok(Some(arr.elements[idx as usize].clone()));
}
