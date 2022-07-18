mod environment;
use environment::Environment as Env;
pub use environment::*;

use crate::value::{ Value, ValRef, Lambda, List, Macro };
use std::rc::Rc;

use std::sync::atomic::{AtomicUsize, Ordering};
fn get_id() -> usize {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}


pub fn evaluate(valref: ValRef, env: &mut Env) -> ValRef {
    let id = get_id();
    eprintln!("{} - evaling: {}", id, valref);
    let ret = match &*valref {
        Value::ConsCell(_) => eval_list(valref, env),
        Value::Quote(r) => Rc::clone(r),
        Value::Symbol(s) => env.get(s),
        _ => valref,
    };
    eprintln!("{} - returning: {}", id, ret);

    ret
}

fn eval_list(list: ValRef, env: &mut Env) -> ValRef {
    let first = evaluate(List::car(&list), env);
    let rest = List::cdr(&list);

    match &*first {
        Value::Macro(m) => {
            println!("Macro in: {}", rest);
            let ret = exec_macro(&m, rest, env);
            println!("Macro out: {}", ret);
            println!();
            evaluate(ret, env)
        },
        Value::RustMacro(m) => {
            println!("RustMacro in: {}", rest);
            let ret = m.exec(rest, env);
            println!("RustMacro out: {}", ret);
            println!();
            evaluate(ret, env)
        },
        _ => {
            let rest = Value::from_iter(
                List::iter(&rest).map(|valref| evaluate(valref, env))
            );

            match &*first {
                Value::Lambda(l) => {
                    println!("Lambda in: {}", rest);
                    let ret = exec_lambda(&l, rest, env);
                    println!("Lambda out: {}", ret);
                    println!();

                    ret
                },
                Value::RustLambda(l) => {
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
    args: ValRef,
    env: &mut Env
) -> ValRef {
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
    args: ValRef,
    env: &mut Env
) -> ValRef {
    env.push(Scope::new());

    if let Some(key) = m.args().first() {
        env.set(key.to_owned(), List::car(&args));
    }

    let ret = evaluate(m.definition(), env);

    env.pop();

    ret
}
