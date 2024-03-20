use crate::{
    ast::{
        Expression, InfixExpression, Node, Nodetrait, PrefixExpression, Statement,
    },
    object::{
        environment::Environment, is_same_type, Bool, Function, Int, Object,
        ObjectTrait, ObjectType, Return,
    },
    token::Kind,
};

pub fn evaluate(
    node: Node,
    env: &mut Environment<String>,
) -> Result<Option<Object>, ()> {
    match node {
        Node::Statement(stm) => eval_stm(stm, env),
        Node::Expression(exp) => eval_exp(exp, env),
    }
}

fn eval_stm(
    stm: Statement,
    env: &mut Environment<String>,
) -> Result<Option<Object>, ()> {
    match stm {
        Statement::LetStatement(stm) => {
            let ident = stm.identifier;

            let result = eval_exp(stm.value.unwrap(), env);
            if result.is_ok() {
                let value = result.unwrap();
                if value.is_some() {
                    env.set(ident.value, value.unwrap());
                    return Ok(None);
                }
                return Err(());
            } else {
                return Err(());
            }
        }

        Statement::ExpressionStatement(stm) => {
            let exp = stm.expression.unwrap();

            eval_exp(exp, env)
        }

        Statement::BlockStatement(stm) => {
            let stms = stm.statements;
            // result of evaluation of statement block
            let mut result: Result<Option<Object>, ()>;

            // clone outer-context here
            let mut env = Environment::new_inner(env.clone());

            // initialize result to prepare case of blank block
            result = Ok(None);

            for stm in stms {
                result = eval_stm(stm, &mut env);
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
                                    } else {
                                        return Ok(None);
                                    }
                                }
                                // else continue evaluation of block statement
                                _ => {}
                            },
                            None => {}
                        }
                    }
                }
            }
            return result;
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
    env: &mut Environment<String>,
) -> Result<Option<Object>, ()> {
    match exp {
        Expression::Identifier(id_exp) => {
            let key = id_exp.value;
            let obj = env.get_clone(&key);
            if obj.is_some() {
                Ok(obj)
            } else {
                // identifier not found
                Err(())
            }
        }
        Expression::IntegerLiteral(lit) => {
            Ok(Some(Object::Int(Int { value: lit.value })))
        }
        Expression::BooleanLiteral(lit) => {
            Ok(Some(Object::Bool(Bool { value: lit.value })))
        }
        Expression::FunctionLiteral(func) => {
            let fun = Function {
                args: func.parameters,
                block: func.body,
                // have to clone to catch the current lexical environment
                env: env.clone(),
            };
            // if this function have identifier, bind to environment
            if func.ident.is_some() {
                env.set(
                    func.ident.unwrap().to_str(),
                    Object::Function(fun.clone()),
                );
            }
            Ok(Some(Object::Function(fun)))
        }

        Expression::InfixExpression(exp) => eval_infix_exp(exp, env),
        Expression::PrefixExpression(exp) => eval_prefix_exp(exp, env),
        Expression::IfExpression(_) => todo!(),
        Expression::CallExpression(_) => todo!(),
    }
}

fn eval_infix_exp(
    exp: InfixExpression,
    env: &mut Environment<String>,
) -> Result<Option<Object>, ()> {
    // check left, right is valid
    let left = eval_exp(*exp.left, env);
    if left.is_err() {
        return left;
    }
    let left = left.unwrap();
    if left.is_none() {
        return Err(());
    }
    let left = left.unwrap();

    let right = eval_exp(*exp.right, env);
    if right.is_err() {
        return right;
    }
    let right = right.unwrap();
    if right.is_none() {
        return Err(());
    }
    let right = right.unwrap();

    if !is_same_type(&left, &right) {
        return Err(());
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
            return Ok(result);
        }
        ObjectType::Bool => {
            let Object::Bool(left) = left else {unreachable!()};
            let Object::Bool(right) = right else {unreachable!()};

            let result = eval_infix_bool_exp(left, exp.operator.kind, right);
            if result.is_err() {
                return Err(result.err().unwrap());
            }
            let result = Some(result.unwrap());
            return Ok(result);
        }
        ObjectType::Function | ObjectType::Return => {
            return Err(());
        }
    }
}

fn eval_infix_int_exp(left: Int, operator: Kind, right: Int) -> Result<Object, ()> {
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
                // TODO: add err
                return Err(());
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
        _ => Err(()),
    }
}

fn eval_infix_bool_exp(
    left: Bool,
    operator: Kind,
    right: Bool,
) -> Result<Object, ()> {
    match operator {
        Kind::And | Kind::Bit_And => Ok(Object::Bool(Bool {
            value: left.value && right.value,
        })),
        Kind::Or | Kind::Bit_Or => Ok(Object::Bool(Bool {
            value: left.value || right.value,
        })),
        __ => {
            // add err
            Err(())
        }
    }
}

fn eval_prefix_exp(
    exp: PrefixExpression,
    env: &mut Environment<String>,
) -> Result<Option<Object>, ()> {
    let operator = exp.token.kind;

    // evaluate first
    let result = eval_exp(*exp.right, env);

    if result.is_err() {
        return result;
    }

    let result = result.unwrap();
    if result.is_none() {
        // add err
        return Err(());
    }
    let obj = result.unwrap();

    match obj.get_type() {
        ObjectType::Int => {
            let Object::Int(obj) = obj else {unreachable!()};

            let result = eval_prefix_int_exp(operator, obj);
            if result.is_err() {
                return Err(result.err().unwrap());
            }
            return Ok(Some(result.unwrap()));
        }
        ObjectType::Bool => {
            let Object::Bool(obj) = obj else {unreachable!()};

            let result = eval_prefix_bool_exp(operator, obj);
            if result.is_err() {
                return Err(result.err().unwrap());
            }
            return Ok(Some(result.unwrap()));
        }
        ObjectType::Function | ObjectType::Return => Err(()),
    }
}

fn eval_prefix_int_exp(operator: Kind, right: Int) -> Result<Object, ()> {
    match operator {
        Kind::Bang => Ok(Object::Int(Int {
            value: !right.value,
        })),
        Kind::Minus => Ok(Object::Int(Int {
            value: -right.value,
        })),
        __ => Err(()),
    }
}

fn eval_prefix_bool_exp(operator: Kind, right: Bool) -> Result<Object, ()> {
    match operator {
        Kind::Bang => Ok(Object::Bool(Bool {
            value: !right.value,
        })),
        __ => Err(()),
    }
}
