use super::*;

#[test]
fn nil_is_empty_list() {
    let env = mc_env();

    let actual = env.get("NIL");

    let expected = SXRef::nil();

    assert_eq!(expected, actual);
}
