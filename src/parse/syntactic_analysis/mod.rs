mod error;
pub use error::*;

mod syntax;
pub use syntax::Syntax;

#[cfg(test)]
mod test;

use std::mem;

use super::lexical_analysis::Token;

pub fn parse(symbols: Vec<Token>) -> Result<Syntax, SyntaxError> {
    let mut stack = Vec::new();
    let mut current = Syntax::List(Vec::new());

    for sym in symbols.into_iter() {
        match sym {
            Token::OpenList => stack.push(
                mem::replace(&mut current, Syntax::list())
            ),
            Token::CloseList => {
                let child = mem::replace(
                    &mut current,
                    stack.pop().ok_or(SyntaxError::UnmatchedCloseListError)?
                );

                current.as_mut_list().unwrap().push(child);
            },
            Token::Symbol(a) => current.as_mut_list().unwrap()
                .push(Syntax::Atom(a)),
            Token::Quote => current.as_mut_list().unwrap()
                .push(Syntax::atom("quote")),
            Token::String(s) => current.as_mut_list().unwrap()
                .push(Syntax::String(s)),
            Token::Number(num) => current.as_mut_list().unwrap()
                .push(Syntax::Number(num)),
        }
    }

    if !stack.is_empty() {
        return Err(SyntaxError::UnmatchedOpenListError)
    }

    Ok(current)
}