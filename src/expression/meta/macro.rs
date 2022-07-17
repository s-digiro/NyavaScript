use super::*;

#[derive(Debug, PartialEq)]
pub struct Macro(ValRef);

impl Macro {
    pub fn new(e: &ValRef) -> Result<Macro, String> {
        if !e.is_list() {
            return Err(format!("Expression must be a list for it to be turned into a macro"))
        }

        if !List::len(&e) == 3 {
            return Err(format!("Expression must be a list of size 3 for it to be a macro"))
        }

        if !first_arg_is(e, "macro") {
            return Err(format!("first arg must be macro"))
        }

        if !second_arg_is_list_of_symbols(e) {
            return Err(format!("Expression second argument must be a list of all symbol macro"))
        }

        Ok(Macro(ValRef::clone(e)))
    }

    pub fn args(&self) -> Vec<String> {
        List::iter(&List::car(&List::cdr(&self.0)))
            .map(|e| e.as_atom().unwrap().as_symbol().unwrap().to_owned())
            .collect()
    }

    pub fn definition(&self) -> ValRef {
        List::car(&List::cdr(&List::cdr(&self.0)))
    }
}

impl std::fmt::Display for Macro {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[Macro]")
    }
}
