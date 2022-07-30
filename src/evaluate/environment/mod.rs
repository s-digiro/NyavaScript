use crate::s_expression::SExpressionRef as SXRef;
use std::collections::HashMap;

mod mccarthy_scope;
pub use mccarthy_scope::McCarthyScope;

mod fun_scope;
pub use fun_scope::FunScope;

#[cfg(test)]
mod test;

pub type Scope = HashMap<String, SXRef>;

#[derive(Debug)]
pub struct Environment {
    global: Scope,
    lib: Vec<Scope>,
    stack: Vec<Scope>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            stack: vec![Scope::new()],
            lib: vec![],
            global: Scope::new(),
        }
    }

    pub fn pop_lib(&mut self) -> Scope {
        self.lib.pop().unwrap()
    }

    pub fn push_lib(&mut self, c: Scope) {
        self.lib.push(c);
    }

    pub fn has(&self, key: &str) -> bool {
        self.stack.iter().any(|s| s.contains_key(key))
    }

    pub fn get(&self, key: &str) -> SXRef {
        let local = self.stack.iter().rev()
            .find_map(|s| s.get(key).map(|exref| SXRef::clone(exref)));

        if let Some(local) = local {
            return local
        }

        if let Some(global) = self.global.get(key) {
            return SXRef::clone(global)
        }

        let lib = self.lib.iter().rev()
            .find_map(|s| s.get(key).map(|exref| SXRef::clone(exref)));

        if let Some(lib) = lib {
            return lib
        }

        SXRef::nil()
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

    pub fn delete(&mut self, key: &str) {
        self.stack.last_mut().unwrap().remove(key);
    }

    pub fn defun(&mut self, key: String, val: SXRef) {
        self.global.insert(key, val);
    }
}
