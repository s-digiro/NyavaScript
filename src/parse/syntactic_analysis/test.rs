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
        Err(SyntaxError::UnmatchedCloseListError),
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
        Err(SyntaxError::UnclosedRootListError),
    );

    assert_eq!(
        parse(vec![Token::OpenList]),
        Err(SyntaxError::UnclosedRootListError),
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
