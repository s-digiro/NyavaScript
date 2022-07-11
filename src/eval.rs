use std::collections::HashMap;

use super::*;

struct Context<'a> {
    scope: HashMap<String, Expr>,
    parent: &'a mut Context<'a>,
}

impl<'a> Context<'a> {
    fn new(
        scope: HashMap<String, Expr>,
        parent: &'a mut Context<'a>
    ) -> Context<'a> {
        Context {
            scope,
            parent,
        }
    }
}

pub fn eval(expr: Expr, context: &mut Context) -> Expr {
    match expr {
        Expr::Atom(atom) => eval_atom(atom, context),
        Expr::List(list) => eval_list(list, context),
    }
}

pub fn eval_atom(atom: String, context: &mut Context) -> Expr {
    Expr::list()
}

fn eval_lambda<'a>(
    keys: Vec<String>,
    lambda: Expr,
    vals: Vec<Expr>,
    context: &'a mut Context<'a>
) -> Result<Expr, String> {
    if keys.len() != vals.len() {
        Err("number of values does not match number of keys".to_owned())
    } else {
        Ok(
            eval(
                lambda,
                &mut Context::new(
                    keys.into_iter().zip(vals.into_iter()).collect(),
                    context,
                ),
            )
        )
    }
}

fn eval_list(list: Vec<Expr>, context: &mut Context) -> Result<Expr, String> {
    if is_lambda(list)? {
        return eval_lambda(
            list.
    }
    let list = list.into_iter().map(|expr| eval(expr, context));

    Expr::list()
}

