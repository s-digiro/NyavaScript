mod error;
pub use error::*;

mod syntax;
pub use syntax::Syntax;

#[cfg(test)]
mod test;

use std::vec::IntoIter;
use super::lexical_analysis::Token;

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Syntax>, SyntaxError> {
    let mut tokens = tokens.into_iter();

    let mut ret = Vec::new();

    while let Some(token) = tokens.next() {
        match token {
            Token::OpenList => {
                let (syn, it) = parse_list(tokens)?;

                tokens = it;

                ret.push(syn);
            },
            Token::CloseList => return Err(SyntaxError::UnmatchedCloseListError),
            _ => return Err(SyntaxError::FreeAtom),
        }
    }

    Ok(ret)
}

pub fn parse_quote(mut tokens: IntoIter<Token>) -> Result<(Syntax, IntoIter<Token>), SyntaxError> {
    let quoted = match tokens.next() {
        Some(tok) => match tok {
            Token::Dot => todo!(),
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
            Token::Dot => return parse_list_remainder_as_dot(ret, tokens),
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

pub fn parse_list_remainder_as_dot(
    mut list: Vec<Syntax>,
    mut tokens: IntoIter<Token>
) -> Result<(Syntax, IntoIter<Token>), SyntaxError> {
    let car = match list.pop() {
        Some(x) => Some(x),
        None => None,
    };

    match tokens.next() {
        Some(x) => match x {
            Token::Dot => return Err(SyntaxError::BadInfixDotNotation),
            Token::OpenList => {
                let (syn, it) = parse_list(tokens)?;

                tokens = it;

                let ret = Syntax::dot(
                    car,
                    Some(syn),
                );

                list.push(ret);
                let ret = Syntax::List(list);

                match tokens.next() {
                    Some(Token::CloseList) => return Ok((ret, tokens)),
                    None => return Err(SyntaxError::UnmatchedOpenListError),
                    _ => return Err(SyntaxError::BadInfixDotNotation),
                }
            },
            Token::Quote => {
                let (syn, it) = parse_quote(tokens)?;

                tokens = it;

                let ret = Syntax::dot(
                    car,
                    Some(syn),
                );

                list.push(ret);
                let ret = Syntax::List(list);

                match tokens.next() {
                    Some(Token::CloseList) => return Ok((ret, tokens)),
                    None => return Err(SyntaxError::UnmatchedOpenListError),
                    _ => return Err(SyntaxError::BadInfixDotNotation),
                }
            },
            Token::CloseList =>  {
                let ret = Syntax::dot(
                    car,
                    None,
                );

                list.push(ret);
                let ret = Syntax::List(list);

                return Ok((ret, tokens))
            }
            Token::Symbol(_)
            | Token::String(_)
            | Token::Number(_) => {
                let syn = match x {
                    Token::Symbol(s) => Syntax::Symbol(s),
                    Token::String(s) => Syntax::String(s),
                    Token::Number(n) => Syntax::Number(n),
                    _ => panic!("This should never happen"),
                };

                let ret = Syntax::dot(
                    car,
                    Some(syn),
                );

                list.push(ret);
                let ret = Syntax::List(list);

                match tokens.next() {
                    Some(Token::CloseList) => return Ok((ret, tokens)),
                    None => return Err(SyntaxError::UnmatchedOpenListError),
                    _ => return Err(SyntaxError::BadInfixDotNotation),
                }
            },
        },
        None => return Err(SyntaxError::UnmatchedOpenListError)
    };
}
