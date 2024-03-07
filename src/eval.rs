use std::any::Any;

use crate::{
    ast::{
        BooleanLiteral, ExpressionStatement, IntegerLiteral, LetStatement, Node,
        ReturnStatement, Statement,
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
    let ref_ro = &stm as &dyn Any;

    downcast!(ref_ro {
        let_stm is LetStatement=> {
        },
        return_stm is ReturnStatement=> {
        },
        exp_stm is ExpressionStatement=> {
        evaluate(Box::new(exp_stm));
        },
    } if_not_matched {
    });
}

pub fn evaluate(root: Box<dyn Node>) -> Result<Box<dyn Object>, ()> {
    let ref_ro = &root as &dyn Any;

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
      return Err(());
    });
}
