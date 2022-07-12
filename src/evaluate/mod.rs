mod context;
pub use context::*;

use crate::parse::Expression as Expr;
use crate::parse::*;

pub fn evaluate(expr: Expr, context: &mut Context) -> Expr {
    match expr {
        Expr::Atom(a) => eval_atom(a, context),
        Expr::Defun(d) => eval_defun(d, context),
        Expr::List(l) => eval_list(l, context),
        other => other,
    }
}

fn eval_atom(atom: String, context: &mut Context) -> Expr {
    if context.has(&atom) {
        context.val(&atom).unwrap()
    } else {
        Expr::Atom(atom)
    }
}

fn eval_defun(defun: Defun, context: &mut Context) -> Expr {
    let Defun { name, lambda } = defun;

    context.defun(name, lambda);

    Expr::Nil
}

fn eval_list(list: Vec<Expr>, context: &mut Context) -> Expr {
    if list.len() == 0 {
        return Expr::Nil
    }

    let mut list: Vec<Expr> = list.into_iter()
        .map(|e| evaluate(e, context))
        .collect();

    if list[0].is_lambda() {
        let l = list.remove(0).into_lambda().unwrap();

        return exec_lambda(l, list, context)
    }

    if list[0].is_label() {
        let l = list.remove(0).into_label().unwrap();

        return exec_label(l, list, context)
    }

    return Expr::List(list)
}

fn exec_lambda(
    lambda: Lambda,
    mut args: Vec<Expr>,
    context: &mut Context
) -> Expr {
    let Lambda { args: keys, expr } = lambda;

    if keys.len() > args.len() {
        args.resize(keys.len(), Expr::Nil);
    }

    context.push(Scope::new());

    for (key, val) in keys.into_iter().zip(args.into_iter()) {
        context.set(key, val);
    }

    let ret = evaluate(*expr, context);

    context.pop();

    ret
}

fn exec_label(label: Label, args: Vec<Expr>, context: &mut Context) -> Expr {
    let Label { name, lambda } = label;

    context.push(Scope::new());
    context.set(name, Expr::Lambda(lambda.clone()));

    let ret = exec_lambda(lambda, args, context);

    context.pop();

    ret
}
