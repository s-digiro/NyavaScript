#[derive(PartialEq)]
#[derive(Debug)]
pub enum Symbol {
    OpenList,
    CloseList,
    Atom(String),
    Quote,
}

impl Symbol {
    pub fn atom(a: &str) -> Symbol {
        Symbol::Atom(a.to_owned())
    }
}


impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::OpenList => write!(f, "("),
            Symbol::CloseList => write!(f, ")"),
            Symbol::Atom(a) => write!(f, "{}", a),
            Symbol::Quote => write!(f, "'"),
        }
    }
}
