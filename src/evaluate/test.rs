use super::*;
use crate::s_expression::ConsCell;
use crate::evaluate::{
    Error as EvalError,
    McCarthyScope,
    UnboundFnCallError,
};

fn full_env() -> Env {
    let mut ret = Env::new();
    ret.push(McCarthyScope::new());

    ret
}

#[test]
fn eval_nil_returns_nil() {
    let subject = SXRef::nil();

    let expected = SXRef::nil();

    let actual = eval(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn eval_number_returns_number() {
    let subject = SXRef::number(1);

    let expected = SXRef::number(1);

    let actual = eval(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn eval_string_returns_string() {
    let subject = SXRef::string("foo".into());

    let expected = SXRef::string("foo".into());

    let actual = eval(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn eval_undefined_symbol_returns_nil() {
    let subject = SXRef::symbol("foo".into());

    let expected = SXRef::nil();

    let actual = eval(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn eval_defined_symbol_returns_definition() {
    let mut env = Env::new();
    env.defun("foo".into(), SXRef::number(1));

    let subject = SXRef::symbol("foo".into());

    let expected = SXRef::number(1);

    let actual = eval(subject, &mut env).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn eval_quoted_symbol_returns_symbol() {
    let subject = SXRef::quote(SXRef::symbol("foo".into()));

    let expected = SXRef::symbol("foo".into());

    let actual = eval(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn eval_quoted_number_returns_number() {
    let subject = SXRef::quote(SXRef::number(1));

    let expected = SXRef::number(1);

    let actual = eval(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn eval_quoted_string_returns_string() {
    let subject = SXRef::quote(SXRef::string("foo".into()));

    let expected = SXRef::string("foo".into());

    let actual = eval(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn eval_quoted_list_returns_list() {
    let subject = SXRef::quote(SXRef::from(vec![
        SXRef::number(1),
        SXRef::number(2),
    ]));

    let expected= SXRef::from(vec![
        SXRef::number(1),
        SXRef::number(2),
    ]);

    let actual = eval(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn eval_list_with_fn_as_first_arg_evals_fn() {
    let subject = SXRef::from(vec![
        SXRef::symbol("cons".into()),
        SXRef::number(1),
        SXRef::number(2),
    ]);

    let expected = SXRef::cons_cell(ConsCell::new(
        SXRef::number(1),
        SXRef::number(2),
    ));

    let mut env = Env::new();
    env.push(McCarthyScope::new());

    let actual = eval(subject, &mut env).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn eval_list_with_literal_as_first_arg_returns_error() {
    let subject = SXRef::from(vec![
        SXRef::number(3),
        SXRef::number(1),
        SXRef::number(2),
    ]);

    let expected = EvalError::unbound_fn_call_error(
        UnboundFnCallError::new(SXRef::number(3))
    );

    let actual = eval(subject, &mut Env::new());

    match actual {
        Ok(x) => panic!("Expected error. Received {}", x),
        Err(e) => assert_eq!(expected, e),
    }
}

#[test]
fn eval_list_with_macro_as_first_arg_evals_macro() {
    let subject = SXRef::from(vec![
        SXRef::symbol("quote".into()),
        SXRef::number(1),
    ]);

    let expected = SXRef::number(1);

    let mut env = Env::new();
    env.push(McCarthyScope::new());

    let actual = eval(subject, &mut env).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn eval_list_with_undefined_symbol_as_first_arg_returns_error() {
    let subject = SXRef::from(vec![
        SXRef::symbol("foo".into()),
        SXRef::number(1),
        SXRef::number(2),
    ]);

    let expected = EvalError::unbound_fn_call_error(
        UnboundFnCallError::new(SXRef::symbol("foo".into()))
    );

    let actual = eval(subject, &mut Env::new());

    match actual {
        Ok(x) => panic!("Expected error. Received {}", x),
        Err(e) => assert_eq!(expected, e),
    }
}

#[test]
fn eval_all_on_empty() {
    let subject = vec![];

    let expected = EvalError::NoOp;

    let actual = eval_all(subject, &mut Env::new()).err().unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn eval_all_on_one_sxref() {
    let subject = vec![
        SXRef::from(vec![
            SXRef::symbol("cons".into()),
            SXRef::number(1),
            SXRef::number(2),
        ])
    ];

    let expected = SXRef::cons_cell(ConsCell::new(
        SXRef::number(1),
        SXRef::number(2),
    ));

    let actual = eval_all(subject, &mut full_env()).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn eval_all_on_one_atom() {
    let subject = vec![
        SXRef::number(2),
    ];

    let expected = SXRef::number(2);

    let actual = eval_all(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn eval_all_on_multiple_sxref() {
    let subject = vec![
        SXRef::from(vec![
            SXRef::symbol("cons".into()),
            SXRef::number(1),
            SXRef::number(2),
        ]),
        SXRef::from(vec![
            SXRef::symbol("cons".into()),
            SXRef::number(3),
            SXRef::number(4),
        ])
    ];

    let expected = SXRef::cons_cell(ConsCell::new(
        SXRef::number(3),
        SXRef::number(4),
    ));

    let actual = eval_all(subject, &mut full_env()).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn eval_all_env_persists_between_sxref() {
    let subject = vec![
        SXRef::from(vec![
            SXRef::symbol("defun".into()),
            SXRef::symbol("foo".into()),
            SXRef::from(vec![
                SXRef::symbol("x".into()),
            ]),
            SXRef::from(vec![
                SXRef::symbol("cons".into()),
                SXRef::symbol("x".into()),
                SXRef::number(2),
            ]),
        ]),
        SXRef::from(vec![
            SXRef::symbol("foo".into()),
            SXRef::number(3),
        ])
    ];

    let expected = SXRef::cons_cell(ConsCell::new(
        SXRef::number(3),
        SXRef::number(2),
    ));

    let actual = eval_all(subject, &mut full_env()).unwrap();

    assert_eq!(expected, actual);
}
