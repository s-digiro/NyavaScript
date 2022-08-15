mod cload;
mod deref;
mod pipe;
mod procedural;
mod set_at;

use crate::evaluate::Environment as Env;
use crate::evaluate::McCarthyScope;
use crate::s_expression::SExpressionRef as SXRef;
use super::*;

fn env<'a>() -> Env<'a> {
    let mut ret = Env::new();

    ret.push(McCarthyScope::new());

    ret
}

#[test]
fn all_fns_are_defined() {
    let subject = FunScope::new();

    let fns = [
        ";",
        "deref",
        "println",
        "set-at",
        "|>",
    ];

    let missing: Vec<&str> = fns.into_iter()
        .filter(|f| !subject.contains_key(*f))
        .collect();

    if !missing.is_empty() {
        panic!("Expected FunScope to contain a definition for the following keys {:?}", missing);
    }
}
