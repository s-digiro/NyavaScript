mod environment;
use environment::Environment as Env;
pub use environment::*;

use crate::parse::Expression as Expr;
use crate::parse::*;

pub fn evaluate(expr: &Expr, env: &mut Env) -> Expr {
    fn nuke() -> ! {
        panic!("This should never be called. Calling evaluate on Atoms is deprecated and should be removed. If you get here, this is an error")
    }

    match expr {
        Expr::Defun(d) => eval_defun(d, env),
        Expr::Label(l) => Expr::Label(l.clone()),
        Expr::Lambda(l) => Expr::Lambda(l.clone()),
        Expr::List(l) => eval_list(l, env),
        Expr::Nil => Expr::Nil,
        Expr::Number(n) => Expr::Number(*n),
        Expr::Quote(q) => *q.clone(),
        Expr::String(s) => Expr::String(s.to_owned()),
        Expr::Atom(_)
        | Expr::RustFn(_) => nuke(),
    }
}

fn eval_atom<'a>(atom: &str, env: &'a Env) -> &'a Expr {
    match env.get(&atom) {
        Some(Expr::Atom(a)) => eval_atom(&a, env),
        Some(e) => e,
        None => &Expr::Nil,
    }
}

fn eval_defun(defun: &Defun, env: &mut Env) -> Expr {
    let Defun { name, lambda } = defun;

    env.defun(name.to_owned(), lambda.clone());

    Expr::Nil
}

fn eval_list(list: &Vec<Expr>, env: &mut Env) -> Expr {
    if list.len() == 0 {
        return Expr::Nil
    }

    let car = list.get(0).unwrap();

    let car_val = match car {
        Expr::Atom(a) => eval_atom(a, env),
        e => e,
    };

    if env.has_macro("first if first is a string") {
        // do macro
    }

    let cdr = list.iter().skip(1)
        .map(|e| evaluate(e, env))
        .collect::<Vec<Expr>>();

    match car_val {
        Expr::Lambda(l) => exec_lambda(l, cdr, env),
        Expr::Label(l) => exec_label(l, cdr, env),
        _ => {
            let mut ret = vec![car_val.clone()];
            ret.append(&mut cdr);

            Expr::List(ret)
        }
    }

}

fn exec_lambda(
    lambda: &Lambda,
    mut args: Vec<Expr>,
    env: &mut Env
) -> Expr {
    let Lambda { args: keys, expr } = lambda;

    if keys.len() > args.len() {
        args.resize(keys.len(), Expr::Nil);
    }

    env.push(Scope::new());

    for (key, val) in keys.into_iter().zip(args.into_iter()) {
        env.set(key.to_owned(), val);
    }

    let ret = evaluate(expr, env);

    env.pop();

    ret
}

fn exec_label(label: &Label, args: Vec<Expr>, env: &mut Env) -> Expr {
    let Label { name, lambda } = label;

    env.push(Scope::new());
    env.set(name.to_owned(), *lambda.clone());

    let ret = exec_lambda(lambda.as_lambda().unwrap(), args, env);

    env.pop();

    ret
}
