use super::*;

#[test]
fn parse_blank() {
    assert_eq!(
        Value::nil(),
        parse(Syntax::list()),
    );
}

#[test]
fn parse_empty_list() {
    assert_eq!(
        Value::nil(),
        parse(Syntax::list()),
    );
}

#[test]
fn parse_string() {
    assert_eq!(
        List::from(vec![
            List::from(vec![
                Value::string("foo".into()),
                Value::string("bar".into()),
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
                Value::number(105),
                Value::number(-87)
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
                Value::symbol("foo".into()),
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
