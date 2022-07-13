#[cfg(test)]
mod test;

use std::mem;

use crate::parse::data::Symbol;

pub fn parse(code: &str) -> Vec<Symbol> {
    enum State {
        InList,
        InAtom,
        InString,
        InNumber,
        SlashQuote,
    }

    let mut state = State::InList;
    let mut buf = String::new();
    let mut ret = Vec::new();

    for c in code.chars() {
        match (&state, c) {
            (_, '\'') => ret.push(Symbol::Quote),

            (State::InList, ' ')
            | (State::InList, '\t')
            | (State::InList, '\n')
            | (State::InList, '\r') => (),

            (State::InList, '(') => ret.push(Symbol::OpenList),

            (State::InList, ')') => ret.push(Symbol::CloseList),

            (State::InList, '"') => state = State::InString,

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
                ret.push(Symbol::Atom(mem::replace(&mut buf, String::new())));
                state = State::InList;
            },

            (State::InAtom, '(') => {
                ret.push(Symbol::Atom(mem::replace(&mut buf, String::new())));
                ret.push(Symbol::OpenList);
                state = State::InList;
            }

            (State::InAtom, ')') => {
                ret.push(Symbol::Atom(mem::replace(&mut buf, String::new())));
                ret.push(Symbol::CloseList);
                state = State::InList;
            }

            (State::InAtom, c) => buf.push(c),

            (State::SlashQuote, c) => {
                buf.push(c);
                state = State::InString;
            }

            (State::InString, '"') => {
                ret.push(Symbol::String(mem::replace(&mut buf, String::new())));
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
                ret.push(Symbol::Number(num));
                state = State::InList;
            },

            (State::InNumber, '(') => {
                let num = mem::replace(&mut buf, String::new())
                    .parse::<isize>().unwrap();
                ret.push(Symbol::Number(num));
                ret.push(Symbol::OpenList);
                state = State::InList;
            }

            (State::InNumber, ')') => {
                let num = mem::replace(&mut buf, String::new())
                    .parse::<isize>().unwrap();
                ret.push(Symbol::Number(num));
                ret.push(Symbol::CloseList);
                state = State::InList;
            }

            (State::InNumber, n) if n.is_numeric() => buf.push(c),

            (State::InNumber, c) => {
                buf.push(c);
                state = State::InAtom;
            }
        }
    }

    ret
}

pub fn could_be_number(c: char) -> bool {
    ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-'].contains(&c)
}
