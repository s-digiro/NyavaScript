use crate::evaluate::Environment as Env;
use super::*;

#[test]
fn env_can_get_bindings_defined_in_scope_higher_than_current() {
    let mut scope1 = HashMap::new();
    scope1.insert("foo".to_owned(), SXRef::number(1));

    let mut scope2 = HashMap::new();
    scope2.insert("bar".to_owned(), SXRef::number(2));

    let mut env = Env::new();
    env.push(scope1);
    env.push(scope2);

    let expected = SXRef::number(1);

    let actual = env.get("foo");

    assert_eq!(expected, actual);
}

#[test]
fn env_can_shadow_already_defined_bindings() {
    let mut scope1 = HashMap::new();
    scope1.insert("foo".to_owned(), SXRef::number(1));

    let mut scope2 = HashMap::new();
    scope2.insert("foo".to_owned(), SXRef::number(2));

    let mut env = Env::new();
    env.push(scope1);
    env.push(scope2);

    let expected = SXRef::number(2);

    let actual = env.get("foo");

    assert_eq!(expected, actual);
}

#[test]
fn env_can_get_bindings_defined_in_current_scope() {
    let mut scope1 = HashMap::new();
    scope1.insert("foo".into(), SXRef::number(1));

    let mut env = Env::new();
    env.push(scope1);

    let expected = SXRef::number(1);

    let actual = env.get("foo");

    assert_eq!(expected, actual);
}

#[test]
fn env_returns_nil_for_undefined_bindings() {
    let mut scope1 = HashMap::new();
    scope1.insert("foo".into(), SXRef::number(1));

    let mut env = Env::new();
    env.push(scope1);

    let expected = SXRef::nil();

    let actual = env.get("bar");

    assert_eq!(expected, actual);
}

#[test]
fn env_can_define_global_vars() {
    let mut scope1 = HashMap::new();
    scope1.insert("foo".into(), SXRef::number(1));

    let mut env = Env::new();
    env.push(scope1);

    let expected = SXRef::nil();

    let actual = env.get("bar");

    assert_eq!(expected, actual);
}
