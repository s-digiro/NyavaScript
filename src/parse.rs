use std::mem;

#[derive(PartialEq)]
pub enum Symbol {
    List(Vec<Symbol>),
    Atom(String),
}

impl Symbol {
    pub fn list() -> Symbol {
        Symbol::List(Vec::new())
    }

    pub fn atom(s: &str) -> Symbol {
        Symbol::Atom(s.to_owned())
    }

    pub fn as_list(&mut self) -> Option<&mut Vec<Symbol>> {
        match self {
            Symbol::List(l) => Some(l),
            _ => None,
        }
    }

    pub fn as_atom(&mut self) -> Option<&mut String> {
        match self {
            Symbol::Atom(s) => Some(s),
            _ => None,
        }
    }

    pub fn to_list(self) -> Option<Vec<Symbol>> {
        match self {
            Symbol::List(l) => Some(l),
            _ => None,
        }
    }

    pub fn to_atom(self) -> Option<String> {
        match self {
            Symbol::List(l) => Some(l),
            _ => None,
        }
    }
}

impl std::fmt::Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::List(l) => {
                write!(f, "(")?;
                for expr in l {
                    expr.fmt(f)?;
                    write!(f, " ");
                }
                write!(f, ")");
            },
            Symbol::Atom(s) => {
                write!(f, "{}", s)?;
            },
        }

        Ok(())
    }
}

pub fn parse(code: &str) -> Symbol {
    enum State {
        InList,
        InAtom,
    }

    let mut stack = Vec::new();
    let mut current = Vec::new();
    let mut state = State::InList;
    let mut buf = String::new();

    for c in code.chars() {
        match (&state, c) {
            (State::InList, ' ')
            | (State::InList, '\t')
            | (State::InList, '\n')
            | (State::InList, '\r') => (),
            (State::InList, '(') => stack.push(mem::replace(&mut current, Vec::new())),
            (State::InAtom, '(') => panic!("Cannot use ( as part of atom"),
            (State::InAtom, ' ')
            | (State::InAtom, '\t')
            | (State::InAtom, '\n')
            | (State::InAtom, '\r') => {
                current.push(Symbol::Atom(mem::replace(&mut buf, String::new())));
                state = State::InList;
            },
            | (State::InAtom, ')') => {
                current.push(Symbol::Atom(mem::replace(&mut buf, String::new())));

                let child = mem::replace(
                    &mut current,
                    match stack.pop() {
                        Some(p) => p,
                        None => panic!("Unpaired closing parenthesis"),
                    }
                );

                current.push(Symbol::List(child));

                state = State::InList;
            }
            (_, ')') => {
                let child = mem::replace(
                    &mut current,
                    match stack.pop() {
                        Some(p) => p,
                        None => panic!("Unpaired closing parenthesis"),
                    }
                );

                current.push(Symbol::List(child));

                state = State::InList;
            }
            (State::InList, c) => {
                state = State::InAtom;
                buf.push(c);
            },
            (State::InAtom, c) => buf.push(c),
        }
    }

    if !stack.is_empty() {
        panic!("Unmatched open parenthesis");
    }

    return Symbol::List(current);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_works() {
        assert_eq!(
            Symbol::List(Vec::new()),
            parse(""),
        );

        assert_eq!(
            Symbol::List(vec![Symbol::List(vec![])]),
            parse("()"),
        );

        assert_eq!(
            Symbol::List(vec![
                Symbol::List(vec![
                    Symbol::Atom(String::from("foo")),
                    Symbol::Atom(String::from("bar")),
                ]),
            ]),
            parse("(foo bar)"),
        );

        assert_eq!(
            Symbol::List(vec![
                Symbol::List(vec![
                    Symbol::Atom(String::from("foo")),
                    Symbol::List(vec![
                        Symbol::Atom(String::from("bar")),
                        Symbol::Atom(String::from("baz")),
                    ]),
                ]),
            ]),
            parse("(foo (bar baz))"),
        );
    }

    #[test]
    #[should_panic]
    fn parse_unmatched_open_parenthesis_should_panic() {
        parse("()(");
    }

    #[test]
    #[should_panic]
    fn parse_unmatched_close_parenthesis_should_panic() {
        parse("())");
    }
}
