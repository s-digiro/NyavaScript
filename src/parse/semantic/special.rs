use super::{Expr, parse, Syntax};

type Args = Vec<Syntax>;
type SpecialResult = Result<Expr, String>;

pub enum Special {
    Defun,
    Label,
    Lambda,
    Quote,
}

impl Special {
    pub fn parse(&self, args: Args) -> SpecialResult {
        match self {
            Special::Defun => self.parse_defun(args),
            Special::Label => self.parse_label(args),
            Special::Lambda => self.parse_lambda(args),
            Special::Quote => self.parse_quote(args),
        }
    }

    fn parse_defun(&self, mut args: Args) -> SpecialResult {
        if args.len() != 3 {
            return Err("Defun requires 3 args".to_owned())
        }

        let name = args.remove(0).into_atom()
            .ok_or("Defun first arg must be an atom".to_owned())?;

        let vars: Vec<String> = args.remove(0).into_list()
            .ok_or("Defun second arg must be list of atoms".to_owned())?
            .into_iter()
            .map(|syn| syn.into_atom())
            .collect::<Option<Vec<String>>>()
            .ok_or("Defun second arg must be list of atoms".to_owned())?;

        let expr = parse(args.remove(0))?;

        Ok(Expr::defun(name, vars, expr))
    }

    fn parse_label(&self, mut args: Args) -> SpecialResult {
        if args.len() != 2 {
            return Err("Label requires 2 args".to_owned())
        }

        let name = args.remove(0).into_atom()
            .ok_or("Label first arg must be an atom".to_owned())?;

        let lambda = parse(args.remove(0))?;

        if !lambda.is_lambda() {
            return Err("Label second arg must be a lambda".to_owned())
        }

        Ok(Expr::label(name, lambda.into_lambda().unwrap()))
    }

    fn parse_lambda(&self, mut args: Args) -> SpecialResult {
        if args.len() != 2 {
            return Err("Lambda requires 2 args".to_owned())
        }


        let vars: Vec<String> = args.remove(0).into_list()
            .ok_or("Lambda first arg must be list of atoms".to_owned())?
            .into_iter()
            .map(|syn| syn.into_atom())
            .collect::<Option<Vec<String>>>()
            .ok_or("Lambda first arg must be list of atoms".to_owned())?;

        let expr = parse(args.remove(0))?;

        Ok(Expr::lambda(vars, expr))
    }

    fn parse_quote(&self, mut args: Args) -> SpecialResult {
        if args.len() != 1 {
            return Err("Quote requires 1 arg".to_owned())
        }

        let quoted = parse(args.remove(0))?;

        Ok(Expr::quote(quoted))
    }
}

impl std::convert::TryFrom<&str> for Special {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "defun" => Ok(Special::Defun),
            "label" => Ok(Special::Label),
            "lambda" => Ok(Special::Lambda),
            "quote" => Ok(Special::Quote),
            _ => Err(format!("Invalid special type: {:?}", s)),
        }
    }
}
