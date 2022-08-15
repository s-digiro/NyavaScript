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
fn deref_on_valid_ptr() {
    let mut env = env();
    let ptr = env.get("object");

    let expected = SXRef::number(4);
    let actual = FunScope::deref(vec![SXRef::clone(&ptr)], &mut env).unwrap();

    assert_eq!(expected, actual);

    let ptr_num = match &*ptr {
        SX::Number(n) => *n,
        _ => panic!("Expected ptr to be a Number(_), but was {:?}", ptr),
    };

    let ptr = SXRef::number(ptr_num + 4);
    let expected = SXRef::number(5);
    let actual = FunScope::deref(vec![SXRef::clone(&ptr)], &mut env).unwrap();

    assert_eq!(expected, actual);

    let ptr_num = match &*ptr {
        SX::Number(n) => *n,
        _ => panic!("Expected ptr to be a Number(_), but was {:?}", ptr),
    };

    let ptr = SXRef::number(ptr_num + 4);
    let expected = SXRef::number(6);
    let actual = FunScope::deref(vec![ptr], &mut env).unwrap();

    assert_eq!(expected, actual);
}
