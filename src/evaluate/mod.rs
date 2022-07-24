mod environment;
use environment::Environment as Env;
pub use environment::*;

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


pub fn evaluate(valref: SXRef, env: &mut Env) -> SXRef {
    let id = get_id();
    eprintln!("{} - evaling: {}", id, valref);
    let ret = match &*valref {
        SExpression::ConsCell(_) => eval_list(valref, env),
        SExpression::Quote(r) => SXRef::clone(r),
        SExpression::Symbol(s) => env.get(s),
        _ => valref,
    };
    eprintln!("{} - returning: {}", id, ret);

    ret
}

fn eval_list(list: SXRef, env: &mut Env) -> SXRef {
    let first = evaluate(util::car(&list), env);
    let rest = util::cdr(&list);

    let all = util::cons(&first, &rest);

    match &*first {
        SExpression::Macro(m) => {
            println!("Macro in: {}", rest);
            let ret = exec_macro(&m, rest, env);
            println!("Macro out: {}", ret);
            println!();
            evaluate(ret, env)
        },
        SExpression::RustMacro(m) => {
            println!("RustMacro in: {}", rest);
            let ret = m.exec(all, env);
            println!("RustMacro out: {}", ret);
            println!();
            evaluate(ret, env)
        },
        _ => {
            let args: Vec<SXRef> = rest.iter()
                .map(|sx| evaluate(sx, env))
                .collect();

            match &*first {
                SExpression::Function(l) => {
                    println!("Function in: {:?}", args);
                    let ret = l.execute(args, env);
                    println!("Function out: {}", ret);
                    println!();

                    ret
                },
                SExpression::RustFunction(l) => {
                    println!("RustFunction in: {:?}", args);
                    let ret = l.exec(&args, env);
                    println!("RustFunction out: {}", ret);
                    println!();

                    ret
                },
                _ => {
                    println!("fn {} on args {} evaluated to nil!", first, rest);
                    SXRef::nil()
                },
            }
        },
    }
}

fn exec_macro(
    m: &Macro,
    args: SXRef,
    env: &mut Env
) -> SXRef {
    env.push(Scope::new());

    if let Some(key) = m.args().first() {
        env.set(key.to_owned(), util::car(&args));
    }

    let ret = evaluate(m.definition(), env);

    env.pop();

    ret
}
