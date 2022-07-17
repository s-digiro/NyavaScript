mod environment;
use environment::Environment as Env;
pub use environment::*;

use crate::expression::{ Value, ValRef, Lambda, List, Macro };

pub fn evaluate(valref: ValRef, env: &mut Env) -> ValRef {
    match *valref {
        Value::ConsCell(_) => eval_list(valref, env),
        _ => valref,
    }
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
                _ => List::nil(),
            }
        },
    }
}

fn exec_lambda(
    lambda: &Lambda,
    args: ValRef,
    env: &mut Env
) -> ValRef {
    env.push(Scope::new());

    for (key, val) in lambda.args().into_iter().zip(List::iter(&args)) {
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

    for (key, val) in m.args().into_iter().zip(List::iter(&args)) {
        env.set(key.to_owned(), val);
    }

    let ret = evaluate(m.definition(), env);

    env.pop();

    ret
}
