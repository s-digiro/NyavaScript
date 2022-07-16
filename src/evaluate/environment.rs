use std::collections::HashMap;

use crate::expression::ExRef;

pub struct Scope {
    map: HashMap<String, ExRef>
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            map: HashMap::new(),
        }
    }
}

pub struct Environment {
    stack: Vec<Scope>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            stack: vec![
                Scope::new(),
            ],
        }
    }

    pub fn has(&self, key: &str) -> bool {
        self.stack.iter().any(|s| s.map.contains_key(key))
    }

    pub fn has_macro(&self, _key: &str) -> bool {
        false
    }

    pub fn get(&self, key: &str) -> ExRef {
        self.stack.iter().rev()
            .find_map(|s| s.map.get(key).map(|exref| ExRef::clone(exref)))
            .unwrap_or(ExRef::nil())
    }

    pub fn pop(&mut self) -> Scope {
        self.stack.pop().unwrap()
    }

    pub fn push(&mut self, c: Scope) {
        self.stack.push(c);
    }

    pub fn set(&mut self, key: String, val: ExRef) {
        self.stack.last_mut().unwrap().map.insert(key, val);
    }
}
