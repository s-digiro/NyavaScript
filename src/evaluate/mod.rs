mod environment;
use environment::Environment as Env;
pub use environment::*;

use crate::expression::{ Atom, ConsCell, Expression, ValRef, Lambda, List, Macro };

pub fn evaluate(expr: ValRef, env: &mut Env) -> ValRef {
    match expr.as_expression() {
        Some(Expression::Atom(a)) => eval_atom(a, env),
        Some(Expression::ConsCell(c)) => eval_list(c, env),
        Some(Expression::Lambda(_))
        | Some(Expression::Macro(_))
        | Some(Expression::RustLambda(_)) 
        | Some(Expression::RustMacro(_)) => expr,
        None => ValRef::nil(),
    }
}

fn eval_atom(atom: &Atom, env: &Env) -> ValRef {
    match atom {
        Atom::Number(n) => Atom::number(*n),
        Atom::String(s) => Atom::string(&s),
        Atom::Symbol(s) => env.get(&s),
    }
}

fn eval_list(list: &ConsCell, env: &mut Env) -> ValRef {
    let list = List::cons(&list.car, &list.cdr);

    let first = evaluate(List::car(&list), env);
    let rest = List::cdr(&list);

    match first.as_expression() {
        Some(Expression::Macro(m)) => {
            println!("Macro in: {}", rest);
            let ret = exec_macro(m, rest, env);
            println!("Macro out: {}", ret);
            println!();
            evaluate(ret, env)
        },
        Some(Expression::RustMacro(m)) => {
            println!("RustMacro in: {}", rest);
            let ret = m.exec(rest, env);
            println!("RustMacro out: {}", ret);
            println!();
            evaluate(ret, env)
        },
        _ => {
            let rest = List::iter(&rest)
                .map(|exref| evaluate(exref, env)).collect::<ValRef>();

            match first.as_expression() {
                Some(Expression::Lambda(l)) => {
                    println!("Lambda in: {}", rest);
                    let ret = exec_lambda(l, rest, env);
                    println!("Lambda out: {}", ret);
                    println!();

                    ret
                },
                Some(Expression::RustLambda(l)) => {
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
