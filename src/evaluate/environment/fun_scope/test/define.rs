use super::*;

#[test]
pub fn define_can_define_a_number() {
    let mut env = Env::new();

    let subject = SXRef::from(vec![
        SXRef::symbol("define".into()),
        SXRef::symbol("foo".into()),
        SXRef::number(3),
    ]);

    let expected = SXRef::nil();
    let actual = FunScope::define(subject, &mut env).unwrap();

    assert_eq!(expected, actual);

    let expected = SXRef::number(3);
    let actual = env.get("foo");
    println!("{:?}", env);
    assert_eq!(expected, actual);
}
