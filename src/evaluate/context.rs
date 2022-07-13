use std::collections::HashMap;

use super::*;

pub struct Scope {
    map: HashMap<String, Expr>
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            map: HashMap::new(),
        }
    }
}

pub struct Context {
    stack: Vec<Scope>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            stack: vec![
                Scope::new(),
                Scope::new(),
            ],
        }
    }

    pub fn defun(&mut self, key: String, lambda: Lambda) {
        self.stack[1].map.insert(key, Expr::Lambda(lambda));
    }

    pub fn has(&self, key: &str) -> bool {
        self.stack.iter().any(|c| c.map.contains_key(key))
    }

    pub fn pop(&mut self) -> Scope {
        self.stack.pop().unwrap()
    }

    pub fn push(&mut self, c: Scope) {
        self.stack.push(c);
    }

    pub fn set(&mut self, key: String, val: Expr) {
        self.stack.last_mut().unwrap().map.insert(key, val);
    }

    pub fn val(&self, key: &str) -> Option<Expr> {
        for c in self.stack.iter().rev() {
            if let Some(e) = c.map.get(key) {
                return Some(e.clone())
            }
        }

        None
    }
}
