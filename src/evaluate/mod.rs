mod environment;
use environment::Environment as Env;
pub use environment::*;

use crate::data::list::*;
use crate::parse::*;

pub fn evaluate(expr: &ExRef, env: &mut Env) -> ExRef {
    fn nuke() -> ! {
        panic!("This should never be called. Calling evaluate on Atoms is deprecated and should be removed. If you get here, this is an error")
    }

    match expr {
        ExRef::Defun(d) => eval_defun(d, env),
        ExRef::Label(l) => ExRef::Label(l.clone()),
        ExRef::Lambda(l) => ExRef::Lambda(l.clone()),
        ExRef::List(l) => eval_list(l, env),
        ExRef::Nil => ExRef::Nil,
        ExRef::Number(n) => ExRef::Number(*n),
        ExRef::Quote(q) => *q.clone(),
        ExRef::String(s) => ExRef::String(s.to_owned()),
        ExRef::Atom(_)
        | ExRef::RustFn(_) => nuke(),
    }
}

fn eval_atom<'a>(atom: &str, env: &'a Env) -> &'a ExRef {
    match env.get(&atom) {
        Some(ExRef::Atom(a)) => eval_atom(&a, env),
        Some(e) => e,
        None => &ExRef::Nil,
    }
}

fn eval_defun(defun: &Defun, env: &mut Env) -> ExRef {
    let Defun { name, lambda } = defun;

    env.defun(name.to_owned(), lambda.clone());

    ExRef::Nil
}

fn eval_list(list: &Vec<ExRef>, env: &mut Env) -> ExRef {
    if list.len() == 0 {
        return ExRef::Nil
    }

    let car = list.get(0).unwrap();

    let car_val = match car {
        ExRef::Atom(a) => eval_atom(a, env),
        e => e,
    };

    if env.has_macro("first if first is a string") {
        // do macro
    }

    let cdr = list.iter().skip(1)
        .map(|e| evaluate(e, env))
        .collect::<Vec<ExRef>>();

    match car_val {
        ExRef::Lambda(l) => exec_lambda(l, cdr, env),
        ExRef::Label(l) => exec_label(l, cdr, env),
        _ => {
            let mut ret = vec![car_val.clone()];
            ret.append(&mut cdr);

            ExRef::List(ret)
        }
    }

}

fn exec_lambda(
    lambda: &Lambda,
    mut args: Vec<ExRef>,
    env: &mut Env
) -> ExRef {
    let Lambda { args: keys, expr } = lambda;

    if keys.len() > args.len() {
        args.resize(keys.len(), ExRef::Nil);
    }

    env.push(Scope::new());

    for (key, val) in keys.into_iter().zip(args.into_iter()) {
        env.set(key.to_owned(), val);
    }

    let ret = evaluate(expr, env);

    env.pop();

    ret
}

fn exec_label(label: &Label, args: Vec<ExRef>, env: &mut Env) -> ExRef {
    let Label { name, lambda } = label;

    env.push(Scope::new());
    env.set(name.to_owned(), *lambda.clone());

    let ret = exec_lambda(lambda.as_lambda().unwrap(), args, env);

    env.pop();

    ret
}
