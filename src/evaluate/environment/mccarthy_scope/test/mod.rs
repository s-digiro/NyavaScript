use crate::evaluate::Environment as Env;
use crate::s_expression::{
    ConsCell,
    SExpressionRef as SXRef,
};
use std::rc::Rc;
use super::*;

mod atom;
mod car;
mod cdr;
mod cond;
mod cons;
mod defun;
mod equal;
mod lambda;

pub fn dummy_fn(_: &Vec<SXRef>, _: &mut Env) -> SXRef {
    SXRef::nil()
}

pub fn dummy_macro(_: SXRef, _: &mut Env) -> SXRef {
    SXRef::nil()
}


