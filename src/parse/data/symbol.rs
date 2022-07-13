#[derive(PartialEq)]
#[derive(Debug)]
pub enum Symbol {
    Atom(String),
    CloseList,
    Number(isize),
    OpenList,
    Quote,
    String(String),
}

impl Symbol {
    pub fn atom(a: &str) -> Symbol {
        Symbol::Atom(a.to_owned())
    }

    pub fn string(a: &str) -> Symbol {
        Symbol::String(a.to_owned())
    }
}


impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::Atom(a) => write!(f, "{}", a),
            Symbol::CloseList => write!(f, ")"),
            Symbol::Number(num) => write!(f, "{}", num),
            Symbol::OpenList => write!(f, "("),
            Symbol::Quote => write!(f, "'"),
            Symbol::String(s) => write!(f, "\"{}\"", s),
        }
    }
}
