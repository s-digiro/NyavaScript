use super::*;

use crate::s_expression::Macro;

#[test]
pub fn lambda_with_1_arg_and_def_produces_function() {
    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::from(vec![
            SXRef::symbol("x".into()),
        ]),
        SXRef::from(vec![
            SXRef::symbol("equal".into()),
            SXRef::symbol("x".into()),
            SXRef::number(1),
        ]),
    ]);

    let expected =  SXRef::function(Function::lisp_function(
        vec!["x".into()],
        SXRef::from(vec![
            SXRef::symbol("equal".into()),
            SXRef::symbol("x".into()),
            SXRef::number(1),
        ]),
        &mut Env::new(),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn lambda_with_2_arg_and_def_produces_function() {
    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::from(vec![
            SXRef::symbol("x".into()),
            SXRef::symbol("y".into()),
        ]),
        SXRef::from(vec![
            SXRef::symbol("equal".into()),
            SXRef::symbol("x".into()),
            SXRef::symbol("y".into()),
        ]),
    ]);

    let expected =  SXRef::function(Function::lisp_function(
        vec![
            "x".into(),
            "y".into()
        ],
        SXRef::from(vec![
            SXRef::symbol("equal".into()),
            SXRef::symbol("x".into()),
            SXRef::symbol("y".into()),
        ]),
        &mut Env::new(),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn lambda_with_3_arg_and_def_produces_function() {
    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::from(vec![
            SXRef::symbol("x".into()),
            SXRef::symbol("y".into()),
            SXRef::symbol("z".into()),
        ]),
        SXRef::from(vec![
            SXRef::symbol("equal".into()),
            SXRef::symbol("x".into()),
            SXRef::symbol("y".into()),
            SXRef::symbol("z".into()),
        ]),
    ]);

    let expected =  SXRef::function(Function::lisp_function(
        vec![
            "x".into(),
            "y".into(),
            "z".into(),
        ],
        SXRef::from(vec![
            SXRef::symbol("equal".into()),
            SXRef::symbol("x".into()),
            SXRef::symbol("y".into()),
            SXRef::symbol("z".into()),
        ]),
        &mut Env::new(),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn lambda_with_0_arg_and_def_produces_function() {
    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::nil(),
        SXRef::from(vec![
            SXRef::symbol("equal".into()),
            SXRef::symbol("x".into()),
            SXRef::symbol("y".into()),
            SXRef::symbol("z".into()),
        ]),
    ]);

    let empty: Vec<String> = Vec::new();

    let expected =  SXRef::function(Function::lisp_function(
        empty,
        SXRef::from(vec![
            SXRef::symbol("equal".into()),
            SXRef::symbol("x".into()),
            SXRef::symbol("y".into()),
            SXRef::symbol("z".into()),
        ]),
        &mut Env::new(),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn lambda_with_2_arg_and_number_def_produces_function() {
    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::from(vec![
            SXRef::symbol("x".into()),
            SXRef::symbol("y".into()),
        ]),
        SXRef::number(1),
    ]);

    let expected =  SXRef::function(Function::lisp_function(
        vec![
            "x".into(),
            "y".into()
        ],
        SXRef::number(1),
        &mut Env::new(),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn lambda_with_2_arg_and_string_def_produces_function() {
    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::from(vec![
            SXRef::symbol("x".into()),
            SXRef::symbol("y".into()),
        ]),
        SXRef::string("foo".into()),
    ]);

    let expected =  SXRef::function(Function::lisp_function(
        vec![
            "x".into(),
            "y".into()
        ],
        SXRef::string("foo".into()),
        &mut Env::new(),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn lambda_with_2_arg_and_symbol_def_produces_function() {
    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::from(vec![
            SXRef::symbol("x".into()),
            SXRef::symbol("y".into()),
        ]),
        SXRef::symbol("foo".into()),
    ]);

    let expected =  SXRef::function(Function::lisp_function(
        vec![
            "x".into(),
            "y".into()
        ],
        SXRef::symbol("foo".into()),
        &mut Env::new(),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn lambda_with_2_arg_and_quote_def_produces_function() {
    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::from(vec![
            SXRef::symbol("x".into()),
            SXRef::symbol("y".into()),
        ]),
        SXRef::quote(SXRef::symbol("foo".into())),
    ]);

    let expected =  SXRef::function(Function::lisp_function(
        vec![
            "x".into(),
            "y".into()
        ],
        SXRef::quote(SXRef::symbol("foo".into())),
        &mut Env::new(),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn lambda_with_2_arg_and_nil_def_produces_function() {
    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::from(vec![
            SXRef::symbol("x".into()),
            SXRef::symbol("y".into()),
        ]),
        SXRef::nil(),
    ]);

    let expected =  SXRef::function(Function::lisp_function(
        vec![
            "x".into(),
            "y".into()
        ],
        SXRef::nil(),
        &mut Env::new(),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn lambda_with_2_arg_and_function_def_produces_function() {
    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::from(vec![
            SXRef::symbol("x".into()),
            SXRef::symbol("y".into()),
        ]),
        SXRef::function("()".try_into().unwrap()),
    ]);

    let expected =  SXRef::function(Function::lisp_function(
        vec![
            "x".into(),
            "y".into()
        ],
        SXRef::function("()".try_into().unwrap()),
        &mut Env::new(),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn lambda_with_2_arg_and_macro_def_produces_function() {
    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::from(vec![
            SXRef::symbol("x".into()),
            SXRef::symbol("y".into()),
        ]),
        SXRef::r#macro("()".try_into().unwrap()),
    ]);

    let expected =  SXRef::function(Function::lisp_function(
        vec![
            "x".into(),
            "y".into()
        ],
        SXRef::r#macro("()".try_into().unwrap()),
        &mut Env::new(),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn lambda_with_2_arg_and_rust_function_def_produces_function() {
    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::from(vec![
            SXRef::symbol("x".into()),
            SXRef::symbol("y".into()),
        ]),
        RustFunction::new(dummy_fn).into(),
    ]);

    let expected_args: Vec<String> = vec![
        "x".into(),
        "y".into()
    ];

    let actual = McCarthyScope::lambda(subject, &mut Env::new()).unwrap();

    match &*actual {
        SX::Function(f) => match f {
            Function::Lisp(f) => {
                assert_eq!(&expected_args, f.args());

                match &*f.definition() {
                    SX::Function(f) => match f {
                        Function::Rust(_) => (),
                        _ => panic!("Expected definition to be a RustFunction. Was {:?}", f),
                    },
                    _ => panic!("Expected definition to be a RustFunction. Was {:?}", f),
                }
            },
            _ => panic!("Expected Function::Lisp. Recieved: {:?}", f),
        },
        _ => panic!("Expected SX::Function. Recieved: {:?}", actual),
    }
}

#[test]
pub fn lambda_with_2_arg_and_rust_macro_def_produces_function() {
    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::from(vec![
            SXRef::symbol("x".into()),
            SXRef::symbol("y".into()),
        ]),
        SXRef::r#macro(RustMacro::new(dummy_macro).into()),
    ]);

    let expected_args: Vec<String> = vec![
        "x".into(),
        "y".into()
    ];

    let actual = McCarthyScope::lambda(subject, &mut Env::new()).unwrap();

    match &*actual {
        SX::Function(f) => match f {
            Function::Lisp(f) => {
                assert_eq!(&expected_args, f.args());

                match &*f.definition() {
                    SX::Macro(m) => match m {
                        Macro::Rust(_) => (),
                        Macro::Lisp(l) => panic!("Expected definition to be a RustMacro. Was {:?}", l)
                    },
                    x => panic!("Expected definition to be a RustMacro. Was {:?}", x),
                }
            },
            _ => panic!("Expected Function::Lisp. Recieved: {:?}", f),
        },
        _ => panic!("Expected SX::Function. Recieved: {:?}", actual),
    }
}

#[test]
pub fn lambda_with_2_arg_and_no_def_produces_function() {
    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::from(vec![
            SXRef::symbol("x".into()),
            SXRef::symbol("y".into()),
        ]),
    ]);

    let expected =  SXRef::function(Function::lisp_function(
        vec![
            "x".into(),
            "y".into()
        ],
        SXRef::nil(),
        &mut Env::new(),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn lambda_with_no_args_and_no_def_produces_function() {
    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
    ]);

    let empty: Vec<String> = Vec::new();

    let expected =  SXRef::function(Function::lisp_function(
        empty,
        SXRef::nil(),
        &mut Env::new(),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual)
}

#[test]
pub fn lambda_with_non_list_args_returns_function_that_takes_no_args() {
    let empty: Vec<String> = Vec::new();

    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::number(1),
        SXRef::number(1),
    ]);

    let expected =  SXRef::function(Function::lisp_function(
        empty.clone(),
        SXRef::number(1),
        &mut Env::new(),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);

    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::symbol("foo".into()),
        SXRef::number(1),
    ]);

    let expected =  SXRef::function(Function::lisp_function(
        empty.clone(),
        SXRef::number(1),
        &mut Env::new(),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);

    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::string("foo".into()),
        SXRef::number(1),
    ]);

    let expected =  SXRef::function(Function::lisp_function(
        empty.clone(),
        SXRef::number(1),
        &mut Env::new(),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);

    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::quote(SXRef::string("foo".into())),
        SXRef::number(1),
    ]);

    let expected =  SXRef::function(Function::lisp_function(
        empty.clone(),
        SXRef::number(1),
        &mut Env::new(),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);

    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::r#macro("()".try_into().unwrap()),
        SXRef::number(1),
    ]);

    let expected =  SXRef::function(Function::lisp_function(
        empty.clone(),
        SXRef::number(1),
        &mut Env::new(),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);

    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::function("()".try_into().unwrap()),
        SXRef::number(1),
    ]);

    let expected =  SXRef::function(Function::lisp_function(
        empty.clone(),
        SXRef::number(1),
        &mut Env::new(),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);

    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        RustFunction::new(dummy_fn).into(),
        SXRef::number(1),
    ]);

    let expected =  SXRef::function(Function::lisp_function(
        empty.clone(),
        SXRef::number(1),
        &mut Env::new(),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);

    let subject = SXRef::from(vec![
        SXRef::symbol("lambda".into()),
        SXRef::r#macro(RustMacro::new(dummy_macro).into()),
        SXRef::number(1),
    ]);

    let expected =  SXRef::function(Function::lisp_function(
        empty.clone(),
        SXRef::number(1),
        &mut Env::new(),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new()).unwrap();

    assert_eq!(expected, actual);
}
