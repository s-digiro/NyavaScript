mod dyn_c_lib;
use dyn_c_lib::{ DynCLib, DynCSym };

use std::fmt::{ Debug, Formatter, Result as fmtResult };
use super::Scope;

#[derive(Debug)]
pub struct DynCLibScope {
    lib: DynCLib,
    path: String,
}

impl DynCLib {
    pub fn load(path: &str) -> Result<DynCLibScope, ()> {
    }

    pub fn contains_key(&self, key: &str) -> bool {
        match self.lib.get_sym(key) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn get(&self, key: &str) -> Option<SXRef> {
        let ptr = match self.lib.get_sym(key) {
            Some(ptr) => ptr,
            None => return None,
        };

    }
}
