use super::*;

#[test]
fn cadr() {
    let expected: SXRef = LispFunction::new(
        vec!["x".into()],
        vec![
            SXRef::symbol("car".into()),
            vec![
                SXRef::symbol("cdr".into()),
                SXRef::symbol("x".into()),
            ].into(),
        ].into(),
    ).into();

    let actual = mc_env().get("cadr");

    assert_eq!(expected, actual)
}

#[test]
fn cdar() {
    let expected: SXRef = LispFunction::new(
        vec!["x".into()],
        vec![
            SXRef::symbol("cdr".into()),
            vec![
                SXRef::symbol("car".into()),
                SXRef::symbol("x".into()),
            ].into(),
        ].into(),
    ).into();

    let actual = mc_env().get("cdar");

    assert_eq!(expected, actual)
}

#[test]
fn caar() {
    let expected: SXRef = LispFunction::new(
        vec!["x".into()],
        vec![
            SXRef::symbol("car".into()),
            vec![
                SXRef::symbol("car".into()),
                SXRef::symbol("x".into()),
            ].into(),
        ].into(),
    ).into();

    let actual = mc_env().get("caar");

    assert_eq!(expected, actual)
}

#[test]
fn cddr() {
    let expected: SXRef = LispFunction::new(
        vec!["x".into()],
        vec![
            SXRef::symbol("cdr".into()),
            vec![
                SXRef::symbol("cdr".into()),
                SXRef::symbol("x".into()),
            ].into(),
        ].into(),
    ).into();

    let actual = mc_env().get("cddr");

    assert_eq!(expected, actual)
}

#[test]
fn cdadr() {
    let expected: SXRef = LispFunction::new(
        vec!["x".into()],
        vec![
            SXRef::symbol("cdr".into()),
            vec![
                SXRef::symbol("car".into()),
                vec![
                    SXRef::symbol("cdr".into()),
                    SXRef::symbol("x".into()),
                ].into(),
            ].into(),
        ].into(),
    ).into();

    let actual = mc_env().get("cdadr");

    assert_eq!(expected, actual)
}

#[test]
fn ccr() {
    let expected = SXRef::nil();

    let actual = mc_env().get("ccdr");

    assert_eq!(expected, actual)
}
