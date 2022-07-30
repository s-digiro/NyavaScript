mod error;
pub use error::*;

mod syntax;
pub use syntax::Syntax;

#[cfg(test)]
mod test;

use crate::s_expression::{
    ConsCell,
    SExpressionRef as SXRef,
    util,
};
use std::vec::IntoIter;
use super::lexical_analysis::Token;

type TokenIter = IntoIter<Token>;
type SXResult = Result<(SXRef, TokenIter), SyntaxError>;

pub fn parse(tokens: Vec<Token>) -> Result<Vec<SXRef>, SyntaxError> {
    let mut ret = Vec::new();

    let mut tokens = tokens.into_iter();

    while let Some(token) = tokens.next() {
        match token {
            Token::OpenList => {
                let (sx, it) = parse_list(tokens)?;

                ret.push(sx);
                tokens = it;
            },
            Token::CloseList => return Err(SyntaxError::UnmatchedCloseListError),
            Token::Dot => todo!("Free dot error"),
            Token::Quote => todo!("Free quote error"),
            Token::Number(_)
            | Token::String(_)
            | Token::Symbol(_) => return Err(SyntaxError::FreeAtom),
        }
    }

    Ok(ret)
}

fn parse_quote(mut tokens: TokenIter) -> SXResult {
    let token = if let Some(token) = tokens.next() {
        token
    } else {
        return Err(SyntaxError::QuoteMissingItemError)
    };

    let sx = match token {
        Token::Quote => {
            let (sx, it) = parse_quote(tokens)?;

            tokens = it;
            sx
        },
        Token::OpenList => {
            let (sx, it) = parse_list(tokens)?;

            tokens = it;
            sx
        },
        Token::Symbol(s) => SXRef::symbol(s),
        Token::String(s) => SXRef::string(s),
        Token::Number(n) => SXRef::number(n),
        Token::Dot
        | Token::CloseList => return Err(SyntaxError::QuoteMissingItemError),
    };

    let sx = SXRef::quote(sx);

    Ok((sx, tokens))

}

fn parse_list(mut tokens: TokenIter) -> SXResult {
    let mut list = Vec::new();

    while let Some(tok) = tokens.next() {
        match tok {
            Token::Dot => {
                let car = match list.pop() {
                    Some(car) => car,
                    None => SXRef::nil(),
                };

                let (cdr, it) = parse_dot(tokens)?;

                tokens = it;

                let mut ret = SXRef::cons_cell(ConsCell::new(car, cdr));

                for sx in list.iter().rev() {
                    ret = util::cons(&sx, &ret);
                }

                return Ok((ret, tokens))
            },
            Token::Symbol(s) => list.push(SXRef::symbol(s)),
            Token::String(s) => list.push(SXRef::string(s)),
            Token::Number(n) => list.push(SXRef::number(n)),
            Token::Quote => list.push({
                let (sx, it) = parse_quote(tokens)?;

                tokens = it;
                sx
            }),
            Token::OpenList => list.push({
                let (sx, it) = parse_list(tokens)?;

                tokens = it;
                sx
            }),
            Token::CloseList =>  {
                let sx = SXRef::from(list);

                return Ok((sx, tokens))
            }
        }
    }


    Err(SyntaxError::UnmatchedOpenListError)
}

pub fn parse_dot(mut tokens: TokenIter) -> SXResult {
    let token = if let Some(token) = tokens.next() {
        token
    } else {
        return Err(SyntaxError::UnmatchedOpenListError)
    };

    let cdr = match token {
        Token::Dot => return Err(SyntaxError::BadInfixDotNotation),
        Token::OpenList => {
            let (sx, it) = parse_list(tokens)?;
            tokens = it;
            sx
        },
        Token::Quote => {
            let (sx, it) = parse_quote(tokens)?;
            tokens = it;
            sx
        },
        Token::CloseList => return Ok((SXRef::nil(), tokens)),
        Token::Symbol(s) => SXRef::symbol(s),
        Token::String(s) => SXRef::string(s),
        Token::Number(n) => SXRef::number(n),
    };

    match tokens.next() {
        Some(Token::CloseList) => (),
        None => return Err(SyntaxError::UnmatchedOpenListError),
        _ => return Err(SyntaxError::BadInfixDotNotation),
    }

    Ok((cdr, tokens))
}
