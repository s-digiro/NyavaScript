mod environment;
use environment::Environment as Env;
pub use environment::*;

use crate::expression::{ Atom, ConsCell, Expression, ExRef, Lambda, List, Macro };

pub fn evaluate(expr: ExRef, env: &mut Env) -> ExRef {
    fn nuke() -> ! {
        panic!("This should never be called. Calling evaluate on Atoms is deprecated and should be removed. If you get here, this is an error")
    }

    match expr.as_expression() {
        Some(Expression::Atom(a)) => eval_atom(a, env),
        Some(Expression::ConsCell(c)) => eval_list(c, env),
        Some(Expression::Lambda(_))
        | Some(Expression::Macro(_))
        | Some(Expression::RustLambda(_)) 
        | Some(Expression::RustMacro(_)) => expr,
        None => ExRef::nil(),
    }
}

fn eval_atom(atom: &Atom, env: &Env) -> ExRef {
    match atom {
        Atom::Number(n) => Atom::number(*n),
        Atom::String(s) => Atom::string(&s),
        Atom::Symbol(s) => env.get(&s),
    }
}

fn eval_list(list: &ConsCell, env: &mut Env) -> ExRef {
    let arg1 = evaluate(ExRef::clone(&list.car), env);

    let list = match arg1.as_expression() {
        Some(Expression::Macro(m)) => exec_macro(m, list, env),
        Some(Expression::RustMacro(m)) => m.exec(list, env),
        _ => List::cons(&list.car, &list.cdr),
    };

    let list = List::iter(&list)
        .map(|exref| evaluate(exref, env)).collect::<ExRef>();

    match List::car(&list).as_expression() {
        Some(Expression::Lambda(l)) =>
            exec_lambda(l, &list.as_list().unwrap(), env),
        Some(Expression::RustLambda(l)) => l.exec(&list.as_list().unwrap(), env),
        _ => list,
    }
}

fn exec_lambda(
    lambda: &Lambda,
    args: &ConsCell,
    env: &mut Env
) -> ExRef {
    env.push(Scope::new());

    for (key, val) in lambda.args().into_iter().zip(args.into_iter()) {
        env.set(key.to_owned(), val);
    }

    let ret = evaluate(lambda.definition(), env);

    env.pop();

    ret
}

fn exec_macro(
    m: &Macro,
    args: &ConsCell,
    env: &mut Env
) -> ExRef {
    env.push(Scope::new());

    for (key, val) in m.args().into_iter().zip(args.iter()) {
        env.set(key.to_owned(), val);
    }

    let ret = evaluate(m.definition(), env);

    env.pop();

    ret
}
