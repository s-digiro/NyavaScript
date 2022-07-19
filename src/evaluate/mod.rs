mod environment;
use environment::Environment as Env;
pub use environment::*;

use crate::s_expression::{
    Function,
    List,
    Macro,
    SExpression,
    SExpressionRef as SXRef,
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
    let first = evaluate(List::car(&list), env);
    let rest = List::cdr(&list);

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
            let ret = m.exec(rest, env);
            println!("RustMacro out: {}", ret);
            println!();
            evaluate(ret, env)
        },
        _ => {
            let rest = SExpression::from_iter(
                List::iter(&rest).map(|valref| evaluate(valref, env))
            );

            match &*first {
                SExpression::Function(l) => {
                    println!("Function in: {}", rest);
                    let ret = exec_lambda(&l, rest, env);
                    println!("Function out: {}", ret);
                    println!();

                    ret
                },
                SExpression::RustLambda(l) => {
                    println!("RustLambda in: {}", rest);
                    let ret = l.exec(rest, env);
                    println!("RustLambda out: {}", ret);
                    println!();

                    ret
                },
                _ => {
                    println!("fn {} on args {} evaluated to nil!", first, rest);
                    List::nil()
                },
            }
        },
    }
}

fn exec_lambda(
    f: &Function,
    args: SXRef,
    env: &mut Env
) -> SXRef {
    eprintln!("exec function args ->");
    env.push(Scope::new());

    for (key, val) in f.args().into_iter().zip(List::iter(&args)) {
        eprintln!("    {} = {}", key, val);
        env.set(key.to_owned(), val);
    }

    let ret = evaluate(f.definition(), env);

    env.pop();

    ret
}

fn exec_macro(
    m: &Macro,
    args: SXRef,
    env: &mut Env
) -> SXRef {
    env.push(Scope::new());

    if let Some(key) = m.args().first() {
        env.set(key.to_owned(), List::car(&args));
    }

    let ret = evaluate(m.definition(), env);

    env.pop();

    ret
}
