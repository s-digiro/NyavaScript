#[cfg(test)]
mod test;

use super::Scope;
use crate::evaluate::{
    evaluate,
    Environment as Env,
};
use crate::s_expression::{
    Function,
    RustFunction,
    RustMacro,
    SExpression as SX,
    SExpressionRef as SXRef,
    util,
};

static PROCEDURAL: &'static str = "(lambda () (()))";

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
           Function::try_from(PROCEDURAL).unwrap().into(),
        );

        ret.insert(
            "println".into(),
            RustFunction::new(
                |args, _env| {
                    match args.get(0) {
                        Some(sx) => match &**sx {
                            SX::Nil => println!(),
                            SX::Macro(_) => println!("[macro]"),
                            SX::Function(_) => println!("[lambda]"),
                            SX::Number(n) => println!("{}", n),
                            SX::String(s) => println!("{}", s),
                            SX::Symbol(s) => println!("{}", s),
                            SX::Quote(q) => println!("'{}", q),
                            SX::ConsCell(c) => println!("{}", c),
                            SX::RustMacro(_) => println!("[rust macro]"),
                            SX::RustFunction(_) => println!("[rust function]"),
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
            println!("first pre eval: {:?}", first);
            let first = evaluate(first, env);
            println!("first post eval: {:?}", first);

            let mut last = first;

            for arg in it {
                println!("pipe arg: {:?}", arg);
                let arg = util::push(&arg, &SXRef::quote(last));
                println!("final: {:?}", arg);
                last = evaluate(arg, env);
            }

            last
        } else {
            SXRef::nil()
        }
    }
}
