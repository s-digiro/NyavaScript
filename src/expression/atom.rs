use super::ExRef;

#[derive(Debug, PartialEq)]
pub enum Atom {
    Symbol(String),
    String(String),
    Number(isize),
}

impl Atom {
    pub fn as_symbol(&self) -> Option<&str> {
        match self {
            Atom::Symbol(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            Atom::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_number(&self) -> Option<isize> {
        match self {
            Atom::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn is_symbol(&self) -> bool {
        match self {
            Atom::Symbol(_) => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            Atom::String(_) => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            Atom::Number(_) => true,
            _ => false,
        }
    }

    pub fn nil() -> ExRef {
        ExRef::nil()
    }

    pub fn symbol(s: &str) -> ExRef {
        ExRef::atom(Atom::Symbol(s.to_owned()))
    }

    pub fn string(s: &str) -> ExRef {
        ExRef::atom(Atom::String(s.to_owned()))
    }

    pub fn number(n: isize) -> ExRef {
        ExRef::atom(Atom::Number(n))
    }
}

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Atom::Number(n) => write!(f, "{}", n),
            Atom::String(s) => write!(f, "{}", s),
            Atom::Symbol(s) => write!(f, "{}", s),
        }
    }
}
