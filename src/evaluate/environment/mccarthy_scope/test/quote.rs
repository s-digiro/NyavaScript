use super::*;

use crate::s_expression::Macro;

#[test]
pub fn quote_symbol_returns_quoted_symbol() {
    let subject = SXRef::from(vec![
        SXRef::symbol("quote".into()),
        SXRef::symbol("x".into()),
    ]);

    let expected =  SXRef::quote(SXRef::symbol("x".into()));

    let actual = McCarthyScope::quote(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);
}

#[test]
pub fn quote_nothing_returns_quoted_nil() {
    let subject = SXRef::from(vec![
        SXRef::symbol("quote".into()),
    ]);

    let expected =  SXRef::quote(SXRef::nil());

    let actual = McCarthyScope::quote(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);
}

#[test]
pub fn quote_string_returns_quoted_string() {
    let subject = SXRef::from(vec![
        SXRef::symbol("quote".into()),
        SXRef::string("x".into()),
    ]);

    let expected =  SXRef::quote(SXRef::string("x".into()));

    let actual = McCarthyScope::quote(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);
}

#[test]
pub fn quote_number_returns_quoted_number() {
    let subject = SXRef::from(vec![
        SXRef::symbol("quote".into()),
        SXRef::number(1),
    ]);

    let expected =  SXRef::quote(SXRef::number(1));

    let actual = McCarthyScope::quote(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);
}

#[test]
pub fn quote_list_returns_quoted_list() {
    let subject = SXRef::from(vec![
        SXRef::symbol("quote".into()),
        SXRef::from(vec![
            SXRef::symbol("eq".into()),
            SXRef::number(1),
            SXRef::number(2),
        ]),
    ]);

    let expected =  SXRef::quote(
        SXRef::from(vec![
            SXRef::symbol("eq".into()),
            SXRef::number(1),
            SXRef::number(2),
        ]),
    );

    let actual = McCarthyScope::quote(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);
}
#[test]
pub fn quote_quote_returns_quoted_quote() {
    let subject = SXRef::from(vec![
        SXRef::symbol("quote".into()),
        SXRef::quote(SXRef::number(1)),
    ]);

    let expected =  SXRef::quote(
        SXRef::quote(SXRef::number(1)),
    );

    let actual = McCarthyScope::quote(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);
}

#[test]
pub fn quote_list_with_sublists_returns_quoted_list_with_sublists() {
    let subject = SXRef::from(vec![
        SXRef::symbol("quote".into()),
        SXRef::from(vec![
            SXRef::from(vec![
                SXRef::symbol("cond".into()),
                SXRef::from(vec![
                    SXRef::nil(),
                    SXRef::symbol("car".into()),
                ]),
                SXRef::from(vec![
                    SXRef::number(1),
                    SXRef::symbol("cdr".into()),
                ]),
            ]),
            SXRef::from(vec![
                SXRef::number(1),
                SXRef::number(2),
                SXRef::number(3),
            ]),
        ]),
    ]);

    let expected =  SXRef::quote(
        SXRef::from(vec![
            SXRef::from(vec![
                SXRef::symbol("cond".into()),
                SXRef::from(vec![
                    SXRef::nil(),
                    SXRef::symbol("car".into()),
                ]),
                SXRef::from(vec![
                    SXRef::number(1),
                    SXRef::symbol("cdr".into()),
                ]),
            ]),
            SXRef::from(vec![
                SXRef::number(1),
                SXRef::number(2),
                SXRef::number(3),
            ]),
        ]),
    );

    let actual = McCarthyScope::quote(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);
}

#[test]
pub fn quote_function_returns_quoted_function() {
    let subject = SXRef::from(vec![
        SXRef::symbol("quote".into()),
        SXRef::function("()".try_into().unwrap()),
    ]);

    let expected =  SXRef::quote(
        SXRef::function("()".try_into().unwrap()),
    );

    let actual = McCarthyScope::quote(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);
}

#[test]
pub fn quote_macro_returns_quoted_macro() {
    let subject = SXRef::from(vec![
        SXRef::symbol("quote".into()),
        SXRef::r#macro("()".try_into().unwrap()),
    ]);

    let expected =  SXRef::quote(
        SXRef::r#macro("()".try_into().unwrap()),
    );

    let actual = McCarthyScope::quote(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);
}

#[test]
pub fn quote_rust_function_returns_quoted_rust_function() {
    let subject = SXRef::from(vec![
        SXRef::symbol("quote".into()),
        RustFunction::new(dummy_fn).into(),
    ]);

    let actual = McCarthyScope::quote(subject, &mut Env::new()).unwrap();

    match &*actual {
        SX::Quote(sx) => match &**sx {
            SX::Function(f) => match f {
                Function::Rust(_) => (),
                Function::Lisp(_) => panic!("Expected SX::Quote(SX::RustFunction). Recieved: {:?}", sx),
            },
            sx => panic!("Expected SX::Quote(SX::RustFunction). Recieved: {:?}", sx),
        }
        sx => panic!("Expected SX::Quote(SX::RustFunction). Recieved: {:?}", sx),
    }
}

#[test]
pub fn quote_rust_macro_returns_quoted_rust_macro() {
    let subject = SXRef::from(vec![
        SXRef::symbol("quote".into()),
        SXRef::r#macro(RustMacro::new(dummy_macro).into()),
    ]);

    let actual = McCarthyScope::quote(subject, &mut Env::new()).unwrap();

    match &*actual {
        SX::Quote(sx) => match &**sx {
            SX::Macro(m) => match m {
                Macro::Rust(_) => (),
                Macro::Lisp(_) => panic!("Expected SX::Quote(SX::RustMacro). Recieved: {:?}", sx),
            },
            sx => panic!("Expected SX::Quote(SX::RustMacro). Recieved: {:?}", sx),
        }
        sx => panic!("Expected SX::Quote(SX::RustMacro). Recieved: {:?}", sx),
    }
}
