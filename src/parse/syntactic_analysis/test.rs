use super::*;

#[test]
fn unmatched_open_parenthesis_fails() {
    assert_eq!(
        parse(vec![Token::OpenList]),
        Err(SyntaxError::UnmatchedOpenListError),
    );
}

#[test]
fn unmatched_close_parenthesis_fails() {
    assert_eq!(
        parse(vec![Token::CloseList]),
        Err(SyntaxError::UnmatchedCloseListError),
    );
}

#[test]
fn parse_blank_works() {
    let empty = Vec::new();
    assert_eq!(
        parse(empty).unwrap(),
        Syntax::list(),
    );
}


#[test]
fn parse_nil_list_works() {
    assert_eq!(
        parse(vec![Token::OpenList, Token::CloseList]).unwrap(),
        Syntax::List(vec![Syntax::list()]),
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
        Syntax::List(vec![Syntax::List(vec![Syntax::String("foo".to_owned())])])
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
        Syntax::List(vec![Syntax::List(vec![Syntax::Number(105)])])
    );
}

#[test]
fn parse_works() {
    assert_eq!(
        parse(vec![
              Token::OpenList,
              Token::symbol("foo"),
              Token::symbol("bar"),
              Token::CloseList]).unwrap(),
        Syntax::List(vec![
            Syntax::List(vec![Syntax::symbol("foo"), Syntax::symbol("bar")])]));

    assert_eq!(
        parse(vec![
              Token::OpenList,
              Token::symbol("foo"),
              Token::OpenList,
              Token::symbol("bar"),
              Token::symbol("baz"),
              Token::CloseList,
              Token::CloseList]).unwrap(),
        Syntax::List(vec![
            Syntax::List(vec![
                Syntax::symbol("foo"),
                Syntax::List(vec![
                    Syntax::symbol("bar"),
                    Syntax::symbol("baz")])])
                ]));

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
                Syntax::List(vec![
                    Syntax::symbol("foo"),
                    Syntax::symbol("bar")]),
                Syntax::symbol("baz")])]));
}
