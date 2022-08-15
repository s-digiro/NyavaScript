use crate::{
    evaluate::{
        DynCFunction,
        Environment as Env,
        Result as EvalResult,
    },
    s_expression::{ SExpressionRef as SXRef, SExpression as SX },
};
use std::{
    ffi::{ CString, OsStr },
    os::unix::ffi::OsStrExt,
};

#[derive(Clone, Debug, PartialEq)]
pub struct DynCLibFunction {
    f: DynCFunction,
    name: String,
}

impl DynCLibFunction {
    pub fn new(name: String, f: DynCFunction) -> DynCLibFunction {
        DynCLibFunction { f, name }
    }

    pub fn execute(&self, args: Vec<SXRef>, _env: &mut Env) -> EvalResult {
        let mut bufs = Vec::new();

        let args = args.iter().map(|sx| match &**sx {
            SX::Number(n) => *n as *const () as usize,
            SX::String(s) => {
                let s = OsStr::new(s);
                let s = CString::new(s.as_bytes()).unwrap();
                bufs.push(s);

                bufs.last().unwrap().as_ptr() as usize
            },
            _ => 0,
        }).collect();

        let ret = self.f.call(args);

        Ok(SXRef::number(ret as *const () as isize))
    }
}

impl std::fmt::Display for DynCLibFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[DynCLibFunction '{}']", self.name)
    }
}
