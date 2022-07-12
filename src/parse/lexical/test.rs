use super::*;

#[test]
fn blank_text_is_empty_list() {
    let empty: Vec<Symbol> = Vec::new();

    assert_eq!(
        empty,
        parse(""),
    );
}

#[test]
fn nil_list_works() {
    assert_eq!(
        vec![Symbol::OpenList, Symbol::CloseList],
        parse("()"),
    );
}

#[test]
fn parse_quote_works() {
    assert_eq!(
        vec![
            Symbol::OpenList,
            Symbol::Quote,
            Symbol::OpenList,
            Symbol::atom("1"),
            Symbol::atom("2"),
            Symbol::CloseList,
            Symbol::CloseList,
        ],
        parse("( '(1 2))"),
    );
}

#[test]
fn parse_works() {
    assert_eq!(
        vec![
            Symbol::OpenList,
            Symbol::atom("foo"),
            Symbol::atom("bar"),
            Symbol::CloseList],
        parse("(foo bar)"),
    );

    assert_eq!(
        vec![
            Symbol::OpenList,
            Symbol::atom("foo"),
            Symbol::OpenList,
            Symbol::atom("bar"),
            Symbol::atom("baz"),
            Symbol::CloseList,
            Symbol::CloseList],
        parse("(foo (bar baz))"),
    );

    assert_eq!(
        vec![
            Symbol::OpenList,
            Symbol::atom("print"),
            Symbol::Quote,
            Symbol::OpenList,
            Symbol::atom("1"),
            Symbol::atom("2"),
            Symbol::CloseList,
            Symbol::CloseList],
        parse("(print '(1 2))"),
    );
}
