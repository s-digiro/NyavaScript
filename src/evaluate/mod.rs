mod environment;
use environment::Environment as Env;
pub use environment::*;

#[cfg(test)]
mod test;

use crate::s_expression::{
    SExpression as SX,
    SExpressionRef as SXRef,
    util,
};

use std::sync::atomic::{AtomicUsize, Ordering};

fn get_id() -> usize {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}


pub fn evaluate(sx: SXRef, env: &mut Env) -> SXRef {
    let id = get_id();

    eprintln!("{} Evaluating: {}", id, sx);

    let ret = match &*sx {
        SX::ConsCell(_) => eval_list(sx, env),
        SX::Quote(r) => SXRef::clone(r),
        SX::Symbol(s) => env.get(s),
        _ => sx,
    };

    eprintln!("{} Returning: {}", id, ret);

    ret
}

fn eval_list(list: SXRef, env: &mut Env) -> SXRef {
    let first = evaluate(util::car(&list), env);

    let rest = util::cdr(&list);

    let list = util::cons(&first, &rest);

    match &*first {
        SX::Macro(m) => evaluate(m.execute(list, env), env),
        SX::Function(f) => {
            let args = rest.iter().map(|sx| evaluate(sx, env)).collect();
            f.execute(args, env)
        },
        _ => SXRef::nil(),
    }
}
