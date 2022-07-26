use super::*;

#[test]
fn unmatched_open_parenthesis_fails() {
    assert_eq!(
        parse(vec![Token::OpenList, Token::OpenList, Token::CloseList]),
        Err(SyntaxError::UnmatchedOpenListError),
    );
}

#[test]
fn unmatched_close_parenthesis_fails() {
    assert_eq!(
        parse(vec![Token::OpenList, Token::CloseList, Token::CloseList]),
        Err(SyntaxError::UnexpectedTrailingTokensError),
    );
}

#[test]
fn unlisted_extra_tokens_fails() {
    assert_eq!(
        parse(vec![Token::OpenList, Token::CloseList, Token::symbol("foo")]),
        Err(SyntaxError::UnexpectedTrailingTokensError),
    );
}

#[test]
fn parse_empty_symbols_list_fails() {
    assert_eq!(
        parse(vec![]),
        Err(SyntaxError::NoSymbolsError),
    );
}

#[test]
fn parse_missing_open_root_list_fails() {
    assert_eq!(
        parse(vec![Token::symbol("test")]),
        Err(SyntaxError::NoRootListError),
    );
}

#[test]
fn parse_missing_close_root_list_fails() {
    assert_eq!(
        parse(vec![Token::OpenList, Token::symbol("test")]),
        Err(SyntaxError::UnmatchedOpenListError),
    );

    assert_eq!(
        parse(vec![Token::OpenList]),
        Err(SyntaxError::UnmatchedOpenListError),
    );
}

#[test]
fn parse_nil_list_works() {
    assert_eq!(
        parse(vec![Token::OpenList, Token::CloseList]).unwrap(),
        Syntax::list(),
    );
}

#[test]
fn parse_string_works() {
    assert_eq!(
        parse(vec![
              Token::OpenList,
              Token::String("foo".to_owned()),
              Token::CloseList
        ]).unwrap(),
        Syntax::List(vec![Syntax::String("foo".to_owned())])
    );
}

#[test]
fn parse_number_works() {
    assert_eq!(
        parse(vec![
              Token::OpenList,
              Token::Number(105),
              Token::CloseList
        ]).unwrap(),
        Syntax::List(vec![Syntax::Number(105)])
    );
}

#[test]
fn parse_list_works() {
    assert_eq!(
        parse(vec![
              Token::OpenList,
              Token::symbol("foo"),
              Token::string("bar"),
              Token::Number(105),
              Token::CloseList,
        ]).unwrap(),
        Syntax::List(vec![
            Syntax::symbol("foo"),
            Syntax::string("bar"),
            Syntax::Number(105),
        ]),
    );
}

#[test]
fn parse_quote_works() {
    assert_eq!(
        parse(vec![
              Token::OpenList,
              Token::Quote,
              Token::symbol("foo"),
              Token::CloseList,
        ]).unwrap(),
        Syntax::List(vec![
            Syntax::symbol("quote"),
            Syntax::symbol("foo"),
        ]),
    );
}

#[test]
fn quote_before_close_list_fails() {
    assert_eq!(
        parse(vec![
              Token::OpenList,
              Token::Quote,
              Token::CloseList,
        ]),
        Err(SyntaxError::QuoteMissingItemError),
    );
}

#[test]
fn quote_at_end_of_token_list_fails() {
    assert_eq!(
        parse(vec![
              Token::OpenList,
              Token::Quote,
        ]),
        Err(SyntaxError::QuoteMissingItemError),
    );
}

#[test]
fn parse_quote_as_not_first_value_in_list() {
    assert_eq!(
        parse(vec![
              Token::OpenList,
              Token::symbol("car"),
              Token::Quote,
              Token::OpenList,
              Token::symbol("foo"),
              Token::symbol("bar"),
              Token::CloseList,
              Token::CloseList,
        ]).unwrap(),
        Syntax::List(vec![
            Syntax::symbol("car"),
            Syntax::List(vec![
                Syntax::symbol("quote"),
                Syntax::List(vec![
                    Syntax::symbol("foo"),
                    Syntax::symbol("bar"),
                ]),
            ]),
        ]),
    );
}

