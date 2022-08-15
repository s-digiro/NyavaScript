use std::path::PathBuf;
use super::*;

mod func;
mod gnu_ifunc;
mod object;

fn lib_folder_path() -> PathBuf {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("src/evaluate/environment/dyn_c_lib_scope/dyn_c_lib/test/c_lib/");

    d
}

fn lib_path() -> PathBuf {
    let mut d = lib_folder_path();
    d.push("libtest.so");

    d
}

fn makefile_path() -> PathBuf {
    let mut d = lib_folder_path();
    d.push("Makefile");

    d
}

fn test_lib() -> DynCLib {
    DynCLib::dlopen(lib_path().to_str().unwrap()).unwrap()
}

#[test]
fn test_lib_exists() {
    let path = lib_path();

    assert!(path.exists());
}

#[test]
fn load_valid_lib() {
    let subject = lib_path();

    let actual = DynCLib::dlopen(subject.to_str().unwrap());

    actual.unwrap();
}

#[test]
fn load_malformed_path() {
    let subject = "foo";

    let actual = DynCLib::dlopen(subject);

    assert!(
        matches!(actual, Err(Error::Load(_))),
        "Expected actual to match Error::Load(_). Got {:?}",
        actual,
    );
}

#[test]
fn load_path_to_non_lib() {
    let subject = makefile_path();

    let actual = DynCLib::dlopen(subject.to_str().unwrap());

    assert!(
        matches!(actual, Err(Error::Load(_))),
        "Expected actual to match Error::Load(_). Got {:?}",
        actual,
    );
}

#[test]
fn get_non_existant_symbol() {
    let subject = test_lib();

    let actual = subject.get_sym("foo");

    assert!(
        matches!(actual, Err(Error::GetSym(_))),
        "Expected actual to match Error::GetSym(_). Got {:?}",
        actual
    );
}

#[test]
#[should_panic]
fn get_symbol_no_type() {
    let subject = test_lib();
    let actual = subject.get_sym("undefined");
    actual.unwrap();
}

#[test]
#[should_panic]
fn get_symbol_file() {
    let subject = test_lib();
    let actual = subject.get_sym("lib.c");
    actual.unwrap();
}

#[test]
#[should_panic]
fn get_symbol_tls() {
    let subject = test_lib();
    let actual = subject.get_sym("tls");
    actual.unwrap();
}

#[test]
fn get_symbol_gnu_ifunc() {
    let subject = test_lib();
    let actual = subject.get_sym("ifunc");
    actual.unwrap();
}

