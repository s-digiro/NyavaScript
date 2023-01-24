#[cfg(test)]
mod test;

use super::*;
use crate::evaluate::{
    eval,
    Environment as Env,
    Result as EvalResult,
};
use crate::s_expression::{
    RustFunction,
    RustMacro,
    SExpression as SX,
    SExpressionRef as SXRef,
    util,
};

pub struct FunScope;

impl FunScope {
    pub fn new() -> HashScope {
        let mut ret = HashScope::new();

        ret.insert(
            "|>".into(),
            RustMacro::new(Self::pipe).into(),
        );

        ret.insert(
            ";".into(),
            RustMacro::new(Self::procedural).into(),
        );

        ret.insert(
            "println".into(),
            RustFunction::new(
                |args, _env| {
                    match args.get(0) {
                        Some(sx) => match &**sx {
                            SX::Nil => println!("[NIL]"),
                            SX::Macro(_) => println!("[macro]"),
                            SX::Function(_) => println!("[function]"),
                            SX::Number(n) => println!("{}", n),
                            SX::String(s) => println!("{}", s),
                            SX::Symbol(s) => println!("{}", s),
                            SX::Quote(q) => println!("'{}", q),
                            SX::ConsCell(c) => println!("{}", c),
                        },
                        None => println!(),
                    }

                    Ok(SXRef::nil())
                }
            ).into(),
        );

        ret.insert(
            "print".into(),
            RustFunction::new(
                |args, _env| {
                    match args.get(0) {
                        Some(sx) => match &**sx {
                            SX::Nil => print!("[NIL]"),
                            SX::Macro(_) => print!("[macro]"),
                            SX::Function(_) => print!("[function]"),
                            SX::Number(n) => print!("{}", n),
                            SX::String(s) => print!("{}", s),
                            SX::Symbol(s) => print!("{}", s),
                            SX::Quote(q) => print!("'{}", q),
                            SX::ConsCell(c) => print!("{}", c),
                        },
                        None => println!(),
                    }

                    Ok(SXRef::nil())
                }
            ).into(),
        );

        ret.insert(
            "define".into(),
            RustMacro::new(Self::define).into(),
        );

        ret.insert(
            "add".into(),
            RustFunction::new(Self::add).into(),
        );

        ret
    }

    pub fn pipe(sx: SXRef, env: &mut Env) -> EvalResult {
        let mut it = sx.iter().skip(1);

        if let Some(first) = it.next() {
            let first = eval(first, env)?;

            let mut last = first;

            for arg in it {
                let arg = util::push(&arg, &SXRef::quote(last));
                last = eval(arg, env)?;
            }

            Ok(last)
        } else {
            Ok(SXRef::nil())
        }
    }

    pub fn procedural(sx: SXRef, env: &mut Env) -> EvalResult {
        let mut last = SXRef::nil();

        for sx in sx.iter().skip(1) {
            last = eval(sx, env)?;
        }

        Ok(last)
    }

    pub fn define(sx: SXRef, env: &mut Env) -> EvalResult {
        let mut iter = sx.iter().skip(1);

        match iter.next() {
            Some(sxref) => match &*sxref {
                SX::Symbol(s) => {
                    let val = match iter.next() {
                        Some(sx) => SXRef::clone(&sx),
                        None => SXRef::nil(),
                    };

                    let val = eval(val, env)?;

                    env.defun(s.into(), val);
                }
                _ => (),
            },
            None => (),
        }

        Ok(SXRef::nil())
    }

    pub fn add(args: Vec<SXRef>, _env: &mut Env) -> EvalResult {
        let get_num = |n: usize| match args.get(n) {
            Some(sx) => match &**sx {
                SX::Number(num) => *num,
                _ => 0,
            },
            None => 0,
        };

        let a = get_num(1);
        let b = get_num(2);
        let ret = a + b;

        Ok(SXRef::number(ret))
    }
}
