use crate::evaluate::Environment as Env;
use crate::s_expression::{
    ConsCell,
    SExpressionRef as SXRef,
};
use std::rc::Rc;
use super::*;

mod and;
mod atom;
mod car;
mod cdr;
mod cond;
mod cons;
mod defun;
mod equal;
mod lambda;
mod null;
mod quote;

fn dummy_fn(_: &Vec<SXRef>, _: &mut Env) -> SXRef {
    SXRef::nil()
}

fn dummy_macro(_: SXRef, _: &mut Env) -> SXRef {
    SXRef::nil()
}

fn mc_env() -> Env {
    let mut ret = Env::new();

    ret.push(McCarthyScope::new());

    ret
}
