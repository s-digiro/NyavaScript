mod error;
pub use error::*;

mod syntax;
pub use syntax::Syntax;

#[cfg(test)]
mod test;

use std::vec::IntoIter;
use super::lexical_analysis::Token;

pub fn parse(tokens: Vec<Token>) -> Result<Syntax, SyntaxError> {
    let mut tokens = tokens.into_iter();

    let ret = match tokens.next() {
        Some(tok) => match tok {
            Token::OpenList => {
                let (syn, it) = parse_list(tokens)?;

                if it.count() > 0 {
                    Err(SyntaxError::UnexpectedTrailingTokensError)
                } else {
                    Ok(syn)
                }
            }
            _ => Err(SyntaxError::NoRootListError),
        },
        None => Err(SyntaxError::NoSymbolsError),
    };

    ret
}

pub fn parse_quote(mut tokens: IntoIter<Token>) -> Result<(Syntax, IntoIter<Token>), SyntaxError> {
    let quoted = match tokens.next() {
        Some(tok) => match tok {
            Token::Symbol(s) => Syntax::Symbol(s),
            Token::String(s) => Syntax::String(s),
            Token::Number(n) => Syntax::Number(n),
            Token::OpenList => {
                let (syn, it) = parse_list(tokens)?;

                tokens = it;
                syn
            }
            Token::CloseList => return Err(SyntaxError::QuoteMissingItemError),
            Token::Quote => {
                let (syn, it) = parse_quote(tokens)?;

                tokens = it;
                syn
            }
        },
        None => return Err(SyntaxError::QuoteMissingItemError),
    };

    Ok((
        Syntax::List(vec![
            Syntax::symbol("quote"),
            quoted,
        ]),
        tokens
    ))
}

pub fn parse_list(mut tokens: IntoIter<Token>) -> Result<(Syntax, IntoIter<Token>), SyntaxError> {
    let mut ret = Vec::new();

    while let Some(tok) = tokens.next() {
        match tok {
            Token::Symbol(s) => ret.push(Syntax::Symbol(s)),
            Token::String(s) => ret.push(Syntax::String(s)),
            Token::Number(n) => ret.push(Syntax::Number(n)),
            Token::Quote => ret.push({
                let (syn, it) = parse_quote(tokens)?;

                tokens = it;
                syn
            }),
            Token::OpenList => ret.push({
                let (syn, it) = parse_list(tokens)?;

                tokens = it;
                syn
            }),
            Token::CloseList =>  {
                let mut x = false;
                if ret.len() == 1 {
                    if let Some(Syntax::List(l)) = ret.first() {
                        if let Some(Syntax::Symbol(s)) = l.first() {
                            if s == "quote" {
                                x = true;
                            }
                        }
                    }
                }

                if x {
                    ret = match ret.remove(0) {
                        Syntax::List(l) => l,
                        _ => panic!("This should never happen"),
                    }
                }
                return Ok((Syntax::List(ret), tokens))
            }
        }
    }


    Err(SyntaxError::UnmatchedOpenListError)
}
