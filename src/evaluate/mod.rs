mod environment;
use environment::Environment as Env;
pub use environment::*;

#[cfg(test)]
mod test;

use crate::s_expression::{
    Function,
    Macro,
    SExpression,
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
        SExpression::ConsCell(_) => eval_list(sx, env),
        SExpression::Quote(r) => SXRef::clone(r),
        SExpression::Symbol(s) => env.get(s),
        _ => sx,
    };

    eprintln!("{} Returning: {}", id, ret);

    ret
}

fn eval_list(list: SXRef, env: &mut Env) -> SXRef {
    let first = evaluate(util::car(&list), env);
    let rest = util::cdr(&list);

    let all = util::cons(&first, &rest);

    match &*first {
        SExpression::Macro(m) => {
            let ret = m.execute(all, env);
            evaluate(ret, env)
        },
        SExpression::RustMacro(m) => {
            let ret = m.execute(all, env);
            evaluate(ret, env)
        },
        _ => {
            let args: Vec<SXRef> = rest.iter()
                .map(|sx| evaluate(sx, env))
                .collect();

            match &*first {
                SExpression::Function(l) => {
                    let ret = l.execute(args, env);

                    ret
                },
                _ => {
                    SXRef::nil()
                },
            }
        },
    }
}
