#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Atom(String),
    Defun(Defun),
    Label(Label),
    Lambda(Lambda),
    List(Vec<Expression>),
    Nil,
    Quote(Box<Expression>),
}

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
pub struct Lambda {
    pub args: Vec<String>,
    pub expr: Box<Expression>,
}

impl Lambda {
    pub fn new(args: Vec<String>, expr: Expression) -> Lambda {
        Lambda {
            args,
            expr: Box::new(expr),
        }
    }
}

impl Expression {
    pub fn defun(
        name: String,
        args: Vec<String>,
        expr: Expression
    ) -> Expression {
        Expression::Defun(
            Defun {
                name,
                lambda: Lambda::new(args, expr),
            }
        )
    }

    pub fn label(name: String, lambda: Lambda) -> Expression {
        Expression::Label(Label { name, lambda })
    }

    pub fn lambda(args: Vec<String>, expr: Expression) -> Expression {
        Expression::Lambda(Lambda::new(args, expr))
    }

    pub fn list() -> Expression {
        Expression::List(Vec::new())
    }

    pub fn quote(expr: Expression) -> Expression {
        Expression::Quote(Box::new(expr))
    }

    pub fn is_label(&self) -> bool {
        match self {
            Expression::Label(..) => true,
            _ => false,
        }
    }

    pub fn is_lambda(&self) -> bool {
        match self {
            Expression::Lambda(..) => true,
            _ => false,
        }
    }

    pub fn is_nil(expr: Expression) -> bool {
        match expr {
            Expression::Nil => true,
            Expression::List(l) => l.is_empty(),
            _ => false,
        }
    }

    pub fn into_lambda(self) -> Option<Lambda> {
        match self {
            Expression::Lambda(l) => Some(l),
            _ => None,
        }
    }

    pub fn into_label(self) -> Option<Label> {
        match self {
            Expression::Label(l) => Some(l),
            _ => None,
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expression::Atom(a) => write!(f, "{}", a),
            Expression::Defun(d) => write!(f, "{}", d),
            Expression::Label(l) => write!(f, "{}", l),
            Expression::Lambda(l) => write!(f, "{}", l),
            Expression::List(l) => {
                write!(f, "(")?;
                for x in l {
                    write!(f, "{} ", x)?;
                }
                write!(f, ")")?;

                Ok(())
            },
            Expression::Nil => write!(f, "NIL"),
            Expression::Quote(q) => write!(f, "'{}", q),
        }
    }
}

impl std::fmt::Display for Defun {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "defun {} {}", self.name, self.lambda)
    }
}

impl std::fmt::Display for Lambda {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "lambda (")?;

        for x in self.args.iter() {
            write!(f, "{} ", x)?;
        }

        write!(f, ") {}", self.expr)
    }
}

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "label {} ({})", self.name, self.lambda)
    }
}
