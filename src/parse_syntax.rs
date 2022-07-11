use super::*;

pub enum Expr {
    Atom(String),
    List(Vec<Expr>),
    Lambda(Vec<String>, Box<Expr>),
}

pub fn parse_syntax(sym: Symbol) -> Result<Expr, String> {
    match sym {
        Symbol::Atom(atom) => Ok(parse_atom(atom)),
        Symbol::List(list) => parse_list(list),
    }
}

pub fn parse_atom(atom: String) -> Expr {
    Expr::Atom(atom)
}

pub fn parse_list(list: Vec<Symbol>) -> Result<Expr, String> {
    if is_lambda(&list)? {
        Ok(
            Expr::Lambda(
                list[1].to_list().unwrap().into_iter()
                    .map(|sym| sym.to_atom().unwrap())
                    .collect(),
                Box::new(parse_syntax(list[2])?),
            )
        )
    } else {
        let res: Result<Vec<Expr>, String> = list.into_iter().map(|sym| parse_syntax(sym)).collect();

        Ok(
            Expr::List(
                res.unwrap()
            )
        )
    }
}

fn is_lambda(list: &Vec<Symbol>) -> Result<bool, String> {
    let err = Err("lambda definition must take form (lambda (v1 ... vn) e) where v are all atoms and there are as many e as v".to_owned());
    if let Some(Symbol::Atom(atom)) = list.get(0) {
        if atom.as_str() == "lambda" {
            if list.len() != 3 {
                return err;
            }

            if let Some(Symbol::List(keys)) = list.get(1) {
                if !keys.iter().all(|key| matches!(key, Symbol::Atom(_))) {
                    return err;
                }
            }

            return Ok(true);
        }
    }

    return Ok(false);
}
