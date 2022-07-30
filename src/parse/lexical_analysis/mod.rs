mod error;
pub use error::*;

mod token;
pub use token::*;

#[cfg(test)]
mod test;

use std::mem;

pub fn parse(text: &str) -> Result<Vec<Token>, LexError> {
    enum State {
        Dot,
        InAtom,
        InList,
        InNumber,
        InString,
        SlashQuote,
        MaybeComment,
        LineComment,
        Comment,
        MaybeEndComment,
    }

    let mut state = State::InList;
    let mut buf = String::new();
    let mut ret = Vec::new();

    for c in text.chars() {
        match (&state, c) {
            (State::MaybeEndComment, '/') => state = State::InList,
            (State::MaybeEndComment, _) => state = State::Comment,

            (State::Comment, '*') => state = State::MaybeEndComment,
            (State::Comment, _) => (),

            (State::LineComment, '\n') => state = State::InList,
            (State::LineComment, _) => (),

            (State::MaybeComment, '/') => state = State::LineComment,
            (State::MaybeComment, '*') => state = State::Comment,
            (State::MaybeComment, c) => {
                buf.push('/');
                buf.push(c);
                state = State::InAtom;
            },

            (State::InList, '\'') => ret.push(Token::Quote),
            (State::InList, ' ')
            | (State::InList, '\t')
            | (State::InList, '\n')
            | (State::InList, '\r') => (),
            (State::InList, '(') => ret.push(Token::OpenList),
            (State::InList, ')') => ret.push(Token::CloseList),
            (State::InList, '"') => state = State::InString,
            (State::InList, '.') => state = State::Dot,
            (State::InList, '/') => state = State::MaybeComment,
            (State::InList, n) if could_be_number(n) => {
                state = State::InNumber;
                buf.push(n);
            },
            (State::InList, c) => {
                state = State::InAtom;
                buf.push(c);
            },

            (State::InAtom, ' ')
            | (State::InAtom, '\t')
            | (State::InAtom, '\n')
            | (State::InAtom, '\r') => {
                ret.push(Token::Symbol(mem::replace(&mut buf, String::new())));
                state = State::InList;
            },
            (State::InAtom, '(') => {
                ret.push(Token::Symbol(mem::replace(&mut buf, String::new())));
                ret.push(Token::OpenList);
                state = State::InList;
            }
            (State::InAtom, ')') => {
                ret.push(Token::Symbol(mem::replace(&mut buf, String::new())));
                ret.push(Token::CloseList);
                state = State::InList;
            }
            (State::InAtom, c) => buf.push(c),

            (State::SlashQuote, c) => {
                buf.push(c);
                state = State::InString;
            }

            (State::InString, '"') => {
                ret.push(Token::String(mem::replace(&mut buf, String::new())));
                state = State::InList;
            },
            (State::InString, '\\') => state = State::SlashQuote,
            (State::InString, c) => buf.push(c),

            (State::InNumber, ' ')
            | (State::InNumber, '\t')
            | (State::InNumber, '\n')
            | (State::InNumber, '\r') => {
                let num = mem::replace(&mut buf, String::new())
                    .parse::<isize>().unwrap();
                ret.push(Token::Number(num));
                state = State::InList;
            },
            (State::InNumber, '(') => {
                let num = mem::replace(&mut buf, String::new())
                    .parse::<isize>().unwrap();
                ret.push(Token::Number(num));
                ret.push(Token::OpenList);
                state = State::InList;
            }
            (State::InNumber, ')') => {
                let num = mem::replace(&mut buf, String::new())
                    .parse::<isize>().unwrap();
                ret.push(Token::Number(num));
                ret.push(Token::CloseList);
                state = State::InList;
            }
            (State::InNumber, n) if n.is_numeric() => buf.push(c),
            (State::InNumber, c) => {
                buf.push(c);
                state = State::InAtom;
            }

            (State::Dot, ' ')
            | (State::Dot, '\t')
            | (State::Dot, '\n')
            | (State::Dot, '\r') => {
                ret.push(Token::Dot);
                state = State::InList;
            },
            (State::Dot, '(') => {
                ret.push(Token::Dot);
                ret.push(Token::OpenList);
                state = State::InList;
            }
            (State::Dot, ')') => {
                ret.push(Token::Dot);
                ret.push(Token::CloseList);
                state = State::InList;
            }
            (State::Dot, c) => {
                buf.push('.');
                buf.push(c);
                state = State::InAtom;
            }
        }
    }

    match state {
        State::InAtom => ret.push(Token::Symbol(mem::take(&mut buf))),
        State::InNumber => ret.push(
            Token::Number(mem::take(&mut buf).parse::<isize>().unwrap())
        ),
        State::InString => {
            let bad_string = format!("\"{}", buf);
            let index = text.find(&bad_string).unwrap();
            let pre_bad = &text[0..index];
            let line = pre_bad.matches("\n").count() + 1;
            let column = index - pre_bad.rfind("\n").unwrap();

            return Err(
                LexError::unterminated_string_error(bad_string, line, column)
            )
        }
        _ => (),
    }

    Ok(ret)
}

pub fn could_be_number(c: char) -> bool {
    ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-'].contains(&c)
}
