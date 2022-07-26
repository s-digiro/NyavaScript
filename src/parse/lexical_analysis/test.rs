use super::*;

#[test]
fn blank_text_is_no_root_error() {
    assert_eq!(
        Err(LexError::no_root_list_error()),
        parse(""),
    );
}

#[test]
fn no_first_parenthesis_makes_no_root_error() {
    assert_eq!(
        Err(LexError::no_root_list_error()),
        parse("foo bar baz)"),
    );
}

#[test]
fn nil_list_works() {
    assert_eq!(
        vec![Token::OpenList, Token::CloseList],
        parse("()").unwrap(),
    );
}

#[test]
fn parse_quote_works() {
    assert_eq!(
        vec![
            Token::OpenList,
            Token::Quote,
            Token::OpenList,
            Token::symbol("a"),
            Token::symbol("b"),
            Token::CloseList,
            Token::CloseList,
        ],
        parse("( '(a b))").unwrap(),
    );
}

#[test]
fn parse_number_in_quote_works() {
    assert_eq!(
        vec![
            Token::OpenList,
            Token::Quote,
            Token::OpenList,
            Token::Number(1),
            Token::Number(2),
            Token::CloseList,
            Token::CloseList,
        ],
        parse("( '(1 2))").unwrap(),
    );
}

#[test]
fn parse_quote_as_not_first_in_list_works() {
    assert_eq!(
        vec![
            Token::OpenList,
            Token::symbol("car"),
            Token::Quote,
            Token::OpenList,
            Token::symbol("a"),
            Token::symbol("b"),
            Token::CloseList,
            Token::CloseList,
        ],
        parse("(car '(a b))").unwrap(),
    );
}

#[test]
fn parse_number_works() {
    assert_eq!(
        vec![
            Token::OpenList,
            Token::Number(1),
            Token::Number(2),
            Token::CloseList,
        ],
        parse("(1 2)").unwrap(),
    );
}

#[test]
fn parse_negative_number_works() {
    assert_eq!(
        vec![
            Token::OpenList,
            Token::Number(-1),
            Token::CloseList,
        ],
        parse("(-1)").unwrap(),
    );
}

#[test]
fn parse_simple_string() {
    assert_eq!(
        vec![
            Token::OpenList,
            Token::string("foo"),
            Token::CloseList,
        ],
        parse("(\"foo\")").unwrap(),
    );
}

#[test]
fn parse_string_with_spaces() {
    assert_eq!(
        vec![
            Token::OpenList,
            Token::string("foo bar"),
            Token::CloseList,
        ],
        parse("(\"foo bar\")").unwrap(),
    );
}

#[test]
fn parse_multiple_strings() {
    assert_eq!(
        vec![
            Token::OpenList,
            Token::string("foo bar"),
            Token::string("baz"),
            Token::CloseList,
        ],
        parse("(\"foo bar\"   \"baz\")").unwrap(),
    );
}

#[test]
fn parse_string_with_symbol_and_list() {
    assert_eq!(
        vec![
            Token::OpenList,
            Token::symbol("print"),
            Token::string("foo"),
            Token::OpenList,
            Token::string("bar"),
            Token::Number(1),
            Token::CloseList,
            Token::CloseList,
        ],
        parse("(print \"foo\" (\"bar\" 1))").unwrap(),
    );
}

#[test]
fn parse_blank_string() {
    assert_eq!(
        vec![
            Token::OpenList,
            Token::string(""),
            Token::CloseList,
        ],
        parse("(\"\")").unwrap(),
    );
}

#[test]
#[should_panic]
fn parse_unclosed_double_quote_returns_error() {
    parse("(\")").unwrap();
}

#[test]
#[should_panic]
fn parse_triple_quote_returns_error() {
    parse("(\"\"\")").unwrap();
}

#[test]
fn unterminated_string_error_location_is_correct() {
    let s = "(\nfoo \"bar)";
    assert_eq!(
        Err(LexError::unterminated_string_error("\"bar)".to_owned(), 2, 5)),
        parse(s),
    )
}

#[test]
fn backslash_escapes_double_quote_in_string() {
    assert_eq!(
        vec![
            Token::OpenList,
            Token::string("\""),
            Token::CloseList,
        ],
        parse("(\"\\\"\")").unwrap(),
    );
}

#[test]
fn backslash_escapes_char_in_string() {
    assert_eq!(
        vec![
            Token::OpenList,
            Token::string("ab"),
            Token::CloseList,
        ],
        parse("(\"a\\b\")").unwrap(),
    );
}

#[test]
fn backslash_does_not_escape_in_symbol() {
    assert_eq!(
        vec![
            Token::OpenList,
            Token::symbol("a\\b"),
            Token::CloseList,
        ],
        parse("(a\\b)").unwrap(),
    );
}

#[test]
fn single_backslash_symbol() {
    assert_eq!(
        vec![
            Token::OpenList,
            Token::symbol("\\"),
            Token::CloseList,
        ],
        parse("(\\)").unwrap(),
    );
}

#[test]
fn symbol_can_contain_1_double_quote() {
    assert_eq!(
        vec![
            Token::OpenList,
            Token::symbol("g\"h"),
            Token::CloseList,
        ],
        parse("(g\"h)").unwrap(),
    );
}

#[test]
fn symbol_can_contain_2_double_quote() {
    assert_eq!(
        vec![
            Token::OpenList,
            Token::symbol("g\"h\""),
            Token::CloseList,
        ],
        parse("(g\"h\")").unwrap(),
    );
}

#[test]
fn parse_backslash_in_string_works() {
    assert_eq!(
        vec![
            Token::OpenList,
            Token::string("s"),
            Token::string("\""),
            Token::string("\\"),
            Token::CloseList,
        ],
        parse("(\"\\s\" \"\\\"\" \"\\\\\")").unwrap(),
    );
}

#[test]
fn parse_works() {
    assert_eq!(
        vec![
            Token::OpenList,
            Token::symbol("foo"),
            Token::symbol("bar"),
            Token::CloseList],
        parse("(foo bar)").unwrap(),
    );

    assert_eq!(
        vec![
            Token::OpenList,
            Token::symbol("foo"),
            Token::OpenList,
            Token::symbol("bar"),
            Token::symbol("baz"),
            Token::CloseList,
            Token::CloseList],
        parse("(foo (bar baz))").unwrap(),
    );

    assert_eq!(
        vec![
            Token::OpenList,
            Token::symbol("print"),
            Token::Quote,
            Token::OpenList,
            Token::Number(1),
            Token::Number(2),
            Token::CloseList,
            Token::CloseList],
        parse("(print '(1 2))").unwrap(),
    );
}

#[test]
fn lone_dot_parses_into_dot_token() {
    panic!("FAIL");
}

#[test]
fn dot_as_part_of_atom_does_not_parse_into_dot_token() {
    panic!("FAIL");
}
