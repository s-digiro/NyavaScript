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
            Symbol::atom("a"),
            Symbol::atom("b"),
            Symbol::CloseList,
            Symbol::CloseList,
        ],
        parse("( '(a b))"),
    );
}

#[test]
fn parse_number_in_quote_works() {
    assert_eq!(
        vec![
            Symbol::OpenList,
            Symbol::Quote,
            Symbol::OpenList,
            Symbol::Number(1),
            Symbol::Number(2),
            Symbol::CloseList,
            Symbol::CloseList,
        ],
        parse("( '(1 2))"),
    );
}

#[test]
fn parse_number_works() {
    assert_eq!(
        vec![
            Symbol::OpenList,
            Symbol::Number(1),
            Symbol::Number(2),
            Symbol::CloseList,
        ],
        parse("(1 2)"),
    );
}

#[test]
fn parse_string_works() {
    assert_eq!(
        vec![
            Symbol::OpenList,
            Symbol::atom("print"),
            Symbol::string("Hello World!"),
            Symbol::CloseList,
        ],
        parse("(print \"Hello World!\")"),
    );
}

#[test]
fn parse_backslash_in_string_works() {
    assert_eq!(
        vec![
            Symbol::OpenList,
            Symbol::string("s"),
            Symbol::string("\""),
            Symbol::string("\\"),
            Symbol::CloseList,
        ],
        parse("(\"\\s\" \"\\\"\" \"\\\\\")"),
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
            Symbol::Number(1),
            Symbol::Number(2),
            Symbol::CloseList,
            Symbol::CloseList],
        parse("(print '(1 2))"),
    );
}
