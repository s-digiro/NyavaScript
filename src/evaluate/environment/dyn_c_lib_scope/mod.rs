mod dyn_c_lib;
pub use dyn_c_lib::{ DynCLib, DynCFunction, Error as DynCLibErr };

use crate::s_expression::{
    SExpressionRef as SXRef,
    Function,
    DynCLibFunction,
};

#[derive(Debug)]
pub struct DynCLibScope {
    lib: DynCLib,
    path: String,
}

impl DynCLibScope {
    pub fn load(path: &str) -> Result<DynCLibScope, DynCLibErr> {
        let lib = DynCLib::dlopen(path)?;

        let ret = Self {
            lib,
            path: path.into(),
        };

        Ok(ret)
    }

    pub fn contains_key(&self, key: &str) -> bool {
        match self.lib.get_fn(key) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn get(&self, key: &str) -> Option<SXRef> {
        match self.lib.get_fn(key) {
            Ok(f) => {
                let f = DynCLibFunction::new(key.into(), f);
                let f = Function::DynCLib(f);
                let f = SXRef::function(f);
                Some(f)
            },
            Err(_) => None,
        }
    }
}