#[test]
fn parse_works() {
    assert_eq!(
        parse(vec![
            Token::OpenList,
            Token::symbol("foo"),
            Token::symbol("bar"),
            Token::CloseList,
        ]).unwrap(),
        Syntax::List(vec![Syntax::symbol("foo"), Syntax::symbol("bar")]),
    );

    assert_eq!(
        parse(vec![
            Token::OpenList,
            Token::symbol("foo"),
            Token::OpenList,
            Token::symbol("bar"),
            Token::symbol("baz"),
            Token::CloseList,
            Token::CloseList
        ]).unwrap(),
        Syntax::List(vec![
            Syntax::symbol("foo"),
            Syntax::List(vec![
                Syntax::symbol("bar"),
                Syntax::symbol("baz")])])
    );

    assert_eq!(
        parse(vec![
              Token::OpenList,
              Token::OpenList,
              Token::symbol("foo"),
              Token::symbol("bar"),
              Token::CloseList,
              Token::symbol("baz"),
              Token::CloseList]).unwrap(),
        Syntax::List(vec![
            Syntax::List(vec![
                Syntax::symbol("foo"),
                Syntax::symbol("bar")]),
            Syntax::symbol("baz")]));
}

#[test]
fn dot_with_car_and_cdr() {
    let subject = vec![
        Token::OpenList,
        Token::Number(1),
        Token::Dot,
        Token::Number(2),
        Token::CloseList,
    ];

    let expected = Syntax::List(vec![
        Syntax::dot(
            Some(Syntax::Number(1)),
            Some(Syntax::Number(2)),
        ),
    ]);

    let actual = parse(subject).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn dot_with_car_but_no_cdr() {
    let subject = vec![
        Token::OpenList,
        Token::Number(1),
        Token::Dot,
        Token::CloseList,
    ];

    let expected = Syntax::List(vec![
        Syntax::dot(
            Some(Syntax::Number(1)),
            None,
        ),
    ]);

    let actual = parse(subject).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn dot_with_cdr_but_no_car() {
    let subject = vec![
        Token::OpenList,
        Token::Dot,
        Token::Number(2),
        Token::CloseList,
    ];

    let expected = Syntax::List(vec![
        Syntax::dot(
            None,
            Some(Syntax::Number(2)),
        ),
    ]);

    let actual = parse(subject).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn dot_with_no_car_and_no_cdr() {
    let subject = vec![
        Token::OpenList,
        Token::Dot,
        Token::CloseList,
    ];

    let expected = Syntax::List(vec![
        Syntax::dot(
            None,
            None,
        ),
    ]);

    let actual = parse(subject).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn dot_as_second_last_item_in_list() {
    let subject = vec![
        Token::OpenList,
        Token::Number(1),
        Token::Number(2),
        Token::Dot,
        Token::Number(3),
        Token::CloseList,
    ];

    let expected = Syntax::List(vec![
        Syntax::Number(1),
        Syntax::dot(
            Some(Syntax::Number(2)),
            Some(Syntax::Number(3)),
        ),
    ]);

    let actual = parse(subject).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn dot_as_last_item_in_list() {
    let subject = vec![
        Token::OpenList,
        Token::Number(1),
        Token::Number(2),
        Token::Dot,
        Token::CloseList,
    ];

    let expected = Syntax::List(vec![
        Syntax::Number(1),
        Syntax::dot(
            Some(Syntax::Number(2)),
            None,
        ),
    ]);

    let actual = parse(subject).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn dot_with_more_than_one_item_after_it_in_list() {
    let subject = vec![
        Token::OpenList,
        Token::Number(1),
        Token::Number(2),
        Token::Dot,
        Token::Number(3),
        Token::Number(4),
        Token::CloseList,
    ];

    let expected = SyntaxError::BadInfixDotNotation;

    let actual = parse(subject).err().unwrap();

    assert_eq!(expected, actual);
}
