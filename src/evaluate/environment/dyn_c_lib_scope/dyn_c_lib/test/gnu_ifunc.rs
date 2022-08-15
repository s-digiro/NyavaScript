use super::*;

#[test]
fn get_ifunc() {
    let subject = test_lib();

    let actual = subject.get_sym("ifunc");

    let f = actual.unwrap();

    let f = match f {
        DynCType::Func(f) => f,
        _ => panic!(
            "Expected to match DynCType::Function(_), found {:?}",
            f
        ),
    };

    // Make sure below does not panic
    f.call(Vec::new());
}
