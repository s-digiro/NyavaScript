use super::SExpression as SX;
use super::SExpressionRef as SXRef;
use super::*;

use crate::evaluate::Environment as Env;

fn dummy_fn(_args: &Vec<SXRef>, _env: &mut Env) -> SXRef {
    SXRef::nil()
}

fn dummy_macro(_sx: SXRef, _env: &mut Env) -> SXRef {
    SXRef::nil()
}

#[test]
fn is_rust_function_returns_true_for_rust_function() {
    let subject = SX::RustFunction(RustFunction::new(dummy_fn));

    let expected = true;

    let actual = subject.is_rust_function();

    assert_eq!(expected, actual);
}

#[test]
fn is_rust_function_returns_false_for_non_rust_function() {
    let subjects = vec![
        SX::from(vec![SXRef::number(1)]),
        SX::Function("()".try_into().unwrap()),
        SX::Macro("()".try_into().unwrap()),
        SX::Nil,
        SX::Number(1),
        SX::Quote(SXRef::number(1)),
        SX::RustMacro(RustMacro::new(dummy_macro)),
        SX::String("foo".into()),
        SX::Symbol("foo".into()),
    ];

    let expected = false;

    for subject in subjects {
        let actual = subject.is_rust_function();

        assert_eq!(expected, actual);
    }
}

#[test]
fn is_rust_macro_returns_true_for_rust_macro() {
    let subject = SX::RustMacro(RustMacro::new(dummy_macro));

    let expected = true;

    let actual = subject.is_rust_macro();

    assert_eq!(expected, actual);
}

#[test]
fn is_rust_macro_returns_false_for_non_rust_macro() {
    let subjects = vec![
        SX::from(vec![SXRef::number(1)]),
        SX::Function("()".try_into().unwrap()),
        SX::Macro("()".try_into().unwrap()),
        SX::Nil,
        SX::Number(1),
        SX::Quote(SXRef::number(1)),
        SX::RustFunction(RustFunction::new(dummy_fn)),
        SX::String("foo".into()),
        SX::Symbol("foo".into()),
    ];

    let expected = false;

    for subject in subjects {
        let actual = subject.is_rust_macro();

        assert_eq!(expected, actual);
    }
}
