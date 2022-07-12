use super::*;

#[test]
fn unmatched_open_parenthesis_fails() {
    assert_eq!(
        parse(vec![Symbol::OpenList]),
        Err(Error::UnmatchedOpenListError),
    );
}

#[test]
fn unmatched_close_parenthesis_fails() {
    assert_eq!(
        parse(vec![Symbol::CloseList]),
        Err(Error::UnmatchedCloseListError),
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
        parse(vec![Symbol::OpenList, Symbol::CloseList]).unwrap(),
        Syntax::List(vec![Syntax::list()]),
    );
}

#[test]
fn parse_works() {
    assert_eq!(
        parse(vec![
              Symbol::OpenList,
              Symbol::atom("foo"),
              Symbol::atom("bar"),
              Symbol::CloseList]).unwrap(),
        Syntax::List(vec![
            Syntax::List(vec![Syntax::atom("foo"), Syntax::atom("bar")])]));

    assert_eq!(
        parse(vec![
              Symbol::OpenList,
              Symbol::atom("foo"),
              Symbol::OpenList,
              Symbol::atom("bar"),
              Symbol::atom("baz"),
              Symbol::CloseList,
              Symbol::CloseList]).unwrap(),
        Syntax::List(vec![
            Syntax::List(vec![
                Syntax::atom("foo"),
                Syntax::List(vec![
                    Syntax::atom("bar"),
                    Syntax::atom("baz")])])
                ]));

    assert_eq!(
        parse(vec![
              Symbol::OpenList,
              Symbol::OpenList,
              Symbol::atom("foo"),
              Symbol::atom("bar"),
              Symbol::CloseList,
              Symbol::atom("baz"),
              Symbol::CloseList]).unwrap(),
        Syntax::List(vec![
            Syntax::List(vec![
                Syntax::List(vec![
                    Syntax::atom("foo"),
                    Syntax::atom("bar")]),
                Syntax::atom("baz")])]));
}
