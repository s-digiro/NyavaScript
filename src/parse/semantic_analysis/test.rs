use super::*;
use crate::s_expression::ConsCell;

#[test]
fn parse_syn_blank() {
    assert_eq!(
        SXRef::nil(),
        parse_syn(Syntax::list()).unwrap(),
    );
}

#[test]
fn parse_syn_empty_list() {
    assert_eq!(
        SXRef::nil(),
        parse_syn(Syntax::list()).unwrap(),
    );
}

#[test]
fn parse_syn_string() {
    assert_eq!(
        SXRef::from(vec![
            SXRef::from(vec![
                SXRef::string("foo".into()),
                SXRef::string("bar".into()),
            ]),
            SXRef::nil(),
        ]),
        parse_syn(
            Syntax::List(vec![
                Syntax::List(vec![
                    Syntax::String("foo".into()),
                    Syntax::String("bar".into()),
                ]),
                Syntax::list(),
            ])
        ).unwrap(),
    );
}

#[test]
fn parse_syn_number() {
    assert_eq!(
        SXRef::from(vec![
            SXRef::from(vec![
                SXRef::number(105),
                SXRef::number(-87)
            ]),
            SXRef::nil(),
        ]),
        parse_syn(
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

#[test]
fn parse_syn_symbol() {
    assert_eq!(
        SXRef::from(vec![
            SXRef::from(vec![
                SXRef::symbol("foo".into()),
            ]),
            SXRef::nil(),
        ]),
        parse_syn(
            Syntax::List(vec![
                Syntax::List(vec![
                    Syntax::symbol("foo"),
                ]),
                Syntax::list(),
            ])
        ).unwrap(),
    );
}

#[test]
fn dot_as_only_item_in_list() {
    let subject = Syntax::List(vec![
        Syntax::dot(
            Some(Syntax::Number(1)),
            Some(Syntax::Number(2)),
        ),
    ]);

    let expected = SXRef::cons_cell(ConsCell::new(
        SXRef::number(1),
        SXRef::number(2),
    ));

    let actual = parse_syn(subject).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn dot_with_no_cdr() {
    let subject = Syntax::List(vec![
        Syntax::dot(
            Some(Syntax::Number(1)),
            None,
        ),
    ]);

    let expected = SXRef::cons_cell(
        ConsCell::new(
            SXRef::number(1),
            SXRef::nil(),
        )
    );

    let actual = parse_syn(subject).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn dot_with_no_car() {
    let subject = Syntax::List(vec![
        Syntax::dot(
            None,
            Some(Syntax::Number(1)),
        ),
    ]);

    let expected = SXRef::cons_cell(
        ConsCell::new(
            SXRef::nil(),
            SXRef::number(1),
        )
    );

    let actual = parse_syn(subject).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn dot_with_no_cdr_or_car() {
    let subject = Syntax::List(vec![
        Syntax::dot(
            None,
            None,
        ),
    ]);

    let expected = SXRef::cons_cell(
        ConsCell::new(
            SXRef::nil(),
            SXRef::nil(),
        )
    );

    let actual = parse_syn(subject).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn dot_as_last_item_in_list() {
    let subject = Syntax::List(vec![
        Syntax::Number(1),
        Syntax::Number(2),
        Syntax::dot(
            Some(Syntax::Number(3)),
            Some(Syntax::Number(4)),
        ),
    ]);

    let expected = SXRef::cons_cell(ConsCell::new(
        SXRef::number(1),
        SXRef::cons_cell(ConsCell::new(
            SXRef::number(2),
            SXRef::cons_cell(ConsCell::new(
                SXRef::number(3),
                SXRef::number(4),
            )),
        )),
    ));

    let actual = parse_syn(subject).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn dot_as_not_last_item_in_last() {
    let subject = Syntax::List(vec![
        Syntax::Number(1),
        Syntax::dot(
            Some(Syntax::Number(3)),
            Some(Syntax::Number(4)),
        ),
        Syntax::Number(2),
    ]);

    let expected = SemanticError::DotSyntaxNotAtListEnd;

    let actual = parse_syn(subject).err().unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn multiple_syntaxes() {
    let subject = vec![
        Syntax::List(vec![
            Syntax::Number(1),
            Syntax::Number(2),
        ]),
        Syntax::List(vec![
            Syntax::Number(3),
            Syntax::Number(4),
        ]),
    ];

    let expected = vec![
        SXRef::from(vec![
            SXRef::number(1),
            SXRef::number(2),
        ]),
        SXRef::from(vec![
            SXRef::number(3),
            SXRef::number(4),
        ]),
    ];

    let actual = parse(subject).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn one_syntax() {
    let subject = vec![
        Syntax::List(vec![
            Syntax::Number(3),
            Syntax::Number(4),
        ]),
    ];

    let expected = vec![
        SXRef::from(vec![
            SXRef::number(3),
            SXRef::number(4),
        ]),
    ];

    let actual = parse(subject).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn no_syntaxes() {
    let subject: Vec<Syntax> = vec![];

    let expected: Vec<SXRef> = vec![];

    let actual = parse(subject).unwrap();

    assert_eq!(expected, actual);
}
