#[derive(PartialEq)]
#[derive(Debug)]
pub enum Token {
    CloseList,
    Number(isize),
    OpenList,
    Quote,
    String(String),
    Symbol(String),
}

impl Token {
    pub fn string(a: &str) -> Token {
        Token::String(a.to_owned())
    }

    pub fn symbol(a: &str) -> Token {
        Token::Symbol(a.to_owned())
    }
}


impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::CloseList => write!(f, ")"),
            Token::Number(num) => write!(f, "{}", num),
            Token::OpenList => write!(f, "("),
            Token::Quote => write!(f, "'"),
            Token::String(s) => write!(f, "\"{}\"", s),
            Token::Symbol(a) => write!(f, "{}", a),
        }
    }
}
