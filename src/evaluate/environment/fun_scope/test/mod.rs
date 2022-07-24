mod pipe;

use super::*;
use crate::s_expression::SExpressionRef as SXRef;
use crate::evaluate::Environment as Env;
use crate::evaluate::McCarthyScope;

fn env() -> Env {
    let mut ret = Env::new();

    ret.push(McCarthyScope::new());

    ret
}
