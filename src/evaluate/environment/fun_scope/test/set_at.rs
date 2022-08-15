use std::path::PathBuf;
use super::*;

fn path() -> PathBuf {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("src/evaluate/environment/dyn_c_lib_scope/dyn_c_lib/test/c_lib/libtest.so");

    d
}

fn env<'a>() -> Env<'a> {
    let path = path();
    let path = path.to_str().unwrap();

    let mut ret = Env::new();

    FunScope::cload(
        vec![SXRef::string(path.into())],
        &mut ret,
    ).unwrap();

    ret
}

#[test]
fn set_at_on_valid_ptr() {
    let mut env = env();
    let ptr = env.get("object");

    panic!("FAIL");
}
