mod environment;
use environment::Environment as Env;
pub use environment::*;

mod error;
pub use error::*;

#[cfg(test)]
mod test;

pub type Result = std::result::Result<SXRef, Error>;

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


pub fn evaluate(sx: SXRef, env: &mut Env) -> Result {
    let id = get_id();

    eprintln!("{} Evaluating: {}", id, sx);

    let ret = match &*sx {
        SX::ConsCell(_) => eval_list(sx, env)?,
        SX::Quote(r) => SXRef::clone(r),
        SX::Symbol(s) => env.get(s),
        _ => sx,
    };

    eprintln!("{} Returning: {}", id, ret);

    Ok(ret)
}

fn eval_list(list: SXRef, env: &mut Env) -> Result {
    let first = util::car(&list);

    let proc = evaluate(SXRef::clone(&first), env)?;

    let rest = util::cdr(&list);

    let list = util::cons(&proc, &rest);

    match &*proc {
        SX::Macro(m) => {
            let new_list = m.execute(list, env)?;
            let ret = evaluate(new_list, env)?;
            Ok(ret)
        },
        SX::Function(f) => {
            let args = rest.iter()
                .map(|sx| evaluate(sx, env))
                .collect::<std::result::Result<Vec<SXRef>, Error>>()?;
            let ret = f.execute(args, env)?;
            Ok(ret)
        },
        _ => Err(UnboundFnCallError::new(first).into()),
    }
}
