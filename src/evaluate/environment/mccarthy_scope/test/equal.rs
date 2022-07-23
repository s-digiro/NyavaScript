use super::*;

#[test]
pub fn equal_returns_1_when_comparing_two_same_symbols() {
    let subject = vec![
        SXRef::symbol("equal".into()),
        SXRef::symbol("foo".into()),
        SXRef::symbol("foo".into()),
    ];

    let expected = SXRef::number(1);

    let actual = McCarthyScope::equal(&subject, &mut Env::new());

    assert_eq!(expected, actual)
}

#[test]
pub fn equal_returns_nil_when_comparing_two_diff_symbols() {
    panic!("FAIL")
}

#[test]
pub fn equal_returns_nil_when_comparing_one_symbols() {
    panic!("FAIL")
}

#[test]
pub fn equal_returns_1_when_comparing_two_same_strings() {
    panic!("FAIL")
}

#[test]
pub fn equal_returns_nil_when_comparing_two_diff_strings() {
    panic!("FAIL")
}

#[test]
pub fn equal_returns_nil_when_comparing_one_string() {
    panic!("FAIL")
}

#[test]
pub fn equal_returns_1_when_comparing_two_nil() {
    panic!("FAIL")
}

#[test]
pub fn equal_returns_1_when_comparing_one_nil() {
    panic!("FAIL")
}

#[test]
pub fn equal_returns_1_when_comparing_two_same_numbers() {
    panic!("FAIL")
}

#[test]
pub fn equal_returns_nil_when_comparing_two_diff_numbers() {
    panic!("FAIL")
}

#[test]
pub fn equal_returns_nil_when_comparing_one_number() {
    panic!("FAIL")
}

#[test]
pub fn equal_returns_1_when_comparing_two_same_quote() {
    panic!("FAIL")
}

#[test]
pub fn equal_returns_nil_when_comparing_two_diff_quote() {
    panic!("FAIL")
}

#[test]
pub fn equal_returns_nil_when_comparing_one_quote() {
    panic!("FAIL")
}

#[test]
pub fn equal_returns_nil_when_called_on_function() {
    panic!("FAIL")
}

#[test]
pub fn equal_returns_nil_when_called_on_macro() {
    panic!("FAIL")
}

#[test]
pub fn equal_returns_nil_when_called_on_rust_macro() {
    panic!("FAIL")
}

#[test]
pub fn equal_returns_nil_when_called_on_rust_function() {
    panic!("FAIL")
}

#[test]
pub fn equal_returns_nil_when_called_on_diff_types() {
    panic!("FAIL")
}

#[test]
pub fn equal_ignores_args_past_2nd() {
    panic!("FAIL")
}
