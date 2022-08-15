use super::*;

#[test]
fn get_symbol_object() {
    let subject = test_lib();
    let actual = subject.get_sym("object");
    let o = actual.unwrap();

    let o = match o {
        DynCType::USize(u) => u,
        _ => panic!("Expected to match DynCType::Usize(_), found {:?}", o),
    };

    let o = o as *mut i32;

    let first;
    let second;
    let third;

    unsafe {
        first = o;
        second = o.offset(1);
        third = o.offset(2);

        assert_eq!(1, *first);
        assert_eq!(2, *second);
        assert_eq!(3, *third);
    }

    unsafe {
        *first = 4;
        *second = 5;
        *third = 6;
    }

    unsafe {
        assert_eq!(4, *first);
        assert_eq!(5, *second);
        assert_eq!(6, *third);
    }
}

