use super::{ ExRef, List };

mod lambda;
pub use lambda::Lambda;

mod r#macro;
pub use r#macro::Macro;

mod rust_lambda;
pub use rust_lambda::RustLambda;

mod rust_macro;
pub use rust_macro::RustMacro;

#[derive(Debug, PartialEq)]
pub struct Label {
    pub name: String,
    pub lambda: Lambda,
}


#[derive(Debug, PartialEq)]
pub struct Defun {
    pub name: String,
    pub lambda: Lambda,
}


fn first_arg_is(e: &ExRef, symbol: &str) -> bool {
    List::car(&e).as_atom()
        .map(|a| a.as_symbol().map(|s| s == symbol).unwrap_or(false))
        .unwrap_or(false)
}

fn second_arg_is_list_of_symbols(e: &ExRef) -> bool {
    let arg2 = List::car(&List::cdr(&e));

    arg2.is_list() && List::iter(&arg2).all(|e| e.as_atom().map(|a| a.is_symbol()).unwrap_or(false))
}
