use crate::{
    ast::{
        BooleanLiteral, Expression, ExpressionStatement, FunctionLiteral, IntegerLiteral,
        LetStatement, ReturnStatement, Statement,
    },
    object::{Bool, Int, Object},
};

macro_rules! downcast {
    ($src:ident { $($dst:ident is $ty:ty => $body:block ,)+ } if_not_matched $ifnot_matched:block )=> {
        $(if let Some($dst) = $src.downcast_ref::<$ty>() {
            $body
        })+
        $ifnot_matched
    }
}

pub fn manage_stm(stm: Box<dyn Statement>) {
    let ref_ro = stm.to_any();

    downcast!(ref_ro {
        let_stm is LetStatement=> {
        },
        return_stm is ReturnStatement=> {
        },
        exp_stm is ExpressionStatement=> {
        let rst = evaluate(Box::new(exp_stm.expression.as_ref().unwrap().as_ref()));
        println!("{:?}",rst);
        },
    } if_not_matched {
    });
}

pub fn evaluate(root: Box<&dyn Expression>) -> Result<Box<dyn Object>, bool> {
    let ref_ro = root.to_any();

    downcast!(ref_ro {
        int_lit is IntegerLiteral => {
            return Ok(Box::new(Int {
                value: int_lit.value
            }));
        },
        bool_lit is BooleanLiteral => {
            return Ok(Box::new(Bool{
                value: bool_lit.value
            }));
        },
    } if_not_matched {
      return Err(false)
    });
    unreachable!()
}
