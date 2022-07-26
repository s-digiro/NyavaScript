use std::fmt;

#[derive(Debug, PartialEq)]
pub enum SemanticError {
    DotSyntaxNotAtListEnd
}

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SemanticError: ")?;

        match self {
            Self::DotSyntaxNotAtListEnd => write!(f, "DotSyntaxNotAtListEnd: Dot syntax token must be last or second to last item in list."),
        }
    }
}
