use super::*;

#[test]
fn t_is_1() {
    let env = mc_env();

    let expected = SXRef::number(1);

    let actual = env.get("T");

    assert_eq!(expected, actual);
}
