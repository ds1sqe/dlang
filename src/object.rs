use std::fmt::Debug;

pub trait Object: Debug {}

#[derive(Debug)]
pub struct Int {
    pub value: i64,
}

impl Object for Int {}

#[derive(Debug)]
pub struct Bool {
    pub value: bool,
}

impl Object for Bool {}
