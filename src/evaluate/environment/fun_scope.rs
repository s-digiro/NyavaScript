use super::Scope;
use crate::evaluate::evaluate;
use crate::s_expression::{ SExpression, Function, List, Macro, RustLambda, RustMacro };

pub struct FunScope;

impl FunScope {
    pub fn new() -> Scope {
        let mut ret = Scope::new();

        ret.insert(
            "|>".into(),
            RustMacro::from(
                |args, env| {
                    let mut last = SExpression::nil();

                    for arg in List::iter(&args) {
                        let arg = List::push(&arg, &last);
                        last = SExpression::quote(evaluate(arg, env))
                    }

                    last
                }
            ),
        );

        ret.insert(
            ";".into(),
           Function::try_from("(lambda () (()))").unwrap().sxref(),
        );

        ret.insert(
            "println".into(),
            RustLambda::from(
                |args, _env| {
                    match List::iter(&args).next() {
                        Some(val) => match &*val {
                            SExpression::Nil => println!(),
                            SExpression::Macro(_) => println!("[macro]"),
                            SExpression::Function(_) => println!("[lambda]"),
                            SExpression::Number(n) => println!("{}", n),
                            SExpression::String(s) => println!("{}", s),
                            SExpression::Symbol(s) => println!("{}", s),
                            SExpression::Quote(q) => println!("'{}", q),
                            SExpression::ConsCell(c) => println!("{}", c),
                            SExpression::RustMacro(_) => println!("[rust macro]"),
                            SExpression::RustLambda(_) => println!("[rust lambda]"),
                        },
                        None => println!(),
                    }

                    SExpression::nil()
                }
            ),
        );

        ret
    }
}
