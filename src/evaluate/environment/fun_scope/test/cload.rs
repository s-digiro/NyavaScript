use std::path::PathBuf;
use super::*;

fn path() -> PathBuf {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("src/evaluate/environment/dyn_c_lib_scope/dyn_c_lib/test/c_lib/libtest.so");

    d
}

#[test]
pub fn cload_can_load_test_lib() {
    let path = path();
    let path = path.to_str().unwrap();

    let mut env = Env::new();

    let actual = FunScope::cload(
        vec![SXRef::string(path.into())],
        &mut env,
    ).unwrap();

    assert_eq!(
        SXRef::nil(),
        actual,
    );

    assert_eq!(
        1,
        env.dynclib.len(),
    );
}

#[test]
pub fn cload_fails_gracefully_when_loading_invalid_lib() {
    let path = "foo";

    let mut env = Env::new();

    let actual = FunScope::cload(
        vec![SXRef::string(path.into())],
        &mut env,
    ).unwrap();

    assert_eq!(
        SXRef::number(-1),
        actual,
    );

    assert_eq!(
        0,
        env.dynclib.len(),
    );
}
