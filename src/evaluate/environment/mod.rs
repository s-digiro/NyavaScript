use crate::s_expression::SExpressionRef as SXRef;
use std::collections::HashMap;

mod dyn_c_lib_scope;

mod mccarthy_scope;
pub use mccarthy_scope::McCarthyScope;

mod fun_scope;
pub use fun_scope::FunScope;

mod scope;
pub use scope::Scope;

#[cfg(test)]
mod test;

pub type HashScope = HashMap<String, SXRef>;

type BoxScope<'a> = Box<dyn Scope + 'a>;

#[derive(Debug)]
pub struct Environment<'a> {
    global: HashScope,
    lib: Vec<BoxScope<'a>>,
    stack: Vec<BoxScope<'a>>,
}

impl<'a> Environment<'a> {
    pub fn new() -> Environment<'a> {
        Environment {
            stack: vec![],
            lib: vec![],
            global: HashScope::new(),
        }
    }

    pub fn pop_lib(&mut self) -> Option<BoxScope> {
        self.lib.pop()
    }

    pub fn push_lib(&mut self, c: impl Scope + 'a) {
        self.lib.push(Box::new(c));
    }

    pub fn has(&self, key: &str) -> bool {
        self.stack.iter().any(|s| s.contains_key(key))
    }

    pub fn get(&self, key: &str) -> SXRef {
        let local = self.stack.iter().rev()
            .find_map(|s| s.get(key));

        if let Some(local) = local {
            return local
        }

        if let Some(global) = self.global.get(key) {
            return SXRef::clone(global)
        }

        let lib = self.lib.iter().rev()
            .find_map(|s| s.get(key));

        if let Some(lib) = lib {
            return lib
        }

        SXRef::nil()
    }

    pub fn pop(&mut self) -> Option<BoxScope> {
        self.stack.pop()
    }

    pub fn push(&mut self, c: impl Scope + 'a) {
        self.stack.push(Box::new(c));
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
