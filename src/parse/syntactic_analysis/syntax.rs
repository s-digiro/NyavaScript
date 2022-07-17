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

    pub fn string(s: &str) -> Syntax {
        Syntax::String(s.to_owned())
    }

    pub fn as_mut_list(&mut self) -> Option<&mut Vec<Syntax>> {
        match self {
            Syntax::List(l) => Some(l),
            _ => None,
        }
    }
}
