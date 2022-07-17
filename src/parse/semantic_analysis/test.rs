use super::*;

#[test]
fn parse_blank() {
    assert_eq!(
        ValRef::nil(),
        parse(Syntax::list()),
    );
}

#[test]
fn parse_empty_list() {
    assert_eq!(
        ValRef::nil(),
        parse(Syntax::list()),
    );
}

#[test]
fn parse_string() {
    assert_eq!(
        List::from(vec![
            List::from(vec![
                ValRef::string("foo".into()),
                ValRef::string("bar".into()),
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
                ValRef::number(105),
                ValRef::number(-87)
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
                ValRef::symbol("foo".into()),
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
