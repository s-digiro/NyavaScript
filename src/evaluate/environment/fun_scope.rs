use super::Scope;
use crate::evaluate::evaluate;
use crate::value::{ Value, Lambda, List, Macro, RustLambda, RustMacro };

pub struct FunScope;

impl FunScope {
    pub fn new() -> Scope {
        let mut ret = Scope::new();

        ret.insert(
            "|>".into(),
            RustMacro::from(
                |args, env| {
                    let mut last = Value::nil();

                    for arg in List::iter(&args) {
                        let arg = List::push(&arg, &last);
                        last = Value::quote(evaluate(arg, env))
                    }

                    last
                }
            ),
        );

        ret.insert(
            ";".into(),
            Lambda::from("(lambda () (()))"),
        );

        ret.insert(
            "println".into(),
            RustLambda::from(
                |args, _env| {
                    match List::iter(&args).next() {
                        Some(val) => match &*val {
                            Value::Nil => println!(),
                            Value::Macro(_) => println!("[macro]"),
                            Value::Lambda(_) => println!("[lambda]"),
                            Value::Number(n) => println!("{}", n),
                            Value::String(s) => println!("{}", s),
                            Value::Symbol(s) => println!("{}", s),
                            Value::Quote(q) => println!("'{}", q),
                            Value::ConsCell(c) => println!("{}", c),
                            Value::RustMacro(_) => println!("[rust macro]"),
                            Value::RustLambda(_) => println!("[rust lambda]"),
                        },
                        None => println!(),
                    }

                    Value::nil()
                }
            ),
        );

        ret
    }
}
