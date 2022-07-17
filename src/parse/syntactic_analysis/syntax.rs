#[derive(Debug, Clone, PartialEq)]
pub enum Syntax {
    Atom(String),
    List(Vec<Syntax>),
    Number(isize),
    String(String),
}

impl Syntax {
    pub fn list() -> Syntax {
        Syntax::List(Vec::new())
    }

    pub fn atom(s: &str) -> Syntax {
        Syntax::Atom(s.to_owned())
    }

    pub fn as_atom(&self) -> Option<&str> {
        match self {
            Syntax::Atom(a) => Some(a),
            _ => None,
        }
    }

    pub fn as_mut_list(&mut self) -> Option<&mut Vec<Syntax>> {
        match self {
            Syntax::List(l) => Some(l),
            _ => None,
        }
    }

    pub fn into_list(self) -> Option<Vec<Syntax>> {
        match self {
            Syntax::List(l) => Some(l),
            _ => None,
        }
    }

    pub fn into_atom(self) -> Option<String> {
        match self {
            Syntax::Atom(s) => Some(s),
            _ => None,
        }
    }

    pub fn is_atom(&self) -> bool {
        match self {
            Syntax::Atom(_) => true,
            _ => false,
        }
    }
}
