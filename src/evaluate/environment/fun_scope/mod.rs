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

        ret
    }

    pub fn load(args: Vec<SXRef>, env: &mut Env) -> EvalResult {
        unimplemented!()
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
}
