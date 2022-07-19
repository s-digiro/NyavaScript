use crate::s_expression::SExpressionRef as SXRef;
use std::collections::HashMap;

mod mccarthy_scope;
pub use mccarthy_scope::McCarthyScope;

mod fun_scope;
pub use fun_scope::FunScope;

pub type Scope = HashMap<String, SXRef>;

#[derive(Debug)]
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
        self.stack.iter().any(|s| s.contains_key(key))
    }

    pub fn has_macro(&self, _key: &str) -> bool {
        false
    }

    pub fn get(&self, key: &str) -> SXRef {
        self.stack.iter().rev()
            .find_map(|s| s.get(key).map(|exref| SXRef::clone(exref)))
            .unwrap_or(SXRef::nil())
    }

    pub fn pop(&mut self) -> Scope {
        self.stack.pop().unwrap()
    }

    pub fn push(&mut self, c: Scope) {
        self.stack.push(c);
    }

    pub fn set(&mut self, key: String, val: SXRef) {
        self.stack.last_mut().unwrap().insert(key, val);
    }
}
