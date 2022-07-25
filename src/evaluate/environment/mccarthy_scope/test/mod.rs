use crate::evaluate::{
    Environment as Env,
    Result as EvalResult,
};
use crate::s_expression::SExpressionRef as SXRef;
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
mod not;
mod null;
mod or;
mod quote;

fn dummy_fn(_: Vec<SXRef>, _: &mut Env) -> EvalResult {
    Ok(SXRef::nil())
}

fn dummy_macro(_: SXRef, _: &mut Env) -> EvalResult {
    Ok(SXRef::nil())
}

fn mc_env() -> Env {
    let mut ret = Env::new();

    ret.push(McCarthyScope::new());

    ret
}
