use std::{collections::HashMap, hash::Hash};

use super::Object;

#[derive(Debug, Clone)]
pub struct Environment<T: Hash + Eq + PartialEq> {
    binding: HashMap<T, Object>,
    outer: Option<Box<Self>>,
    // 0 for global, the bigger is the outter
    level: usize,
}

impl<T: Hash + Eq + PartialEq> Environment<T> {
    // get object ref from environment
    pub fn get(&self, key: &T) -> Option<&Object> {
        // first, try get object from current scope
        let rst = self.binding.get(key);

        if rst.is_none() && self.outer.is_some() {
            // if not found, try get object from outer scope
            return self.outer.as_ref().unwrap().get(key);
        }
        return rst;
    }

    // get object clone from environment
    pub fn get_clone(&self, key: &T) -> Option<Object> {
        // first, try get object from current scope
        let rst = self.binding.get(key);

        if rst.is_none() && self.outer.is_some() {
            // if not found, try get object from outer scope
            return self.outer.as_ref().unwrap().get_clone(key);
        }
        return rst.cloned();
    }

    // set object to environment
    pub fn set(&mut self, key: T, obj: Object) -> Option<Object> {
        self.binding.insert(key, obj)
    }

    pub fn new() -> Self {
        Environment {
            binding: HashMap::new(),
            outer: None,
            level: 0,
        }
    }

    pub fn new_inner(outer: Self) -> Self {
        Environment {
            binding: HashMap::new(),
            level: outer.level + 1,
            outer: Some(Box::new(outer)),
        }
    }
}
