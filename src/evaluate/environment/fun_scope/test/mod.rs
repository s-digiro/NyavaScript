mod define;
mod pipe;
mod procedural;

use super::*;
use crate::s_expression::SExpressionRef as SXRef;
use crate::evaluate::Environment as Env;
use crate::evaluate::McCarthyScope;

fn env<'a>() -> Env<'a> {
    let mut ret = Env::new();

    ret.push(McCarthyScope::new());

    ret
}
