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
    pub fn cload(args: Vec<SXRef>, env: &mut Env) -> EvalResult {
        if let Some(arg1) = args.get(0) {
            if let SX::String(path) = &**arg1 {
                if let Ok(scope) = DynCLibScope::load(path) {
                    env.dynclib.push(scope);
                    return Ok(SXRef::nil())
                }
            }
        }

        Ok(SXRef::number(-1))
    }

    pub fn deref(args: Vec<SXRef>, _env: &mut Env) -> EvalResult {
        if let Some(arg1) = args.get(0) {
            match &**arg1 {
                SX::Number(n) => {
                    let n = *n;
                    let ret = unsafe { *(n as *const u8) };

                    Ok(SXRef::number(ret.into()))
                },
                _ => Ok(SXRef::nil()),
            }
        } else {
            Ok(SXRef::nil())
        }
    }

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
            "cload".into(),
            RustFunction::new(Self::cload).into(),
        );

        ret.insert(
            "deref".into(),
            RustFunction::new(Self::deref).into(),
        );

        ret.insert(
            "println".into(),
            RustFunction::new(
                |args, _env| {
                    match args.get(0) {
                        Some(sx) => match &**sx {
                            SX::Nil => println!(),
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
            "set-at".into(),
            RustFunction::new(Self::set_at).into(),
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

    pub fn set_at(args: Vec<SXRef>, _env: &mut Env) -> EvalResult {
        let arg1 = args.get(0);
        let arg2 = args.get(1);

        match (arg1, arg2) {
            (Some(arg1), Some(arg2)) => {
                match &**arg1 {
                    SX::Number(n) => {
                        let ptr = *n as *mut u8;

                        match &**arg2 {
                            SX::Number(n) => unsafe {
                                let valptr = *n as *const u8;
                                *ptr = *valptr;

                                let ptr = ptr.offset(1);
                                let valptr = valptr.offset(1);
                            },
                            _ => unimplemented!(),
                        }

                        Ok(SXRef::nil())
                    },
                    _ => Ok(SXRef::number(-1)),
                }
            },
            _ => Ok(SXRef::number(-1)),
        }
    }
}
