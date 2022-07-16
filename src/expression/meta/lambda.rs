use super::*;

#[derive(Debug, PartialEq)]
pub struct Lambda(ExRef);

impl Lambda {
    pub fn new(e: &ExRef) -> Result<Lambda, String> {
        if !e.is_list() {
            return Err(format!("Expression must be a list for it to be turned into a lambda"))
        }

        if !List::len(&e) == 3 {
            return Err(format!("Expression must be a list of size 3 for it to be a lambda"))
        }

        if !first_arg_is(e, "lambda") {
            return Err(format!("first arg must be lambda"))
        }

        if !second_arg_is_list_of_symbols(e) {
            return Err(format!("Expression second argument must be a list of all symbol atoms"))
        }

        Ok(Lambda(ExRef::clone(e)))
    }

    pub fn args(&self) -> Vec<String> {
        List::iter(&List::car(&List::cdr(&self.0)))
            .map(|e| e.as_atom().unwrap().as_symbol().unwrap().to_owned())
            .collect()
    }

    pub fn definition(&self) -> ExRef {
        List::car(&List::cdr(&List::cdr(&self.0)))
    }
}

impl std::fmt::Display for Lambda {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[Lambda]")
    }
}
