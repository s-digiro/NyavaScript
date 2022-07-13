use super::*;

#[test]
fn parse_blank() {
    assert_eq!(
        Expr::list(),
        parse(Syntax::list()).unwrap(),
    );
}

#[test]
fn parse_empty_list() {
    assert_eq!(
        Expr::list(),
        parse(Syntax::list()).unwrap(),
    );
}

#[test]
fn parse_string() {
    assert_eq!(
        Expr::List(vec![
            Expr::List(vec![
                Expr::String("foo".to_owned()),
                Expr::String("bar".to_owned())
            ]),
            Expr::list(),
        ]),
        parse(
            Syntax::List(vec![
                Syntax::List(vec![
                    Syntax::String("foo".to_owned()),
                    Syntax::String("bar".to_owned()),
                ]),
                Syntax::list(),
            ])
        ).unwrap(),
    );
}

#[test]
fn parse_number() {
    assert_eq!(
        Expr::List(vec![
            Expr::List(vec![
                Expr::Number(105),
                Expr::Number(-87)
            ]),
            Expr::list(),
        ]),
        parse(
            Syntax::List(vec![
                Syntax::List(vec![
                    Syntax::Number(105),
                    Syntax::Number(-87),
                ]),
                Syntax::list(),
            ])
        ).unwrap(),
    );
}
