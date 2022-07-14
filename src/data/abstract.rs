use super::list::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Defun {
    pub name: String,
    pub lambda: Lambda,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Label {
    pub name: String,
    pub lambda: Lambda,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Lambda(Expr);

impl Lambda {
    pub fn new(e: ExRef) -> Error<Lambda, String> {
        if !e.is_list() {
            return Err(format!("{} must be a list for it to be turned into a lambda", e))
        }

        if !List::len(&e) == 3 {
            return Err(format!("{} must be a list of size 3 for it to be a lambda", e))
        }

        let arg1 = List::car(&e);

        if 
    }
}
