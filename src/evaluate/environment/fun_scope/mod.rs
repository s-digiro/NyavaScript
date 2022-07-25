#[cfg(test)]
mod test;

use super::Scope;
use crate::evaluate::{
    evaluate,
    Environment as Env,
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
    pub fn new() -> Scope {
        let mut ret = Scope::new();

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
                            SX::RustMacro(_) => println!("[rust macro]"),
                        },
                        None => println!(),
                    }

                    SXRef::nil()
                }
            ).into(),
        );

        ret
    }

    pub fn pipe(sx: SXRef, env: &mut Env) -> SXRef {
        let mut it = sx.iter().skip(1);

        if let Some(first) = it.next() {
            let first = evaluate(first, env);

            let mut last = first;

            for arg in it {
                let arg = util::push(&arg, &SXRef::quote(last));
                last = evaluate(arg, env);
            }

            last
        } else {
            SXRef::nil()
        }
    }

    pub fn procedural(sx: SXRef, env: &mut Env) -> SXRef {
        let mut last = SXRef::nil();

        for sx in sx.iter().skip(1) {
            last = evaluate(sx, env);
        }

        last
    }
}
