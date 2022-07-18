mod environment;
use environment::Environment as Env;
pub use environment::*;

use crate::s_expression::{ SExpression, SExpressionRef, Lambda, List, Macro };
use std::rc::Rc;

use std::sync::atomic::{AtomicUsize, Ordering};
fn get_id() -> usize {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}


pub fn evaluate(valref: SExpressionRef, env: &mut Env) -> SExpressionRef {
    let id = get_id();
    eprintln!("{} - evaling: {}", id, valref);
    let ret = match &*valref {
        SExpression::ConsCell(_) => eval_list(valref, env),
        SExpression::Quote(r) => Rc::clone(r),
        SExpression::Symbol(s) => env.get(s),
        _ => valref,
    };
    eprintln!("{} - returning: {}", id, ret);

    ret
}

fn eval_list(list: SExpressionRef, env: &mut Env) -> SExpressionRef {
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
                SExpression::Lambda(l) => {
                    println!("Lambda in: {}", rest);
                    let ret = exec_lambda(&l, rest, env);
                    println!("Lambda out: {}", ret);
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
    lambda: &Lambda,
    args: SExpressionRef,
    env: &mut Env
) -> SExpressionRef {
    eprintln!("exec lambda args ->");
    env.push(Scope::new());

    for (key, val) in lambda.args().into_iter().zip(List::iter(&args)) {
        eprintln!("    {} = {}", key, val);
        env.set(key.to_owned(), val);
    }

    let ret = evaluate(lambda.definition(), env);

    env.pop();

    ret
}

fn exec_macro(
    m: &Macro,
    args: SExpressionRef,
    env: &mut Env
) -> SExpressionRef {
    env.push(Scope::new());

    if let Some(key) = m.args().first() {
        env.set(key.to_owned(), List::car(&args));
    }

    let ret = evaluate(m.definition(), env);

    env.pop();

    ret
}
