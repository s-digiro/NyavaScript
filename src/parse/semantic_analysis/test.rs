use super::*;

#[test]
fn parse_blank() {
    assert_eq!(
        SXRef::nil(),
        parse(Syntax::list()),
    );
}

#[test]
fn parse_empty_list() {
    assert_eq!(
        SXRef::nil(),
        parse(Syntax::list()),
    );
}

#[test]
fn parse_string() {
    assert_eq!(
        SXRef::from(vec![
            SXRef::from(vec![
                SXRef::string("foo".into()),
                SXRef::string("bar".into()),
            ]),
            SXRef::nil(),
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
        SXRef::from(vec![
            SXRef::from(vec![
                SXRef::number(105),
                SXRef::number(-87)
            ]),
            SXRef::nil(),
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
        SXRef::from(vec![
            SXRef::from(vec![
                SXRef::symbol("foo".into()),
            ]),
            SXRef::nil(),
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

#[test]
fn dot_as_only_item_in_list() {
    let subject = Syntax::List(vec![
        Syntax::List(vec![
            Syntax::symbol("foo"),
        ]),
        Syntax::list(),
    ]);
    panic!("FAIL");
}

#[test]
fn dot_as_last_item_in_list() {
    panic!("FAIL")
}

#[test]
fn dot_as_not_last_item_in_last() {
    panic!("FAIL")
}
