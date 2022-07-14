use super::*;

#[test]
fn parse_blank() {
    assert_eq!(
        Atom::nil(),
        parse(Syntax::list()).unwrap(),
    );
}

#[test]
fn parse_empty_list() {
    assert_eq!(
        Atom::nil(),
        parse(Syntax::list()).unwrap(),
    );
}

#[test]
fn parse_string() {
    assert_eq!(
        List::from(vec![
            List::from(vec![
                Atom::string("foo"),
                Atom::string("bar"),
            ]),
            List::new(),
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
        List::from(vec![
            List::from(vec![
                Atom::number(105),
                Atom::number(-87)
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
        ).unwrap(),
    );
}
