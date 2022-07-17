mod error;
pub use error::*;

mod syntax;
pub use syntax::Syntax;

#[cfg(test)]
mod test;

use std::mem;

use super::lexical_analysis::Token;

pub fn parse(symbols: Vec<Token>) -> Result<Syntax, SyntaxError> {
    if symbols.is_empty() {
        return Err(SyntaxError::NoSymbolsError)
    }

    if symbols.first() != Some(&Token::OpenList) { 
        return Err(SyntaxError::NoRootListError)
    }

    if symbols.last() != Some(&Token::CloseList) { 
        return Err(SyntaxError::UnclosedRootListError)
    }

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
                .push(Syntax::Symbol(a)),
            Token::Quote => current.as_mut_list().unwrap()
                .push(Syntax::symbol("quote")),
            Token::String(s) => current.as_mut_list().unwrap()
                .push(Syntax::String(s)),
            Token::Number(num) => current.as_mut_list().unwrap()
                .push(Syntax::Number(num)),
        }
    }

    if !stack.is_empty() {
        return Err(SyntaxError::UnmatchedOpenListError)
    }

    Ok(current.as_mut_list().unwrap().remove(0))
}
