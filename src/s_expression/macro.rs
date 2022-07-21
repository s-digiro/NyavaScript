use crate::parse::{ parse, ParseError };
use crate::s_expression::{ SExpressionRef as SXRef, list };

#[derive(Debug, PartialEq)]
pub struct Macro {
    definition: SXRef,
    args: Vec<String>,
}

impl Macro {
    pub fn new(args: Vec<String>, definition: SXRef) -> Macro {
        Macro {
            definition,
            args,
        }
    }

    pub fn args(&self) -> &Vec<String> {
        &self.args
    }

    pub fn definition(&self) -> SXRef {
        SXRef::clone(&self.definition)
    }
}

impl TryFrom<&str> for Macro {
    type Error = ParseError;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let ast = parse(text)?;

        Ok(ast.into())
    }
}

impl From<SXRef> for Macro {
    fn from(sx: SXRef) -> Self {
        let args = list::car(&list::cdr(&sx)).iter()
            .filter_map(|sx| sx.as_symbol().map(|s| s.into()))
            .collect();

        let definition = list::car(&list::cdr(&list::cdr(&sx)));

        Macro::new(args, definition)
    }
}

impl std::fmt::Display for Macro {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[Macro]")
    }
}
