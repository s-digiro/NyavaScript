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
        parse(Syntax::List(vec![Syntax::list()])).unwrap(),
    );
}
