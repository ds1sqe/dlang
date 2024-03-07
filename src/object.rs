pub trait Object {}

pub struct Int {
    pub value: i64,
}

impl Object for Int {}

pub struct Bool {
    pub value: bool,
}

impl Object for Bool {}
