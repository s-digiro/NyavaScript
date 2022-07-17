#[derive(Debug, Clone, PartialEq)]
pub enum Syntax {
    List(Vec<Syntax>),
    Number(isize),
    String(String),
    Symbol(String),
}

impl Syntax {
    pub fn list() -> Syntax {
        Syntax::List(Vec::new())
    }

    pub fn symbol(s: &str) -> Syntax {
        Syntax::Symbol(s.to_owned())
    }

    pub fn as_symbol(&self) -> Option<&str> {
        match self {
            Syntax::Symbol(a) => Some(a),
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

    pub fn into_symbol(self) -> Option<String> {
        match self {
            Syntax::Symbol(s) => Some(s),
            _ => None,
        }
    }

    pub fn is_symbol(&self) -> bool {
        match self {
            Syntax::Symbol(_) => true,
            _ => false,
        }
    }
}
