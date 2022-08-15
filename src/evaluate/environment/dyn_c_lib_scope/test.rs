use crate::s_expression::SExpression as SX;
use std::path::PathBuf;
use super::*;

fn path() -> PathBuf {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("src/evaluate/environment/dyn_c_lib_scope/dyn_c_lib/test/c_lib/libtest.so");

    d
}

fn lib() -> DynCLibScope {
    DynCLibScope::load(path().to_str().unwrap()).unwrap()
}

#[test]
fn lib_loads() {
    lib();
}

#[test]
fn lib_can_get_func() {
    let lib = lib();
    let actual = lib.get("func").unwrap();
    let actual = &*actual;
    assert!(
        matches!(actual, SX::Function(_)),
        "Expected an SX::Function, but got {:?}",
        actual,
    );
}

#[test]
fn lib_can_get_ifunc() {
    let lib = lib();
    let actual = lib.get("ifunc").unwrap();
    let actual = &*actual;
    assert!(
        matches!(actual, SX::Function(_)),
        "Expected an SX::Function, but got {:?}",
        actual,
    );
}

#[test]
fn lib_can_get_obj() {
    let lib = lib();
    let actual = lib.get("object").unwrap();
    let actual = &*actual;
    assert!(
        matches!(actual, SX::Function(_)),
        "Expected an SX::Function, but got {:?}",
        actual,
    );
}
