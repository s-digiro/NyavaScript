use crate::evaluate::{
    Environment as Env,
    Result as EvalResult,
};
use crate::s_expression::SExpressionRef as SXRef;
use super::*;

mod and;
mod atom;
mod cadr;
mod car;
mod cdr;
mod cond;
mod cons;
mod defun;
mod equal;
mod label;
mod lambda;
mod list;
mod nil;
mod not;
mod null;
mod or;
mod quote;
mod t;

fn dummy_fn(_: Vec<SXRef>, _: &mut Env) -> EvalResult {
    Ok(SXRef::nil())
}

fn dummy_macro(_: SXRef, _: &mut Env) -> EvalResult {
    Ok(SXRef::nil())
}

fn mc_env<'a>() -> Env<'a> {
    let mut ret = Env::new();

    ret.push(McCarthyScope::new());

    ret
}

#[test]
fn all_mccarthy_functions_are_defined() {
    let subject = McCarthyScope::new();

    let fns = [
        "NIL",
        "T",
        "and",
        "car",
        "cdr",
        "cond",
        "cons",
        "defun",
        "equal",
        "eval",
        "label",
        "lambda",
        "list",
        "not",
        "null",
        "or",
        "quote",
    ];

    let mut missing: Vec<&str> = fns.into_iter()
        .filter(|f| !subject.contains_key(*f))
        .collect();

    if let None = subject.get("cadr") {
        missing.push("cadr");
    }

    if !missing.is_empty() {
        panic!("Expected McCarthyScope to contain a definition for the following keys {:?}", missing);
    }
}
