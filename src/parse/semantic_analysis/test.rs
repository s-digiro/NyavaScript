use super::*;

#[test]
fn parse_blank() {
    assert_eq!(
        SExpressionRef::nil(),
        parse(Syntax::list()),
    );
}

#[test]
fn parse_empty_list() {
    assert_eq!(
        SExpressionRef::nil(),
        parse(Syntax::list()),
    );
}

#[test]
fn parse_string() {
    assert_eq!(
        List::from(vec![
            List::from(vec![
                SExpressionRef::string("foo".into()),
                SExpressionRef::string("bar".into()),
            ]),
            List::new(),
        ]),
        parse(
            Syntax::List(vec![
                Syntax::List(vec![
                    Syntax::String("foo".into()),
                    Syntax::String("bar".into()),
                ]),
                Syntax::list(),
            ])
        ),
    );
}

#[test]
fn parse_number() {
    assert_eq!(
        List::from(vec![
            List::from(vec![
                SExpressionRef::number(105),
                SExpressionRef::number(-87)
            ]),
            List::new(),
        ]),
        parse(
            Syntax::List(vec![
                Syntax::List(vec![
                    Syntax::Number(105),
                    Syntax::Number(-87),
                ]),
                Syntax::list(),
            ])
        ),
    );
}

#[test]
fn parse_symbol() {
    assert_eq!(
        List::from(vec![
            List::from(vec![
                SExpressionRef::symbol("foo".into()),
            ]),
            List::new(),
        ]),
        parse(
            Syntax::List(vec![
                Syntax::List(vec![
                    Syntax::symbol("foo"),
                ]),
                Syntax::list(),
            ])
        ),
    );
}
