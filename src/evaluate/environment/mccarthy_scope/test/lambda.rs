use super::*;

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

    let expected =  SXRef::function(Function::new(
        vec!["x".into()],
        SXRef::from(vec![
            SXRef::symbol("equal".into()),
            SXRef::symbol("x".into()),
            SXRef::number(1),
        ]),
    ));

    let actual = McCarthyScope::lambda(subject, &mut Env::new());

    assert_eq!(expected, actual)
}

#[test]
pub fn lambda_with_2_arg_and_def_produces_function() {
    panic!("FAIL")
}

#[test]
pub fn lambda_with_3_arg_and_def_produces_function() {
    panic!("FAIL")
}

#[test]
pub fn lambda_with_0_arg_and_def_produces_function() {
    panic!("FAIL")
}

#[test]
pub fn lambda_with_2_arg_and_number_def_produces_function() {
    panic!("FAIL")
}

#[test]
pub fn lambda_with_2_arg_and_string_def_produces_function() {
    panic!("FAIL")
}

#[test]
pub fn lambda_with_2_arg_and_symbol_def_produces_function() {
    panic!("FAIL")
}

#[test]
pub fn lambda_with_2_arg_and_quote_def_produces_function() {
    panic!("FAIL")
}

#[test]
pub fn lambda_with_2_arg_and_nil_def_produces_function() {
    panic!("FAIL")
}

#[test]
pub fn lambda_with_2_arg_and_function_def_produces_function() {
    panic!("FAIL")
}

#[test]
pub fn lambda_with_2_arg_and_macro_def_produces_function() {
    panic!("FAIL")
}

#[test]
pub fn lambda_with_2_arg_and_rust_function_def_produces_function() {
    panic!("FAIL")
}

#[test]
pub fn lambda_with_2_arg_and_rust_macro_def_produces_function() {
    panic!("FAIL")
}

#[test]
pub fn lambda_with_2_arg_and_no_def_produces_function() {
    panic!("FAIL")
}

#[test]
pub fn lambda_with_no_args_and_no_def_produces_function() {
    panic!("FAIL")
}
