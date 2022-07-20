use super::Scope;
use crate::evaluate::evaluate;
use crate::s_expression::{
    SExpression as SX,
    SExpressionRef as SXRef,
    Function,
    List,
    RustFunction,
    RustMacro
};

pub struct FunScope;

impl FunScope {
    pub fn new() -> Scope {
        let mut ret = Scope::new();

        ret.insert(
            "|>".into(),
            RustMacro::from(
                |args, env| {
                    let mut last = SXRef::nil();

                    for arg in List::iter(&args) {
                        let arg = List::push(&arg, &last);
                        last = SXRef::quote(evaluate(arg, env))
                    }

                    last
                }
            ),
        );

        ret.insert(
            ";".into(),
           Function::try_from("(lambda () (()))").unwrap().into(),
        );

        ret.insert(
            "println".into(),
            RustFunction::from(
                |args, _env| {
                    match List::iter(&args).next() {
                        Some(val) => match &*val {
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
            ),
        );

        ret
    }
}
