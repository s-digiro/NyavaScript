use super::*;
use super::DynCObjectError;

#[test]
fn get_symbol_object() {
    let subject = test_lib();
    let actual = subject.get_sym("object");
    let o = actual.unwrap();

    let o = match o {
        DynCType::Object(o) => o,
        _ => panic!("Expected to match DynCType::Object(_), found {:?}", o),
    };

    let slice = o.get();

    assert_eq!(1, slice[0]);
    assert_eq!(2, slice[4]);
    assert_eq!(3, slice[8]);

    o.set(&[4, 0, 0, 0, 5, 0, 0, 0, 6, 0, 0, 0]).unwrap();

    assert_eq!(4, slice[0]);
    assert_eq!(5, slice[4]);
    assert_eq!(6, slice[8]);

    let e = o.set(&[4, 0, 0, 0, 5, 0, 0, 0, 6, 0, 0, 0, 0]).err().unwrap();

    assert_eq!(e, DynCObjectError::BufferOverflow { buflen: 12, datalen: 13 });
}

