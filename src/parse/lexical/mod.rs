#[cfg(test)]
mod test;

use std::mem;

use crate::parse::data::Symbol;

pub fn parse(code: &str) -> Vec<Symbol> {
    enum State {
        InList,
        InAtom,
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
        }
    }

    ret
}
